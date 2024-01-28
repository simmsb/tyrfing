#![no_std]
#![no_main]

use panic_halt as _;

use atxtiny_hal::prelude::*;
use atxtiny_hal::pac;
use atxtiny_hal::twi::{Twi, Error, NackSource};

use atxtiny_hal::embedded_hal::i2c::I2c;

#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Constrain a few peripherals into our HAL types
    let clkctrl = dp.CLKCTRL.constrain();
    let portmux = dp.PORTMUX.constrain();

    // Configure our clocks
    let clocks = clkctrl.freeze();

    // Split the PORTA/B peripheral into its pins
    let b = dp.PORTB.split();

    // Grab the TWI pins
    let sclpin = b.pb0.into_peripheral();
    let sdapin = b.pb1.into_peripheral();

    // Multiplex the TWI pins
    let twi_pair = (sclpin, sdapin);
    let twi_pair = twi_pair.mux(&portmux);

    // Create a TWI abstraction
    let mut twi = Twi::new(dp.TWI0, twi_pair, 100000.Hz(), clocks);

    // Send a string to address 3
    twi.write(0x03, "Hello over I2C".as_bytes()).unwrap();

    // I2C eeprom at address 0x50
    // Write 1 byte 0x55 to EEPROM offset 0x0000
    twi.write(0x50, &[0, 0, 0x55]).unwrap();

    // Wait for the EEPROM to finish the write
    // While the EEPROM writes, it won't ACK any addressing attempts
    while twi.read(0x50, &mut []) == Result::Err(Error::Nack(NackSource::Address)) {}

    // Read 1 byte from EEPROM offset 0x0000
    let mut buf = [0u8];
    twi.write_read(0x50, &[0x00, 0x00], &mut buf).unwrap();

    if buf[0] == 0x55 {
        twi.write(0x03, "EEPROM read success".as_bytes()).unwrap();
    } else {
        twi.write(0x03, "EEPROM read failure".as_bytes()).unwrap();
    }

    loop { }
}
