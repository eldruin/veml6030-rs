# Rust VEML6030/VEML7700 High Accuracy Ambient Light Sensor Driver

[![crates.io](https://img.shields.io/crates/v/veml6030.svg)](https://crates.io/crates/veml6030)
[![Docs](https://docs.rs/veml6030/badge.svg)](https://docs.rs/veml6030)
[![Build Status](https://github.com/eldruin/veml6030-rs/workflows/Build/badge.svg)](https://github.com/eldruin/veml6030-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/veml6030-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/veml6030-rs?branch=master)

This is a platform agnostic Rust driver for the VEML6030 and VEML7700 high accuracy ambient
light sensors using the [`embedded-hal`] traits.

This driver allows you to:
- Enable/disable the device. See: `enable()`.
- Read the measured lux value. See: `read_lux()`.
- Read the white channel measurement. See: `read_white()`.
- Read the measured ALS value in raw format. See: `read_raw()`.
- Calculate the compensated lux for a raw ALS value. See: `convert_raw_als_to_lux()`.
- Set the gain. See: `set_gain()`.
- Set the integration time. See: `set_integration_time()`.
- Set the fault count. See: `set_fault_count()`.
- Enable/disable and configure power saving mode. See: `enable_power_saving()`.
- Enable/disable interrupts. See: `enable_interrupts()`.
- Read the interrupt status. See: `read_interrupt_status()`.
- Set the high/low thresholds in lux or raw. See: `set_high_threshold_lux()`.
- Calculate the compensated raw threshold value ahead of time. See: `calculate_raw_threshold_value()`.

[Introductory blog post](https://blog.eldruin.com/veml6030-ambient-light-sensor-driver-in-rust/)

## The devices

Vishay's VEML6030 and VEML7700 are high accuracy ambient light digital 16-bit
resolution sensors in a miniature transparent package. They include
a high sensitive photodiode, a low noise amplifier, a 16-bit A/D converter
and support an easy to use I2C bus communication interface and additional
interrupt feature.
The ambient light result is as digital value available.

Datasheets: [VEML6030](https://www.vishay.com/docs/84366/veml6030.pdf) - [VEML7700](https://www.vishay.com/docs/84286/veml7700.pdf)

Application Notes:
- [Designing the VEML6030 into an application](https://www.vishay.com/docs/84367/designingveml6030.pdf)
- [Designing the VEML7700 into an application](https://www.vishay.com/docs/84323/designingveml7700.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

VEML6030 and VEML7700 expose the same interface over I2C. To communicate with a VEML7700
simply use this driver as if communicating with a VEML6030.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
use linux_embedded_hal::I2cdev;
use veml6030::{SlaveAddr, Veml6030};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut sensor = Veml6030::new(dev, address);
    sensor.enable().unwrap();
    loop {
        let lux = sensor.read_lux().unwrap();
        println!("lux: {:2}", lux);
    }
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/veml6030-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
