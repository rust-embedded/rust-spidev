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
use std::os::unix::prelude::*;
use super::SpiModeFlags;

const SPI_IOC_MAGIC: u8 = 'k' as u8;

const SPI_IOC_NR_TRANSFER: u8 = 0;
const SPI_IOC_NR_MODE: u8 = 1;
const SPI_IOC_NR_LSB_FIRST: u8 = 2;
const SPI_IOC_NR_BITS_PER_WORD: u8 = 3;
const SPI_IOC_NR_MAX_SPEED_HZ: u8 = 4;
const SPI_IOC_NR_MODE32: u8 = 5;

/// Structure that is used when performing communication
/// with the kernel.
///
/// From the kernel documentation:
///
/// ```
/// struct spi_ioc_transfer - describes a single SPI transfer
/// @tx_buf: Holds pointer to userspace buffer with transmit data, or null.
///   If no data is provided, zeroes are shifted out.
/// @rx_buf: Holds pointer to userspace buffer for receive data, or null.
/// @len: Length of tx and rx buffers, in bytes.
/// @speed_hz: Temporary override of the device's bitrate.
/// @bits_per_word: Temporary override of the device's wordsize.
/// @delay_usecs: If nonzero, how long to delay after the last bit transfer
///      before optionally deselecting the device before the next transfer.
/// @cs_change: True to deselect device before starting the next transfer.
///
/// This structure is mapped directly to the kernel spi_transfer structure;
/// the fields have the same meanings, except of course that the pointers
/// are in a different address space (and may be of different sizes in some
/// cases, such as 32-bit i386 userspace over a 64-bit x86_64 kernel).
/// Zero-initialize the structure, including currently unused fields, to
/// accomodate potential future updates.
///
/// SPI_IOC_MESSAGE gives userspace the equivalent of kernel spi_sync().
/// Pass it an array of related transfers, they'll execute together.
/// Each transfer may be half duplex (either direction) or full duplex.
///
///      struct spi_ioc_transfer mesg[4];
///      ...
///      status = ioctl(fd, SPI_IOC_MESSAGE(4), mesg);
///
/// So for example one transfer might send a nine bit command (right aligned
/// in a 16-bit word), the next could read a block of 8-bit data before
/// terminating that command by temporarily deselecting the chip; the next
/// could send a different nine bit command (re-selecting the chip), and the
/// last transfer might write some register values.
/// ```
#[derive(Debug)]
struct spi_ioc_transfer {
    pub tx_buf: u64,
    pub rx_buf: u64,
    pub len: u32,

    // optional overrides
    pub speed_hz: u32,
    pub delay_usecs: u16,
    pub bits_per_word: u8,
    pub cs_change: u8,
    pub pad: u32,
}

/// Representation of a spidev transfer that is shared
/// with external users
#[derive(Default)]
pub struct SpidevTransfer {
    pub tx_buf: Option<Box<[u8]>>,
    pub rx_buf: Option<Box<[u8]>>,
    len: u32,
    speed_hz: u32,
    delay_usecs: u16,
    bits_per_word: u8,
    cs_change: u8,
    pad: u32,
}

impl SpidevTransfer {
    pub fn read(length: u32) -> SpidevTransfer {
        let rx_buf_vec: Vec<u8> = Vec::with_capacity(length as usize);
        SpidevTransfer {
            tx_buf: None,
            rx_buf: Some(rx_buf_vec.into_boxed_slice()),
            len: length as u32,
            ..Default::default()
        }
    }

    pub fn write(tx_buf: &[u8]) -> SpidevTransfer {
        let len = tx_buf.len();
        let rx_buf_vec: Vec<u8> = vec![0; len];
        let mut tx_buf_vec: Vec<u8> = Vec::with_capacity(len);
        tx_buf_vec.clone_from_slice(tx_buf);
        SpidevTransfer {
            tx_buf: Some(tx_buf_vec.into_boxed_slice()),
            rx_buf: Some(rx_buf_vec.into_boxed_slice()),
            len: tx_buf.len() as u32,
            ..Default::default()
        }
    }

    fn as_spi_ioc_transfer(&self) -> spi_ioc_transfer {
        spi_ioc_transfer {
            tx_buf: match self.tx_buf {
                Some(ref bufbox) => bufbox.as_ptr() as u64,
                None => 0,
            },
            rx_buf: match self.rx_buf {
                Some(ref bufbox) => bufbox.as_ptr() as u64,
                None => 0,
            },
            len: self.len,
            speed_hz: self.speed_hz,
            delay_usecs: self.delay_usecs,
            bits_per_word: self.bits_per_word,
            cs_change: self.cs_change,
            pad: self.pad,
        }
    }
}

