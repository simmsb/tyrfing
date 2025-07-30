//! # 16-bit Timer/Counter Type A

// TODO: support more than one TCA (right now all is defined for TCA0, we need macros)
// TODO: support split mode, right now only normal is supported

use core::marker::PhantomData;

#[cfg(feature = "enumset")]
use enumset::EnumSetType;

use crate::portmux::{PortmuxProof, PortmuxTCARoutes, TCA0PortA};
use crate::{clkctrl::Clocks, pac::tca0::*, time::*, Toggle};

/// Interrupts for TCA
#[derive(ufmt::derive::uDebug, Debug)]
#[cfg_attr(feature = "enumset", derive(EnumSetType))]
#[cfg_attr(not(feature = "enumset"), derive(Copy, Clone, PartialEq, Eq))]
pub enum Interrupt {
    /// Overflow interrupt
    HUnderflow,
    /// Overflow interrupt
    LUnderflow,

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

    fn prepare_clock_source(&mut self, _clk: Self::ClockSource) {
        self.split_ctrld().modify(|_, w| w.splitm().set_bit());
    }

    #[inline(always)]
    fn set_prescaler(&mut self, psc: u16) {
        self.split_ctrla()
            .modify(|_, w| w.clksel().variant(into_clksrc(psc)));
    }

    #[inline(always)]
    fn read_prescaler(&self) -> u16 {
        from_clksrc(self.split_ctrla().read().clksel().variant())
    }

    #[inline(always)]
    fn get_valid_prescalers(_clk: Self::ClockSource) -> &'static [u16] {
        &[1, 2, 4, 8, 16, 64, 256, 1024]
    }
}

impl super::General for TCA0 {
    const TIMER_WIDTH_BITS: u8 = 8;
    type CounterValue = u8;
    type Interrupt = Interrupt;
    type Event = Event;

    #[inline(always)]
    fn reset_counter_peripheral(&mut self) {
        self.split_ctrleset().write(|w| w.cmden().both());
        self.split_ctrleset().write(|w| w.cmd().reset());
    }

    #[inline(always)]
    fn enable_counter(&mut self) {
        self.split_ctrla().modify(|_, w| w.enable().set_bit());
    }

    #[inline(always)]
    fn disable_counter(&mut self) {
        self.split_ctrla().modify(|_, w| w.enable().clear_bit());
    }

    #[inline(always)]
    fn is_counter_enabled(&self) -> bool {
        self.split_ctrla().read().enable().bit_is_set()
    }

    // FIXME: turn this into reset_peripheral and issue RESET cmd?
    #[inline(always)]
    fn reset_count(&mut self) {
        self.split_hcnt().reset();
        self.split_lcnt().reset();
    }

    #[inline(always)]
    fn read_count(&self) -> (Self::CounterValue, Self::CounterValue) {
        (
            self.split_hcnt().read().bits(),
            self.split_lcnt().read().bits(),
        )
    }

    // #[inline(always)]
    // fn configure_interrupt(&mut self, interrupt: Self::Interrupt, enable: impl Into<Toggle>) {
    //     let enable: Toggle = enable.into();
    //     let enable: bool = enable.into();
    //     match interrupt {
    //         Interrupt::HUnderflow => self.split_intctrl().modify(|_, w| w.hunf().bit(enable)),
    //         Interrupt::LUnderflow => self.split_intctrl().modify(|_, w| w.lunf().bit(enable)),
    //         Interrupt::CompareChannel0 => self.split_intctrl().modify(|_, w| w.lcmp0().bit(enable)),
    //         Interrupt::CompareChannel1 => self.split_intctrl().modify(|_, w| w.lcmp1().bit(enable)),
    //         Interrupt::CompareChannel2 => self.split_intctrl().modify(|_, w| w.lcmp2().bit(enable)),
    //     }
    // }

    // #[inline(always)]
    // fn is_interrupt_configured(&self, interrupt: Self::Interrupt) -> bool {
    //     let intctrl = self.split_intctrl().read();
    //     match interrupt {
    //         Interrupt::Overflow => intctrl.ovf().bit(),
    //         Interrupt::CompareChannel0 => intctrl.cmp0().bit(),
    //         Interrupt::CompareChannel1 => intctrl.cmp1().bit(),
    //         Interrupt::CompareChannel2 => intctrl.cmp2().bit(),
    //     }
    // }

    // #[inline(always)]
    // fn is_event_triggered(&self, event: Self::Event) -> bool {
    //     let intflags = self.split_intflags().read();
    //     match event {
    //         Event::Overflow => intflags.ovf().bit(),
    //         Event::CompareChannel0 => intflags.cmp0().bit(),
    //         Event::CompareChannel1 => intflags.cmp1().bit(),
    //         Event::CompareChannel2 => intflags.cmp2().bit(),
    //     }
    // }

