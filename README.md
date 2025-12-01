# opt40xx
[![docs.rs](https://docs.rs/opt40xx/badge.svg)](https://docs.rs/opt40xx)
[![crates.io](https://img.shields.io/crates/d/opt40xx.svg)](https://crates.io/crates/opt40xx)
[![crates.io](https://img.shields.io/crates/v/opt40xx.svg)](https://crates.io/crates/opt40xx)

Barebones driver for OPT40xx ambient light sensor family based on [`embedded-hal`] traits.
This sensor family uses _almost the same_ register map for all its members.

Each member differ by:
- Calculation formula of lux values.
- Definition of their result registers (FIFO or different channel or both).
- Output interrupt mechanism.

It's possible to fully support each member individually by introducing member marker generics.
This driver does not track sensor state, it exposes an interface to set whole registers instead, to keep things simple.

Implemented:
- [x] Read the measurement in raw value. See: `read_raw()`.
- [x] Read the conversion status. See: `read_status()`.
- [x] Configure main and secondary configurations. See: `set_config_a()` and `set_config_b()`.
- [x] Read the device ID. See: `read_device_id()`.
- [ ] Set the low and high limits. (Low level side is implemented but not exposed).

## Usage
```rust
use opt40xx::*;

let mut i2c = // i2c bus that implements e-hal trait.
let mut sensor = Sensor::new(i2c, Address::Gnd);

sensor.set_config_a(ConfigA {
    operating_mode: OperatingMode::Continuous,
    conv_time: ConversionTime::Ms25,
    ..Default::default()
})
.unwrap();

loop {
    let measurement = sensor.read_raw(Channel::Ch0, BurstRead::Enabled).unwrap();
    info!("measurement: {:?}, count: {:?}", measurement.0, measurement.1);
}
```

## License

Licensed under either of
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
