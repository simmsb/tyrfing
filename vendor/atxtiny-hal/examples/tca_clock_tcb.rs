#![no_std]
#![no_main]

use panic_halt as _;

use atxtiny_hal::prelude::*;
use atxtiny_hal::pac;
use atxtiny_hal::timer::FTimer;

#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Constrain a few peripherals into our HAL types
    let clkctrl = dp.CLKCTRL.constrain();

    // Configure our clocks
    let clocks = clkctrl.freeze();

    // Split the PORTB peripheral into its pins
    let b = dp.PORTB.split();

    // Grab a pin for an LED
    let mut led = b.pb6.into_push_pull_output();
    let mut led2 = b.pb5.into_push_pull_output();

    // Create a timer with a fixed frequency using TCA0
    // If the frequency cannot be met given the constrained prescalers of the
    // passed counter in conjunction with the clock supplying the timer peripheral
    // an error is returned.
    let t = FTimer::<_, 312500>::new(dp.TCA0, clocks).unwrap();

    // Get the 312.5KHz clock out of the TCA timer
    let tca0_clk = t.use_as_clock_source();

    // Create a counter with 100ms period on TCA0
    let mut c1 = t.counter();
    c1.start(100.millis()).unwrap();

    // Create another timer on TCB0 with the same frequency as TCA0 by using the
    // generated clock in TCA0 to clock TCB0
    // TCB0 has this feature because it itself only has prescalers of 1 and 2 for
    // the peripheral clock. To have more flexibility, you can clock TCB0 from TCA0
    let tcb = FTimer::<_, 312500>::new(dp.TCB0, tca0_clk).unwrap();
    let mut c2 = tcb.counter();
    c2.start(50.millis()).unwrap();

    loop {
        if c1.wait().is_ok() {
            // Toggle the LED
            led.toggle().unwrap();
        }

        if c2.wait().is_ok() {
            // Toggle the other LED
            led2.toggle().unwrap();
        }
    }
}
