#![no_std]
#![no_main]

use panic_halt as _;

use core::cell::RefCell;

use avr_device::{interrupt, interrupt::Mutex};

use atxtiny_hal::prelude::*;
use atxtiny_hal::pac::{self, USART0};
use atxtiny_hal::gpio::{Porta, Input, Output, Stateless, U, Pin};
use atxtiny_hal::serial::{Serial, UartPinset};

static USART: Mutex<
    RefCell<
        Option<
            Serial<
                USART0,
                UartPinset<
                    USART0,
                    Pin<Porta, U<2>, Input>,
                    Pin<Porta, U<1>, Output<Stateless>>
                >
            >
        >
    >
> = Mutex::new(RefCell::new(None));

macro_rules! serial_println {
    ($($arg:tt)*) => {
        ::avr_device::interrupt::free(|cs| {
            if let Some(serial) = &mut *USART.borrow(cs).borrow_mut() {
                ::ufmt::uwriteln!(serial, $($arg)*)
            } else {
                Ok(())
            }
        }).unwrap()
    }
}

#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Constrain a few peripherals into our HAL types
    let clkctrl = dp.CLKCTRL.constrain();
    let portmux = dp.PORTMUX.constrain();

    // Configure our clocks
    let clocks = clkctrl.freeze();

    // Split the PORTA/B peripheral into its pins
    let a = dp.PORTA.split();

    let usart_pair = (a.pa2.into_peripheral::<USART0>(), a.pa1.into_peripheral::<USART0>()).mux(&portmux);
    let s = Serial::new(dp.USART0, usart_pair, 115200u32.bps(), clocks);

    // Initialize global USART variable
    interrupt::free(|cs| {
        let mut u = USART.borrow(cs).borrow_mut();
        *u = Some(s);
    });

    serial_println!("Hello!");

    loop { }
}
