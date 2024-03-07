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
                ButtonEvent::Click1 => {
                    saved_level = on_ramping(saved_level).await;
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
                }
                _ => {}
            }
        }
    }
}

pub async fn on_ramping(level: u8) -> u8 {
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

        #[cfg(feature = "logging")]
        crate::serial_println!("Got event");
    }
}
