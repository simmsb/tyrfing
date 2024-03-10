use core::cell::Cell;

use embassy_time::{Duration, Instant};

use crate::{
    events::{ButtonEvent, BUTTON_EVENTS},
    nonatomic::NonAtomicBool,
    power::blink,
    with_timeout::with_timeout,
};

pub static IS_TORCH_UNLOCKED: NonAtomicBool = NonAtomicBool::new(false);

const DEFAULT_LEVEL: u8 = 27;

#[embassy_executor::task]
pub async fn torch_ui() {
    let mut saved_level = DEFAULT_LEVEL;

    loop {
        let unlocked = IS_TORCH_UNLOCKED.load();

        if unlocked {
            let evt = crate::with_timeout::with_timeout(
                Some(Duration::from_secs(60 * 3)),
                BUTTON_EVENTS.wait(),
            )
            .await;
            let Ok(evt) = evt else {
                blink(1).await;
                IS_TORCH_UNLOCKED.store(false);
                continue;
            };
            match evt {
                ButtonEvent::Click1 | ButtonEvent::Hold1 => {
                    saved_level = on_ramping(if evt == ButtonEvent::Click1 {
                        saved_level
                    } else {
                        DEFAULT_LEVEL
                    })
                    .await;
                }
                ButtonEvent::Hold2 => {
                    on_fadeout().await;
                }
                ButtonEvent::Hold3 => {
                    on_strobe().await;
                }
                ButtonEvent::Click4 => {
                    blink(1).await;
                    IS_TORCH_UNLOCKED.store(false);
                }
                _ => {}
            }
        } else {
            let evt = BUTTON_EVENTS.wait().await;
            match evt {
                ButtonEvent::Click3 => {
                    blink(1).await;
                    IS_TORCH_UNLOCKED.store(true);
                    saved_level = DEFAULT_LEVEL;
                }
                _ => {}
            }
        }
    }
}

async fn on_strobe() {
    let level = Cell::new(DEFAULT_LEVEL);
    let period = Cell::new(Duration::from_hz(100));

    let strobe = async {
        loop {
            embassy_time::Timer::after(period.get()).await;
            crate::power::set_level(level.get())
        }
    };

    let control = async {
        let mut last_hold_release = Instant::now();
        loop {
            match BUTTON_EVENTS.wait().await {
                crate::events::ButtonEvent::Click1 => {
                    return;
                }
                crate::events::ButtonEvent::Hold1 => {
                    let direction = if last_hold_release.elapsed() > Duration::from_millis(500) {
                        1
                    } else {
                        -1
                    };
                    loop {
                        if with_timeout(Some(Duration::from_millis(16)), BUTTON_EVENTS.wait())
                            .await
                            .is_err()
                        {
                            level.set(level.get().saturating_add_signed(direction));
                        } else {
                            break;
                        }
                    }
                    if direction == 1 {
                        last_hold_release = Instant::now();
                    }
                }
                crate::events::ButtonEvent::Hold2 => loop {
                    if with_timeout(Some(Duration::from_millis(16)), BUTTON_EVENTS.wait())
                        .await
                        .is_err()
                    {
                        level.set(level.get().saturating_sub(1));
                    } else {
                        break;
                    }
                },
                crate::events::ButtonEvent::Hold3 => loop {
                    if with_timeout(Some(Duration::from_millis(32)), BUTTON_EVENTS.wait())
                        .await
                        .is_err()
                    {
                        period.set(Duration::from_ticks(
                            period.get().as_ticks().saturating_sub(1),
                        ));
                    } else {
                        break;
                    }
                },
                crate::events::ButtonEvent::Hold4 => loop {
                    if with_timeout(Some(Duration::from_millis(32)), BUTTON_EVENTS.wait())
                        .await
                        .is_err()
                    {
                        period.set(Duration::from_ticks(
                            period.get().as_ticks().saturating_add(1),
                        ));
                    } else {
                        break;
                    }
                },
                _ => {}
            }
        }
    };

    embassy_futures::select::select(strobe, control).await;

    crate::power::set_level_gradual(0);
}

