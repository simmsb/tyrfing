//! # Real Time Counter

#[cfg(feature = "attiny1616")]
use avr_device::attiny1616::rtc::pitctrla::PERIOD_A;
#[cfg(feature = "avr32dd20")]
use avr_device::avr32dd20::rtc::pitctrla::PERIOD_A;

#[cfg(feature = "enumset")]
use enumset::EnumSetType;

use crate::{
    pac::{rtc::ctrla, RTC},
    time::*,
};

use super::TimerClock;

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
            RTCClockSource::OSCULP32K_32K => self.clksel().write(|w| w.clksel().osc32k()),
            RTCClockSource::OSCULP32K_1K => self.clksel().write(|w| w.clksel().osc1k()),
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
