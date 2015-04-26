// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

use libc;
use libc::{c_int};
use std::mem;
use std::io;
use nix::from_ffi;
use std::os::unix::io::RawFd;

pub fn from_nix_error(err: ::nix::Error) -> io::Error {
    // taken from mio::io, line 178
    use std::mem;

    // TODO: Remove insane hacks once `std::io::Error::from_os_error` lands
    //       rust-lang/rust#24028
    #[allow(dead_code)]
    enum Repr {
        Os(i32),
        Custom(*const ()),
    }

    unsafe {
        mem::transmute(Repr::Os(err.errno() as i32))
    }
}


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

bitflags! {
    flags IoctlDirFlags: u8 {
        const IOC_NONE  = 0x00,
        const IOC_WRITE = 0x01,
        const IOC_READ  = 0x02,
    }
}


pub fn build_op(dir: IoctlDirFlags, ioctl_type: u8, nr: u8, size: u16) -> u32 {
    (((dir.bits as u32) << IOC_DIRSHIFT) |
     ((ioctl_type as u32) << IOC_TYPESHIFT) |
     ((nr as u32) << IOC_NRSHIFT) |
     ((size as u32) << IOC_SIZESHIFT))
}

pub fn build_op_none(ioctl_type: u8, nr: u8) -> u32 {
    build_op(IOC_NONE, ioctl_type, nr, 0)
}

pub fn build_op_read(ioctl_type: u8, nr: u8, size: u16) -> u32 {
    build_op(IOC_READ, ioctl_type, nr, size)
}

pub fn build_op_write(ioctl_type: u8, nr: u8, size: u16) -> u32 {
    build_op(IOC_WRITE, ioctl_type, nr, size)
}

pub fn build_op_read_write(ioctl_type: u8, nr: u8, size: u16) -> u32 {
    build_op(IOC_WRITE | IOC_READ, ioctl_type, nr, size)
}


/// Ioctl call that is expected to return a result
/// but which does not take any additional arguments on the input side
pub fn ioctl_read<T>(fd: RawFd, op: u32) -> io::Result<T> {
    unsafe {
        // allocate memory for the result (should get a value from kernel)
        let mut dst: T = mem::zeroed();
        let dst_ptr: *mut T = &mut dst;
        let ioctl_res: c_int = libc::funcs::bsd44::ioctl(fd as c_int, op as c_int, dst_ptr);
        match from_ffi(ioctl_res) {
            Err(err) => Err(from_nix_error(err)),
            Ok(_) => Ok(dst),
        }
    }
}