async fn on_fadeout() {
    let level = Cell::new(DEFAULT_LEVEL);
    let expiry = Cell::new(Instant::now() + Duration::from_secs(60 * 4));

    let fade = async {
        loop {
            embassy_time::Timer::after_millis(100).await;

            let Some(time_left) = expiry.get().checked_duration_since(Instant::now()) else {
                break;
            };

            let brightness = if time_left > Duration::from_secs(60 * 4) {
                level.get()
            } else {
                let remaining = time_left.as_ticks() / Duration::from_secs(60 * 4).as_ticks();
                let remaining = (remaining >> 24) as u8;
                cichlid::math::scale_u8(level.get(), 255 - remaining)
            };

            crate::power::set_level_gradual(brightness)
        }
    };

    let control = async {
        let mut last_hold_release = Instant::now();
        loop {
            match BUTTON_EVENTS.wait().await {
                crate::events::ButtonEvent::Click1 => {
                    return;
                }
                crate::events::ButtonEvent::Hold1 => {
                    let direction = if last_hold_release.elapsed() > Duration::from_millis(500) {
                        1
                    } else {
                        -1
                    };
                    loop {
                        if with_timeout(Some(Duration::from_millis(16)), BUTTON_EVENTS.wait())
                            .await
                            .is_err()
                        {
                            level.set(level.get().saturating_add_signed(direction));
                        } else {
                            break;
                        }
                    }
                    if direction == 1 {
                        last_hold_release = Instant::now();
                    }
                }
                crate::events::ButtonEvent::Hold2 => loop {
                    if with_timeout(Some(Duration::from_millis(16)), BUTTON_EVENTS.wait())
                        .await
                        .is_err()
                    {
                        level.set(level.get().saturating_sub(1));
                    } else {
                        break;
                    }
                },
                crate::events::ButtonEvent::Hold3 => loop {
                    if with_timeout(Some(Duration::from_millis(500)), BUTTON_EVENTS.wait())
                        .await
                        .is_err()
                    {
                        blink(1).await;
                        expiry.set(expiry.get() + Duration::from_secs(60));
                    } else {
                        break;
                    }
                },
                _ => {}
            }
        }
    };

    embassy_futures::select::select(fade, control).await;

    crate::power::set_level_gradual(0);
}

async fn on_ramping(level: u8) -> u8 {
    let mut level_before_boost = level;
    let mut level = level;

    let mut last_hold_release = Instant::now();

    loop {
        crate::power::set_level_gradual(level);

        match BUTTON_EVENTS.wait().await {
            crate::events::ButtonEvent::Click1 => {
                crate::power::set_level_gradual(0);
                return level;
            }
            crate::events::ButtonEvent::Click2 => {
                if level == 255 {
                    level = level_before_boost;
                } else {
                    level_before_boost = level;
                    level = 255;
                }
            }
            crate::events::ButtonEvent::Hold1 => {
                let direction = if last_hold_release.elapsed() > Duration::from_millis(500) {
                    1
                } else {
                    -1
                };
                loop {
                    if with_timeout(Some(Duration::from_millis(16)), BUTTON_EVENTS.wait())
                        .await
                        .is_err()
                    {
                        level = level.saturating_add_signed(direction);
                        crate::power::set_level_gradual(level);
                    } else {
                        break;
                    }
                }
                if direction == 1 {
                    last_hold_release = Instant::now();
                }
            }
            crate::events::ButtonEvent::Hold2 => loop {
                if with_timeout(Some(Duration::from_millis(16)), BUTTON_EVENTS.wait())
                    .await
                    .is_err()
                {
                    level = level.saturating_sub(1);
                    crate::power::set_level_gradual(level);
                } else {
                    break;
                }
            },
            _ => {}
        }
    }
}
