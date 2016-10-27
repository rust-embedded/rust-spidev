extern crate spidev;
use spidev::{Spidev, SpidevOptions, SPI_MODE_0};
use spidev::spidevioctl::SpidevTransfer;

fn main() {
    let mut spidev = Spidev::open("/dev/spidev0.0").unwrap();
    let options = SpidevOptions::new()
                      .bits_per_word(8)
                      .max_speed_hz(5000)
                      .lsb_first(false)
                      .mode(SPI_MODE_0)
                      .build();
    spidev.configure(&options).unwrap();

    println!("===== Single transfer =========");
    let tx_buf = [0xaa, 0xbb, 0xcc, 0xdd, 0xee];
    let mut rx_buf = [0; 5];
    let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
    println!("{:?}", spidev.transfer(&mut transfer));

    println!("===== Multi Transfer =========");
    let mut rx_buf1 = [0; 10];
    let tx_buf2 = [0x00, 0x01, 0x02, 0x03];
    let tx_buf3 = [0xff, 0xfe, 0xfd];
    let mut rx_buf3 = [0; 3];
    let result = {
        let mut transfers = vec![SpidevTransfer::read(&mut rx_buf1),
                             SpidevTransfer::write(&tx_buf2),
                             SpidevTransfer::read_write(&tx_buf3, &mut rx_buf3)];
        spidev.transfer_multiple(&mut transfers)
    };
    match result {
        Ok(_) => {
            println!("Read {:?}", rx_buf1);
            println!("Wrote {:?}", tx_buf2);
            println!("Wrote {:?} and read {:?}", tx_buf3, rx_buf3);
        }
        Err(err) => println!("{:?}", err),
    }
}
