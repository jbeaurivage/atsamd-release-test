# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.1](https://github.com/jbeaurivage/atsamd-release-test/compare/grand_central_m4-0.6.0...grand_central_m4-0.6.1) - 2024-09-30

### Other

- Bump usb-device version ([#753](https://github.com/jbeaurivage/atsamd-release-test/pull/753))
- Various small fixes ([#749](https://github.com/jbeaurivage/atsamd-release-test/pull/749))
- Re-organize using a proc-macro to support more devices ([#728](https://github.com/jbeaurivage/atsamd-release-test/pull/728))
# Unreleased

- update path of Cargo config
- update to `atsamd-hal-0.17` and other dependencies (#753)

# v0.6.0

- Use correct alternate for USB (#661)
- update to `atsamd-hal-0.15` (v2 drivers of peripherals and removal of deprecated things)
- correction to the USB clock in the bsp convenience function

# v0.5.0

- update to `atsamd-hal-0.14` and other latest dependencies (#564)
- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)
- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
- move `usbd-x` crates used only in examples to `[dev-dependencies]`
- removed unnecessary dependency on `nb` and `panic_rtt` (#510)

---

Changelog tracking started at v0.4.0
