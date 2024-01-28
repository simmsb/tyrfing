#![no_std]
#![no_main]

use panic_halt as _;

use atxtiny_hal::prelude::*;
use atxtiny_hal::pac;
use atxtiny_hal::timer::{FTimer, Timer, Channel, tca::WaveformGenerationMode};

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

    // Create a timer with a dynamic frequency using TCA0
    let t = Timer::new(dp.TCA0, clocks);

    // Build a set of PWM pins and multiplex them accordingly
    let pwm_pins = (
        b.pb0.into_stateless_push_pull_output().mux(&portmux),
        b.pb1.into_stateless_push_pull_output().mux(&portmux),
        b.pb2.into_stateless_push_pull_output().mux(&portmux),
    );

    // Use the now configured timer to create a PWM abstraction
    // If the frequency cannot be met given the constrained prescalers of the
    // passed counter in conjunction with the clock supplying the timer peripheral
    // an error is returned.
    // At 20MHz CLK_PER and a prescaler of 1024, a 1kHz period boils down to a duty cycle of 19969
    let mut pwm = t.pwm_hz(pwm_pins, 1.kHz(), WaveformGenerationMode::SingleSlope).unwrap();
    let max_duty = pwm.get_max_duty() as u16;

    // Enable all three channels and set a duty cycle
    pwm.set_duty(Channel::C1, 0);
    pwm.enable(Channel::C1);

    pwm.set_duty(Channel::C2, 5000);
    pwm.enable(Channel::C2);

    pwm.set_duty(Channel::C3, 1000);
    pwm.enable(Channel::C3);

    // Let's use TCB for an accurate delay
    // TCB only has prescalers of 1 and 2, so we have to make it tick at 10MHz
    // The delay() function is just going to let the timer expire multiple times
    // to achieve longer delays
    let mut d = FTimer::<_, 10000000>::new(dp.TCB0, clocks.into()).unwrap().delay();

    let mut i = 0;

    loop {
        pwm.set_duty(Channel::C1, i);

        i += 10;

        if i > max_duty {
            i = 0;
        }

        d.delay(1.millis());
    }
}
