// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

// macros import
use super::SpiModeFlags;
use nix::{ioctl_read, ioctl_write_buf, ioctl_write_ptr};
use std::io;
use std::marker::PhantomData;
use std::os::unix::prelude::*;

fn from_nix_result<T>(res: ::nix::Result<T>) -> io::Result<T> {
    match res {
        Ok(r) => Ok(r),
        Err(err) => Err(err.into()),
    }
}

/// Structure that is used when performing communication
/// with the kernel.
///
/// From the kernel documentation:
///
/// ```text
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
/// accommodate potential future updates.
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
#[allow(non_camel_case_types)]
#[derive(Debug, Default)]
#[repr(C)]
pub struct spi_ioc_transfer<'a, 'b> {
    tx_buf: u64,
    rx_buf: u64,
    len: u32,

    // optional overrides
    pub speed_hz: u32,
    pub delay_usecs: u16,
    pub bits_per_word: u8,
    pub cs_change: u8,
    pub pad: u32,

    tx_buf_ref: PhantomData<&'a [u8]>,
    rx_buf_ref: PhantomData<&'b mut [u8]>,
}

impl<'a, 'b> spi_ioc_transfer<'a, 'b> {
    /// Create a read transfer
    pub fn read(buff: &'b mut [u8]) -> Self {
        spi_ioc_transfer {
            rx_buf: buff.as_ptr() as *const () as usize as u64,
            len: buff.len() as u32,
            ..Default::default()
        }
    }

    /// Create a write transfer
    pub fn write(buff: &'a [u8]) -> Self {
        spi_ioc_transfer {
            tx_buf: buff.as_ptr() as *const () as usize as u64,
            len: buff.len() as u32,
            ..Default::default()
        }
    }

    /// Create a read/write transfer.
    /// Note that the `tx_buf` and `rx_buf` must be the same length.
    pub fn read_write(tx_buf: &'a [u8], rx_buf: &'b mut [u8]) -> Self {
        assert_eq!(tx_buf.len(), rx_buf.len());
        spi_ioc_transfer {
            rx_buf: rx_buf.as_ptr() as *const () as usize as u64,
            tx_buf: tx_buf.as_ptr() as *const () as usize as u64,
            len: tx_buf.len() as u32,
            ..Default::default()
        }
    }

    /// Create a delay transfer of a number of microseconds
    pub fn delay(microseconds: u16) -> Self {
        spi_ioc_transfer {
            delay_usecs: microseconds,
            len: 0,
            ..Default::default()
        }
    }
}

mod ioctl {
    use super::*;

    const SPI_IOC_MAGIC: u8 = b'k';
    const SPI_IOC_NR_TRANSFER: u8 = 0;
    const SPI_IOC_NR_MODE: u8 = 1;
    const SPI_IOC_NR_LSB_FIRST: u8 = 2;
    const SPI_IOC_NR_BITS_PER_WORD: u8 = 3;
    const SPI_IOC_NR_MAX_SPEED_HZ: u8 = 4;
    const SPI_IOC_NR_MODE32: u8 = 5;

    ioctl_read!(get_mode_u8, SPI_IOC_MAGIC, SPI_IOC_NR_MODE, u8);
    ioctl_read!(get_mode_u32, SPI_IOC_MAGIC, SPI_IOC_NR_MODE32, u32);
    ioctl_write_ptr!(set_mode, SPI_IOC_MAGIC, SPI_IOC_NR_MODE, u8);
    ioctl_write_ptr!(set_mode32, SPI_IOC_MAGIC, SPI_IOC_NR_MODE32, u32);

    ioctl_read!(get_lsb_first, SPI_IOC_MAGIC, SPI_IOC_NR_LSB_FIRST, u8);
    ioctl_write_ptr!(set_lsb_first, SPI_IOC_MAGIC, SPI_IOC_NR_LSB_FIRST, u8);

    ioctl_read!(
        get_bits_per_word,
        SPI_IOC_MAGIC,
        SPI_IOC_NR_BITS_PER_WORD,
        u8
    );
    ioctl_write_ptr!(
        set_bits_per_word,
        SPI_IOC_MAGIC,
        SPI_IOC_NR_BITS_PER_WORD,
        u8
    );