    // #[inline(always)]
    // fn clear_event(&mut self, event: Self::Event) {
    //     match event {
    //         Event::Overflow => self.split_intflags().modify(|_, w| w.ovf().set_bit()),
    //         Event::CompareChannel0 => self.split_intflags().modify(|_, w| w.cmp0().set_bit()),
    //         Event::CompareChannel1 => self.split_intflags().modify(|_, w| w.cmp1().set_bit()),
    //         Event::CompareChannel2 => self.split_intflags().modify(|_, w| w.cmp2().set_bit()),
    //     }
    // }
}

impl super::PeriodicMode for TCA0 {
    // #[inline(always)]
    // fn set_periodic_mode(&mut self) {
    //     self.split_ctrlb()
    //         .modify(|_, w| w.wgmode().variant(split_ctrlb::WGMODE_A::NORMAL));
    // }

    #[inline(always)]
    unsafe fn set_period_unchecked(&mut self, period: u8) {
        self.split_hper().write(|w| w.bits(period));
        self.split_lper().write(|w| w.bits(period));
    }

    #[inline(always)]
    fn read_period() -> (Self::CounterValue, Self::CounterValue) {
        let tim = unsafe { &*TCA0::ptr() };
        (
            tim.split_hper().read().bits(),
            tim.split_lper().read().bits(),
        )
    }

    #[inline(always)]
    fn trigger_update(&mut self) {
        // self.split_ctrleset().write(|w| w.cmd().update());
    }

    #[inline(always)]
    fn max_period() -> u8 {
         u8::MAX
    }

    #[inline(always)]
    fn clear_overflow(&mut self) {
        self.split_intflags().modify(|_, w| w.lunf().set_bit().hunf().set_bit());
    }

    #[inline(always)]
    fn get_overflow(&self) -> bool {
        let r = self.split_intflags().read();
        r.lunf().bit_is_set() || r.hunf().bit_is_set()
    }
}

impl super::WithPwm for TCA0 {
    const CH_NUMBER: u8 = 3;
    // type GenerationMode = WaveformGenerationMode;
    type CompareValue = u8;

    fn set_pwm_mode(&mut self) {
        // self.split_ctrlb()
        //     .modify(|_, w| w.wgmode().variant(mode.into()));
    }

    fn enable_channel(channel: u8, b: bool) {
        let tim = unsafe { &*TCA0::ptr() };
        match channel {
            0 => tim.split_ctrlb().modify(|_, w| w.lcmp0en().bit(b)),
            1 => tim.split_ctrlb().modify(|_, w| w.lcmp1en().bit(b)),
            2 => tim.split_ctrlb().modify(|_, w| w.lcmp2en().bit(b)),
            3 => tim.split_ctrlb().modify(|_, w| w.hcmp0en().bit(b)),
            4 => tim.split_ctrlb().modify(|_, w| w.hcmp1en().bit(b)),
            5 => tim.split_ctrlb().modify(|_, w| w.hcmp2en().bit(b)),
            _ => panic!("invalid channel number"),
        }
    }

    fn set_compare_value(channel: u8, value: Self::CompareValue) {
        let tim = unsafe { &*TCA0::ptr() };
        match channel {
            0 => tim.split_lcmp0().write(|w| w.bits(value)),
            1 => tim.split_lcmp1().write(|w| w.bits(value)),
            2 => tim.split_lcmp2().write(|w| w.bits(value)),
            3 => tim.split_hcmp0().write(|w| w.bits(value)),
            4 => tim.split_hcmp1().write(|w| w.bits(value)),
            5 => tim.split_hcmp2().write(|w| w.bits(value)),
            _ => panic!("invalid channel number"),
        }
    }

    fn read_compare_value(channel: u8) -> Self::CompareValue {
        let tim = unsafe { &*TCA0::ptr() };
        match channel {
            0 => tim.split_lcmp0().read().bits(),
            1 => tim.split_lcmp1().read().bits(),
            2 => tim.split_lcmp2().read().bits(),
            3 => tim.split_hcmp0().read().bits(),
            4 => tim.split_hcmp1().read().bits(),
            5 => tim.split_hcmp2().read().bits(),
            _ => panic!("invalid channel number"),
        }
    }

