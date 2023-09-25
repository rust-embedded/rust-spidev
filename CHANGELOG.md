# Spidev Crate Changelog

## Not yet released

[Full Changelog](https://github.com/rust-embedded/rust-spidev/compare/0.6.0...HEAD)

- Added support for querying the configuration of a SPI device.

## 0.6.0 / 2023-08-03

[Full Changelog](https://github.com/rust-embedded/rust-spidev/compare/0.5.2...0.6.0)

- Nix updated to 0.26
- bitflags updated to 2.3
- Minimum Supported Rust Version is now 1.56.1

## 0.5.2 / 2023-08-02

[Full Changelog](https://github.com/rust-embedded/rust-spidev/compare/0.5.1...0.5.2)

- Added support for delay transactions.

## 0.5.1 / 2021-11-22

[Full Changelog](https://github.com/rust-embedded/rust-spidev/compare/0.5.0...0.5.1)

- Implement `AsRawFd` for `Spidev` to allow access to the underlying file descriptor.
- Updated nix to version `0.23`.

## 0.5.0 / 2021-09-21

[Full Changelog](https://github.com/rust-embedded/rust-spidev/compare/0.4.1...0.5.0)

- Added `Debug` implementation for `Spidev`
- Added `Debug`, `Default`, `Copy` and `PartialEq` implementations for `SpidevOptions`
- Nix bumped to 0.22
- bitflags updated to 1.3
- Minimum supported rust version is now 1.46.0

## 0.4.1 / 2021-02-21

[Full Changelog](https://github.com/rust-embedded/rust-spidev/compare/0.4.0...0.4.1)

- Support Rust 2018 edition
- Minimum supported rust version is now 1.31.0

## 0.4.0 / 2019-05-29

[Full Changelog](https://github.com/rust-embedded/rust-spidev/compare/0.3.0...0.4.0)

- Bitflags dep bumped to 1.0
- Nix bumped to 0.14
- Minimum supported rust version is now 1.26.0
- APIs added to expose underlying file object ([#13](https://github.com/rust-embedded/rust-spidev/pull/13)).

## 0.3.0 / 2016-10-26

[Full Changelog](https://github.com/rust-embedded/rust-spidev/compare/0.2.1...0.3.0)

- The older supported version of rustc for this release is 1.7.0
- Bump to nix 0.6.0
- [transfer_multiple](http://posborne.github.io/rust-spidev/spidev/struct.Spidev.html#method.transfer_multiple) now
  receives an `IntoIterator` rather than requiring that a Vec reference be
  provided. ([#7](https://github.com/rust-embedded/rust-spidev/pull/7))
- [transfer_multiple](http://posborne.github.io/rust-spidev/spidev/struct.Spidev.html#method.transfer_multiple) no
  longer performs heap allocations internally (nor does it require heap
  allocations to be used). ([#8](https://github.com/rust-embedded/rust-spidev/pull/8))

## 0.2.1 / 2016-4-12

[Full Changelog](https://github.com/posborne/rust-spidev/compare/0.2.0...0.2.1)

- Bump to newer version of nix to support older versions of rust

## 0.2.0 / 2015-12-10

[Full Changelog](https://github.com/posborne/rust-spidev/compare/0.1.0...0.2.0)

- Miscellaneous non-functional code changes
- Updates to work with upstream versions of nix and other libraries
- Minor API changes and testing improvements

## 0.1.0 / 2015-05-19

[Full Changelog](https://github.com/posborne/rust-spidev/compare/0baf4916a6276315e28aef6a8508b10f8d35276f...0.1.0)

Initial release of the library with all major planned features, tested
on nightlies and on 1.0.0.  Major features supported include the
following:

- Support for opening and configuring a SPI device
- Support for performing half-duplex reads/writes
- Support for performing single full-duplex SPI transfers
- Support for performing multiple chained SPI transfers
- Support for configuring settings for each SPI transfer individually
