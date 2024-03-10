//! # Real Time Counter

use avr_device::attiny1616::rtc::pitctrla::PERIOD_A;
#[cfg(feature = "enumset")]
use enumset::EnumSetType;

use crate::{
    pac::{rtc::ctrla, RTC},
    time::*,
    Toggle,
};

use super::{General, Instance, PeriodicMode, TimerClock};

/// Interrupts for RTC
#[derive(ufmt::derive::uDebug, Debug)]
#[cfg_attr(feature = "enumset", derive(EnumSetType))]
#[cfg_attr(not(feature = "enumset"), derive(Copy, Clone, PartialEq, Eq))]
pub enum Interrupt {
    CompareMatch,
    Overflow,
}

/// Status events for RTC
#[derive(ufmt::derive::uDebug, Debug)]
#[cfg_attr(feature = "enumset", derive(EnumSetType))]
#[cfg_attr(not(feature = "enumset"), derive(Copy, Clone, PartialEq, Eq))]
pub enum Event {
    CompareMatch,
    Overflow,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum RTCClockSource {
    OSCULP32K_32K,
    OSCULP32K_1K,
    //XOSC32K,          // FIXME: retrieve an object for this from CLKCTRL and enable it when doing so
    TOSC1(Hertz),
}

impl Instance for RTC {}

impl TimerClock for RTC {
    type ClockSource = RTCClockSource;

    #[inline(always)]
    fn get_input_clock_rate(clk: Self::ClockSource) -> Hertz {
        match clk {
            RTCClockSource::OSCULP32K_32K => 32_768.Hz(),
            RTCClockSource::OSCULP32K_1K => 1_024.Hz(),
            //RTCClockSource::XOSC32K => 32_768.Hz(),
            RTCClockSource::TOSC1(h) => h,
        }
    }

    #[inline(always)]
    fn prepare_clock_source(&mut self, clk: Self::ClockSource) {
        match clk {
            RTCClockSource::OSCULP32K_32K => self.clksel().write(|w| w.clksel().int32k()),
            RTCClockSource::OSCULP32K_1K => self.clksel().write(|w| w.clksel().int1k()),
            //RTCClockSource::XOSC32K => self.clksel().write(|w| w.clksel().tosc32k()),
            RTCClockSource::TOSC1(_) => self.clksel().write(|w| w.clksel().extclk()),
        }
    }

    #[inline(always)]
    fn get_valid_prescalers(_clk: Self::ClockSource) -> &'static [u16] {
        &[
            1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768,
        ]
    }

    #[inline(always)]
    fn set_prescaler(&mut self, psc: u16) {
        while self.status().read().ctrlabusy().bit_is_set() {}
        self.ctrla()
            .modify(|_, w| w.prescaler().variant(into_prescaler(psc)));
    }

    #[inline(always)]
    fn read_prescaler(&self) -> u16 {
        from_prescaler(self.ctrla().read().prescaler().variant())
    }
}

impl General for RTC {
    const TIMER_WIDTH_BITS: u8 = 16;
    type CounterValue = u16;
    type Interrupt = Interrupt;
    type Event = Event;

    #[inline(always)]
    fn reset_counter_peripheral(&mut self) {}

    #[inline(always)]
    fn enable_counter(&mut self) {
        while self.status().read().ctrlabusy().bit_is_set() {}
        self.ctrla().modify(|_, w| w.rtcen().set_bit());
    }

    #[inline(always)]
    fn disable_counter(&mut self) {
        while self.status().read().ctrlabusy().bit_is_set() {}
        self.ctrla().modify(|_, w| w.rtcen().clear_bit());
    }

    #[inline(always)]
    fn is_counter_enabled(&self) -> bool {
        self.ctrla().read().rtcen().bit_is_set()
    }

    #[inline(always)]
    fn reset_count(&mut self) {
        while self.status().read().cntbusy().bit_is_set() {}
        self.cnt().reset();
    }

    #[inline(always)]
    fn read_count(&self) -> Self::CounterValue {
        self.cnt().read().bits()
    }

    #[inline(always)]
    fn configure_interrupt(&mut self, interrupt: Self::Interrupt, enable: impl Into<Toggle>) {
        let enable: Toggle = enable.into();
        let enable: bool = enable.into();
        match interrupt {
            Interrupt::CompareMatch => self.intctrl().modify(|_, w| w.cmp().bit(enable)),
            Interrupt::Overflow => self.intctrl().modify(|_, w| w.ovf().bit(enable)),
        }
    }

    #[inline(always)]
    fn is_interrupt_configured(&self, interrupt: Self::Interrupt) -> bool {
        let intctrl = self.intctrl().read();
        match interrupt {
            Interrupt::CompareMatch => intctrl.cmp().bit(),
            Interrupt::Overflow => intctrl.ovf().bit(),
        }
    }

