//! # Analog comparator

use crate::{pac::AC0, gpio::{Analog, Output, Stateless}, dac::DACOutputToAC};
use core::marker::PhantomData;

/// Enabled Comparator (type state)
pub struct Enabled;

/// Disabled Comparator (type state)
pub struct Disabled;

pub trait ED {}
impl ED for Enabled {}
impl ED for Disabled {}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config {
    pub hysteresis: Hysteresis,
    pub low_power_mode: bool,
    pub inverted: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hysteresis: Hysteresis::Off,
            low_power_mode: false,
            inverted: false,
        }
    }
}

impl Config {
    pub fn hysteresis(mut self, hysteresis: Hysteresis) -> Self {
        self.hysteresis = hysteresis;
        self
    }

    pub fn low_power_mode(mut self) -> Self {
        self.low_power_mode = true;
        self
    }

    pub fn output_inverted(mut self) -> Self {
        self.inverted = true;
        self
    }

    pub fn output_polarity(mut self, inverted: bool) -> Self {
        self.inverted = inverted;
        self
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Hysteresis {
    Off = 0,
    _10mV = 1,
    _25mV = 2,
    _50mV = 3,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InterruptMode {
    BothEdges = 0,
    NegativeEdge = 2,
    PositiveEdge = 3,
}

pub struct Comparator<AC, ED> {
    regs: AC,
    _enabled: PhantomData<ED>,
}

pub trait ComparatorExt<AC> {
    /// Initializes a comparator
    fn comparator<P: PositiveInput<AC>, N: NegativeInput<AC>>(
        self,
        positive_input: P,
        negative_input: N,
        config: Config,
    ) -> Comparator<AC, Disabled>;
}

macro_rules! impl_comparator {
    ($COMP:ty, $comp:ident) => {
        impl ComparatorExt<$COMP> for $COMP {
            fn comparator<P: PositiveInput<$COMP>, N: NegativeInput<$COMP>>(
                self,
                positive_input: P,
                negative_input: N,
                config: Config,
            ) -> Comparator<$COMP, Disabled> {
                self.ctrla().modify(|_, w| w
                    .hysmode().bits(config.hysteresis as u8)
                    .lpmode().bit(config.low_power_mode)
                );

                self.muxctrla().modify(|_, w| w.invert().bit(config.inverted));
                positive_input.setup(&self);
                negative_input.setup(&self);

                Comparator {
                    regs: self,
                    _enabled: PhantomData,
                }
            }
        }

        impl Comparator<$COMP, Disabled> {
            /// Initializes a comparator
            pub fn $comp<P: PositiveInput<$COMP>, N: NegativeInput<$COMP>>(
                comp: $COMP,
                positive_input: P,
                negative_input: N,
                config: Config,
            ) -> Self {
                comp.comparator(positive_input, negative_input, config)
            }

            /// Enables the comparator
            pub fn enable(self) -> Comparator<$COMP, Enabled> {
                self.regs.ctrla().modify(|_, w| w.enable().set_bit());
                Comparator {
                    regs: self.regs,
                    _enabled: PhantomData,
                }
            }

            /// Enables raising the comparator interrupt at the specified output signal edge
            #[inline]
            pub fn listen(&self, mode: InterruptMode) {
                self.regs.ctrla().modify(|_, w| unsafe { w.intmode().bits(mode as u8) });
                self.regs.intctrl().write(|w| w.cmp().set_bit());
            }
        }

        impl Comparator<$COMP, Enabled> {
            /// Returns the value of the output of the comparator
            #[inline]
            pub fn output(&self) -> bool {
                self.regs.status().read().state().bit_is_set()
            }

            /// Disables the comparator
            pub fn disable(self) -> Comparator<$COMP, Disabled> {
                self.regs.ctrla().modify(|_, w| w.enable().clear_bit());
                Comparator {
                    regs: self.regs,
                    _enabled: PhantomData,
                }
            }
        }

        impl<ED> Comparator<$COMP, ED> {
            /// Disables raising interrupts for the output signal
            #[inline]
            pub fn unlisten(&self) {
                self.regs.intctrl().modify(|_, w| w.cmp().clear_bit());
            }

            /// Returns `true` if the output signal interrupt is pending
            #[inline]
            pub fn is_pending(&self) -> bool {
                self.regs.status().read().cmp().bit_is_set()
            }

            /// Unpends the output signal interrupt
            #[inline]
            pub fn unpend(&self) {
                self.regs.status().modify(|_, w| w.cmp().set_bit());
            }

            /// Configures a GPIO pin to output the signal of the comparator
            #[inline]
            pub fn output_pin<P: ComparatorOutput<$COMP>>(&self, pin: P) {
                pin.setup(&self.regs);
            }
        }
    };
}

pub trait NegativeInput<AC>: crate::private::Sealed { fn setup(&self, comp: &AC); }
pub trait PositiveInput<AC>: crate::private::Sealed { fn setup(&self, comp: &AC); }
pub trait ComparatorOutput<AC>: crate::private::Sealed { fn setup(&self, comp: &AC); }

macro_rules! positive_input_pin {
    ($COMP:ident, $pin:ty, $variant:expr) => {
        impl PositiveInput<$COMP> for $pin {
            #[inline]
            fn setup(&self, comp: &$COMP) {
                comp.muxctrla().modify(|_, w| w.muxpos().variant($variant))
            }
        }
    };
}

macro_rules! negative_input_pin {
    ($COMP:ident, $pin:ty, $variant:expr) => {
        impl NegativeInput<$COMP> for $pin {
            #[inline]
            fn setup(&self, comp: &$COMP) {
                comp.muxctrla().modify(|_, w| w.muxneg().variant($variant))
            }
        }
    };
}

macro_rules! output_pin {
    ($COMP:ident, $pin:ty) => {
        impl ComparatorOutput<$COMP> for $pin {
            #[inline]
            fn setup(&self, comp: &$COMP) {
                comp.ctrla().modify(|_, w| w.outen().set_bit())
            }
        }
    };
}

macro_rules! refint_input {
    ($COMP:ident, $reft:ty, $variant:expr) => {
        impl NegativeInput<$COMP> for $reft {
            #[inline]
            fn setup(&self, comp: &$COMP) {
                comp.muxctrla().modify(|_, w| w.muxneg().variant($variant))
            }
        }
    };
}

impl_comparator!(AC0, ac0);

positive_input_pin!(AC0, crate::gpio::porta::PA7<Analog>, crate::pac::ac0::muxctrla::MUXPOS_A::PIN0);
positive_input_pin!(AC0, crate::gpio::portb::PB5<Analog>, crate::pac::ac0::muxctrla::MUXPOS_A::PIN1);

negative_input_pin!(AC0, crate::gpio::porta::PA6<Analog>, crate::pac::ac0::muxctrla::MUXNEG_A::PIN0);
negative_input_pin!(AC0, crate::gpio::portb::PB4<Analog>, crate::pac::ac0::muxctrla::MUXNEG_A::PIN1);

impl NegativeInput<AC0> for DACOutputToAC<0> {
    #[inline]
    fn setup(&self, comp: &AC0) {
        comp.muxctrla().modify(|_, w| w.muxneg().dac())
    }
}

output_pin!(AC0, crate::gpio::porta::PA5<Output<Stateless>>);

use crate::vref::DACReferenceVoltage;
refint_input!(AC0, DACReferenceVoltage<0>, crate::pac::ac0::muxctrla::MUXNEG_A::VREF);


use crate::evsys::ChannelConfigurator;
use crate::evsys::{EventGenerator, Channel, Unconfigured, GeneratorAssigned};

impl<Evsys, Index, AC> EventGenerator<Evsys, crate::evsys::Async, Index> for Comparator<AC, Disabled>
where
    Evsys: crate::evsys::marker::Evsys,
    Index: crate::evsys::marker::Index,
{
    type EventSource = ();

    fn connect_event_generator(&mut self, mut channel: Channel<Evsys, crate::evsys::Async, Index, Unconfigured>, _source: ()) -> Channel<Evsys, crate::evsys::Async, Index, GeneratorAssigned> {
        channel.set_generator(0x03);
        channel.into_state()
    }
}
