//! # 16-bit Timer/Counter Type A

// TODO: support more than one TCA (right now all is defined for TCA0, we need macros)
// TODO: support split mode, right now only normal is supported

#[cfg(feature = "enumset")]
use enumset::EnumSetType;

use crate::{
    Toggle,
    pac::tca0::*,
    time::*,
    clkctrl::Clocks,
};

/// Enum for waveform genreation modes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WaveformGenerationMode {
    Frequency,
    SingleSlope,
    DualSlopeStop,
    DualSlopeBoth,
    DualSlopeBottom,
}

impl From<WaveformGenerationMode> for single_ctrlb::WGMODE_A {
    fn from(value: WaveformGenerationMode) -> Self {
        use single_ctrlb::WGMODE_A::*;
        match value {
            WaveformGenerationMode::Frequency => FRQ,
            WaveformGenerationMode::SingleSlope => SINGLESLOPE,
            WaveformGenerationMode::DualSlopeStop => DSTOP,
            WaveformGenerationMode::DualSlopeBoth => DSBOTH,
            WaveformGenerationMode::DualSlopeBottom => DSBOTTOM,
        }
    }
}

/// Interrupts for TCA
#[derive(ufmt::derive::uDebug, Debug)]
#[cfg_attr(feature = "enumset", derive(EnumSetType))]
#[cfg_attr(not(feature = "enumset"), derive(Copy, Clone, PartialEq, Eq))]
pub enum Interrupt {
    /// Overflow interrupt
    Overflow,

    /// Compare match interrupt for channel 0
    CompareChannel0,

    /// Compare match interrupt for channel 1
    CompareChannel1,

    /// Compare match interrupt for channel 2
    CompareChannel2,
}

/// Status events for TCA
#[derive(ufmt::derive::uDebug, Debug)]
#[cfg_attr(feature = "enumset", derive(EnumSetType))]
#[cfg_attr(not(feature = "enumset"), derive(Copy, Clone, PartialEq, Eq))]
pub enum Event {
    /// Overflow interrupt
    Overflow,

    /// Compare match interrupt for channel 0
    CompareChannel0,

    /// Compare match interrupt for channel 1
    CompareChannel1,

    /// Compare match interrupt for channel 2
    CompareChannel2,
}

use crate::pac::TCA0;

impl super::Instance for TCA0 {}

impl super::TimerClock for TCA0 {
    type ClockSource = Clocks;

    #[inline(always)]
    fn get_input_clock_rate(clocks: Clocks) -> Hertz {
        clocks.per()
    }

    fn prepare_clock_source(&mut self, _clk: Self::ClockSource) {}

    #[inline(always)]
    fn set_prescaler(&mut self, psc: u16) {
        self.single_ctrla().modify(|_, w| w.clksel().variant(into_clksrc(psc)));
    }

    #[inline(always)]
    fn read_prescaler(&self) -> u16 {
        from_clksrc(self.single_ctrla().read().clksel().variant())
    }

    #[inline(always)]
    fn get_valid_prescalers(_clk: Self::ClockSource) -> &'static [u16] {
        &[1, 2, 4, 8, 16, 64, 256, 1024]
    }
}

impl super::General for TCA0 {
    const TIMER_WIDTH_BITS: u8 = 16;
    type CounterValue = u16;
    type Interrupt = Interrupt;
    type Event = Event;

    #[inline(always)]
    fn reset_counter_peripheral(&mut self) {
        self.single_ctrleset().write(|w| w.cmd().reset());
    }

    #[inline(always)]
    fn enable_counter(&mut self) {
        self.single_ctrla().modify(|_, w| w.enable().set_bit());
    }

    #[inline(always)]
    fn disable_counter(&mut self) {
        self.single_ctrla().modify(|_, w| w.enable().clear_bit());
    }

    #[inline(always)]
    fn is_counter_enabled(&self) -> bool {
        self.single_ctrla().read().enable().bit_is_set()
    }

    // FIXME: turn this into reset_peripheral and issue RESET cmd?
    #[inline(always)]
    fn reset_count(&mut self) {
        self.single_cnt().reset();
    }

    #[inline(always)]
    fn read_count(&self) -> Self::CounterValue {
        self.single_cnt().read().bits()
    }

