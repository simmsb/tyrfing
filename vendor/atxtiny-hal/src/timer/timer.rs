use crate::time::*;
use super::{Instance, General, Error, AsClockSource};

/// Timer wrapper for dynamic precision timers
///
/// This wrapper can be used for all different timer variants (A, B and D)
pub struct Timer<TIM: Instance> {
    pub(crate) tim: TIM,
    pub(crate) clk: TIM::ClockSource,
}

impl<TIM: Instance + General> Timer<TIM> {
    /// Initialize timer
    pub fn new(mut tim: TIM, clk: TIM::ClockSource) -> Self {
        tim.reset_counter_peripheral();
        Self {
            tim,
            clk,
        }
    }

    /// Releases the TIM peripheral
    pub fn release(self) -> TIM {
        self.tim
    }
}

/// Timer wrapper for fixed precision timers
///
/// Uses `fugit::TimerDurationU32` for most of operations
pub struct FTimer<TIM, const FREQ: u32> {
    pub(crate) tim: TIM,
}

// /// `FTimer` with precision of 1 μs (1 MHz sampling)
// pub type FTimerUs<TIM> = FTimer<TIM, 1_000_000>;
// 
// /// `FTimer` with precision of 1 ms (1 kHz sampling)
// pub type FTimerMs<TIM> = FTimer<TIM, 1_000>;

impl<TIM: Instance + General, const FREQ: u32> FTimer<TIM, FREQ> {
    /// Initialize timer
    pub fn new(mut tim: TIM, clk: TIM::ClockSource) -> Result<Self, Error> {
        tim.reset_counter_peripheral();
        let mut t = Self { tim };
        t.configure(clk)?;
        Ok(t)
    }

    /// Calculate and set prescaler depending on `Clocks` state
    pub fn configure(&mut self, clk: TIM::ClockSource) -> Result<(), Error> {
        self.tim.prepare_clock_source(clk);

        let clk_rate = TIM::get_input_clock_rate(clk);
        if clk_rate.raw() % FREQ != 0 {
            return Err(Error::ImpossiblePrescaler)
        }

        let psc = (clk_rate.raw() / FREQ) as u16;
        if !TIM::is_prescaler_valid(psc, clk) {
            return Err(Error::ImpossiblePrescaler)
        }

        self.tim.set_prescaler(psc);

        Ok(())
    }

    /// Releases the TIM peripheral
    pub fn release(self) -> TIM {
        self.tim
    }
}

impl<TIM: AsClockSource, const FREQ: u32> FTimer<TIM, FREQ> {
    pub fn use_as_clock_source(&self) -> TIM::OutputClock {
        self.tim.use_as_clock_source(Hertz::from_raw(FREQ))
    }
}
