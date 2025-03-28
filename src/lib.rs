#![cfg_attr(not(test), no_std)]

#[allow(dead_code)]
mod constants;
pub mod i2c;

/// Temperature/pressure oversampling
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oversampling {
    /// 1x oversampling, 0.78 Pa RMS noise, max rate 498 Hz
    Oversampling1X,
    /// 2x oversampling, 0.58 Pa RMS noise, max rate 374 Hz
    Oversampling2X,
    /// 4x oversampling, 0.41 Pa RMS noise, max rate 255 Hz
    Oversampling4X,
    /// 8x oversampling, 0.30 Pa RMS noise, max rate 155 Hz
    Oversampling8X,
    /// 16x oversampling, 0.21 Pa RMS noise, max rate 87 Hz
    Oversampling16x,
    /// 32x oversampling, 0.15 Pa RMS noise, max rate 46 Hz
    Oversampling32x,
    /// 64x oversampling, 0.11 Pa RMS noise, max rate 24 Hz
    Oversampling64x,
    /// 128x oversampling, 0.08 Pa RMS noise, max rate 12s Hz
    Oversampling128x,
}

impl Default for Oversampling {
    fn default() -> Self {
        Self::Oversampling1X
    }
}

impl From<Oversampling> for u8 {
    fn from(oversampling: Oversampling) -> u8 {
        match oversampling {
            Oversampling::Oversampling1X => constants::BMP5_OSR_1X,
            Oversampling::Oversampling2X => constants::BMP5_OSR_2X,
            Oversampling::Oversampling4X => constants::BMP5_OSR_4X,
            Oversampling::Oversampling8X => constants::BMP5_OSR_8X,
            Oversampling::Oversampling16x => constants::BMP5_OSR_16X,
            Oversampling::Oversampling32x => constants::BMP5_OSR_32X,
            Oversampling::Oversampling64x => constants::BMP5_OSR_64X,
            Oversampling::Oversampling128x => constants::BMP5_OSR_128X,
        }
    }
}

/// IIR low-pass filter
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IIRFilter {
    /// No filtering
    Bypass,
    /// -3dB cutoff @ 0.1147 Hz
    Coeff1,
    /// -3dB cutoff @ 0.0459 Hz
    Coeff3,
    /// -3dB cutoff @ 0.0212 Hz
    Coeff7,
    /// -3dB cutoff @ 0.01025 Hz
    Coeff15,
    /// -3dB cutoff @ 0.00504 Hz
    Coeff31,
    /// -3dB cutoff @ 0.00250 Hz
    Coeff63,
    /// -3dB cutoff @ 0.00125 Hz
    Coeff127,
}

impl From<IIRFilter> for u8 {
    fn from(coeff: IIRFilter) -> u8 {
        match coeff {
            IIRFilter::Bypass => constants::BMP5_IIR_FILTER_BYPASS,
            IIRFilter::Coeff1 => constants::BMP5_IIR_FILTER_COEFF_1,
            IIRFilter::Coeff3 => constants::BMP5_IIR_FILTER_COEFF_3,
            IIRFilter::Coeff7 => constants::BMP5_IIR_FILTER_COEFF_7,
            IIRFilter::Coeff15 => constants::BMP5_IIR_FILTER_COEFF_15,
            IIRFilter::Coeff31 => constants::BMP5_IIR_FILTER_COEFF_31,
            IIRFilter::Coeff63 => constants::BMP5_IIR_FILTER_COEFF_63,
            IIRFilter::Coeff127 => constants::BMP5_IIR_FILTER_COEFF_127,
        }
    }
}

impl Default for IIRFilter {
    fn default() -> Self {
        Self::Bypass
    }
}

/// Output data rate
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutputDataRate {
    /// 0.125 Hz
    OutputDataRate0_125Hz,
    /// 0.250 Hz
    OutputDataRate0_250Hz,
    /// 0.5 Hz
    OutputDataRate0_5Hz,
    /// 1 Hz
    OutputDataRate1Hz,
    /// 2 Hz
    OutputDataRate2Hz,
    /// 3 Hz
    OutputDataRate3Hz,
    /// 4 Hz
    OutputDataRate4Hz,
    /// 5 Hz
    OutputDataRate5Hz,
    /// 10 Hz
    OutputDataRate10Hz,
    /// 15 Hz
    OutputDataRate15Hz,
    /// 20 Hz
    OutputDataRate20Hz,
    /// 25 Hz
    OutputDataRate25Hz,
    /// 30 Hz
    OutputDataRate30Hz,
    /// 35 Hz
    OutputDataRate35Hz,
    /// 40 Hz
    OutputDataRate40Hz,
    /// 45 Hz
    OutputDataRate45Hz,
    /// 50 Hz
    OutputDataRate50Hz,
    /// 60 Hz
    OutputDataRate60Hz,
    /// 70 Hz
    OutputDataRate70Hz,
    /// 80 Hz
    OutputDataRate80Hz,
    /// 89.6 Hz
    OutputDataRate89_6Hz,
    /// 100.2 Hz
    OutputDataRate100_2Hz,
    /// 110.1 Hz
    OutputDataRate110_1Hz,
    /// 120 Hz
    OutputDataRate120Hz,
    /// 129.8 Hz
    OutputDataRate129_8Hz,
    /// 140 Hz
    OutputDataRate140Hz,
    /// 149.3 Hz
    OutputDataRate149_3Hz,
    /// 160 Hz
    OutputDataRate160Hz,
    /// 179.2 Hz
    OutputDataRate179_2Hz,
    /// 199.1 Hz
    OutputDataRate199_1Hz,
    /// 218.5 Hz
    OutputDataRate218_5Hz,
    /// 240 Hz
    OutputDataRate240Hz,
}

