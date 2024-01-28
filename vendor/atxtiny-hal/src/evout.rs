//! # Event output pins

use core::marker::PhantomData;

use crate::pac::EVSYS;
use crate::gpio::Peripheral;

/// Event output channel 0 (EVOUT0)
pub const EVOUT0: u8 = 0;

/// Event output channel 1 (EVOUT1)
pub const EVOUT1: u8 = 1;

/// Event output channel 2 (EVOUT2)
pub const EVOUT2: u8 = 2;

/// A pin can be marked with this when it can be used as an event output pin
pub trait EventOutputPin<EVSYS, const EVOUT: u8> {}

/// Pin set for the port multiplexer
pub struct EventOutputPinset<EVSYS, EventOutput: EventOutputPin<EVSYS, EVOUT>, const EVOUT: u8> {
    _evsys: PhantomData<EVSYS>,
    output: EventOutput,
}

impl<EVSYS, EventOutput, const EVOUT: u8> EventOutputPinset<EVSYS, EventOutput, EVOUT>
where
    EventOutput: EventOutputPin<EVSYS, EVOUT>,
{
    pub(crate) fn new(output: EventOutput) -> Self {
        EventOutputPinset { _evsys: PhantomData, output }
    }

    pub fn free(self) -> EventOutput { 
        self.output
    }
}

impl EventOutputPin<EVSYS, EVOUT0> for crate::gpio::porta::PA2<Peripheral<EVSYS>> {}
impl EventOutputPin<EVSYS, EVOUT1> for crate::gpio::portb::PB2<Peripheral<EVSYS>> {}
impl EventOutputPin<EVSYS, EVOUT2> for crate::gpio::portc::PC2<Peripheral<EVSYS>> {}

use crate::evsys::{Evsys, Async, EventUser};

impl EventUser<Evsys, Async> for EventOutputPinset<EVSYS, crate::gpio::porta::PA2<Peripheral<EVSYS>>, EVOUT0> { const MULTIPLEXER_INDEX: u8 = 8 + EVOUT0; }
impl EventUser<Evsys, Async> for EventOutputPinset<EVSYS, crate::gpio::portb::PB2<Peripheral<EVSYS>>, EVOUT1> { const MULTIPLEXER_INDEX: u8 = 8 + EVOUT1; }
impl EventUser<Evsys, Async> for EventOutputPinset<EVSYS, crate::gpio::portc::PC2<Peripheral<EVSYS>>, EVOUT2> { const MULTIPLEXER_INDEX: u8 = 8 + EVOUT2; }
