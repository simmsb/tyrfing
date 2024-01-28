#![no_std]
#![no_main]

use panic_halt as _;
use core::fmt::Write;

use atxtiny_hal::prelude::*;
use atxtiny_hal::pac;
use atxtiny_hal::serial::Serial;
use atxtiny_hal::embedded_io::Read;

#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Constrain a few peripherals into our HAL types
    let clkctrl = dp.CLKCTRL.constrain();
    let portmux = dp.PORTMUX.constrain();

    // Configure our clocks
    let clocks = clkctrl.freeze();

    // Split the PORTA peripheral into its pins
    let a = dp.PORTA.split();

    // Grab the serial port pins
    // We need to annotate the pins with the peripheral here because PA1/2 can
    // also be used as TWI pins and we need to tell the MUX what bit to flip
    let rxpin = a.pa2.into_peripheral::<pac::USART0>();
    let txpin = a.pa1.into_peripheral::<pac::USART0>();

    // Multiplex the serial port pins
    let usart_pair = (rxpin, txpin);
    let usart_pair = usart_pair.mux(&portmux);

    // Create a serial port abstraction
    let mut s = Serial::new(dp.USART0, usart_pair, 115200u32.bps(), clocks);

    // Say Hello
    s.write_str("Hello World\r\n".into()).unwrap();

    let buf = &mut [0u8];
    loop {
        s.read(&mut buf[..]).unwrap();
        ufmt::uwriteln!(s, "Received character: 0x{:x}", buf[0]).unwrap();
    }
}
