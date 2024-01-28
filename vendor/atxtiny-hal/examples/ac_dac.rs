// FIXME: TODO

#![no_std]
#![no_main]

use panic_halt as _;

use atxtiny_hal::prelude::*;
use atxtiny_hal::pac;
use atxtiny_hal::vref::{VrefExt, ReferenceVoltage};
use atxtiny_hal::dac::DacExt;
use atxtiny_hal::ac::{ComparatorExt, Config};
use atxtiny_hal::timer::FTimer;

#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Constrain a few peripherals into our HAL types
    let clkctrl = dp.CLKCTRL.constrain();

    // Configure our clocks
    let clocks = clkctrl.freeze();

    // Split the PORTA and PORTB peripheral into its pins
    let a = dp.PORTA.split();
    let b = dp.PORTB.split();

    // Blinky things
    let mut led = b.pb6.into_push_pull_output();

    // Setup VREF for DAC to 2.5V
    let mut vref = dp.VREF.constrain();
    let dacref = vref.dac0(ReferenceVoltage::_2V50);

    // Setup the DAC
    let mut dac = dp.DAC0.constrain(dacref);
    dac.dac_set_value(0);
    let dac = dac.enable();

    // Lock the DAC into an enabled state, now it cannot be disabled anymore
    // but we can also get output objects that can be passed into other
    // peripherals like the negative AC0 pin
    let mut dac = dac.lock_enable();
    
    // Grab AINP0
    let ainp0 = a.pa7.into_analog_input();

    // Grab the DAC as AINN0
    let ainn0 = dac.dac_get_ac0_input();

    // Grab the AC output pin and disable its pullup
    let mut acout = a.pa5.into_stateless_push_pull_output();
    acout.internal_pull_up(Toggle::Off);

    // Create a comparator
    let ac = dp.AC0.comparator(ainp0, ainn0, Config {
        hysteresis: atxtiny_hal::ac::Hysteresis::_50mV,
        ..Default::default()
    });
    ac.output_pin(acout);
    let _ac = ac.enable();

    // Create a delay timer
    let t = FTimer::<_, 312500>::new(dp.TCA0, clocks).unwrap();
    let mut d = t.delay();

    let mut i: u8 = 0;

    loop {
        led.toggle().unwrap();
        dac.dac_set_value(i);
        
        i = i.wrapping_add(4);
        d.delay(50.millis());
    }
}
