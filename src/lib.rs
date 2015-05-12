// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_type = "lib"]
#![crate_name = "spidev"]

//! # Spidev
//!
//! The `spidev` crate provides access to Linux spidev devices
//! from rust.  The wrapping of the interface is pretty direct
//! and should provide few big surprises.
//!
//! Additional information on the interface may be found in
//! [the kernel documentation
//! for spidev](https://www.kernel.org/doc/Documentation/spi/spidev).
//!
//! # Examples
//!
//! ```no_run
//! extern crate spidev;
//! use std::io;
//! use std::io::prelude::*;
//! use spidev::{Spidev, SpidevOptions, SpidevTransfer, SPI_MODE_0};
//!
//! fn create_spi() -> io::Result<Spidev> {
//!     let mut spi = try!(Spidev::open("/dev/spidev0.0"));
//!     let mut options = SpidevOptions::new()
//!          .bits_per_word(8)
//!          .max_speed_hz(20_000)
//!          .mode(SPI_MODE_0);
//!     try!(spi.configure(&options));
//!     Ok(spi)
//! }
//!
//! /// perform half duplex operations using Read and Write traits
//! fn half_duplex(spi: &mut Spidev) -> io::Result<()> {
//!     let mut rx_buf = [0_u8; 10];
//!     try!(spi.write(&[0x01, 0x02, 0x03]));
//!     try!(spi.read(&mut rx_buf));
//!     println!("{:?}", rx_buf);
//!     Ok(())
//! }
//!
//! /// Perform full duplex operations using Ioctl
//! fn full_duplex(spi: &mut Spidev) -> io::Result<()> {
//!     // "write" transfers are also reads at the same time with
//!     // the read having the same length as the write
//!     let mut transfer = SpidevTransfer::write(&[0x01, 0x02, 0x03]);
//!     try!(spi.transfer(&mut transfer));
//!     println!("{:?}", transfer.rx_buf);
//!     Ok(())
//! }
//!
//! fn main() {
//!     let mut spi = create_spi().unwrap();
//!     println!("{:?}", half_duplex(&mut spi).unwrap());
//!     println!("{:?}", full_duplex(&mut spi).unwrap());
//! }
//! ```

// clone_from_slice
#![feature(collections)]

extern crate libc;
extern crate nix;

#[macro_use]
extern crate bitflags;

pub mod spidevioctl;
pub use spidevioctl::SpidevTransfer;

use std::io;
use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::os::unix::prelude::*;

// Constants extracted from linux/spi/spidev.h
bitflags! {
    flags SpiModeFlags: u32 {
        /// Clock Phase
        const SPI_CPHA = 0x01,
        /// Clock Polarity
        const SPI_CPOL = 0x02,
        /// Chipselect Active High?
        const SPI_CS_HIGH = 0x04,
        /// Per-word Bits On Wire
        const SPI_LSB_FIRST = 0x08,
        /// SI/SO Signals Shared
        const SPI_3WIRE = 0x10,
        /// Loopback Mode
        const SPI_LOOP = 0x20,
        /// 1 dev/bus, no chipselect
        const SPI_NO_CS = 0x40,
        /// Slave pulls low to pause
        const SPI_READY = 0x80,

        // Common Configurations
        const SPI_MODE_0 = 0x00,
        const SPI_MODE_1 = SPI_CPHA.bits,
        const SPI_MODE_2 = SPI_CPOL.bits,
        const SPI_MODE_3 = (SPI_CPOL.bits | SPI_CPHA.bits),

        // == Only Supported with 32-bits ==

        /// Transmit with 2 wires
        const SPI_TX_DUAL = 0x100,
        /// Transmit with 4 wires
        const SPI_TX_QUAD = 0x200,
        /// Receive with 2 wires
        const SPI_RX_DUAL = 0x400,
        /// Receive with 4 wires
        const SPI_RX_QUAD = 0x800,
    }
}

/// Provide high-level access to Linux Spidev Driver
pub struct Spidev {
    devfile : File,
}

/// Options that control defaults for communication on a device
///
/// Individual settings may be overriden via parameters that
/// are specified as part of any individual SpiTransfer when
/// using `transfer` or `transfer_multiple`.
///
/// Options that are not configured with one of the builder
/// functions will not be modified in the kernel when
/// `configure` is called.
#[derive(Clone)]
pub struct SpidevOptions {
    pub bits_per_word: Option<u8>,
    pub max_speed_hz: Option<u32>,
    pub lsb_first: Option<bool>,
    pub spi_mode: Option<SpiModeFlags>,
}

impl SpidevOptions {