impl From<OutputDataRate> for u8 {
    fn from(output_data_rate: OutputDataRate) -> u8 {
        match output_data_rate {
            OutputDataRate::OutputDataRate0_125Hz => constants::BMP5_ODR_0_125_HZ,
            OutputDataRate::OutputDataRate0_250Hz => constants::BMP5_ODR_0_250_HZ,
            OutputDataRate::OutputDataRate0_5Hz => constants::BMP5_ODR_0_5_HZ,
            OutputDataRate::OutputDataRate1Hz => constants::BMP5_ODR_01_HZ,
            OutputDataRate::OutputDataRate2Hz => constants::BMP5_ODR_02_HZ,
            OutputDataRate::OutputDataRate3Hz => constants::BMP5_ODR_03_HZ,
            OutputDataRate::OutputDataRate4Hz => constants::BMP5_ODR_04_HZ,
            OutputDataRate::OutputDataRate5Hz => constants::BMP5_ODR_05_HZ,
            OutputDataRate::OutputDataRate10Hz => constants::BMP5_ODR_10_HZ,
            OutputDataRate::OutputDataRate15Hz => constants::BMP5_ODR_15_HZ,
            OutputDataRate::OutputDataRate20Hz => constants::BMP5_ODR_20_HZ,
            OutputDataRate::OutputDataRate25Hz => constants::BMP5_ODR_25_HZ,
            OutputDataRate::OutputDataRate30Hz => constants::BMP5_ODR_30_HZ,
            OutputDataRate::OutputDataRate35Hz => constants::BMP5_ODR_35_HZ,
            OutputDataRate::OutputDataRate40Hz => constants::BMP5_ODR_40_HZ,
            OutputDataRate::OutputDataRate45Hz => constants::BMP5_ODR_45_HZ,
            OutputDataRate::OutputDataRate50Hz => constants::BMP5_ODR_50_HZ,
            OutputDataRate::OutputDataRate60Hz => constants::BMP5_ODR_60_HZ,
            OutputDataRate::OutputDataRate70Hz => constants::BMP5_ODR_70_HZ,
            OutputDataRate::OutputDataRate80Hz => constants::BMP5_ODR_80_HZ,
            OutputDataRate::OutputDataRate89_6Hz => constants::BMP5_ODR_89_6_HZ,
            OutputDataRate::OutputDataRate100_2Hz => constants::BMP5_ODR_100_2_HZ,
            OutputDataRate::OutputDataRate110_1Hz => constants::BMP5_ODR_110_1_HZ,
            OutputDataRate::OutputDataRate120Hz => constants::BMP5_ODR_120_HZ,
            OutputDataRate::OutputDataRate129_8Hz => constants::BMP5_ODR_129_8_HZ,
            OutputDataRate::OutputDataRate140Hz => constants::BMP5_ODR_140_HZ,
            OutputDataRate::OutputDataRate149_3Hz => constants::BMP5_ODR_149_3_HZ,
            OutputDataRate::OutputDataRate160Hz => constants::BMP5_ODR_160_HZ,
            OutputDataRate::OutputDataRate179_2Hz => constants::BMP5_ODR_179_2_HZ,
            OutputDataRate::OutputDataRate199_1Hz => constants::BMP5_ODR_199_1_HZ,
            OutputDataRate::OutputDataRate218_5Hz => constants::BMP5_ODR_218_5_HZ,
            OutputDataRate::OutputDataRate240Hz => constants::BMP5_ODR_240_HZ,
        }
    }
}

impl Default for OutputDataRate {
    fn default() -> Self {
        Self::OutputDataRate25Hz
    }
}

/// Configuration
#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// Temperature oversampling
    pub temperature_oversampling: Oversampling,
    /// Temperature IIR low-pass filter
    pub temperature_iir_filter: IIRFilter,
    /// Pressure oversampling
    pub pressure_oversampling: Oversampling,
    /// Pressure IIR low-pass filter
    pub pressure_iir_filter: IIRFilter,
    /// Output data rate
    pub output_data_rate: OutputDataRate,
}

/// Measurement data
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Measurement {
    /// Temperature in degrees Celsius
    pub temperature: f32,
    /// Pressure in Pascals
    pub pressure: f32,
}
