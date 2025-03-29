use crate::{Config, Measurement, constants};
use byteorder::{ByteOrder, LittleEndian};
use embedded_hal_async::i2c::I2c;

// I2C address
pub const BMP5_ADDRESS: u8 = 0x47; // SDO to GND
pub const BMP5_ADDRESS_ALT: u8 = 0x46; // SDO to VDDIO

/// BMP5 driver error
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E> {
    I2c(E),
    InvalidChipId(u8),
    InvalidConfig,
}

/// BMP5 driver
pub struct Bmp5<I2C, D> {
    i2c: I2C,
    delay: D,
    address: u8,
    config: Config,
}

impl<I2C, D, E> Bmp5<I2C, D>
where
    I2C: I2c<Error = E>,
    D: embedded_hal_async::delay::DelayNs,
{
    pub fn new(i2c: I2C, delay: D, address: u8, config: Config) -> Self {
        Self {
            i2c,
            delay,
            address,
            config,
        }
    }

    pub async fn init(&mut self) -> Result<(), Error<E>> {
        self.soft_reset().await?;
        self.verify_chip_id().await?;
        self.configure().await?;
        Ok(())
    }

    pub async fn measure(&mut self) -> Result<Measurement, Error<E>> {
        // Wait for the next sample to be ready.
        self.wait_for_drdy().await?;

        // Read in the measurement.
        let mut buf = [0u8; 6];
        self.i2c
            .write_read(
                self.address,
                &[constants::BMP5_REG_TEMP_DATA_XLSB],
                &mut buf,
            )
            .await
            .map_err(Error::I2c)?;

        let raw_temp = LittleEndian::read_i24(&buf);
        let temp_c = raw_temp as f32 / 65536.0;

        let raw_press = LittleEndian::read_i24(&buf[3..]);
        let press_pa = raw_press as f32 / 64.0;

        Ok(Measurement {
            temperature: temp_c,
            pressure: press_pa,
        })
    }

    pub async fn soft_reset(&mut self) -> Result<(), Error<E>> {
        self.write_reg(constants::BMP5_REG_CMD, constants::BMP5_CMD_SOFT_RESET)
            .await?;
        self.delay.delay_us(2500).await;
        Ok(())
    }

    async fn verify_chip_id(&mut self) -> Result<(), Error<E>> {
        let chip_id = self.read_reg(constants::BMP5_REG_CHIP_ID).await?;
        if chip_id != constants::BMP5_CHIP_ID && chip_id != constants::BMP5_CHIP_ID_ALT {
            return Err(Error::InvalidChipId(chip_id));
        }
        Ok(())
    }

    async fn configure(&mut self) -> Result<(), Error<E>> {
        // Enter standby mode
        let odr_config = constants::BMP5_ODR_DEEPSLEEP_DIS | constants::BMP5_MODE_STANDBY;
        self.update_reg(
            constants::BMP5_REG_ODR_CONFIG,
            constants::BMP5_ODR_DEEPSLEEP_DIS | constants::BMP5_MODE_MASK,
            odr_config,
        )
        .await?;

        self.delay.delay_us(2500).await;

        // Configure oversampling for pressure and temperature
        let osr_config: u8 = constants::BMP5_OSR_PRESS_EN
            | (u8::from(self.config.pressure_oversampling) << 3)
            | u8::from(self.config.temperature_oversampling);

        self.update_reg(
            constants::BMP5_REG_OSR_CONFIG,
            constants::BMP5_OSR_MASK | constants::BMP5_OSR_PRESS_EN,
            osr_config,
        )
        .await?;

        // Configure IIR low-pass filter
        let iir_config: u8 = (u8::from(self.config.pressure_iir_filter) << 3)
            | u8::from(self.config.temperature_iir_filter);
        self.write_reg(constants::BMP5_REG_DSP_IIR, iir_config)
            .await?;

        // Set output data rate (25 Hz)
        self.update_reg(
            constants::BMP5_REG_ODR_CONFIG,
            constants::BMP5_ODR_MASK,
            self.config.output_data_rate.into(),
        )
        .await?;

        // Enable data ready interrupt
        self.update_reg(
            constants::BMP5_REG_INT_SOURCE,
            constants::BMP5_INT_ENABLE_DRDY,
            constants::BMP5_INT_ENABLE_DRDY,
        )
        .await?;

        // Return to normal operation mode
        self.update_reg(
            constants::BMP5_REG_ODR_CONFIG,
            constants::BMP5_MODE_MASK,
            constants::BMP5_MODE_NORMAL,
        )
        .await?;

        // Check if ODR and OSR settings are valid or we are
        // operating in a degraded mode
        let osr_eff = self.read_reg(constants::BMP5_REG_OSR_EFF).await?;
        if osr_eff & constants::BMP5_OSR_EFF_VALID_ODR == 0 {
            return Err(Error::InvalidConfig);
        }

        Ok(())
    }

    async fn wait_for_drdy(&mut self) -> Result<(), Error<E>> {
        loop {
            let int_status = self.read_reg(constants::BMP5_REG_INT_STATUS).await?;
            if int_status & constants::BMP5_INT_STATUS_DRDY != 0 {
                break;
            }
            self.delay
                .delay_us(self.config.output_data_rate.period_us() / 4)
                .await;
        }
        Ok(())
    }

    async fn update_reg(&mut self, reg: u8, mask: u8, value: u8) -> Result<(), Error<E>> {
        let current = self.read_reg(reg).await?;
        let new = (current & !mask) | (value & mask);
        self.write_reg(reg, new).await
    }

    async fn read_reg(&mut self, reg: u8) -> Result<u8, Error<E>> {
        let mut buf = [0u8];
        self.i2c
            .write_read(self.address, &[reg], &mut buf)
            .await
            .map_err(Error::I2c)?;
        Ok(buf[0])
    }

    async fn write_reg(&mut self, reg: u8, value: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[reg, value])
            .await
            .map_err(Error::I2c)
    }
}
