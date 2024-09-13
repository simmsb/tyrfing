#![no_std]
#![no_main]
#![feature(const_trait_impl)]
#![feature(type_alias_impl_trait)]
#![feature(abi_avr_interrupt)]
#![feature(rustc_attrs)]
#![feature(inline_const)]
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
use avr_hal_generic::prelude::_unwrap_infallible_UnwrapInfallible;
use fugit::Rate;

pub mod adc;
pub mod aux;
pub mod events;
pub mod gpio;
pub mod logger;
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

    time::init_system_time(dp.RTC);

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

    spawner.must_spawn(sensing::watchdock_tickler(watchdog, adc));

    spawner.must_spawn(events::debouncer(c.pc3));
    spawner.must_spawn(events::event_generator());
    spawner.must_spawn(states::torch_ui());

    let mut dac_pin = a.pa6.into_stateless_push_pull_output();
    dac_pin.internal_pull_up(Toggle::Off);

    let dac0vref = vref.dac0(ReferenceVoltage::_0V55);

    let mut dac = dp.DAC0.constrain(dac0vref);
    dac.output_pin(dac_pin);
    dac.dac_set_value(0);
    let dac = dac.enable();

    vref.dac0(ReferenceVoltage::_1V50);

    let power_paths = power::PowerPaths::new(dac0vref, dac, a.pa7, b.pb5, b.pb4, b.pb3, b.pb2);
    spawner.must_spawn(power::power_controller(power_paths));

    aux::setup(
        spawner, dp.TCA0, dp.TCB0, clocks, &portmux, b.pb0, b.pb1, c.pc0,
    );

    sleep::set_sleep_mode(SMODE_A::PDOWN);

    powersave_pins!(a.pa0, a.pa3, a.pa4, a.pa5);
    powersave_pins!(b.pb6, b.pb7);
    powersave_pins!(c.pc1, c.pc2, c.pc4, c.pc5);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();

    let mut p = unsafe { atxtiny_hal::avr_device::attiny1616::PORTC::steal() }
        .split()
        .pc1
        .into_push_pull_output();

    loop {
        p.toggle().unwrap_infallible();

        atxtiny_hal::avr_device::asm::delay_cycles(1000000);
    }
}
