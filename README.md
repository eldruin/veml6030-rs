# Rust VEML6030 High Accuracy Ambient Light Sensor Driver

<!-- TODO
[![crates.io](https://img.shields.io/crates/v/veml6030.svg)](https://crates.io/crates/veml6030)
[![Docs](https://docs.rs/veml6030/badge.svg)](https://docs.rs/veml6030)
-->
[![Build Status](https://travis-ci.org/eldruin/veml6030-rs.svg?branch=master)](https://travis-ci.org/eldruin/veml6030-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/veml6030-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/veml6030-rs?branch=master)

This is a platform agnostic Rust driver for the VEML6030 high accuracy ambient
light sensor using the [`embedded-hal`] traits.

<!-- TODO
This driver allows you to:
-->
<!-- TODO
[Introductory blog post]()
-->

## The device

VEML6030 is a high accuracy ambient light digital 16-bit resolution sensor in
a miniature transparent 2mm x 2mm package. It includes a high sensitive
photodiode, a low noise amplifier, a 16-bit A/D converter and supports an easy
to use I2C bus communication interface and additional interrupt feature.
The ambient light result is as digital value available.

Datasheet:
- [VEML6030](https://www.vishay.com/docs/84366/veml6030.pdf)

Application Note:
- [Designing the VEML6030 into an application](https://www.vishay.com/docs/84367/designingveml6030.pdf)

<!-- TODO
## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples
```rust
```
-->

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
