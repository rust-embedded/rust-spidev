extern crate spidev;
use spidev::{Spidev,SpidevOptions,SPI_MODE_0};
use spidev::spidevioctl::SpidevTransfer;

fn main() {
    let mut spidev = Spidev::open("/dev/spidev0.0").unwrap();
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(5000)
        .lsb_first(false)
        .mode(SPI_MODE_0);
    spidev.configure(&options).unwrap();
    let mut transfer = SpidevTransfer::write(&[0xaa, 0xbb, 0xcc, 0xdd, 0xee]);
    spidev.transfer(&mut transfer).unwrap();
    for b in transfer.rx_buf.unwrap().iter() {
        println!("{:?}", b);
    }
}
