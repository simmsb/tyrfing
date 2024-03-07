use avr_device::attiny1616::TCB0;

use crate::{
    Toggle,
    time::*
};

use super::tcb::{Event, Interrupt};

pub struct TCB8Bit {
    pub (crate) tim: TCB0
}

impl super::Instance for TCB8Bit {}
impl crate::private::Sealed for TCB8Bit {}

impl super::TimerClock for TCB8Bit {
    type ClockSource = <avr_device::attiny1616::TCB0 as super::TimerClock>::ClockSource;

    #[inline(always)]
    fn get_input_clock_rate(clk: Self::ClockSource) -> Hertz {
        TCB0::get_input_clock_rate(clk)
    }

    #[inline(always)]
    fn prepare_clock_source(&mut self, clk: Self::ClockSource) {
        self.tim.prepare_clock_source(clk)
    }

    #[inline(always)]
    fn get_valid_prescalers(clk: Self::ClockSource) -> &'static [u16] {
        TCB0::get_valid_prescalers(clk)
    }

    #[inline(always)]
    fn set_prescaler(&mut self, psc: u16) {
        self.tim.set_prescaler(psc)
    }

    #[inline(always)]
    fn read_prescaler(&self) -> u16 {
        self.tim.read_prescaler()
    }
}

impl super::General for TCB8Bit {
    const TIMER_WIDTH_BITS: u8 = 8;
    type CounterValue = u8;
    type Interrupt = Interrupt;
    type Event = Event;

    #[inline(always)]
    fn reset_counter_peripheral(&mut self) {

    }

    #[inline(always)]
    fn enable_counter(&mut self) {
        self.tim.enable_counter();
    }

    #[inline(always)]
    fn disable_counter(&mut self) {
        self.tim.disable_counter();
    }

    #[inline(always)]
    fn is_counter_enabled(&self) -> bool {
        self.tim.is_counter_enabled()
    }

    #[inline(always)]
    fn reset_count(&mut self) {
        self.tim.reset_count();
    }

    #[inline(always)]
    fn read_count(&self) -> Self::CounterValue {
        self.tim.read_count() as u8
    }

    #[inline(always)]
    fn configure_interrupt(&mut self, interrupt: Self::Interrupt, enable: impl Into<Toggle>) {
        self.tim.configure_interrupt(interrupt, enable)
    }

    #[inline(always)]
    fn is_interrupt_configured(&self, interrupt: Self::Interrupt) -> bool {
        self.tim.is_interrupt_configured(interrupt)
    }

    #[inline(always)]
    fn is_event_triggered(&self, event: Self::Event) -> bool {
        self.tim.is_event_triggered(event)
    }

    #[inline(always)]
    fn clear_event(&mut self, event: Self::Event) {
        self.tim.clear_event(event)
    }
}

impl super::PeriodicMode for TCB8Bit {
    #[inline(always)]
    fn set_periodic_mode(&mut self) {
        self.tim.ctrlb().modify(|_, w| w.cntmode().pwm8())
    }

    #[inline(always)]
    fn read_period() -> Self::CounterValue {
        // FIXME: function needs to be called from PwmChannel where we don't
        //        have a reference to the Timer, hence this stuff
        //        When the split pwm channels get a ref to the timer, we can
        //        get rid of this again
        let tim = unsafe { &*TCB0::ptr() };
        tim.ccmpl().read().bits()
    }

    #[inline(always)]
    fn trigger_update(&mut self) {
        // no double buffering, no updating...
    }

    #[inline(always)]
    unsafe fn set_period_unchecked(&mut self, period: Self::CounterValue) {
        self.tim.ccmpl().write(|w| w.bits(period));
    }

    #[inline(always)]
    fn max_period() -> Self::CounterValue {
        u8::MAX
    }

    #[inline(always)]
    fn clear_overflow(&mut self) {
        self.tim.intflags().modify(|_, w| w.capt().set_bit());
    }

    #[inline(always)]
    fn get_overflow(&self) -> bool {
        self.tim.intflags().read().capt().bit_is_set()
    }
}

impl super::WithPwm for TCB8Bit {
    const CH_NUMBER: u8 = 1;
    type GenerationMode = ();
    type CompareValue = u8;

    // Period: CCMPL
    // Compare: CCMPH

    fn set_pwm_mode(&mut self, _mode: Self::GenerationMode) {
        self.tim.ctrlb().write(|w| w.cntmode().pwm8());
    }

    fn enable_channel(channel: u8, b: bool) {
        let tim = unsafe { &*TCB0::ptr() };
        match channel {
            0 => tim.ctrlb().modify(|_, w| w.ccmpen().bit(b)),
            _ => panic!("invalid channel number"),
        }
    }

    fn set_compare_value(channel: u8, value: Self::CompareValue) {
        let tim = unsafe { &*TCB0::ptr() };
        match channel {
            0 => tim.ccmph().write(|w| unsafe { w.bits(value) }),
            _ => panic!("invalid channel number"),
        }
    }

    fn read_compare_value(channel: u8) -> Self::CompareValue {
        let tim = unsafe { &*TCB0::ptr() };
        match channel {
            0 => tim.ccmph().read().bits(),
            _ => panic!("invalid channel number"),
        }
    }

    #[inline(always)]
    fn clear_compare_match(channel: u8) {
        let tim = unsafe { &*TCB0::ptr() };
        match channel {
            0 => tim.intflags().modify(|_, w| w.capt().set_bit()),
            _ => panic!("invalid channel number"),
        }
    }

    #[inline(always)]
    fn get_compare_match(channel: u8) -> bool {
        let tim = unsafe { &*TCB0::ptr() };
        match channel {
            0 => tim.intflags().read().capt().bit_is_set(),
            _ => panic!("invalid channel number"),
        }
    }
}
