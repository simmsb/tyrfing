use embassy_time::Duration;

use crate::{events::BUTTON_EVENTS, with_timeout::with_timeout};

async fn set_output(level: u8) {}

pub async fn on_ramping() {
    let mut level = 0;

    loop {
        match BUTTON_EVENTS.wait().await {
            crate::events::ButtonEvent::Click1 => {
                set_output(0).await;
                return;
            }
            crate::events::ButtonEvent::Click2 => {
                level = 100;
            }
            crate::events::ButtonEvent::Hold1 => loop {
                if with_timeout(Some(Duration::from_millis(200)), BUTTON_EVENTS.wait())
                    .await
                    .is_err()
                {
                    level += 1;
                }
            },
            crate::events::ButtonEvent::Hold2 => loop {
                if with_timeout(Some(Duration::from_millis(200)), BUTTON_EVENTS.wait())
                    .await
                    .is_err()
                {
                    level -= 1;
                }
            },
            _ => {}
        }

        set_output(level).await;
    }
}