    #[inline(always)]
    fn is_event_triggered(&self, event: Self::Event) -> bool {
        let intflags = self.intflags().read();
        match event {
            Event::CompareMatch => intflags.cmp().bit(),
            Event::Overflow => intflags.ovf().bit(),
        }
    }

    #[inline(always)]
    fn clear_event(&mut self, event: Self::Event) {
        match event {
            Event::CompareMatch => self.intflags().modify(|_, w| w.cmp().set_bit()),
            Event::Overflow => self.intflags().modify(|_, w| w.ovf().set_bit()),
        }
    }
}

impl PeriodicMode for RTC {
    #[inline(always)]
    fn set_periodic_mode(&mut self) {}

    #[inline(always)]
    fn read_period() -> Self::CounterValue {
        // FIXME: function needs to be called from PwmChannel where we don't
        //        have a reference to the Timer, hence this stuff
        //        When the split pwm channels get a ref to the timer, we can
        //        get rid of this again
        let rtc = unsafe { &*RTC::ptr() };
        rtc.per().read().bits()
    }

    #[inline(always)]
    fn trigger_update(&mut self) {
        // no double buffering, no updating...
    }

    #[inline(always)]
    unsafe fn set_period_unchecked(&mut self, period: Self::CounterValue) {
        while self.status().read().perbusy().bit_is_set() {}
        self.per().write(|w| w.bits(period));
    }

    #[inline(always)]
    fn max_period() -> Self::CounterValue {
        u16::MAX
    }

    #[inline(always)]
    fn clear_overflow(&mut self) {
        self.intflags().modify(|_, w| w.ovf().set_bit());
    }

    #[inline(always)]
    fn get_overflow(&self) -> bool {
        self.intflags().read().ovf().bit_is_set()
    }
}

impl Pit {
    pub fn from_rtc(mut rtc: RTC, clk: RTCClockSource, period: PERIOD_A) -> Pit {
        rtc.prepare_clock_source(clk);
        while rtc.pitstatus().read().ctrlbusy().bit_is_set() {}
        rtc.pitctrla().modify(|_, w| w.period().variant(period));

        Pit(rtc)
    }

    pub fn reconfigure(&mut self, clk: RTCClockSource, period: PERIOD_A) {
        self.0.prepare_clock_source(clk);
        while self.0.pitstatus().read().ctrlbusy().bit_is_set() {}
        self.0.pitctrla().modify(|_, w| w.period().variant(period));
    }

    pub fn enable_interrupt(&mut self) {
        self.0.pitintctrl().modify(|_, w| w.pi().set_bit());
    }

    pub fn start(&mut self) {
        // self.
        while self.0.pitstatus().read().ctrlbusy().bit_is_set() {}
        self.0.pitctrla().modify(|_, w| w.piten().set_bit());
    }

    pub fn clear_interrupt(&mut self) {
        self.0.pitintflags().modify(|_, w| w.pi().set_bit());
    }

    pub fn set_period(&mut self, period: PERIOD_A) {
        self.0.pitctrla().modify(|_, w| w.period().variant(period));
    }
}

pub struct Pit(RTC);

// FIXME: implement compare mode for RTC
// FIXME: implement PIT in RTC

fn into_prescaler(prescaler: u16) -> ctrla::PRESCALER_A {
    use ctrla::PRESCALER_A::*;
    match prescaler {
        1 => DIV1,
        2 => DIV2,
        4 => DIV4,
        8 => DIV8,
        16 => DIV16,
        32 => DIV32,
        64 => DIV64,
        128 => DIV128,
        256 => DIV256,
        512 => DIV512,
        1024 => DIV1024,
        2048 => DIV2048,
        4096 => DIV4096,
        8192 => DIV8192,
        16384 => DIV16384,
        32768 => DIV32768,
        _ => panic!("Invalid prescaler"),
    }
}

fn from_prescaler(prescaler: ctrla::PRESCALER_A) -> u16 {
    use ctrla::PRESCALER_A::*;
    match prescaler {
        DIV1 => 1,
        DIV2 => 2,
        DIV4 => 4,
        DIV8 => 8,
        DIV16 => 16,
        DIV32 => 32,
        DIV64 => 64,
        DIV128 => 128,
        DIV256 => 256,
        DIV512 => 512,
        DIV1024 => 1024,
        DIV2048 => 2048,
        DIV4096 => 4096,
        DIV8192 => 8192,
        DIV16384 => 16384,
        DIV32768 => 32768,
    }
}

impl crate::private::Sealed for crate::pac::RTC {}
