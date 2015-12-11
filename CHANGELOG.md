# Spidev Crate Changelog

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

