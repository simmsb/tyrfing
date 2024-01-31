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
use futures_util::pin_mut;
use core::future::Future;
use embassy_time::Duration;
use embassy_time::Instant;
use fugit::Rate;

pub mod gpio;
pub mod peripheral_ref;
pub mod time;
pub mod states;

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


// embassy_futures::select is more efficient the futures_util select that embassy_time uses
pub async fn with_timeout<F: Future>(timeout: Duration, fut: F) -> Result<F::Output, ()> {
    let timeout_fut = embassy_time::Timer::after(timeout);
    pin_mut!(fut);
    match embassy_futures::select::select(fut, timeout_fut).await {
        embassy_futures::select::Either::First(r) => Ok(r),
        embassy_futures::select::Either::Second(_) => Err(()),
    }
}


pub enum ButtonEvent {
    Click1,
    Click2,
    Click3,
    Click4,
    Click5,
    Click6,
    Click7,

    Hold1,
    Hold2,
    Hold3,
    Hold4,
    Hold5,
    Hold6,
    Hold7,

    HoldEnd,
}

impl ButtonEvent {
    fn click_from_count(n: u8) -> Self {
        match n {
            1 => Self::Click1,
            2 => Self::Click2,
            3 => Self::Click3,
            4 => Self::Click4,
            5 => Self::Click5,
            6 => Self::Click6,
            _ => Self::Click7,
        }
    }

    fn hold_from_count(n: u8) -> Self {
        match n {
            1 => Self::Hold1,
            2 => Self::Hold2,
            3 => Self::Hold3,
            4 => Self::Hold4,
            5 => Self::Hold5,
            6 => Self::Hold6,
            _ => Self::Hold7,
        }
    }
}

static BUTTON_EVENTS: embassy_sync::signal::Signal<
    embassy_sync::blocking_mutex::raw::ThreadModeRawMutex,
    ButtonEvent,
> = embassy_sync::signal::Signal::new();

#[derive(Clone, Copy)]
enum EventGenState {
    FirstClick,
    ForHigh { clicks: u8 },
    ForLow { clicks: u8 },
    HoldFinish,
}

// looks bad becasue we have to reuse the same code for as much of the awaits as
// possible
#[embassy_executor::task]
async fn event_generator(mut t: gpio::Pin<Input>) {
    let mut state = EventGenState::FirstClick;

    loop {
        let (wait_until, wait_for) = match state {
            EventGenState::FirstClick => (None, Edge::Rising),
            EventGenState::ForHigh { .. } => (Some(Duration::from_millis(300)), Edge::Rising),
            EventGenState::ForLow { .. } => (Some(Duration::from_millis(300)), Edge::Falling),
            EventGenState::HoldFinish => (None, Edge::Falling),
        };

        let now = Instant::now();
        let r = if let Some(wait_until) = wait_until {
            with_timeout(wait_until, t.wait(wait_for))
                .await
                .is_ok()
        } else {
            t.wait(wait_for).await;
            true
        };

        let (state_, evt) = match state {
            EventGenState::FirstClick => (EventGenState::ForLow { clicks: 1 }, None),
            EventGenState::ForHigh { clicks } => {
                if r {
                    (EventGenState::ForLow { clicks }, None)
                } else {
                    (
                        EventGenState::FirstClick,
                        Some(ButtonEvent::click_from_count(clicks)),
                    )
                }
            }
            EventGenState::ForLow { clicks } => {
                if r {
                    (
                        EventGenState::ForHigh {
                            clicks: clicks
                                + if now.elapsed() > Duration::from_millis(10) {
                                    1
                                } else {
                                    0
                                },
                        },
                        None,
                    )
                } else {
                    (
                        EventGenState::HoldFinish,
                        Some(ButtonEvent::hold_from_count(clicks)),
                    )
                }
            }
            EventGenState::HoldFinish => (EventGenState::FirstClick, Some(ButtonEvent::HoldEnd)),
        };
        state = state_;
        if let Some(evt) = evt {
            BUTTON_EVENTS.signal(evt);
        }
    }
}

#[embassy_executor::task]
async fn suck_events() {
    loop {
        states::on_ramping().await;
    }
}

#[embassy_executor::task]
async fn lol(p: atxtiny_hal::gpio::PC0<Input>) {
    let mut p = p.into_push_pull_output();
    p.toggle().unwrap_infallible();
    loop {
        // with_sleep_mode(SleepMode::Idle, || async {
        //     embassy_time::Timer::after_millis(100).await;
        // }).await;

        embassy_time::Timer::after_millis(500).await;
        // t.wait(Edge::Rising).await;
        p.toggle().unwrap_infallible();
        // t.wait(Edge::Falling).await;
    }
}

#[embassy_executor::task]
async fn watchdock_tickler(mut wd: WatchdogTimer) {
    // This should also do ADC stuff
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

    let _bod = dp
        .BOD
        .constrain()
        .sleep_mode(atxtiny_hal::bod::Mode::Sampled)
        .configure();

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
    watchdog.start(WatchdogTimeout::S1);
    watchdog.lock();

    spawner.must_spawn(watchdock_tickler(watchdog));

    spawner.must_spawn(event_generator(gpio::Pin::new(
        b.pb2.downgrade().downgrade(),
    )));
    spawner.must_spawn(lol(c.pc0));
    spawner.must_spawn(suck_events());

    set_sleep_mode(SleepMode::PowerDown);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();

    loop {}
}
