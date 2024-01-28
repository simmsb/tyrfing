#![no_std]
#![no_main]

use panic_halt as _;

use atxtiny_hal::prelude::*;
use atxtiny_hal::pac;
use atxtiny_hal::vref::{VrefExt, ReferenceVoltage};
use atxtiny_hal::ac::{ComparatorExt, Config};

#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Constrain a few peripherals into our HAL types
    let clkctrl = dp.CLKCTRL.constrain();

    // Configure our clocks
    let _clocks = clkctrl.freeze();

    // Split the PORTA peripheral into its pins
    let a = dp.PORTA.split();

    // Setup VREF for DAC to 2.5V
    // This reference voltage can be passed into the AC negative input to
    // compare against
    let mut vref = dp.VREF.constrain();
    let dacref = vref.dac0(ReferenceVoltage::_2V50);

    // Grab AINP0
    let ainp0 = a.pa7.into_analog_input();

    // Grab the AC output pin and disable its pullup
    let mut acout = a.pa5.into_stateless_push_pull_output();
    acout.internal_pull_up(Toggle::Off);

    // Create a comparator
    let ac = dp.AC0.comparator(ainp0, dacref, Config {
        hysteresis: atxtiny_hal::ac::Hysteresis::_50mV,
        ..Default::default()
    });
    ac.output_pin(acout);
    let _ac = ac.enable();

    loop { }
}
