extern crate spidev;
use spidev::Spidev;
use std::io::prelude::*;

fn main() {
    let mut spidev = Spidev::open("/dev/spidev0.0").unwrap();
    let wrote = spidev.write(&[0xAA, 0x00, 0x01, 0x02, 0x04]).unwrap();

    let mut buf: [u8; 10] = [0; 10];
    let read = spidev.read(&mut buf).unwrap(); // read 10
    println!("Wrote: {}, Read: {}, Data: {:?}", wrote, read, buf);
}