fn spidev_ioc_read<T>(fd: RawFd, nr: u8) -> io::Result<T> {
    let size: u16 = mem::size_of::<T>() as u16;
    let op = ioctl::op_read(SPI_IOC_MAGIC, nr, size);
    ioctl::read(fd, op)
}

fn spidev_ioc_write<T>(fd: RawFd, nr: u8, data: &T) -> io::Result<()> {
    let size: u16 = mem::size_of::<T>() as u16;
    let op = ioctl::op_write(SPI_IOC_MAGIC, nr, size);
    try!(ioctl::write(fd, op, data));
    Ok(())
}

pub fn get_mode(fd: RawFd) -> io::Result<u8> {
    // #define SPI_IOC_RD_MODE _IOR(SPI_IOC_MAGIC, 1, __u8)
    spidev_ioc_read::<u8>(fd, SPI_IOC_NR_MODE)
}

pub fn set_mode(fd: RawFd, mode: SpiModeFlags) -> io::Result<()> {
    // #define SPI_IOC_WR_MODE _IOW(SPI_IOC_MAGIC, 1, __u8)
    spidev_ioc_write(fd, SPI_IOC_NR_MODE, &mode.bits)
}

pub fn get_lsb_first(fd: RawFd) -> io::Result<bool> {
    // #define SPI_IOC_RD_LSB_FIRST _IOR(SPI_IOC_MAGIC, 2, __u8)
    Ok(try!(spidev_ioc_read::<u8>(fd, SPI_IOC_NR_LSB_FIRST)) != 0)
}

pub fn set_lsb_first(fd: RawFd, lsb_first: bool) -> io::Result<()> {
    // #define SPI_IOC_WR_LSB_FIRST _IOW(SPI_IOC_MAGIC, 2, __u8)
    let lsb_first_value: u8 = if lsb_first { 1 } else { 0 };
    spidev_ioc_write(fd, SPI_IOC_NR_LSB_FIRST, &lsb_first_value)
}

pub fn get_bits_per_word(fd: RawFd) -> io::Result<u8> {
    // #define SPI_IOC_RD_BITS_PER_WORD _IOR(SPI_IOC_MAGIC, 3, __u8)
    spidev_ioc_read::<u8>(fd, SPI_IOC_NR_BITS_PER_WORD)
}

pub fn set_bits_per_word(fd: RawFd, bits_per_word: u8) -> io::Result<()> {
    // #define SPI_IOC_WR_BITS_PER_WORD _IOW(SPI_IOC_MAGIC, 3, __u8)
    spidev_ioc_write(fd, SPI_IOC_NR_BITS_PER_WORD, &bits_per_word)
}

pub fn get_max_speed_hz(fd: RawFd) -> io::Result<u32> {
    // #define SPI_IOC_RD_MAX_SPEED_HZ _IOR(SPI_IOC_MAGIC, 4, __u32)
    spidev_ioc_read::<u32>(fd, SPI_IOC_NR_MAX_SPEED_HZ)
}

pub fn set_max_speed_hz(fd: RawFd, max_speed_hz: u32) -> io::Result<()> {
    // #define SPI_IOC_WR_MAX_SPEED_HZ _IOW(SPI_IOC_MAGIC, 4, __u32)
    spidev_ioc_write(fd, SPI_IOC_NR_MAX_SPEED_HZ, &max_speed_hz)
}

pub fn transfer(fd: RawFd, transfer: &mut SpidevTransfer) -> io::Result<()> {
    let mut raw_transfer = transfer.as_spi_ioc_transfer();
    let op = ioctl::op_write(
            SPI_IOC_MAGIC,
            SPI_IOC_NR_TRANSFER,
            1 * mem::size_of::<spi_ioc_transfer>() as u16);

    // The kernel will directly modify the rx_buf of the SpidevTransfer
    // rx_buf if present, so there is no need to do any additional work
    try!(ioctl::read_write(fd, op, &mut raw_transfer));
    Ok(())
}

pub fn transfer_multiple(fd: RawFd, transfers: Vec<&mut SpidevTransfer>) -> io::Result<()> {
    Ok(())  // TODO: implement this in the future
}
