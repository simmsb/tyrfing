#![no_std]
#![no_main]

use panic_halt as _;

use atxtiny_hal::prelude::*;
use atxtiny_hal::pac;

#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let clkctrl = dp.CLKCTRL.constrain();

    let _clocks = clkctrl.freeze();

    let b = dp.PORTB.split();
    let mut btn = b.pb7.into_pull_up_input();
    let mut led = b.pb6.into_push_pull_output();
    let mut led2 = b.pb5.into_push_pull_output();

    let mut i = 0;

    loop {
        if btn.is_low().unwrap() {
            led2.set_high().unwrap();
        } else {
            led2.set_low().unwrap();
        }

        if i % 10000 == 0 {
            led.toggle().unwrap();
        }

        i += 1;
    }
}
