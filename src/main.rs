#![no_std]
#![no_main]

use arduino_hal as arduino;
use embedded_hal as hal;

use hal::prelude::*;

use panic_halt as _;

#[arduino::entry]
fn main() -> ! {
    let peripherals = arduino::Peripherals::take().unwrap();
    let pins = arduino::pins!(peripherals);
    let mut serial = arduino::default_serial!(peripherals, pins, 57600);

    let msg = "Hello, World!";
    let mut encoded = [[false; 11]; 13];
    for (slot, c) in encoded.iter_mut().zip(msg.chars()) {
        *slot = encode_char(c, false, [true, true]).unwrap();
    }

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!").ok();

    for slot in encoded {
        ufmt::uwriteln!(&mut serial, "{:?}", slot).ok();
    }
    loop {}
}

fn encode_char(c: char, start: bool, end: [bool; 2]) -> Option<[bool; 11]> {
    if !c.is_ascii() {
        return None;
    }

    // turn char into byte so we can shift right
    let c = c as u8;

    // set start and end bits to wanted value
    let mut msg = [false; 11];
    msg[0] = start;
    msg[9] = end[0];
    msg[10] = end[1];

    // get bits in character and put it in msg
    for i in 0..7 {
        let bit = (c >> i) % 2 == 1;
        msg[i + 1] = bit;
        msg[8] = msg[8] != bit;
    }

    // return msg
    Some(msg)
}

fn decode_char(encoded: [bool; 11]) -> char {
    let mut c = 0b00000000u8;
    for (i, bit) in encoded[1..8].iter().enumerate() {
        c += if *bit {1} else {0} << i;
    }

    c as char
}
