#![no_std]
#![no_main]
#![feature(const_trait_impl)]
#![feature(type_alias_impl_trait)]
#![feature(abi_avr_interrupt)]
#![feature(rustc_attrs)]
#![feature(generic_arg_infer)]
#![allow(internal_features)]

use atxtiny_hal::dac::DacExt;
use atxtiny_hal::gpio::Edge;
use atxtiny_hal::gpio::Input;
use atxtiny_hal::pac;
use atxtiny_hal::prelude::*;
use atxtiny_hal::slpctrl::SleepMode;
use atxtiny_hal::vref::ReferenceVoltage;
use atxtiny_hal::vref::VrefExt;
use atxtiny_hal::watchdog::WatchdogTimer;
use avr_device::attiny1616::SLPCTRL;
use avr_hal_generic::prelude::_unwrap_infallible_UnwrapInfallible;
use core::future::Future;
use fugit::Rate;

pub mod gpio;
pub mod peripheral_ref;
pub mod time;

#[export_name = "__sleep"]
unsafe fn sleep() {
    SLPCTRL::steal().ctrla().modify(|_, w| w.sen().set_bit());
    avr_device::asm::sleep();
    SLPCTRL::steal().ctrla().modify(|_, w| w.sen().clear_bit());
}

pub async fn with_sleep_mode<F, Fut>(mode: SleepMode, f: F) -> Fut::Output
where
    F: FnOnce() -> Fut,
    Fut: Future,
{
    let current = get_sleep_mode();
    set_sleep_mode(mode);
    let result = f().await;
    set_sleep_mode(current);
    result
}

pub fn get_sleep_mode() -> SleepMode {
    unsafe {
        SLPCTRL::steal()
            .ctrla()
            .read()
            .smode()
            .variant()
            .unwrap_unchecked()
            .into()
    }
}

pub fn set_sleep_mode(mode: SleepMode) {
    unsafe {
        SLPCTRL::steal()
            .ctrla()
            .modify(|_, w| w.smode().variant(mode.into()));
    }
}

#[embassy_executor::task]
async fn lol(mut t: gpio::Pin<Input>, p: atxtiny_hal::gpio::PC0<Input>) {
    let mut p = p.into_push_pull_output();
    p.toggle().unwrap_infallible();
    loop {
        // embassy_futures::yield_now().await;
        embassy_time::Timer::after_millis(100).await;

        // with_sleep_mode(SleepMode::Idle, || async {
        //     embassy_time::Timer::after_millis(100).await;
        // }).await;

        // embassy_time::Timer::after_millis(500).await;
        // t.wait(Edge::Rising).await;
        p.toggle().unwrap_infallible();
        // t.wait(Edge::Falling).await;
    }
}

#[embassy_executor::task]
async fn watchdock_tickler(mut wd: WatchdogTimer) {
    loop {
        wd.feed();

        embassy_time::Timer::after_millis(500).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let dp = pac::Peripherals::take().unwrap();
    let clkctrl = dp.CLKCTRL.constrain();
    let _portmux = dp.PORTMUX.constrain();
    let clocks = clkctrl
        .per_clk_freq(Rate::<u32, _, _>::MHz(20u32) / 10)
        .freeze();

    let a = dp.PORTA.split();
    let b = dp.PORTB.split();
    let c = dp.PORTC.split();

    // let mut pin = c.pc0.into_push_pull_output();
    // let _ = pin.set_high();

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

    // embassy_time::Timer::after_secs(3).await;

    let mut watchdog = dp.WDT.constrain();
    watchdog.start(WatchdogTimeout::S4);
    watchdog.lock();

    spawner.must_spawn(watchdock_tickler(watchdog));

    spawner.must_spawn(lol(gpio::Pin::new(b.pb2.downgrade().downgrade()), c.pc0));

    set_sleep_mode(SleepMode::PowerDown);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();

    loop {}
}