    #[inline(always)]
    fn configure_interrupt(&mut self, interrupt: Self::Interrupt, enable: impl Into<Toggle>) {
        let enable: Toggle = enable.into();
        let enable: bool = enable.into();
        match interrupt {
            Interrupt::Overflow => self.single_intctrl().modify(|_, w| w.ovf().bit(enable)),
            Interrupt::CompareChannel0 => self.single_intctrl().modify(|_, w| w.cmp0().bit(enable)),
            Interrupt::CompareChannel1 => self.single_intctrl().modify(|_, w| w.cmp1().bit(enable)),
            Interrupt::CompareChannel2 => self.single_intctrl().modify(|_, w| w.cmp2().bit(enable)),
        }
    }

    #[inline(always)]
    fn is_interrupt_configured(&self, interrupt: Self::Interrupt) -> bool {
        let intctrl = self.single_intctrl().read();
        match interrupt {
            Interrupt::Overflow => intctrl.ovf().bit(),
            Interrupt::CompareChannel0 => intctrl.cmp0().bit(),
            Interrupt::CompareChannel1 => intctrl.cmp1().bit(),
            Interrupt::CompareChannel2 => intctrl.cmp2().bit(),
        }
    }

    #[inline(always)]
    fn is_event_triggered(&self, event: Self::Event) -> bool {
        let intflags = self.single_intflags().read();
        match event {
            Event::Overflow => intflags.ovf().bit(),
            Event::CompareChannel0 => intflags.cmp0().bit(),
            Event::CompareChannel1 => intflags.cmp1().bit(),
            Event::CompareChannel2 => intflags.cmp2().bit(),
        }
    }

    #[inline(always)]
    fn clear_event(&mut self, event: Self::Event) {
        match event {
            Event::Overflow => self.single_intflags().modify(|_, w| w.ovf().set_bit()),
            Event::CompareChannel0 => self.single_intflags().modify(|_, w| w.cmp0().set_bit()),
            Event::CompareChannel1 => self.single_intflags().modify(|_, w| w.cmp1().set_bit()),
            Event::CompareChannel2 => self.single_intflags().modify(|_, w| w.cmp2().set_bit()),
        }
    }
}

impl super::PeriodicMode for TCA0 {
    #[inline(always)]
    fn set_periodic_mode(&mut self) {
        self.single_ctrlb().modify(|_, w| w.wgmode().variant(single_ctrlb::WGMODE_A::NORMAL));
    }

    #[inline(always)]
    unsafe fn set_period_unchecked(&mut self, period: u16) {
        self.single_perbuf().write(|w| w.bits(period));
    }

    #[inline(always)]
    fn read_period() -> Self::CounterValue {
        let tim = unsafe { &*TCA0::ptr() };
        tim.single_per().read().bits()
    }

    #[inline(always)]
    fn trigger_update(&mut self) {
        self.single_ctrleset().write(|w| w.cmd().update());
    }

    #[inline(always)]
    fn max_period() -> u16 {
        u16::MAX
    }

    #[inline(always)]
    fn clear_overflow(&mut self) {
        self.single_intflags().modify(|_, w| w.ovf().set_bit());
    }

    #[inline(always)]
    fn get_overflow(&self) -> bool {
        self.single_intflags().read().ovf().bit_is_set()
    }
}

impl super::WithPwm for TCA0 {
    const CH_NUMBER: u8 = 3;
    type GenerationMode = WaveformGenerationMode;
    type CompareValue = u16;

    fn set_pwm_mode(&mut self, mode: Self::GenerationMode) {
        self.single_ctrlb().modify(|_, w| w.wgmode().variant(mode.into()));
    }

    fn enable_channel(channel: u8, b: bool) {
        let tim = unsafe { &*TCA0::ptr() };
        match channel {
            0 => tim.single_ctrlb().modify(|_, w| w.cmp0en().bit(b)),
            1 => tim.single_ctrlb().modify(|_, w| w.cmp1en().bit(b)),
            2 => tim.single_ctrlb().modify(|_, w| w.cmp2en().bit(b)),
            _ => panic!("invalid channel number"),
        }
    }

    fn set_compare_value(channel: u8, value: Self::CompareValue) {
        let tim = unsafe { &*TCA0::ptr() };
        match channel {
            0 => tim.single_cmp0buf().write(|w| w.bits(value)),
            1 => tim.single_cmp1buf().write(|w| w.bits(value)),
            2 => tim.single_cmp2buf().write(|w| w.bits(value)),
            _ => panic!("invalid channel number"),
        }
    }

    fn read_compare_value(channel: u8) -> Self::CompareValue {
        let tim = unsafe { &*TCA0::ptr() };
        match channel {
            0 => tim.single_cmp0().read().bits(),
            1 => tim.single_cmp1().read().bits(),
            2 => tim.single_cmp2().read().bits(),
            _ => panic!("invalid channel number"),
        }
    }

