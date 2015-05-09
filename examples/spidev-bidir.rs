extern crate spidev;
use spidev::{Spidev,SpidevOptions,SPI_MODE_0};
use spidev::spidevioctl::SpidevTransfer;

fn pprint_transfer(transfer: &SpidevTransfer) {
    match transfer.rx_buf {
        None => println!("Empty!"),
        Some(ref rx_buf) => {
            for b in rx_buf.iter() {
                print!("{:?} ", b);
            }
            println!("");
        }
    }
}

fn main() {
    let mut spidev = Spidev::open("/dev/spidev0.0").unwrap();
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(5000)
        .lsb_first(false)
        .mode(SPI_MODE_0);
    spidev.configure(&options).unwrap();

    println!("===== Single transfer =========");
    let mut transfer = SpidevTransfer::write(&[0xaa, 0xbb, 0xcc, 0xdd, 0xee]);
    match spidev.transfer(&mut transfer) {
        Ok(_) => pprint_transfer(&transfer),
        Err(err) => println!("{:?}", err),
    }

    println!("===== Multi Transfer =========");
    let transfers = vec!(
        SpidevTransfer::read(10),
        SpidevTransfer::write(&[0x00, 0x01, 0x02, 0x03]),
        SpidevTransfer::write(&[0xff, 0xfe, 0xfd]));
    match spidev.transfer_multiple(&transfers) {
        Ok(_) => {
            for (i, transfer) in transfers.iter().enumerate() {
                println!("{}...", i);
                pprint_transfer(transfer);
            }
        },
        Err(err) => println!("{:?}", err),
    }
}
