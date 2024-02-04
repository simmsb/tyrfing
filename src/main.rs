#![no_std]
#![no_main]
#![feature(const_trait_impl)]
#![feature(type_alias_impl_trait)]
#![feature(abi_avr_interrupt)]
#![feature(rustc_attrs)]
#![feature(generic_arg_infer)]
#![allow(internal_features)]

use atxtiny_hal::bod::BodExt;
use atxtiny_hal::dac::DacExt;
use atxtiny_hal::pac;
use atxtiny_hal::prelude::*;
use atxtiny_hal::vref::ReferenceVoltage;
use atxtiny_hal::vref::VrefExt;
use avr_device::attiny1616::slpctrl::ctrla::SMODE_A;
use fugit::Rate;

#[cfg(feature = "logging")]
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};

pub mod adc;
pub mod events;
pub mod gpio;
pub mod peripheral_ref;
pub mod sensing;
pub mod sleep;
pub mod states;
pub mod time;
pub mod with_timeout;

use adc::AdcExt as _;

#[cfg(feature = "logging")]
pub static SERIAL: Mutex<
    ThreadModeRawMutex,
    Option<
        atxtiny_hal::serial::Serial<
            avr_device::attiny1616::USART0,
            atxtiny_hal::serial::UartPinset<
                avr_device::attiny1616::USART0,
                atxtiny_hal::gpio::Pin<atxtiny_hal::gpio::Porta, atxtiny_hal::gpio::U<2>, atxtiny_hal::gpio::Input>,
                atxtiny_hal::gpio::Pin<atxtiny_hal::gpio::Porta, atxtiny_hal::gpio::U<1>, atxtiny_hal::gpio::Output<atxtiny_hal::gpio::Stateless>>,
            >,
        >,
    >,
> = Mutex::new(None);

#[cfg(feature = "logging")]
#[macro_export]
macro_rules! serial_println {
    ($($arg:tt)*) => {{
        let mut s = crate::SERIAL.lock().await;
        if let Some(w) = s.as_mut() {
            let _ = ::ufmt::uwriteln!(w, $($arg)*);
        }
    }}
}

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

    //
    let mut dac_pin = a.pa6.into_stateless_push_pull_output();
    dac_pin.internal_pull_up(Toggle::Off);

    let mut vref = dp.VREF.constrain();
    let dac0vref = vref.dac0(ReferenceVoltage::_0V55);

    let mut dac = dp.DAC0.constrain(dac0vref);
    dac.output_pin(dac_pin);
    dac.dac_set_value(128);
    let _dac = dac.enable();

    vref.dac0(ReferenceVoltage::_1V50);

    let adc = dp.ADC0.constrain(vref.adc0(ReferenceVoltage::_1V10));

    #[cfg(feature = "logging")]
    {
        let pinset = (a.pa2.into_peripheral::<avr_device::attiny1616::USART0>(), a.pa1.into_peripheral()).mux(&portmux);
        let usart = atxtiny_hal::serial::Serial::new(dp.USART0, pinset, 115200u32.bps(), clocks);
        *SERIAL.lock().await = Some(usart);
    }

    let mut watchdog = dp.WDT.constrain();
    watchdog.start(WatchdogTimeout::S4);
    watchdog.lock();

    spawner.must_spawn(sensing::watchdock_tickler(watchdog, c.pc0, adc));

    spawner.must_spawn(events::event_generator(b.pb2));
    spawner.must_spawn(suck_events());

    sleep::set_sleep_mode(SMODE_A::PDOWN);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();

    loop {}
}
