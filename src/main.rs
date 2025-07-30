#![no_std]
#![no_main]
#![feature(const_trait_impl)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(abi_avr_interrupt)]
#![feature(rustc_attrs)]
// #![feature(
//     const_option,
//     const_mut_refs,
//     maybe_uninit_uninit_array,
//     maybe_uninit_array_assume_init,
//     const_maybe_uninit_array_assume_init,
//     const_maybe_uninit_uninit_array,
//     const_maybe_uninit_write
// )]
#![allow(internal_features)]

use atxtiny_hal::bod::BodExt;
use atxtiny_hal::dac::DacExt;
use atxtiny_hal::pac;
use atxtiny_hal::prelude::*;

use atxtiny_hal::vref::ReferenceVoltage;
use atxtiny_hal::vref::VrefExt;
use avr_device::avr32dd20::slpctrl::ctrla::SMODE_A;
use avr_hal_generic::port::mode::Output;
use avr_hal_generic::prelude::_unwrap_infallible_UnwrapInfallible;
use fugit::Rate;

pub mod adc;
pub mod aux;
pub mod events;
pub mod gpio;
pub mod nonatomic;
pub mod peripheral_ref;
pub mod power;
pub mod power_levels;
pub mod sensing;
pub mod sleep;
pub mod states;
pub mod time;
pub mod with_timeout;

use adc::AdcExt as _;

macro_rules! powersave_pins {
    ($($pin:expr),* $(,)?) => {
        {
            $(::core::mem::forget($pin.into_pull_up_input());)*
        }
    };
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let dp = pac::Peripherals::take().unwrap();
    let clkctrl = dp.CLKCTRL.constrain();
    #[allow(unused)]
    let portmux = dp.PORTMUX.constrain();
    #[allow(unused)]
    let clocks = clkctrl.per_clk_freq(Rate::<u32, _, _>::MHz(2u32)).freeze();

    blink_once();

    let _bod = dp
        .BOD
        .constrain()
        .sleep_mode(atxtiny_hal::bod::Mode::Sampled)
        .configure();

    let a = dp.PORTA.split();
    let c = dp.PORTC.split();
    let d = dp.PORTD.split();

    time::init_system_time(dp.RTC);

    let mut vref = dp.VREF.constrain();

    let adc = dp.ADC0.constrain(vref.adc0(ReferenceVoltage::_1V024));

    let mut watchdog = dp.WDT.constrain();
    watchdog.start(WatchdogTimeout::S4);
    watchdog.lock();

    spawner.must_spawn(sensing::watchdock_tickler(watchdog, adc));

    // spawner.must_spawn(blink(a.pa4));
    spawner.must_spawn(events::debouncer(d.pd4));
    spawner.must_spawn(events::event_generator());
    spawner.must_spawn(states::torch_ui());

    let mut dac_pin = d.pd6.into_stateless_push_pull_output();
    dac_pin.internal_pull_up(Toggle::Off);

    let dac0vref = vref.dac0(ReferenceVoltage::_1V024);

    let mut dac = dp.DAC0.constrain(dac0vref);
    dac.output_pin(dac_pin);
    dac.dac_set_value(0);
    let dac = dac.enable();

    vref.dac0(ReferenceVoltage::_1V024);

    let power_paths = power::PowerPaths::new(dac0vref, dac, a.pa7, a.pa6, a.pa5, c.pc1);
    spawner.must_spawn(power::power_controller(power_paths));

    let portmuxparts = portmux.split();

    aux::setup(
        spawner,
        dp.TCA0,
        clocks,
        portmuxparts.tca_routes,
        a.pa1,
        a.pa2,
        a.pa3,
    );

    sleep::set_sleep_mode(SMODE_A::PDOWN);
}

// #[embassy_executor::task]
// async fn blink(p: atxtiny_hal::gpio::PA4<atxtiny_hal::gpio::Input>) {
//     let mut p = p.into_push_pull_output();
//     loop {
//         p.toggle().unwrap_infallible();

//         embassy_time::Timer::after_millis(300).await;
//     }
// }

fn blink_once() {
    let mut p = unsafe { atxtiny_hal::avr_device::avr32dd20::PORTA::steal() }
        .split()
        .pa4
        .into_push_pull_output();

    p.set_high().unwrap_infallible();

    atxtiny_hal::avr_device::asm::delay_cycles(1000000);

    p.set_low().unwrap_infallible();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();

    let mut p = unsafe { atxtiny_hal::avr_device::avr32dd20::PORTA::steal() }
        .split()
        .pa4
        .into_push_pull_output();

    loop {
        p.toggle().unwrap_infallible();

        atxtiny_hal::avr_device::asm::delay_cycles(1000000);
    }
}
