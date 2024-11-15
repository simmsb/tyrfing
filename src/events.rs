use atxtiny_hal::embedded_hal::digital::InputPin;
use atxtiny_hal::embedded_hal::digital::OutputPin;

use atxtiny_hal::gpio::Edge;
use atxtiny_hal::gpio::GpioExt;
use atxtiny_hal::gpio::Input;
use avr_hal_generic::prelude::_unwrap_infallible_UnwrapInfallible;

use embassy_time::Duration;

#[derive(PartialEq, Clone, Copy)]
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
    pub fn click_from_count(n: u8) -> Self {
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

    pub fn hold_from_count(n: u8) -> Self {
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

pub static BUTTON_EVENTS: embassy_sync::signal::Signal<
    embassy_sync::blocking_mutex::raw::ThreadModeRawMutex,
    ButtonEvent,
> = embassy_sync::signal::Signal::new();

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Depress,
    Press,
}

static BUTTON_STATES: embassy_sync::signal::Signal<
    embassy_sync::blocking_mutex::raw::ThreadModeRawMutex,
    ButtonState,
> = embassy_sync::signal::Signal::new();

pub static LOCKOUT_BUTTON_STATES: embassy_sync::signal::Signal<
    embassy_sync::blocking_mutex::raw::ThreadModeRawMutex,
    ButtonState,
> = embassy_sync::signal::Signal::new();

#[derive(Clone, Copy)]
pub enum EventGenState {
    FirstClick,
    ForHigh { clicks: u8 },
    ForLow { clicks: u8 },
    HoldFinish,
}

#[embassy_executor::task]
pub async fn debouncer(t: atxtiny_hal::gpio::PC3<Input>) {
    let mut t = crate::gpio::Pin::new(t.into_floating_input());
    let mut l = unsafe {
        atxtiny_hal::avr_device::attiny1616::PORTC::steal()
            .split()
            .pc1
            .into_push_pull_output()
    };

    loop {
        l.set_low().unwrap_infallible();

        // wait for a pin event on the button pin (either a press, or bouncing)
        t.wait(Edge::Falling).await;
        let v = t.pin().is_low().unwrap_infallible();

        // if the button isn't pressed, abort
        if !v {
            continue;
        }

        embassy_time::Timer::after_millis(16).await;

        // if the button is still pressed after 16ms, consider it debounced and pressed
        if t.pin().is_low().unwrap_infallible() {
            BUTTON_STATES.signal(ButtonState::Press);
            LOCKOUT_BUTTON_STATES.signal(ButtonState::Press);
            crate::serial_println!("Got button: {}", v);
            // crate::power::set_level_gradual(if v { 255 } else { 0 });
        } else {
            continue;
        }
        l.set_high().unwrap_infallible();

        // once pressed, we poll the button for depresses since sometimes the
        // edge interrupt can be missed
        loop {
            embassy_time::Timer::after_millis(16).await;
            // if the button is still pressed, do nothing
            if t.pin().is_low().unwrap_infallible() {
                continue;
            }

            embassy_time::Timer::after_millis(16).await;

            // if the button has been depressed for two cycles, consider it
            // debounced and depressed
            if t.pin().is_high().unwrap_infallible() {
                BUTTON_STATES.signal(ButtonState::Depress);
                LOCKOUT_BUTTON_STATES.signal(ButtonState::Depress);
                break;
            }
        }
    }
}

// looks bad becasue we have to reuse the same code for as much of the awaits as
// possible
#[embassy_executor::task]
pub async fn event_generator() {
    let mut state = EventGenState::FirstClick;
    loop {
        let (wait_until, expecting) = match state {
            EventGenState::FirstClick => (None, ButtonState::Press),
            EventGenState::ForHigh { .. } => (Some(Duration::from_millis(300)), ButtonState::Press),
            EventGenState::ForLow { .. } => {
                (Some(Duration::from_millis(300)), ButtonState::Depress)
            }
            EventGenState::HoldFinish => (None, ButtonState::Depress),
        };

        let r = crate::with_timeout::with_timeout(wait_until, BUTTON_STATES.wait()).await;

        let r = match r {
            Ok(state) if state == expecting => true,
            Ok(_) => {
                state = EventGenState::FirstClick;
                continue;
            }
            Err(_) => false,
        };

        // r: true if pressed, false if held

        let (state_, evt) = match state {
            EventGenState::FirstClick => (EventGenState::ForLow { clicks: 1 }, None),
            EventGenState::ForHigh { clicks } => {
                if r {
                    (EventGenState::ForLow { clicks: clicks + 1 }, None)
                } else {
                    (
                        EventGenState::FirstClick,
                        Some(ButtonEvent::click_from_count(clicks)),
                    )
                }
            }
            EventGenState::ForLow { clicks } => {
                if r {
                    (EventGenState::ForHigh { clicks }, None)
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
