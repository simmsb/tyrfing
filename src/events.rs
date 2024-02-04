use core::future::Future;
use futures_util::pin_mut;
use embassy_time::Instant;
use embassy_time::Duration;
use atxtiny_hal::gpio::Edge;
use atxtiny_hal::gpio::Input;
use atxtiny_hal;
use embassy_sync;

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

#[derive(Clone, Copy)]
pub enum EventGenState {
    FirstClick,
    ForHigh { clicks: u8 },
    ForLow { clicks: u8 },
    HoldFinish,
}

// looks bad becasue we have to reuse the same code for as much of the awaits as
// possible
#[embassy_executor::task]
pub async fn event_generator(t: atxtiny_hal::gpio::PB2<Input>) {
    let mut t = crate::gpio::Pin::new(t.into_pull_up_input());
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

        let r = crate::with_timeout::with_timeout(wait_until, wait_f)
            .await
            .is_ok();

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
