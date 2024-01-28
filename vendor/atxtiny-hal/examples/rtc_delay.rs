#![no_std]
#![no_main]

use panic_halt as _;

use atxtiny_hal::prelude::*;
use atxtiny_hal::pac;
use atxtiny_hal::timer::{FTimer, rtc::RTCClockSource};

#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Constrain a few peripherals into our HAL types
    let clkctrl = dp.CLKCTRL.constrain();

    // Configure our clocks
    let _clocks = clkctrl.freeze();

    // Split the PORTB peripheral into its pins
    let b = dp.PORTB.split();

    // Grab a pin for an LED
    let mut led = b.pb6.into_push_pull_output();

    // Create a timer with a fixed frequency using TCA0
    // If the frequency cannot be met given the constrained prescalers of the
    // passed counter in conjunction with the clock supplying the timer peripheral
    // an error is returned.
    let t = FTimer::<_, 1024>::new(dp.RTC, RTCClockSource::OSCULP32K_32K).unwrap();

    // Use the now configured fixed frequency timer to create a delay
    let mut d = t.delay();

    loop {
        // Toggle the LED
        led.toggle().unwrap();

        // Sleep
        d.delay(500.millis());
    }
}