    #[inline(always)]
    fn clear_compare_match(channel: u8) {
        let tim = unsafe { &*TCA0::ptr() };
        match channel {
            0 => tim.single_intflags().modify(|_, w| w.cmp0().set_bit()),
            1 => tim.single_intflags().modify(|_, w| w.cmp1().set_bit()),
            2 => tim.single_intflags().modify(|_, w| w.cmp2().set_bit()),
            _ => panic!("invalid channel number"),
        }
    }

    #[inline(always)]
    fn get_compare_match(channel: u8) -> bool {
        let tim = unsafe { &*TCA0::ptr() };
        match channel {
            0 => tim.single_intflags().read().cmp0().bit_is_set(),
            1 => tim.single_intflags().read().cmp1().bit_is_set(),
            2 => tim.single_intflags().read().cmp2().bit_is_set(),
            _ => panic!("invalid channel number"),
        }
    }
}

fn into_clksrc(prescaler: u16) -> single_ctrla::CLKSEL_A {
    use single_ctrla::CLKSEL_A::*;
    match prescaler {
        1 => DIV1,
        2 => DIV2,
        4 => DIV4,
        8 => DIV8,
        16 => DIV16,
        64 => DIV64,
        256 => DIV256,
        1024 => DIV1024,
        _ => panic!("Invalid prescaler"),
    }
}

fn from_clksrc(prescaler: single_ctrla::CLKSEL_A) -> u16 {
    use single_ctrla::CLKSEL_A::*;
    match prescaler {
        DIV1 => 1,
        DIV2 => 2,
        DIV4 => 4,
        DIV8 => 8,
        DIV16 => 16,
        DIV64 => 64,
        DIV256 => 256,
        DIV1024 => 1024,
    }
}

use core::marker::PhantomData;
use crate::gpio::{Output, Stateless};
use super::pwm::{WaveformOutputPinset, C1, C2, C3};

/// A pin can be marked with this when it can be used as a waveform output pin
pub trait WaveformOutputPin<TCA, const CHAN: u8> {}

impl crate::private::Sealed for crate::pac::TCA0 {}

/// Pin set for the port multiplexer
pub struct TcaPinset<TIM, WaveformOutput: WaveformOutputPin<TIM, CHAN>, const CHAN: u8> {
    _tim: PhantomData<TIM>,
    output: WaveformOutput,
}

impl<TIM, WaveformOutput, const CHAN: u8> TcaPinset<TIM, WaveformOutput, CHAN>
where
    WaveformOutput: WaveformOutputPin<TIM, CHAN>,
{
    pub(crate) fn new(output: WaveformOutput) -> Self {
        TcaPinset { _tim: PhantomData, output }
    }

    pub fn free(self) -> WaveformOutput { 
        self.output
    }
}

impl<WaveformOutput: WaveformOutputPin<TCA0, CHAN>, const CHAN: u8> WaveformOutputPinset<TCA0, CHAN> for TcaPinset<TCA0, WaveformOutput, CHAN> {}

impl WaveformOutputPin<TCA0, C1> for crate::gpio::portb::PB0<Output<Stateless>> {}
impl WaveformOutputPin<TCA0, C2> for crate::gpio::portb::PB1<Output<Stateless>> {}
impl WaveformOutputPin<TCA0, C3> for crate::gpio::portb::PB2<Output<Stateless>> {}
// In split mode:
//impl WaveformOutputPin<TCA0, C4> for crate::gpio::porta::PA3<Output<Stateless>> {}
//impl WaveformOutputPin<TCA0, C5> for crate::gpio::porta::PA4<Output<Stateless>> {}
//impl WaveformOutputPin<TCA0, C6> for crate::gpio::porta::PA5<Output<Stateless>> {}

impl WaveformOutputPin<TCA0, C1> for crate::gpio::portb::PB3<Output<Stateless>> {}
impl WaveformOutputPin<TCA0, C2> for crate::gpio::portb::PB4<Output<Stateless>> {}
impl WaveformOutputPin<TCA0, C3> for crate::gpio::portb::PB5<Output<Stateless>> {}
// In split mode:
//impl WaveformOutputPin<TCA0, C4> for crate::gpio::portc::PC3<Output<Stateless>> {}
//impl WaveformOutputPin<TCA0, C5> for crate::gpio::portc::PC4<Output<Stateless>> {}
//impl WaveformOutputPin<TCA0, C6> for crate::gpio::portc::PC5<Output<Stateless>> {}
