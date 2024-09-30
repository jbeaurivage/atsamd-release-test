# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.10.1](https://github.com/jbeaurivage/atsamd-release-test/compare/samd11_bare-0.10.0...samd11_bare-0.10.1) - 2024-09-30

### Other

- Update PACS to svd2rust 0.34 ([#756](https://github.com/jbeaurivage/atsamd-release-test/pull/756))
- Various small fixes ([#749](https://github.com/jbeaurivage/atsamd-release-test/pull/749))
# Unreleased

- Upgrade PACs to latest SVD and `svd2rust`:
  - All peripheral types are now `PascalCase`
  - All register field accessors are now methods instead of struct members
  - Members of the `Peripherals` struct are now `snake_case`
- update path of Cargo config

# v0.10.0

- Implement `embedded-hal` `1.0` for GPIO, SPI, I2C, UART and fix examples
- Update the PACs to svd2rust 0.30.2.

# v0.9.0
- Update to `atsamd-hal` version `0.16.0`

# v0.8.1
- Update to `atsamd-hal` version `0.15.1`

# v0.8.0
- Update `lib.rs` and examples to reflect removal of `v1` APIs and promotion of `v2` APIs
- Update `i2c_master` convenience function to use the new `sercom::v2::i2c` API
- Add an `i2c` example

# v0.7.0

- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
* move `usbd-x` crates used only in examples to `[dev-dependencies]`
* Bump `cortex-m`/`cortex-m-rt` dependencies to fix a build issue

---

Changelog tracking started at v0.6.0
