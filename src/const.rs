// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

// Constants extracted from linux/spi/spidev.h

use libc::{c_int, uint8_t, uint64_t, uint32_t, uint16_t};

pub const SPI_CPHA: c_int = 0x01;
pub const SPI_CPOL: c_int = 0x02;

pub const SPI_MODE_0: c_int = 0;
pub const SPI_MODE_1: c_int = SPI_CPHA;
pub const SPI_MODE_2: c_int = SPI_CPOL;
pub const SPI_MODE_3: c_int = (SPI_CPOL | SPI_CPHA);

pub const SPI_CS_HIGH: c_int = 0x04;
pub const SPI_LSB_FIRST: c_int = 0x08;
pub const SPI_3WIRE: c_int = 0x10;
pub const SPI_LOOP: c_int = 0x20;
pub const SPI_NO_CS: c_int = 0x40;
pub const SPI_READY: c_int = 0x80;

pub const SPI_IOC_MAGIC: uint8_t = 'k' as uint8_t;

#[repr(C)]
pub struct spi_ioc_transfer {
    pub tx_buf: uint64_t,
    pub rx_buf: uint64_t,
    
    pub len: uint32_t,
    pub speed_hz: uint32_t,
    
    pub delay_usecs: uint16_t,
    pub bits_per_word: uint8_t,
    pub cs_change: uint8_t,
    pub pad: uint32_t,
}
