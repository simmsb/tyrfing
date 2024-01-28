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

    // Constrain the Watchdog
    let mut wd = dp.WDT.constrain();

    // Start it with an 8 second timeout
    wd.start(WatchdogTimeout::S8);

    loop {
        // Feed the watchdog to reset it
        wd.feed();
    }
}
