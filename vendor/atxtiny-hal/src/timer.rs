//! # Basic timer support

mod timer;
mod pwm;

pub use timer::*;
pub use pwm::*;

pub mod tca;
pub mod rtc;

use crate::time::*;

mod sealed {
    use enumset::EnumSetType;

    use super::{Error, TimerClock};
    use crate::time::*;

    pub trait General {
        const TIMER_WIDTH_BITS: u8;
        type CounterValue: Clone + Copy + Into<u32> + TryFrom<u32>;

        cfg_if::cfg_if!{
            if #[cfg(feature = "enumset")] {
                type Interrupt: EnumSetType;
                type Event: EnumSetType;
            } else {
                type Interrupt;
                type Event;
            }
        }

        fn reset_counter_peripheral(&mut self);
        fn enable_counter(&mut self);
        fn disable_counter(&mut self);
        fn is_counter_enabled(&self) -> bool;

        fn reset_count(&mut self);
        fn read_count(&self) -> (Self::CounterValue, Self::CounterValue);

        // fn configure_interrupt(&mut self, interrupt: Self::Interrupt, enable: impl Into<Toggle>);
        // fn is_interrupt_configured(&self, interrupt: Self::Interrupt) -> bool;
        // fn is_event_triggered(&self, event: Self::Event) -> bool;
        // fn clear_event(&mut self, event: Self::Event);
    }

    pub trait AsClockSource: General {
        type OutputClock;
    
        fn use_as_clock_source(&self, timer_clk: Hertz) -> Self::OutputClock;
    }

    pub trait PeriodicMode: General {
        // fn set_periodic_mode(&mut self);

        #[inline(always)]
        fn set_period(&mut self, period: Self::CounterValue) -> Result<(), Error> {
            let p: u32 = period.into();

            if p > 0 && p <= Self::max_period().into() {
                Ok(unsafe { self.set_period_unchecked(period) })
            } else {
                Err(Error::ImpossiblePeriod)
            }
        }

        unsafe fn set_period_unchecked(&mut self, period: Self::CounterValue);
        fn read_period() -> (Self::CounterValue, Self::CounterValue);
        fn trigger_update(&mut self);
        fn max_period() -> Self::CounterValue;

        fn calculate_period_and_prescaler<C: TimerClock>(&self, clk: C::ClockSource, frequency: Hertz) -> Result<(Self::CounterValue, u16), Error> {
            let ticks = C::get_input_clock_rate(clk).raw() / frequency.raw();
            // Round the division up to the next integer to properly determine the
            // prescaler which is an upper bound
            // let prescaler = ticks.div_ceil(1 << Self::TIMER_WIDTH_BITS);  // nightly feature
            let prescaler = (ticks + (1 << Self::TIMER_WIDTH_BITS) - 1) / (1 << Self::TIMER_WIDTH_BITS);
        
            let prescaler = C::get_valid_prescalers(clk).iter()
                                 .find(|e| **e as u32 >= prescaler)
                                 .ok_or(Error::ImpossiblePrescaler)?;
            let period = (ticks / *prescaler as u32) - 1;

            let period = period.try_into().map_err(|_| Error::ImpossiblePeriod)?;
            Ok((period, *prescaler))
        }

        fn clear_overflow(&mut self);
        fn get_overflow(&self) -> bool;
    }

    // FIXME: maybe split the pwm trait and a compare match trait and implement
    //        both for PWM-capable timers? RTC only has compare match but no PWM
    pub trait WithPwm: General + PeriodicMode {
        const CH_NUMBER: u8;
        // type GenerationMode;
        type CompareValue: Clone + Copy + Into<u32> + TryFrom<u32>;

        fn set_pwm_mode(&mut self);

        // FIXME: passing some channel object wrapping a timer pointer or similar
        //        might be the better solution here. Otherwise we always need to
        //        call ptr() and dereference it all the time in these functions
        fn enable_channel(channel: u8, b: bool);
        fn set_compare_value(channel: u8, value: Self::CompareValue);
        fn read_compare_value(channel: u8) -> Self::CompareValue;

        fn clear_compare_match(channel: u8);
        fn get_compare_match(channel: u8) -> bool;
    }

    // FIXME: we need a working event system for TCB single shot mode, I think
    //pub trait SingleShotMode: General {
    //    fn set_single_shot_mode(&mut self);
    //}
}
pub(crate) use sealed::{General, AsClockSource, PeriodicMode, WithPwm};

/// A trait describing one or multiple clock inputs for a timer
pub trait TimerClock {
    /// An enum type that describes all possible clock sources for this timer
    type ClockSource: Copy;

    /// Get the tick rate of a possible clock source before dividing it
    fn get_input_clock_rate(clk: Self::ClockSource) -> Hertz;

    /// Activate the selected clock source for the timer peripheral
    fn prepare_clock_source(&mut self, clk: Self::ClockSource);

    /// Set a prescaler for the selected clock
    fn set_prescaler(&mut self, psc: u16);

    /// Retrieve the current prescaler for the selected clock
    fn read_prescaler(&self) -> u16;

    /// Retrieve a list of available prescalers for the passed clock source
    fn get_valid_prescalers(clk: Self::ClockSource) -> &'static [u16];

    /// Check whether a prescaler is valid for the passed clock source
    #[inline(always)]
    fn is_prescaler_valid(psc: u16, clk: Self::ClockSource) -> bool {
        Self::get_valid_prescalers(clk).contains(&psc)
    }
}

/// A timer instance
pub trait Instance: TimerClock + General + crate::private::Sealed { }

#[derive(ufmt::derive::uDebug, Debug, Eq, PartialEq, Copy, Clone)]
pub enum Error {
    /// Timer is disabled
    Disabled,
    /// Impossible prescaler
    ImpossiblePrescaler,
    /// Impossible Period
    ImpossiblePeriod,
}

