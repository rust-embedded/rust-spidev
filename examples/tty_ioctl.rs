extern crate spidev;

use spidev::ioctl;

// get window size
pub const TIOCGWINSZ: u32 = 0x5413;

#[derive(Clone, Copy, Debug)]
struct Winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

fn main() {
    // 0 is stdout
    match ioctl::read::<Winsize>(0, TIOCGWINSZ) {
        Ok(winsize) => println!("{:?}", winsize),
        Err(err) => println!("{}", err),
    }
}
