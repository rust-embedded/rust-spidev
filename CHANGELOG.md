# Spidev Crate Changelog

## 0.3.0 / 2016-10-26

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

