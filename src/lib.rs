//! This is a platform agnostic Rust driver for the VEML6030 and VEML7700 high-accuracy
//! ambient light sensors using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable the device. See: [`enable()`].
//! - Read the measured lux value. See: [`read_lux()`].
//! - Read the white channel measurement. See: [`read_white()`].
//! - Read the measured ALS value in raw format. See: [`read_raw()`].
//! - Calculate the compensated lux for a raw ALS value. See: [`convert_raw_als_to_lux()`].
//! - Set the gain. See: [`set_gain()`].
//! - Set the integration time. See: [`set_integration_time()`].
//! - Set the fault count. See: [`set_fault_count()`].
//! - Enable/disable and configure power saving mode. See: [`enable_power_saving()`].
//! - Enable/disable interrupts. See: [`enable_interrupts()`].
//! - Read the interrupt status. See: [`read_interrupt_status()`].
//! - Set the high/low thresholds in lux or raw. See: [`set_high_threshold_lux()`].
//! - Calculate the compensated raw threshold value ahead of time. See: [`calculate_raw_threshold_value()`].
//!
//! [`enable()`]: struct.Veml6030.html#method.enable
//! [`read_lux()`]: struct.Veml6030.html#method.read_lux
//! [`read_white()`]: struct.Veml6030.html#method.read_white
//! [`read_raw()`]: struct.Veml6030.html#method.read_raw
//! [`convert_raw_als_to_lux()`]: fn.convert_raw_als_to_lux.html
//! [`set_gain()`]: struct.Veml6030.html#method.set_gain
//! [`set_integration_time()`]: struct.Veml6030.html#method.set_integration_time
//! [`set_fault_count()`]: struct.Veml6030.html#method.set_fault_count
//! [`enable_power_saving()`]: struct.Veml6030.html#method.enable_power_saving
//! [`enable_interrupts()`]: struct.Veml6030.html#method.enable_interrupts
//! [`read_interrupt_status()`]: struct.Veml6030.html#method.read_interrupt_status
//! [`set_high_threshold_lux()`]: struct.Veml6030.html#method.set_high_threshold_lux
//! [`calculate_raw_threshold_value()`]: fn.calculate_raw_threshold_value.html
//!
//! [Introductory blog post](https://blog.eldruin.com/veml6030-ambient-light-sensor-driver-in-rust/)
//!
//! ## The devices
//!
//! Vishay's VEML6030 / VEML7700 are high accuracy ambient light digital 16-bit
//! resolution sensors in a miniature transparent package. They include
//! a high sensitive photodiode, a low noise amplifier, a 16-bit A/D converter
//! and support an easy to use I2C bus communication interface and additional
//! interrupt feature.
//! The ambient light result is as digital value available.
//!
//! Datasheets: [VEML6030](https://www.vishay.com/docs/84366/veml6030.pdf) - [VEML7700](https://www.vishay.com/docs/84286/veml7700.pdf)
//!
//! Application Note:
//! - [Designing the VEML6030 into an application](https://www.vishay.com/docs/84367/designingveml6030.pdf)
//! - [Designing the VEML7700 into an application](https://www.vishay.com/docs/84323/designingveml7700.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//!
//! VEML6030 and VEML7700 expose the same interface over I2C. To communicate with a VEML7700
//! simply use this driver as if communicating with a VEML6030.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Read the lux
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use veml6030::{SlaveAddr, Veml6030};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6030::new(dev, SlaveAddr::default());
//! sensor.enable().unwrap();
//! loop {
//!     let lux = sensor.read_lux().unwrap();
//!     println!("lux: {:2}", lux);
//! }
//! ```
//!
//! ### Provide an alternative address
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use veml6030::{SlaveAddr, Veml6030};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::Alternative(true);
//! let mut sensor = Veml6030::new(dev, address);
//! ```
//!
//! ### Set the gain and integration time
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use veml6030::{Gain, IntegrationTime, SlaveAddr, Veml6030};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6030::new(dev, SlaveAddr::default());
//! sensor.set_gain(Gain::OneQuarter).unwrap();
//! sensor.set_integration_time(IntegrationTime::Ms200).unwrap();
//! sensor.enable().unwrap();
//! ```
//!
//! ### Enable power-saving mode
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use veml6030::{PowerSavingMode, SlaveAddr, Veml6030};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6030::new(dev, SlaveAddr::default());
//! sensor.enable_power_saving(PowerSavingMode::One).unwrap();
//! sensor.enable().unwrap();
//! ```
//!
//! ### Set thresholds, fault count and enable interrupts
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use veml6030::{
//!     FaultCount, Gain, IntegrationTime, SlaveAddr, Veml6030
//! };
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6030::new(dev, SlaveAddr::default());
//! sensor.set_gain(Gain::OneQuarter).unwrap();
//! // this will compensate the value automatically before setting it
//! sensor.set_high_threshold_lux(10000.0).unwrap();
//! sensor.set_low_threshold_lux(100.0).unwrap();
//! sensor.set_fault_count(FaultCount::Four).unwrap();
//! sensor.enable_interrupts().unwrap();
//! sensor.enable().unwrap();
//! ```
//!
//! ### Precalculate and set compensated threshold values
//!
//! Using current device configuration
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use veml6030::{Gain, IntegrationTime, SlaveAddr, Veml6030};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6030::new(dev, SlaveAddr::default());
//! sensor.set_gain(Gain::OneEighth).unwrap();
//! sensor.set_integration_time(IntegrationTime::Ms200).unwrap();
//! let high_th_raw = sensor.calculate_raw_threshold_value(10000.0);
//! // ...
//! sensor.set_high_threshold_raw(high_th_raw).unwrap();
//! // this requires no compensation because the value is < 1000
//! sensor.set_low_threshold_lux(100.0).unwrap();
//! ```
//!
//! ### Precalculate and set compensated threshold values
//!
//! Using free function
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use veml6030::{
//!     calculate_raw_threshold_value,
//!     Gain, IntegrationTime, SlaveAddr, Veml6030
//! };
//!
//! let gain = Gain::OneEighth;
//! let it = IntegrationTime::Ms200;
//! let high_th_raw = calculate_raw_threshold_value(it, gain, 10000.0);
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6030::new(dev, SlaveAddr::default());
//! sensor.set_gain(gain).unwrap();
//! sensor.set_integration_time(it).unwrap();
//! // ...
//! sensor.set_high_threshold_raw(high_th_raw).unwrap();
//! // this requires no compensation because the value is < 1000
//! sensor.set_low_threshold_lux(100.0).unwrap();
//! sensor.enable_interrupts().unwrap();
//! sensor.enable().unwrap();
//! ```
//!
//! ### Read interrupt status
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use veml6030::{SlaveAddr, Veml6030};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6030::new(dev, SlaveAddr::default());
//! // ...
//! loop {
//!     let status = sensor.read_interrupt_status().unwrap();
//!     if status.was_too_high {
//!         // ...
//!     }
//!     if status.was_too_low {
//!         // ...
//!     }
//! }
//! ```
//!
//! ### Read the raw ALS measurement and convert to lux separately
//!
//! Using current device configuration
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use veml6030::{SlaveAddr, Veml6030};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6030::new(dev, SlaveAddr::default());
//! sensor.enable().unwrap();
//! loop {
//!     let raw = sensor.read_raw().unwrap();
//!     // ...
//!     let lux = sensor.convert_raw_als_to_lux(raw);
//!     println!("lux: {:2}", lux);
//! }
//! ```
//!
//! ### Read the raw ALS measurement and convert to lux separately
//!
//! Using free function
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use veml6030::{
//!     convert_raw_als_to_lux,
//!     Gain, IntegrationTime, SlaveAddr, Veml6030
//! };
//!
//! let gain = Gain::OneEighth;
//! let it = IntegrationTime::Ms200;
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6030::new(dev, SlaveAddr::default());
//! sensor.set_gain(gain).unwrap();
//! sensor.set_integration_time(it).unwrap();
//! sensor.enable().unwrap();
//! loop {
//!     let raw = sensor.read_raw().unwrap();
//!     // ...
//!     let lux = convert_raw_als_to_lux(it, gain, raw);
//!     println!("lux: {:2}", lux);
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/veml6030/0.1.2")]
#![deny(unsafe_code, missing_docs)]
#![no_std]

mod correction;
mod device_impl;
pub use crate::correction::calculate_raw_threshold_value;
pub use crate::device_impl::convert_raw_als_to_lux;
mod types;
pub use crate::types::{
    Error, FaultCount, Gain, IntegrationTime, InterruptStatus, PowerSavingMode, SlaveAddr,
};

/// VEML6030 device driver
#[derive(Debug)]
pub struct Veml6030<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    address: u8,
    config: Config,
    gain: Gain,
    it: IntegrationTime,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Config {
    bits: u16,
}
