Rust Spidev
===========

[![Build Status](https://img.shields.io/travis/posborne/rust-spidev.svg)](https://travis-ci.org/posborne/rust-spidev)
[![Version](https://img.shields.io/crates/v/spidev.svg)](https://crates.io/crates/spidev)
[![License](https://img.shields.io/crates/l/spidev.svg)](https://github.com/posborne/rust-spidev/blob/master/README.md#license)

[Documentation](https://posborne.github.io/rust-spidev)

The Rust `spidev` seeks to provide full access to the Linux spidev
device in Rust without the need to wrap any C code or directly make
low-level system calls.  The documentation for the spidev interace can
be found at https://www.kernel.org/doc/Documentation/spi/spidev.

Example/API
-----------

The following is not an exhaustive demonstration of the Spidev
interface but provides a pretty good idea of how to use the library in
practice.

```rust
extern crate spidev;
use std::io;
use std::io::prelude::*;
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SPI_MODE_0};

fn create_spi() -> io::Result<Spidev> {
    let mut spi = try!(Spidev::open("/dev/spidev0.0"));
    let mut options = SpidevOptions::new()
         .bits_per_word(8)
         .max_speed_hz(20_000)
         .mode(SPI_MODE_0);
    try!(spi.configure(&options));
    Ok(spi)
}

/// perform half duplex operations using Read and Write traits
fn half_duplex(spi: &mut Spidev) -> io::Result<()> {
    let mut rx_buf = [0_u8; 10];
    try!(spi.write(&[0x01, 0x02, 0x03]));
    try!(spi.read(&mut rx_buf));
    println!("{:?}", rx_buf);
    Ok(())
}

/// Perform full duplex operations using Ioctl
fn full_duplex(spi: &mut Spidev) -> io::Result<()> {
    // "write" transfers are also reads at the same time with
    // the read having the same length as the write
    let mut transfer = SpidevTransfer::write(&[0x01, 0x02, 0x03]);
    try!(spi.transfer(&mut transfer));
    println!("{:?}", transfer.rx_buf);
    Ok(())
}

fn main() {
    let mut spi = create_spi().unwrap();
    println!("{:?}", half_duplex(&mut spi).unwrap());
    println!("{:?}", full_duplex(&mut spi).unwrap());
}
```

Features
--------

The following features are implemented and planned for the library:

- [x] Implement the Read trait
- [x] Implement the Write trait
- [x] Support for full-duplex transfers
- [x] Support for configuring spidev device
- [ ] Support for querying spidev configuration state

Cross Compiling
---------------

Most likely, the machine you are running on is not your development
machine (although it could be).  In those cases, you will need to
cross-compile.  The following basic instructions should work for the
raspberry pi or beaglebone black:

1. Install rust and cargo
2. Install an appropriate cross compiler.  On an Ubuntu system, this
   can be done by doing `sudo apt-get install g++-arm-linux-gnueabihf`.
3. Build or install rust for your target.  This is necessary in order
   to have libstd available for your target.  For arm-linux-gnueabihf,
   you can find binaries at https://github.com/japaric/ruststrap.
   With this approach or building it yourself, you will need to copy
   the ${rust}/lib/rustlib/arm-unknown-linux-gnueabihf to your system
   rust library folder (it is namespaced by triple, so it shouldn't
   break anything).
4. Tell cargo how to link by adding the lines below to your
   ~/.cargo/config file.
5. Run your build `cargo build --target=arm-unknown-linux-gnueabi`.

The following snippet added to my ~/.cargo/config worked for me:

```
[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

License
-------

Copyright (c) 2015, Paul Osborne <ospbau@gmail.com>

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/license/LICENSE-2.0> or the MIT license
<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
option.  This file may not be copied, modified, or distributed
except according to those terms.
