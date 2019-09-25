//! This is a platform agnostic Rust driver for the VEML6030 high-accuracy
//! ambient light sensor using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - TODO
//!
//! [Introductory blog post](TODO)
//!
//! ## The devices
//!
//!VEML6030 is a high accuracy ambient light digital 16-bit resolution sensor in
//! a miniature transparent 2mm x 2mm package. It includes a high sensitive
//! photodiode, a low noise amplifier, a 16-bit A/D converter and supports an easy
//! to use I2C bus communication interface and additional interrupt feature.
//! The ambient light result is as digital value available.
//!
//! Datasheet:
//! - [VEML6030](https://www.vishay.com/docs/84366/veml6030.pdf)
//!
//! Application Note:
//! - [Designing the VEML6030 into an application](https://www.vishay.com/docs/84367/designingveml6030.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!

#![deny(unsafe_code, missing_docs)]
#![no_std]

extern crate embedded_hal as hal;

mod device_impl;
mod types;
pub use types::{Error, IntegrationTime, SlaveAddr};

/// VEML6030 device driver
#[derive(Debug)]
pub struct Veml6030<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    address: u8,
    config: Config,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Config {
    bits: u16,
}
