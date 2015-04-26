// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

use ioctl;
use std::mem;
use std::io;
use std::os::unix::io::RawFd;

// Constants extracted from linux/spi/spidev.h
bitflags! {
    flags SpiModeFlags: u8 {
        const SPI_CPHA = 0x01,
        const SPI_CPOL = 0x02,
        const SPI_MODE_0 = 0x00,
        const SPI_MODE_1 = SPI_CPHA.bits,
        const SPI_MODE_2 = SPI_CPOL.bits,
        const SPI_MODE_3 = (SPI_CPOL.bits | SPI_CPHA.bits),
    }
}

bitflags! {
    flags SpidevOptionFlags: u8 {
        const SPI_CS_HIGH = 0x04,
        const SPI_LSB_FIRST = 0x08,
        const SPI_3WIRE = 0x10,
        const SPI_LOOP = 0x20,
        const SPI_NO_CS = 0x40,
        const SPI_READY = 0x80,
    }
}

pub const SPI_IOC_MAGIC: u8 = 'k' as u8;

pub const SPI_IOC_NR_MODE: u8 = 1;
pub const SPI_IOC_NR_LSB_FIRST: u8 = 2;
pub const SPI_IOC_NR_BITS_PER_WORD: u8 = 3;
pub const SPI_IOC_NR_MAX_SPEED_HZ: u8 = 4;
pub const SPI_IOC_NR_MODE32: u8 = 5;

fn spidev_ioc_read<T>(fd: RawFd, nr: u8) -> io::Result<T> {
    let size: u16 = mem::size_of::<T>() as u16; // TODO: what if size is too large?
    let op = ioctl::op_read(SPI_IOC_MAGIC, nr, size);
    ioctl::read(fd, op)
}

pub fn spi_ioc_rd_mode(fd: RawFd) -> io::Result<u8> {
    spidev_ioc_read(fd, SPI_IOC_NR_MODE)
}

#[repr(C)]
pub struct spi_ioc_transfer {
    pub tx_buf: u64,
    pub rx_buf: u64,
    
    pub len: u32,
    pub speed_hz: u32,
    
    pub delay_usecs: u16,
    pub bits_per_word: u8,
    pub cs_change: u8,
    pub pad: u32,
}
