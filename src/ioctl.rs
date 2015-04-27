// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

///! Provide helpers for making ioctl system calls
///!
///! # Overview of IOCTLs
///!
///! The `ioctl` system call is a widely support system
///! call on *nix systems providing access to functions
///! and data that do not fit nicely into the standard
///! read and write operations on a file itself.  It is
///! common to see ioctls used for the following purposes:
///!
///! * Provide read/write access to out-of-band data related
///!   to a device such as configuration (for instance, setting
///!   serial port options)
///! * Provide a mechanism for performing full-duplex data
///!   transfers (for instance, xfer on SPI devices).
///! * Provide access to control functions on a device (for example,
///!   on Linux you can send commands like pause, resume, and eject
///!   to the CDROM device.
///! * Do whatever else the device driver creator thought made most sense.
///!
///! Ioctls are synchronous system calls and are similar to read and
///! write calls in that regard.
///!
///! The prototype for the ioctl system call in libc is as follows:
///!
///! ```
///! int ioctl(int fd, unsigned long request, ...);
///! ```
///!
///! Typically, an ioctl takes 3 parameters as arguments:
///! 1. An open file descriptor, `fd`.
///! 2. An device-dependennt request code or operation.  This request
///!    code is referred to as `op` in this module.
///! 3. Either a pointer to a location in memory or an integer.  This
///!    number of pointer may either be used by the kernel or written
///!    to by the kernel depending on how the operation is documented
///!    to work.
///!
///! The `op` request code is essentially an arbitrary integer having
///! a device-driver specific meaning.  Over time, it proved difficult
///! for various driver implementors to use this field sanely, so a
///! convention with macros was introduced to the Linux Kernel that
///! is used by most newer drivers.  See
///! https://github.com/torvalds/linux/blob/master/Documentation/ioctl/ioctl-number.txt
///! for additional details.  The macros exposed by the kernel for
///! consumers are implemented in this module and may be used to
///! instead of calls like `_IOC`, `_IO`, `_IOR`, and `_IOW`.
///!
///! # Interface Overview
///!
///! This ioctl module seeks to tame the ioctl beast by providing
///! a set of safer (although not completely safe) functions
///! implementing the most common ioctl access patterns.
///!
///! The most common access patterns for ioctls are as follows:
///! 1. `read`: A pointer is provided to the kernel which is populated
///!    with a value containing the "result" of the operation.  The
///!    result may be an integer or structure.
///! 2. `write`: A pointer is provided to the kernel containing values
///!    that the kernel will read in order to perform the operation.
///! 3. `read_write`: (TODO: research more.  Is this common?  Is this
///!     typically done with a single structure pointer or multiple
///!     pointer passed into the kernel?)
///! 4. `execute`: The operation is passed to the kernel but no
///!    additional pointer is passed.  The operation is enough
///!    and it either succeeds or results in an error.

use libc;
use libc::{c_int,c_ulong};
use std::mem;
use std::io;
use nix::from_ffi;
use std::os::unix::io::RawFd;

// low-level ioctl functions and definitions matching the
// macros provided in ioctl.h from the kernel
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_DIRBITS: u32 = 2;

const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;

/// Flags indicating the direction of the ioctl operation
/// for ioctls using modern operation conventions
bitflags! {
    flags IoctlDirFlags: u8 {
        /// Indicates that the ioctl data pointer is not used
        const IOC_NONE  = 0x00,
        /// Indicates that the ioctl data pointer contains data that
        /// will be consumed by the operating system
        const IOC_WRITE = 0x01,
        /// Indicates tha the ioctl data pointer contains data that
        /// will be populated by the operating system to be consumed
        /// by userspace
        const IOC_READ  = 0x02,
    }
}

/// Build an ioctl op with the provide parameters.  This is a helper
/// function for IOCTLs in the Linux kernel using the newer conventions
/// for IOCTLs operations.  Many ioctls do not use this newer convention
/// and the constants for those should just be used as-is.
///
/// This provides the same functionality as the Linux `_IOC` macro.
pub fn op(dir: IoctlDirFlags, ioctl_type: u8, nr: u8, size: u16) -> c_ulong {
    // actual number will always fit in 32 bits, but ioctl() expects
    // an unsigned long for the op
    (((dir.bits as u32) << IOC_DIRSHIFT) |
     ((ioctl_type as u32) << IOC_TYPESHIFT) |
     ((nr as u32) << IOC_NRSHIFT) |
     ((size as u32) << IOC_SIZESHIFT)) as c_ulong
}

