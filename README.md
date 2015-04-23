Rust Spidev
===========

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

use spidev::Spidev;

// spidev is opened with default settings
let spi = try!(Spidev::new("/dev/spidev0.1"));

// change a few settings
spi.set_bits_per_word(8u);
spi.set_cshigh(true);
spi.set_loop();
spi.set_lsbfirst();
spi.set_max_speed_hz();
spi.set_mode();
spi.set_threewire();

// half-duplex read/write operations are available
// using the ::std::io::Read and ::std::io::Write
// traits which are implemented for the Spidev struct
let outbuf = [1u8, 2u8, 3u8];
let mut inbuf : [u8; 10] = [0; 10];
try!(spi.write_all(&outbuf));
let _ = try!(spi.read(&mut inbuf)); // will always read 10 bytes

// full-duplex transfers are supported via the xfer
// method.  Here, we perform a read and write of the
// same buffers as used previously
let _ = try!(spi.xfer(outbuf, inbuf));
```

Features
--------

The following features are implemented and planned for the library:

- [ ] Implement the Read trait
- [ ] Implement the Write trait
- [ ] Support for full-duplex transfers
- [ ] Support for configuring spidev device
- [ ] Support for querying spidev configuration state

License
-------


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
