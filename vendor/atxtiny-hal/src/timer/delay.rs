use super::{FTimer, Instance, PeriodicMode};

use core::ops::{Deref, DerefMut};

use fugit::TimerDurationU32;

use crate::time::*;
use crate::embedded_hal::delay::DelayNs;

/// Periodic non-blocking timer that imlements [embedded_hal::blocking::delay] traits
///
/// ### Example: Millisecond precision
///
/// To instantiate this with the TIM2 timer and millisecond precision:
///
/// ```rust
/// let mut delay = dp.TIM2.delay_ms(&clocks);
/// delay.delay_ms(320_u32);
/// ```
///
/// With millisecond precision, you can wait from 2 ms to 49 days.
///
/// ### Example: Microsecond precision
///
/// To instantiate this with the TIM5 timer and microsecond precision:
///
/// ```rust
/// let delay = dp.TIM5.delay_us(&clocks);
/// delay.delay_us(30_u32);
/// delay.delay_ms(5_u32);
/// delay.delay(5.millis());
/// delay.delay(3.secs());
/// ```
///
/// With microsecond precision, you can wait from 2 µs to 71 min.
pub struct Delay<TIM, const FREQ: u32>(pub(super) FTimer<TIM, FREQ>);

impl<T, const FREQ: u32> Deref for Delay<T, FREQ> {
    type Target = FTimer<T, FREQ>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const FREQ: u32> DerefMut for Delay<T, FREQ> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// FIXME: implement the delay for OneShot timers like in STM32F4 HAL
impl<TIM: Instance + PeriodicMode, const FREQ: u32> Delay<TIM, FREQ> {
    // Sleep for given time
    pub fn delay(&mut self, time: TimerDurationU32<FREQ>) {
        self.tim.disable_counter();
        self.tim.set_periodic_mode();
        self.tim.clear_overflow();

        let mut ticks = time.ticks().max(1) - 1;
        while ticks != 0 {
            let period = ticks.min(TIM::max_period().into());

            unsafe {
                // FIXME: add TimerDurationU16 to fugit, then do everything with 16 bits
                self.tim.set_period_unchecked(period.try_into().unwrap_or(TIM::max_period()));
            }

            ticks -= period;

            self.tim.reset_count();
            self.tim.enable_counter();
            while !self.tim.get_overflow() { /* wait */ }
            self.tim.disable_counter();
            self.tim.clear_overflow();
        }
    }

    pub fn max_delay(&self) -> TimerDurationU32<FREQ> {
        // FIXME: add TimerDurationU16 to fugit, then do everything with 16 bits
        TimerDurationU32::from_ticks(TIM::max_period().into())
    }

    /// Releases the TIM peripheral
    pub fn release(mut self) -> FTimer<TIM, FREQ> {
        self.tim.disable_counter();
        self.0
    }
}

impl<TIM: Instance + PeriodicMode, const FREQ: u32> fugit_timer::Delay<FREQ> for Delay<TIM, FREQ> {
    type Error = core::convert::Infallible;

    fn delay(&mut self, duration: TimerDurationU32<FREQ>) -> Result<(), Self::Error> {
        self.delay(duration);
        Ok(())
    }
}

impl<TIM: Instance + PeriodicMode, const FREQ: u32> DelayNs for Delay<TIM, FREQ> {
    fn delay_ns(&mut self, ns: u32) {
        self.delay(ns.nanos());
    }
}
