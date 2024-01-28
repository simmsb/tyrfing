#![no_std]
#![no_main]

use panic_halt as _;

use atxtiny_hal::prelude::*;
use atxtiny_hal::pac;
use atxtiny_hal::timer::{FTimer, Channel, tca::WaveformGenerationMode};

#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Constrain a few peripherals into our HAL types
    let clkctrl = dp.CLKCTRL.constrain();
    let portmux = dp.PORTMUX.constrain();

    // Configure our clocks
    let clocks = clkctrl.freeze();

    // Split the PORTB peripheral into its pins
    let b = dp.PORTB.split();

    // Create a timer with a fixed frequency using TCA0
    // If the frequency cannot be met given the constrained prescalers of the
    // passed counter in conjunction with the clock supplying the timer peripheral
    // an error is returned.
    let t = FTimer::<_, 312500>::new(dp.TCA0, clocks).unwrap();
    let tca0_clk = t.use_as_clock_source();

    // Build a set of PWM pins and multiplex them accordingly
    let pwm_pins = (
        b.pb0.into_stateless_push_pull_output().mux(&portmux),
        b.pb1.into_stateless_push_pull_output().mux(&portmux),
        b.pb2.into_stateless_push_pull_output().mux(&portmux),
    );

    // Use the now configured fixed frequency timer to create a PWM abstraction
    let mut pwm = t.pwm(pwm_pins, 10.millis(), WaveformGenerationMode::SingleSlope).unwrap();

    // Enable all three channels and set a duty cycle
    pwm.set_duty_time(Channel::C1, 1.millis()).unwrap();
    pwm.enable(Channel::C1);

    pwm.set_duty_time(Channel::C2, 1.millis()).unwrap();
    pwm.enable(Channel::C2);

    pwm.set_duty_time(Channel::C3, 9.millis()).unwrap();
    pwm.enable(Channel::C3);

    // Let's use TCB for an accurate delay
    let mut d = FTimer::<_, 312500>::new(dp.TCB0, tca0_clk).unwrap().delay();

    let mut i = 0;

    loop {
        pwm.set_duty_time(Channel::C1, i.millis()).unwrap();

        i += 1;

        if i > 10 {
            i = 0;
        }

        d.delay(100.millis());
    }
}
