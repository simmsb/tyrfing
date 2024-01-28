#![no_std]
#![no_main]

use panic_halt as _;

use atxtiny_hal::prelude::*;
use atxtiny_hal::pac;

#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Constrain a few peripherals into our HAL types
    let clkctrl = dp.CLKCTRL.constrain();

    // Configure our clocks
    let _clocks = clkctrl.freeze();


    // Grab flash access
    let f = dp.NVMCTRL.flash();

    // Writing to flash doesn't always work. It depends how the fuse bits are
    // configured and from where this code is executed. Refer to the chip manual
    // for more info
    f.program(0x2000-3, &[0x12, 0x23, 0x34]).unwrap();

    // Read flash
    let data: &[u8] = f.read(0x1F00, 0x100).unwrap();
    assert!(&data[0x2000-3..0x2000] == &[0x12, 0x23, 0x34]);


    // Grab EEPROM access
    let e = dp.NVMCTRL.eeprom();

    // Write to EEPROM
    e.program(0, &[0x12, 0x23, 0x34]).unwrap();

    // Read from EEPROM
    let data: &[u8] = e.read(0, 3).unwrap();
    assert!(&data == &[0x12, 0x23, 0x34]);

    loop { }
}
