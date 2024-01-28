#![no_std]
#![no_main]

use panic_halt as _;

use atxtiny_hal::prelude::*;
use atxtiny_hal::pac;
use atxtiny_hal::vref::{VrefExt, ReferenceVoltage};
use atxtiny_hal::dac::DacExt;

#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Constrain a few peripherals into our HAL types
    let clkctrl = dp.CLKCTRL.constrain();

    // Configure our clocks
    let _clocks = clkctrl.freeze();

    // Split the PORTA peripheral into its pins
    let a = dp.PORTA.split();

    // Grab the DAC output pin and disable its pullup
    let mut dacout = a.pa6.into_stateless_push_pull_output();
    dacout.internal_pull_up(Toggle::Off);

    // Set up the reference voltage
    // Note: the configured VREF can be cloned to pass it into the DAC and AC
    //       at the same time if needed
    let mut vref = dp.VREF.constrain();
    let dacref = vref.dac0(ReferenceVoltage::_4V34);

    let mut dac = dp.DAC0.constrain(dacref);
    dac.output_pin(dacout);
    dac.dac_set_value(128);
    let _dac = dac.enable();

    loop { }
}