/// Build an op indicating that the data pointer is not used.
/// That is, the command itself is sufficient.
///
/// This provides the same functionality the Linux `_IO` macro.
pub fn op_none(ioctl_type: u8, nr: u8) -> c_ulong {
    op(IOC_NONE, ioctl_type, nr, 0)
}

/// Build an op indicating that the data pointer will be populated
/// with data from the kernel
///
/// This provides the same functionality as the Linux `_IOR` macro.
pub fn op_read(ioctl_type: u8, nr: u8, size: u16) -> c_ulong {
    op(IOC_READ, ioctl_type, nr, size)
}

/// Build an op indicating that the data pointer contains data
/// to be consumed by the kernel (and not written to).
///
/// This provides the same functionality as the Linux `_IOW` macro.
pub fn op_write(ioctl_type: u8, nr: u8, size: u16) -> c_ulong {
    op(IOC_WRITE, ioctl_type, nr, size)
}

/// Build an op indicating that the data pointer both contains
/// data to be consumed by the kernel and contains fields that
/// will be populated by the kernel.
///
/// This provides the same functionality as the Linux `_IOWR` macro.
pub fn op_read_write(ioctl_type: u8, nr: u8, size: u16) -> c_ulong {
    op(IOC_WRITE | IOC_READ, ioctl_type, nr, size)
}

fn from_nix_error(err: ::nix::Error) -> io::Error {
    io::Error::from_raw_os_error(err.errno() as i32)
}

/// Ioctl call that is expected to return a result
/// but which does not take any additional arguments on the input side
pub unsafe fn read<T>(fd: RawFd, op: c_ulong) -> io::Result<T> {
    // allocate memory for the result (should get a value from kernel)
    let mut dst: T = mem::zeroed();
    let dst_ptr: *mut T = &mut dst;
    let ioctl_res: c_int = libc::funcs::bsd44::ioctl(fd as c_int, op as c_int, dst_ptr);
    match from_ffi(ioctl_res) {
        Err(err) => Err(from_nix_error(err)),
        Ok(_) => Ok(dst),
    }
}

/// Ioctl call that sends a value to the kernel but
/// does not return anything (pure side effect).
pub unsafe fn write<T>(fd: RawFd, op: c_ulong, data: &T) -> io::Result<()> {
    let data_ptr: *const T = data;
    let ioctl_res: c_int = libc::funcs::bsd44::ioctl(fd as c_int, op as c_int, data_ptr);
    match from_ffi(ioctl_res) {
        Err(err) => Err(from_nix_error(err)),
        Ok(_) => Ok(()),
    }
}

/// Ioctl call that sends a value to the kernel and expects that the
/// kernel will modify the buffer that is provided, in addition to using it
///
/// This function is identical to `write` except that it requires a
/// the data reference to but mutable.
pub unsafe fn read_write<T>(fd: RawFd, op: c_ulong, data: &mut T) -> io::Result<()> {
    let data_ptr: *mut T = data;
    let ioctl_res: c_int = libc::funcs::bsd44::ioctl(fd as c_int, op as c_int, data_ptr);
    match from_ffi(ioctl_res) {
        Err(err) => Err(from_nix_error(err)),
        Ok(_) => Ok(()),
    }
}

/// Ioctl call for which no data pointer is provided to the kernel.
/// That is, the kernel has sufficient information about what to
/// do based on the op alone.
pub fn execute(fd: RawFd, op: c_ulong) -> io::Result<()> {
    unsafe {
        let ioctl_res: c_int = libc::funcs::bsd44::ioctl(fd as c_int, op as c_int);
        match from_ffi(ioctl_res) {
            Err(err) => Err(from_nix_error(err)),
            Ok(_) => Ok(()),
        }
    }
}