    ioctl_read!(
        get_max_speed_hz,
        SPI_IOC_MAGIC,
        SPI_IOC_NR_MAX_SPEED_HZ,
        u32
    );
    ioctl_write_ptr!(
        set_max_speed_hz,
        SPI_IOC_MAGIC,
        SPI_IOC_NR_MAX_SPEED_HZ,
        u32
    );

    // NOTE: this macro works for single transfers but cannot properly
    // calculate size for multi transfer whose length we will not know
    // until runtime.  We fallback to using the underlying ioctl for that
    // use case.
    ioctl_write_ptr!(
        spidev_transfer,
        SPI_IOC_MAGIC,
        SPI_IOC_NR_TRANSFER,
        spi_ioc_transfer
    );
    ioctl_write_buf!(
        spidev_transfer_buf,
        SPI_IOC_MAGIC,
        SPI_IOC_NR_TRANSFER,
        spi_ioc_transfer
    );
}

/// Representation of a spidev transfer that is shared
/// with external users
pub type SpidevTransfer<'a, 'b> = spi_ioc_transfer<'a, 'b>;

pub fn get_mode(fd: RawFd) -> io::Result<u8> {
    let mut mode: u8 = 0;
    from_nix_result(unsafe { ioctl::get_mode_u8(fd, &mut mode) })?;
    Ok(mode)
}

pub fn set_mode(fd: RawFd, mode: SpiModeFlags) -> io::Result<()> {
    // we will always use the 8-bit mode write unless bits not in
    // the 8-bit mask are used.  This is because WR_MODE32 was not
    // added until later kernels.  This provides a reasonable story
    // for forwards and backwards compatibility
    if (mode.bits & 0xFFFFFF00) != 0 {
        from_nix_result(unsafe { ioctl::set_mode32(fd, &mode.bits) })?;
    } else {
        let bits: u8 = mode.bits as u8;
        from_nix_result(unsafe { ioctl::set_mode(fd, &bits) })?;
    }
    Ok(())
}

pub fn get_lsb_first(fd: RawFd) -> io::Result<u8> {
    let mut lsb_first: u8 = 0;
    from_nix_result(unsafe { ioctl::get_lsb_first(fd, &mut lsb_first) })?;
    Ok(lsb_first)
}

pub fn set_lsb_first(fd: RawFd, lsb_first: bool) -> io::Result<()> {
    let lsb_first_value: u8 = if lsb_first { 1 } else { 0 };
    from_nix_result(unsafe { ioctl::set_lsb_first(fd, &lsb_first_value) })?;
    Ok(())
}

pub fn get_bits_per_word(fd: RawFd) -> io::Result<u8> {
    let mut bits_per_word: u8 = 0;
    from_nix_result(unsafe { ioctl::get_bits_per_word(fd, &mut bits_per_word) })?;
    Ok(bits_per_word)
}

pub fn set_bits_per_word(fd: RawFd, bits_per_word: u8) -> io::Result<()> {
    from_nix_result(unsafe { ioctl::set_bits_per_word(fd, &bits_per_word) })?;
    Ok(())
}

pub fn get_max_speed_hz(fd: RawFd) -> io::Result<u32> {
    let mut max_speed_hz: u32 = 0;
    from_nix_result(unsafe { ioctl::get_max_speed_hz(fd, &mut max_speed_hz) })?;
    Ok(max_speed_hz)
}

pub fn set_max_speed_hz(fd: RawFd, max_speed_hz: u32) -> io::Result<()> {
    from_nix_result(unsafe { ioctl::set_max_speed_hz(fd, &max_speed_hz) })?;
    Ok(())
}

pub fn transfer(fd: RawFd, transfer: &mut SpidevTransfer) -> io::Result<()> {
    // The kernel will directly modify the rx_buf of the SpidevTransfer
    // rx_buf if present, so there is no need to do any additional work
    from_nix_result(unsafe { ioctl::spidev_transfer(fd, transfer) })?;
    Ok(())
}

pub fn transfer_multiple(fd: RawFd, transfers: &mut [SpidevTransfer]) -> io::Result<()> {
    from_nix_result(unsafe { ioctl::spidev_transfer_buf(fd, transfers) })?;
    Ok(())
}
