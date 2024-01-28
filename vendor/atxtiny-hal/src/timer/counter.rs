use super::{Timer, FTimer, Instance, PeriodicMode, AsClockSource, Error};

use core::ops::{Deref, DerefMut};

use fugit::{TimerDurationU32, TimerInstantU32};

use crate::time::*;

/// A counter with dynamic precision which uses [`Hertz`] as Duration units
pub struct CounterHz<TIM: Instance>(pub(super) Timer<TIM>);

impl<T: Instance> Deref for CounterHz<T> {
    type Target = Timer<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Instance> DerefMut for CounterHz<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<TIM: Instance> CounterHz<TIM> {
    /// Releases the TIM peripheral
    pub fn release(mut self) -> Timer<TIM> {
        self.tim.disable_counter();
        self.0
    }
}

impl<TIM: Instance + PeriodicMode> CounterHz<TIM> {
    pub fn start(&mut self, timeout: Hertz) -> Result<(), Error> {
        let clk = self.clk;
        self.tim.prepare_clock_source(clk);

        self.tim.disable_counter();
        self.tim.reset_count();
        self.tim.set_periodic_mode();
        self.tim.clear_overflow();

        let (period, prescaler) = self.tim.calculate_period_and_prescaler::<TIM>(clk, timeout)?;
        self.tim.set_prescaler(prescaler);
        self.tim.set_period(period)?;
        self.tim.trigger_update();

        self.tim.enable_counter();

        Ok(())
    }

    pub fn wait(&mut self) -> nb::Result<(), Error> {
        if self.tim.get_overflow() {
            self.tim.clear_overflow();
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub fn cancel(&mut self) -> Result<(), Error> {
        if !self.tim.is_counter_enabled() {
            return Err(Error::Disabled);
        }

        self.tim.disable_counter();
        Ok(())
    }
}

impl<TIM: Instance + AsClockSource> CounterHz<TIM> {
    pub fn use_as_clock_source(&self) -> TIM::OutputClock {
        self.tim.use_as_clock_source(TIM::get_input_clock_rate(self.clk) / self.tim.read_prescaler() as u32)
    }
}



/// Periodic non-blocking timer that implements [embedded_hal::timer::CountDown]
pub struct Counter<TIM, const FREQ: u32>(pub(super) FTimer<TIM, FREQ>);

impl<T, const FREQ: u32> Deref for Counter<T, FREQ> {
    type Target = FTimer<T, FREQ>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const FREQ: u32> DerefMut for Counter<T, FREQ> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// [`Counter`] with precision of 1 μs (1 MHz sampling)
pub type CounterUs<TIM> = Counter<TIM, 1_000_000>;

/// [`Counter`] with precision of of 1 ms (1 kHz sampling)
pub type CounterMs<TIM> = Counter<TIM, 1_000>;

impl<TIM: Instance + PeriodicMode, const FREQ: u32> Counter<TIM, FREQ> {
    /// Releases the TIM peripheral
    pub fn release(mut self) -> FTimer<TIM, FREQ> {
        self.tim.disable_counter();
        self.0
    }

    pub fn now(&self) -> TimerInstantU32<FREQ> {
        TimerInstantU32::from_ticks(self.tim.read_count().into())
    }

    pub fn start(&mut self, timeout: TimerDurationU32<FREQ>) -> Result<(), Error> {
        self.tim.disable_counter();
        self.tim.reset_count();
        self.tim.set_periodic_mode();
        self.tim.clear_overflow();

        let period = (timeout.ticks() - 1).try_into().map_err(|_| Error::ImpossiblePeriod)?;
        self.tim.set_period(period)?;
        self.tim.trigger_update();
        
        // start counter
        self.tim.enable_counter();

        Ok(())
    }

    pub fn wait(&mut self) -> nb::Result<(), Error> {
        if self.tim.get_overflow() {
            self.tim.clear_overflow();
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub fn cancel(&mut self) -> Result<(), Error> {
        if !self.tim.is_counter_enabled() {
            return Err(Error::Disabled);
        }

        self.tim.disable_counter();
        Ok(())
    }
}

impl<TIM: Instance + PeriodicMode, const FREQ: u32> fugit_timer::Timer<FREQ> for Counter<TIM, FREQ> {
    type Error = Error;

    fn now(&mut self) -> TimerInstantU32<FREQ> {
        Self::now(self)
    }

    fn start(&mut self, duration: TimerDurationU32<FREQ>) -> Result<(), Self::Error> {
        self.start(duration)
    }

    fn cancel(&mut self) -> Result<(), Self::Error> {
        self.cancel()
    }

    fn wait(&mut self) -> nb::Result<(), Self::Error> {
        self.wait()
    }
}

impl<TIM: Instance + AsClockSource, const FREQ: u32> Counter<TIM, FREQ> {
    pub fn use_as_clock_source(&self) -> TIM::OutputClock {
        self.tim.use_as_clock_source(Hertz::from_raw(FREQ))
    }
}
