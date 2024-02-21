#![no_std]
#![no_main]
#![feature(const_trait_impl)]
#![feature(type_alias_impl_trait)]
#![feature(abi_avr_interrupt)]
#![feature(rustc_attrs)]
#![feature(generic_arg_infer)]
#![feature(const_fn_floating_point_arithmetic)]
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
use avr_device::attiny1616::slpctrl::ctrla::SMODE_A;
use fugit::Rate;

pub mod adc;
pub mod events;
pub mod gpio;
pub mod logger;
pub mod nonatomic;
pub mod peripheral_ref;
pub mod power;
pub mod sensing;
pub mod sleep;
pub mod states;
pub mod time;
pub mod with_timeout;

use adc::AdcExt as _;

#[embassy_executor::task]
async fn suck_events() {
    loop {
        states::on_ramping().await;
    }
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let dp = pac::Peripherals::take().unwrap();
    let clkctrl = dp.CLKCTRL.constrain();
    #[allow(unused)]
    let portmux = dp.PORTMUX.constrain();
    #[allow(unused)]
    let clocks = clkctrl
        .per_clk_freq(Rate::<u32, _, _>::MHz(20u32) / 10)
        .freeze();

    let _bod = dp
        .BOD
        .constrain()
        .sleep_mode(atxtiny_hal::bod::Mode::Sampled)
        .configure();

    let a = dp.PORTA.split();
    let b = dp.PORTB.split();
    let c = dp.PORTC.split();

    time::init_system_time(dp.RTC, None);

    let mut vref = dp.VREF.constrain();

    let adc = dp.ADC0.constrain(vref.adc0(ReferenceVoltage::_1V10));

    #[cfg(feature = "logging")]
    {
        let pinset = (
            a.pa2.into_peripheral::<avr_device::attiny1616::USART0>(),
            a.pa1.into_peripheral(),
        )
            .mux(&portmux);
        let usart = atxtiny_hal::serial::Serial::new(dp.USART0, pinset, 115200u32.bps(), clocks);
        avr_device::interrupt::free(|t| {
            let mut u = logger::SERIAL.borrow(t).borrow_mut();
            *u = Some(usart);
        });
    }

    let mut watchdog = dp.WDT.constrain();
    watchdog.start(WatchdogTimeout::S4);
    watchdog.lock();

    spawner.must_spawn(sensing::watchdock_tickler(watchdog, c.pc0, adc));

    spawner.must_spawn(events::debouncer(c.pc3));
    spawner.must_spawn(events::event_generator());
    spawner.must_spawn(suck_events());

    let mut dac_pin = a.pa6.into_stateless_push_pull_output();
    dac_pin.internal_pull_up(Toggle::Off);

    let dac0vref = vref.dac0(ReferenceVoltage::_0V55);

    let mut dac = dp.DAC0.constrain(dac0vref);
    dac.output_pin(dac_pin);
    dac.dac_set_value(0);
    let dac = dac.enable();

    vref.dac0(ReferenceVoltage::_1V50);

    let power_paths =
        power::PowerPaths::new(dac0vref, dac, a.pa7, b.pb5, b.pb4, b.pb3, b.pb2, b.pb0);
    spawner.must_spawn(power::power_controller(power_paths));

    sleep::set_sleep_mode(SMODE_A::PDOWN);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();

    loop {}
}
