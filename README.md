# Rust Driver For BMP5 Pressure Sensors

An `embedded_hal_async` driver for the BMP5x series of pressure sensors from
Bosch Sensortec.

## Usage

```rust
use defmt::info;
use bmp5::{Config as Bmp5Config, i2c::{Bmp5, BMP5_ADDRESS}};

async {
  // Replace i2c with an implementation of embedded_hal_async::i2c::I2c
  let mut sensor = Bmp5::new(i2c, Delay, BMP5_ADDRESS, Bmp5Config::default());

  // Initialize the sensor.
  sensor.init().await.unwrap();

  // Take a measurement.
  let measuement = sensor.measure().await.unwrap();
  info!("Temperature: {}°C, Pressure: {}Pa", measuement.temperature, measuement.pressure);
}
```