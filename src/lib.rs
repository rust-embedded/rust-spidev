// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_type = "lib"]
#![crate_name = "spidev"]

extern crate nix;
use nix::sys::ioctl::ioctl;

use std::io::prelude::*;
use std::io;
use std::os::unix::prelude::*;

///! API for acessing Linux spidev devices
///!
///! The library supports half-duplex read and write
///! operations (via the standard Read and Write traits).
///! Full duplex operation is achieved via xfer.

pub struct Spidev {
    devfile : File,
}

pub struct SpidevOptions {
    pub bits_per_word: Option<u8>,
    pub max_speed_hz: Option<u32>,
    pub lsb_first: Option<bool>,
    pub spi_mode: Option<u32>,
}

impl SpidevOptions {
    pub fn new() -> SpidevOptions {
        SpidevOptions {
            bits_per_word: None,
            max_speed_hz: None,
            lsb_first: None,
            spi_mode: None,
        }
    }

    pub fn bits_per_word(&mut self, bits_per_word: u8) {
        self.bits_per_word = Some(bits_per_word);
    }

    pub fn max_speed_hz(&mut self, max_speed_hz: u32) {
        self.max_speed_hz = Some(max_speed_hz);
    }

    pub fn lsb_first(&mut self, lsb_first: bool) {
        self.lsb_first = lsb_first;
    }

    pub fn mode(&mut self, mode: u32) {
        self.spi_mode = mode;
    }
}

impl Spidev {
    pub fn open<P: AsRef<Path>>(path : P) -> io::Result<Spidev> {
        let devfile = try!(OpenOptions::new()
                           .read(true)
                           .write(true)
                           .create(false)
                           .open(path));
        Spidev {
            devfile: devfile,
        }
    }

    pub fn configure(&mut self, options: &SpidevOptions) {
        // write out each present option to the device.  Options
        // that are None are left as-is, in order to reduce
        // overhead
        if options.bits_per_word.is_some() {
            ioctl();
        }
    }
}

impl io::Read for Spidev {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.devfile.read(buf)
    }
}

impl io::Write for Spidev {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.devfile.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.devfile.flush(buf)
    }
}
