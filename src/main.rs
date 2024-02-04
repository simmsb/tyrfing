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
use atxtiny_hal::vref::ReferenceVoltage;
use atxtiny_hal::vref::VrefExt;
use atxtiny_hal::watchdog::WatchdogTimer;
use avr_device::attiny1616::slpctrl::ctrla::SMODE_A;
use avr_device::attiny1616::ADC0;
use avr_device::attiny1616::SLPCTRL;
use avr_hal_generic::prelude::*;
use core::future::Future;
use embassy_time::Duration;
use embassy_time::Instant;
use fugit::Rate;
use futures_util::pin_mut;

#[cfg(feature = "logging")]
use atxtiny_hal::{
    gpio::{Output, Porta, Stateless},
    serial::Serial,
};
#[cfg(feature = "logging")]
use avr_device::attiny1616::USART0;
#[cfg(feature = "logging")]
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};

pub mod adc;
pub mod gpio;
pub mod peripheral_ref;
pub mod states;
pub mod time;
pub mod with_timeout;

use adc::AdcExt as _;

#[cfg(feature = "logging")]
pub static SERIAL: Mutex<
    ThreadModeRawMutex,
    Option<
        Serial<
            USART0,
            atxtiny_hal::serial::UartPinset<
                USART0,
                atxtiny_hal::gpio::Pin<Porta, atxtiny_hal::gpio::U<2>, Input>,
                atxtiny_hal::gpio::Pin<Porta, atxtiny_hal::gpio::U<1>, Output<Stateless>>,
            >,
        >,
    >,
> = Mutex::new(None);

#[cfg(feature = "logging")]
macro_rules! serial_println {
    ($($arg:tt)*) => {{
        let mut s = crate::SERIAL.lock().await;
        if let Some(w) = s.as_mut() {
            let _ = ::ufmt::uwriteln!(w, $($arg)*);
        }
    }}
}

#[export_name = "__sleep"]
unsafe fn sleep() {
    SLPCTRL::steal().ctrla().modify(|_, w| w.sen().set_bit());
    avr_device::asm::sleep();
    SLPCTRL::steal().ctrla().modify(|_, w| w.sen().clear_bit());
}

macro_rules! with_sleep_mode {
    ($mode:expr, $e:expr) => {{
        let current = crate::get_sleep_mode();
        crate::set_sleep_mode($mode);
        let result = $e;
        crate::set_sleep_mode(current);
        result
    }};
}

pub fn get_sleep_mode() -> SMODE_A {
    unsafe {
        SLPCTRL::steal()
            .ctrla()
            .read()
            .smode()
            .variant()
            .unwrap_unchecked()
    }
}

pub fn set_sleep_mode(mode: SMODE_A) {
    unsafe {
        SLPCTRL::steal()
            .ctrla()
            .modify(|_, w| w.smode().variant(mode));
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
async fn event_generator(t: atxtiny_hal::gpio::PB2<Input>) {
    let mut t = gpio::Pin::new(t.into_pull_up_input());
    let mut state = EventGenState::FirstClick;

    loop {
        let (wait_until, wait_for) = match state {
            EventGenState::FirstClick => (None, Edge::Rising),
            EventGenState::ForHigh { .. } => (Some(Duration::from_millis(300)), Edge::Rising),
            EventGenState::ForLow { .. } => (Some(Duration::from_millis(300)), Edge::Falling),
            EventGenState::HoldFinish => (None, Edge::Falling),
        };

        let now = Instant::now();

        let wait_f = t.wait(wait_for);
        pin_mut!(wait_f);
        let wait_f = wait_f as core::pin::Pin<&mut dyn Future<Output = ()>>;

        let r = with_timeout::with_timeout(wait_until, wait_f).await.is_ok();

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

pub struct Smoother(pub u16);

impl Smoother {
    fn update(&mut self, value: u16) {
        let diff = (value / 8) as i16 - (self.0 / 8) as i16;

        self.0 = self.0.saturating_add_signed(diff);
    }
}

#[embassy_executor::task]
async fn watchdock_tickler(
    mut wd: WatchdogTimer,
    p: atxtiny_hal::gpio::PC0<Input>,
    mut adc: adc::Adc<ADC0, adc::Disabled>,
) {
    let mut p = p.into_push_pull_output();
    p.set_high().unwrap_infallible();

    let mut temp_smoother = Smoother(1970); // 18 c
    let mut volt_smoother = Smoother(1380); // 5.2v (?)

    loop {
        let mut adc_ = adc.enable();
        adc_.run_in_standby(true);

        let (t, v) = with_sleep_mode!(SMODE_A::STANDBY, {
            let t = adc_.read_temp().await.smooth_with(&mut temp_smoother);
            let v = adc_.read_voltage().await.smooth_with(&mut volt_smoother);

            (t, v)
        });

        adc = adc_.disable();
        wd.feed();

        #[cfg(feature = "logging")]
        serial_println!(
            "Temp: {} ({}), Volts: {} ({})|||",
            t.celcius(),
            t.0,
            v.volts_times_100(),
            v.0
        );

        p.toggle().unwrap_infallible();

        embassy_time::Timer::after_millis(500).await;
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
        let pinset = (a.pa2.into_peripheral::<USART0>(), a.pa1.into_peripheral()).mux(&portmux);
        let usart = atxtiny_hal::serial::Serial::new(dp.USART0, pinset, 115200u32.bps(), clocks);
        *SERIAL.lock().await = Some(usart);
    }

    let mut watchdog = dp.WDT.constrain();
    watchdog.start(WatchdogTimeout::S4);
    watchdog.lock();

    spawner.must_spawn(watchdock_tickler(watchdog, c.pc0, adc));

    spawner.must_spawn(event_generator(b.pb2));
    spawner.must_spawn(suck_events());

    set_sleep_mode(SMODE_A::PDOWN);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();

    loop {}
}
