# Rust VEML6030 High Accuracy Ambient Light Sensor Driver

<!-- TODO
[![crates.io](https://img.shields.io/crates/v/veml6030.svg)](https://crates.io/crates/veml6030)
[![Docs](https://docs.rs/veml6030/badge.svg)](https://docs.rs/veml6030)
-->
[![Build Status](https://travis-ci.org/eldruin/veml6030-rs.svg?branch=master)](https://travis-ci.org/eldruin/veml6030-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/veml6030-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/veml6030-rs?branch=master)

This is a platform agnostic Rust driver for the VEML6030 high accuracy ambient
light sensor using the [`embedded-hal`] traits.

This driver allows you to:
- Enable/disable the device. See: `enable()`.
- Read the measured lux value. See: `read_lux()`.
- Read the white channel measurment. See: `read_white()`.
- Set the gain. See: `set_gain()`.
- Set the integration time. See: `set_integration_time()`.
- Set the fault count. See: `set_fault_count()`.
- Enable/disable and configure power saving mode. See: `enable_power_saving()`.
- Enable/disable interrupts. See: `enable_interrupts()`.
- Read the interrupt status. See: `read_interrupt_status()`.
- Set the high/low thresholds in lux or raw. See: `set_high_threshold_lux()`.
- Calculate the compensated raw threshold value ahead of time. See: `calculate_raw_threshold_value()`.

<!-- TODO
[Introductory blog post]()
-->

## The device

Vishay's VEML6030 is a high accuracy ambient light digital 16-bit
resolution sensor in a miniature transparent 2mm x 2mm package. It includes
a high sensitive photodiode, a low noise amplifier, a 16-bit A/D converter
and supports an easy to use I2C bus communication interface and additional
interrupt feature.
The ambient light result is as digital value available.

Datasheet:
- [VEML6030](https://www.vishay.com/docs/84366/veml6030.pdf)

Application Note:
- [Designing the VEML6030 into an application](https://www.vishay.com/docs/84367/designingveml6030.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
extern crate linux_embedded_hal as hal;
extern crate veml6030;
use veml6030::{SlaveAddr, Veml6030};

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
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