    #[inline(always)]
    fn clear_compare_match(channel: u8) {
        let tim = unsafe { &*TCA0::ptr() };
        match channel {
            0 => tim.split_intflags().modify(|_, w| w.lcmp0().set_bit()),
            1 => tim.split_intflags().modify(|_, w| w.lcmp1().set_bit()),
            2 => tim.split_intflags().modify(|_, w| w.lcmp2().set_bit()),
            // 3 => tim.split_intflags().modify(|_, w| w.hcmp0().set_bit()),
            // 4 => tim.split_intflags().modify(|_, w| w.hcmp1().set_bit()),
            // 5 => tim.split_intflags().modify(|_, w| w.hcmp2().set_bit()),
            _ => panic!("invalid channel number"),
        }
    }

    #[inline(always)]
    fn get_compare_match(channel: u8) -> bool {
        let tim = unsafe { &*TCA0::ptr() };
        match channel {
            0 => tim.split_intflags().read().lcmp0().bit_is_set(),
            1 => tim.split_intflags().read().lcmp1().bit_is_set(),
            2 => tim.split_intflags().read().lcmp2().bit_is_set(),
            _ => panic!("invalid channel number"),
        }
    }
}

fn into_clksrc(prescaler: u16) -> split_ctrla::CLKSEL_A {
    use split_ctrla::CLKSEL_A::*;
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

fn from_clksrc(prescaler: split_ctrla::CLKSEL_A) -> u16 {
    use split_ctrla::CLKSEL_A::*;
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

use super::pwm::{C1, C2, C3, C4, C5, C6};
use super::WaveformOutputPinset;
use crate::gpio::{Output, Stateless};

/// A pin can be marked with this when it can be used as a waveform output pin
pub trait WaveformOutputPin<TCA, MuxMode, const CHAN: u8> {}

impl crate::private::Sealed for crate::pac::TCA0 {}

/// Pin set for the port multiplexer
pub struct TcaPinset<'muxproof, TIM, WaveformOutput: WaveformOutputPin<TIM, MuxMode, CHAN>, MuxMode, const CHAN: u8> {
    _tim: PhantomData<TIM>,
    _mux: PhantomData<&'muxproof MuxMode>,
    output: WaveformOutput,
}

impl<'muxproof, TIM, WaveformOutput, MuxMode, const CHAN: u8> TcaPinset<'muxproof, TIM, WaveformOutput, MuxMode, CHAN>
where
    WaveformOutput: WaveformOutputPin<TIM, MuxMode, CHAN>,
{
    pub fn prove(output: WaveformOutput, _proof: &PortmuxProof<'muxproof, PortmuxTCARoutes, MuxMode>) -> Self {
        Self::new(output)
    }

    pub(crate) fn new(output: WaveformOutput) -> Self {
        TcaPinset { _tim: PhantomData, _mux: PhantomData, output }
    }

    pub fn free(self) -> WaveformOutput {
        self.output
    }
}

impl<'muxproof, WaveformOutput: WaveformOutputPin<TCA0, MuxMode, CHAN>, MuxMode, const CHAN: u8> WaveformOutputPinset<TCA0, CHAN>
    for TcaPinset<'muxproof, TCA0, WaveformOutput, MuxMode, CHAN> {}

impl WaveformOutputPin<TCA0, TCA0PortA, C1> for crate::gpio::porta::PA0<Output<Stateless>> {}
impl WaveformOutputPin<TCA0, TCA0PortA, C2> for crate::gpio::porta::PA1<Output<Stateless>> {}
impl WaveformOutputPin<TCA0, TCA0PortA, C3> for crate::gpio::porta::PA2<Output<Stateless>> {}
// In split mode:
impl WaveformOutputPin<TCA0, TCA0PortA, C4> for crate::gpio::porta::PA3<Output<Stateless>> {}
impl WaveformOutputPin<TCA0, TCA0PortA, C5> for crate::gpio::porta::PA4<Output<Stateless>> {}
impl WaveformOutputPin<TCA0, TCA0PortA, C6> for crate::gpio::porta::PA5<Output<Stateless>> {}

// impl WaveformOutputPin<TCA0, C1> for crate::gpio::portb::PB3<Output<Stateless>> {}
// impl WaveformOutputPin<TCA0, C2> for crate::gpio::portb::PB4<Output<Stateless>> {}
// impl WaveformOutputPin<TCA0, C3> for crate::gpio::portb::PB5<Output<Stateless>> {}
// In split mode:
//impl WaveformOutputPin<TCA0, C4> for crate::gpio::portc::PC3<Output<Stateless>> {}
//impl WaveformOutputPin<TCA0, C5> for crate::gpio::portc::PC4<Output<Stateless>> {}
//impl WaveformOutputPin<TCA0, C6> for crate::gpio::portc::PC5<Output<Stateless>> {}