    /// Create a new, empty set of options
    pub fn new() -> SpidevOptions {
        SpidevOptions {
            bits_per_word: None,
            max_speed_hz: None,
            lsb_first: None,
            spi_mode: None,
        }
    }

    /// The number of bits in each SPI transfer word
    ///
    /// The value zero signifies eight bits.
    pub fn bits_per_word(&self, bits_per_word: u8) -> SpidevOptions {
        let mut newopts = self.clone();
        newopts.bits_per_word = Some(bits_per_word);
        newopts
    }

    /// The maximum SPI transfer speed, in Hz
    ///
    /// The controller can't necessarily assign that specific clock speed.
    pub fn max_speed_hz(&self, max_speed_hz: u32) -> SpidevOptions {
        let mut newopts = self.clone();
        newopts.max_speed_hz = Some(max_speed_hz);
        newopts
    }

    /// the bit justification used to transfer SPI words
    ///
    /// Zero indicates MSB-first; other values indicate the less common
    /// LSB-first encoding.  In both cases the specified value is
    /// right-justified in each word, so that unused (TX) or undefined (RX)
    /// bits are in the MSBs.
    pub fn lsb_first(&mut self, lsb_first: bool) -> SpidevOptions {
        let mut newopts = self.clone();
        newopts.lsb_first = Some(lsb_first);
        newopts
    }

    /// Set the SPI Transfer Mode
    ///
    /// Use the constants SPI_MODE_0..SPI_MODE_3; or if you prefer
    /// you can combine SPI_CPOL (clock polarity, idle high iff this
    /// is set) or SPI_CPHA (clock phase, sample on trailing edge
    /// iff this is set) flags.
    ///
    /// Note that this API will always prefer to use SPI_IOC_WR_MODE
    /// rathern than the 32-bit one to target the greatest number of
    /// kernels.  SPI_IOC_WR_MODE32 is only present in 3.15+ kernels.
    /// SPI_IOC_WR_MODE32 will be used iff bits higher than those in
    /// 8bits are provided (e.g. Dual/Quad Tx/Rx).
    pub fn mode(&self, mode: SpiModeFlags) -> SpidevOptions {
        let mut newopts = self.clone();
        newopts.spi_mode = Some(mode);
        newopts
    }
}

impl Spidev {
    /// Open the spidev device with the provided path
    ///
    /// Typically, the path will be something like `"/dev/spidev0.0"`
    /// where the first number if the bus and the second number
    /// is the chip select on that bus for the device being targetted.
    pub fn open<P: AsRef<Path>>(path : P) -> io::Result<Spidev> {
        let devfile = try!(OpenOptions::new()
                           .read(true)
                           .write(true)
                           .create(false)
                           .open(path));
        let spidev = Spidev { devfile: devfile };
        Ok(spidev)
    }

    /// Write the provided configuration to this device
    pub fn configure(&mut self, options: &SpidevOptions) -> io::Result<()> {
        // write out each present option to the device.  Options
        // that are None are left as-is, in order to reduce
        // overhead
        let fd = self.devfile.as_raw_fd();
        if options.bits_per_word.is_some() {
            let bpw = options.bits_per_word.unwrap();
            try!(spidevioctl::set_bits_per_word(fd, bpw));
        }
        if options.max_speed_hz.is_some() {
            let speed = options.max_speed_hz.unwrap();
            try!(spidevioctl::set_max_speed_hz(fd, speed));
        }
        if options.lsb_first.is_some() {
            let lsb_first = options.lsb_first.unwrap();
            try!(spidevioctl::set_lsb_first(fd, lsb_first));
        }
        if options.spi_mode.is_some() {
            let spi_mode_flags = options.spi_mode.unwrap();
            try!(spidevioctl::set_mode(fd, spi_mode_flags));
        }
        Ok(())
    }

    /// Perform a single transfer
    pub fn transfer(&self, transfer: &mut SpidevTransfer) -> io::Result<()> {
        spidevioctl::transfer(self.devfile.as_raw_fd(), transfer)
    }

    /// Perform multiple transfers in a single system call to the kernel
    ///
    /// Chaining together multiple requests like this can reduce latency
    /// and be used for conveniently and efficient implementing some
    /// protocols without extra round trips back to userspace.
    pub fn transfer_multiple(&self, transfers: &Vec<SpidevTransfer>) -> io::Result<()> {
        spidevioctl::transfer_multiple(self.devfile.as_raw_fd(), transfers)
    }
}

impl Read for Spidev {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.devfile.read(buf)
    }
}

impl Write for Spidev {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.devfile.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.devfile.flush()
    }
}
