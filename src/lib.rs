// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_type = "lib"]
#![crate_name = "spidev"]

// clone_from_slice
#![feature(collections)]

extern crate libc;
extern crate nix;

#[macro_use]
extern crate bitflags;

pub mod spidevioctl;

use std::io;
use std::io::prelude::*;
use std::fs::{File,OpenOptions};
use std::path::Path;
use std::os::unix::prelude::*;

// Constants extracted from linux/spi/spidev.h
bitflags! {
    flags SpiModeFlags: u8 {
        const SPI_CPHA = 0x01,
        const SPI_CPOL = 0x02,
        const SPI_CS_HIGH = 0x04,
        const SPI_LSB_FIRST = 0x08,
        const SPI_3WIRE = 0x10,
        const SPI_LOOP = 0x20,
        const SPI_NO_CS = 0x40,
        const SPI_READY = 0x80,
        const SPI_MODE_0 = 0x00,
        const SPI_MODE_1 = SPI_CPHA.bits,
        const SPI_MODE_2 = SPI_CPOL.bits,
        const SPI_MODE_3 = (SPI_CPOL.bits | SPI_CPHA.bits),
    }
}

///! API for acessing Linux spidev devices
///!
///! The library supports half-duplex read and write
///! operations (via the standard Read and Write traits).
///! Full duplex operation is achieved via xfer.

pub struct Spidev {
    devfile : File,
}

#[derive(Clone)]
pub struct SpidevOptions {
    pub bits_per_word: Option<u8>,
    pub max_speed_hz: Option<u32>,
    pub lsb_first: Option<bool>,
    pub spi_mode: Option<SpiModeFlags>,
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

    pub fn bits_per_word(&self, bits_per_word: u8) -> SpidevOptions {
        let mut newopts = self.clone();
        newopts.bits_per_word = Some(bits_per_word);
        newopts
    }

    pub fn max_speed_hz(&self, max_speed_hz: u32) -> SpidevOptions {
        let mut newopts = self.clone();
        newopts.max_speed_hz = Some(max_speed_hz);
        newopts
    }

    pub fn lsb_first(&mut self, lsb_first: bool) -> SpidevOptions {
        let mut newopts = self.clone();
        newopts.lsb_first = Some(lsb_first);
        newopts
    }

    pub fn mode(&self, mode: SpiModeFlags) -> SpidevOptions {
        let mut newopts = self.clone();
        newopts.spi_mode = Some(mode);
        newopts
    }
}

impl Spidev {
    pub fn open<P: AsRef<Path>>(path : P) -> io::Result<Spidev> {
        let devfile = try!(OpenOptions::new()
                           .read(true)
                           .write(true)
                           .create(false)
                           .open(path));
        let spidev = Spidev { devfile: devfile };
        Ok(spidev)
    }

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

    pub fn transfer(&self, transfer: &mut spidevioctl::SpidevTransfer) -> io::Result<()> {
        spidevioctl::transfer(self.devfile.as_raw_fd(), transfer)
    }

    pub fn transfer_multiple(&self, transfers: &Vec<spidevioctl::SpidevTransfer>) -> io::Result<()> {
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
