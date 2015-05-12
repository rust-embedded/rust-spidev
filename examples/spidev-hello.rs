extern crate spidev;
use std::io::prelude::*;
use spidev::Spidev;

fn main() {
    let mut spidev = Spidev::open("/dev/spidev0.0").unwrap();
    spidev.write(&[0xAA, 0x00, 0x01, 0x02, 0x04]).unwrap();

    let mut buf: [u8; 10] = [0; 10];
    spidev.read(&mut buf).unwrap(); // read 10
    println!("{:?}", buf);
}
