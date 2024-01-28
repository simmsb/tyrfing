//! # Two-wire interface (TWI) bus

// TODO: client mode

use core::{
    ops::Deref,
    marker::PhantomData,
};

use embedded_hal::i2c::{I2c, ErrorType, Operation};

use crate::{
    pac::twi0::RegisterBlock,
    clkctrl::Clocks,
    time::*,
    Toggle
};

#[cfg(feature = "enumset")]
use enumset::{EnumSet, EnumSetType};

pub mod config;

/// SCL pin
pub trait SclPin<TWI>: crate::private::Sealed {}

/// SDA pin
pub trait SdaPin<TWI>: crate::private::Sealed {}

/// Pin set for the port multiplexer
pub struct TwiPinset<TWI, Scl: SclPin<TWI>, Sda: SdaPin<TWI>> {
    _twi: PhantomData<TWI>,
    scl: Scl,
    sda: Sda
}

impl<TWI, Scl, Sda> TwiPinset<TWI, Scl, Sda>
where
    Scl: SclPin<TWI>,
    Sda: SdaPin<TWI>,
{
    pub(crate) fn new(scl: Scl, sda: Sda) -> Self {
        TwiPinset { _twi: PhantomData, scl, sda }
    }

    pub fn free(self) -> (Scl, Sda) { 
        (self.scl, self.sda)
    }
}

/// TWI error
#[derive(ufmt::derive::uDebug, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    /// Arbitration loss
    Arbitration,
    /// Bus error
    Bus,
    /// Bus busy
    Busy,
    /// Not Acknowledge received
    Nack(NackSource),
}

/// TWI NACK error source
#[derive(ufmt::derive::uDebug, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NackSource {
    /// NACK received during Address phase
    Address,

    /// NACK received while sending or receiving data
    Data,
}

impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        use embedded_hal::i2c::{ErrorKind, NoAcknowledgeSource};

        match *self {
            Error::Arbitration => ErrorKind::ArbitrationLoss,
            Error::Bus => ErrorKind::Bus,
            Error::Nack(NackSource::Address) => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Address),
            Error::Nack(NackSource::Data) => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Data),
            _ => ErrorKind::Other,
        }
    }
}

/// Status events.
///
/// All events can be cleared by [`Twi::clear_event`] or [`Twi::clear_events`].
/// Some events are also cleared on other conditions.
#[derive(ufmt::derive::uDebug, Debug)]
#[cfg_attr(feature = "enumset", derive(EnumSetType))]
#[cfg_attr(not(feature = "enumset"), derive(Copy, Clone, PartialEq, Eq))]
pub enum Event {
    /// Read Interrupt Flag
    ///
    /// This flag is set to when the host byte read operation is completed.
    #[doc(alias = "RIF")]
    ReadInterrupt,

    /// Write Interrupt Flag
    ///
    /// This flag is set to when a host transmit address or byte write operation
    /// is completed, regardless of the occurrence of a bus error or arbitration
    /// lost condition.
    #[doc(alias = "WIF")]
    WriteInterrupt,

    /// Clock Hold
    ///
    /// When this flag is set, it indicates that the host is currently holding
    /// the SCL low, stretching the TWI clock period.
    #[doc(alias = "CLKHOLD")]
    ClockHold,

    /// Received Acknowledge
    ///
    /// When this flag is read as ‘0’, it indicates that the most recent
    /// Acknowledge bit from the client was ACK, and the client is ready for
    /// more data.
    /// When this flag is read as ‘1’, it indicates that the most recent
    /// Acknowledge bit from the client was NACK, and the client is not able to
    /// or does not need to receive more data.
    #[doc(alias = "RXACK")]
    ReceivedAcknowledge,

    /// Arbitration Lost
    /// 
    /// When this bit is read as ‘1’, it indicates that the host has lost
    /// arbitration. This can happen in one of the following cases:
    /// 1. While transmitting a high data bit.
    /// 2. While transmitting a NACK bit.
    /// 3. While issuing a Start condition (S).
    /// 4. While issuing a repeated Start (Sr).
    #[doc(alias = "ARBLOST")]
    ArbitrationLost,

    /// Bus Error
    ///
    /// The BUSERR flag indicates that an illegal bus operation has occurred.
    /// An illegal bus operation is detected if a protocol violating the
    /// Start (S), repeated Start (Sr), or Stop (P) conditions is detected on
    /// the TWI bus lines. A Start condition directly followed by a Stop
    /// condition is one example of a protocol violation.
    #[doc(alias = "BUSERR")]
    BusError,
}

/// TWI bus state.
/// 
/// Indication of the current TWI bus state.
pub enum BusState {
    /// Unknown bus state
    #[doc(alias = "UNKNOWN")]
    Unknown,

    /// Idle bus state
    #[doc(alias = "IDLE")]
    Idle,

    /// This TWI controls the bus
    #[doc(alias = "OWNER")]
    Owner,

    /// Busy bus state
    #[doc(alias = "BUSY")]
    Busy,
}

/// Interrupts.
///
/// Interrupts that can be enabled or disabled by [`Twi::enable_interrupt`]
/// or [`Twi::disable_interrupt`].
/// When an interrupt occurs, [`Event`] flags in status registers are set which can be read by
/// [`Twi::is_event_triggered`] and cleared by [`Twi::clear_event`].
#[derive(ufmt::derive::uDebug, Debug)]
#[cfg_attr(feature = "enumset", derive(EnumSetType))]
#[cfg_attr(not(feature = "enumset"), derive(Copy, Clone, PartialEq, Eq))]
pub enum Interrupt {
    /// Read Interrupt Enable
    #[doc(alias = "RIEN")]
    Read,

    /// Write Interrupt Enable
    #[doc(alias = "WIEN")]
    Write,
}

/// TWI abstraction in master mode
///
/// This is an abstraction of the TWI peripheral intended to be
/// used in master mode.
pub struct Twi<TWI, Pinset> {
    twi: TWI,
    pinset: Pinset,
}

impl<TWI, SCL, SDA> Twi<TWI, TwiPinset<TWI, SCL, SDA>>
where
    TWI: Instance,
    SCL: SclPin<TWI>,
    SDA: SdaPin<TWI>,
{
    /// Configures the TWI peripheral to work in master mode
    pub fn new<Config>(
        twi: TWI,
        pinset: TwiPinset<TWI, SCL, SDA>,
        config: Config,
        clocks: Clocks,
    ) -> Self 
    where
        Config: Into<config::Config>,
    {
        let config = config.into();

        // FIXME: What about CTRLA.FMPEN? What exactly does this do? Do we need
        //        to set this bit for 1MHz? What about 500KHz?
        let frequency = config.frequency.raw();
        let rise_time = config.rise_time.ticks();
        let f_per = TWI::clock(&clocks).raw();

        let baudrate: u8 = (((f_per/frequency) - ((f_per*rise_time)/1000000000) - 10) / 2) as u8;

        twi.ctrla().modify(|_, w| w.fmpen().variant(config.fast_mode_plus));

        // Set the baud rate divider and enable the peripheral
        twi.mctrla().modify(|_, w| w.enable().clear_bit());
        twi.mbaud().write(|w| w.bits(baudrate));
        twi.mctrla().modify(|_, w| w.enable().set_bit());

        // Force the state-machine into IDLE state
        twi.mstatus().modify(|_, w| w.busstate().idle());

        // Clear a bunch of status flags
        twi.mstatus().modify(|_, w| w
            .rif().set_bit()
            .wif().set_bit()
            .buserr().set_bit()
        );

        Self { twi, pinset }
    }

    /// Get access to the underlying register block.
    ///
    /// # Safety
    ///
    /// This function is not _memory_ unsafe per se, but does not guarantee
    /// anything about assumptions of invariants made in this implementation.
    ///
    /// Changing specific options can lead to un-expected behavior and nothing
    /// is guaranteed.
    pub unsafe fn peripheral(&mut self) -> &mut TWI {
        &mut self.twi
    }

    /// Enable the interrupt for the specified [`Interrupt`].
    #[inline]
    pub fn enable_interrupt(&mut self, interrupt: Interrupt) {
        self.configure_interrupt(interrupt, Toggle::On);
    }

    /// Disable the interrupt for the specified [`Interrupt`].
    #[inline]
    pub fn disable_interrupt(&mut self, interrupt: Interrupt) {
        self.configure_interrupt(interrupt, Toggle::Off);
    }

    /// Enable or disable the interrupt for the specified [`Interrupt`].
    #[inline]
    pub fn configure_interrupt(&mut self, interrupt: Interrupt, enable: impl Into<Toggle>) {
        // Do a round way trip to be convert Into<Toggle> -> bool
        let enable: Toggle = enable.into();
        let enable: bool = enable.into();
        match interrupt {
            Interrupt::Read => self.twi.mctrla().modify(|_, w| w.rien().bit(enable)),
            Interrupt::Write => self.twi.mctrla().modify(|_, w| w.wien().bit(enable)),
        };
    }

    /// Enable or disable interrupt for the specified [`Interrupt`]s.
    ///
    /// Like [`Twi::configure_interrupt`], but instead using an enumset. The corresponding
    /// interrupt for every [`Interrupt`] in the set will be enabled, every other interrupt will be
    /// **disabled**.
    #[cfg(feature = "enumset")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enumset")))]
    #[inline]
    pub fn configure_interrupts(&mut self, interrupts: EnumSet<Interrupt>) {
        for event in interrupts.complement().iter() {
            self.configure_interrupt(event, false);
        }
        for event in interrupts.iter() {
            self.configure_interrupt(event, true);
        }
    }

    /// Check if an interrupt is configured for the [`Interrupt`]
    #[inline]
    pub fn is_interrupt_configured(&self, interrupt: Interrupt) -> bool {
        match interrupt {
            Interrupt::Read => self.twi.mctrla().read().rien().bit_is_set(),
            Interrupt::Write => self.twi.mctrla().read().wien().bit_is_set(),
        }
    }

    /// Check which interrupts are enabled for all [`Interrupt`]s
    #[cfg(feature = "enumset")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enumset")))]
    #[inline]
    pub fn configured_interrupts(&mut self) -> EnumSet<Interrupt> {
        let mut interrupts = EnumSet::new();

        for interrupt in EnumSet::<Interrupt>::all().iter() {
            if self.is_interrupt_configured(interrupt) {
                interrupts |= interrupt;
            }
        }

        interrupts
    }

    /// Check if an event happend.
    #[inline]
    pub fn is_event_triggered(&self, event: Event) -> bool {
        let mstatus = self.twi.mstatus().read();
        match event {
            Event::ReadInterrupt => mstatus.rif().bit(),
            Event::WriteInterrupt => mstatus.wif().bit(),
            Event::ClockHold => mstatus.clkhold().bit(),
            Event::ReceivedAcknowledge => mstatus.rxack().bit(),
            Event::ArbitrationLost => mstatus.arblost().bit(),
            Event::BusError => mstatus.buserr().bit(),
        }
    }

    /// Get an [`EnumSet`] of all fired interrupt events.
    ///
    /// # Examples
    ///
    /// This allows disabling all fired event at once, via the enum set abstraction, like so
    ///
    /// ```rust
    /// for event in twi.triggered_events() {
    ///     twi.configure_interrupt(event, false);
    /// }
    /// ```
    #[cfg(feature = "enumset")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enumset")))]
    #[inline]
    pub fn triggered_events(&self) -> EnumSet<Event> {
        let mut events = EnumSet::new();

        for event in EnumSet::<Event>::all().iter() {
            if self.is_event_triggered(event) {
                events |= event;
            }
        }

        events
    }

    /// Releases the TWI peripheral and associated pins
    pub fn free(self) -> (TWI, TwiPinset<TWI, SCL, SDA>) {
        (self.twi, self.pinset)
    }

    /// Clear the given event flag.
    #[inline]
    pub fn clear_event(&mut self, event: Event) {
        self.twi.mstatus().write(|w| match event {
            Event::ReadInterrupt => w.rif().set_bit(),
            Event::WriteInterrupt => w.wif().set_bit(),
            Event::ClockHold => w.clkhold().set_bit(),
            Event::ReceivedAcknowledge => w,                    // Not clearable
            Event::ArbitrationLost => w.arblost().set_bit(),
            Event::BusError => w.buserr().set_bit(),
        });
    }

    /// Clear **all** events.
    #[inline]
    pub fn clear_events(&mut self) {
        // SAFETY: This atomic write clears all flags that are clearable and spares out the busstate.
        self.twi.mstatus().write(|w| unsafe { w.bits(0b11101100) });
    }
}

macro_rules! busy_wait {
    ($i2c:expr, $nacksource:expr) => {
        loop {
            let mstatus = $i2c.mstatus().read();

            if mstatus.arblost().bit_is_set() {
                // ARBLOST gets cleared on the next MADDR write
                return Err(Error::Arbitration);
            } else if mstatus.buserr().bit_is_set() {
                // BUSERR gets cleared on the next MADDR write
                return Err(Error::Bus);
            } else if (mstatus.wif().bit_is_set() || mstatus.rif().bit_is_set()) {
                // Received NACK
                if mstatus.rxack().bit_is_set() {
                    $i2c.mctrlb().modify(|_, w| w.mcmd().stop());
                    return Err(Error::Nack($nacksource));
                } else {
                    break;
                }
            }
        }
    };
}

macro_rules! wait_ownership {
    ($i2c:expr) => {
        loop {
            let mstatus = $i2c.mstatus().read();

            if mstatus.arblost().bit_is_set() {
                return Err(Error::Arbitration);
            }

            if mstatus.busstate().is_owner() {
                break;
            } 
        }
    };
}

impl<TWI, SCL, SDA> ErrorType for Twi<TWI, TwiPinset<TWI, SCL, SDA>>
where
    TWI: Instance,
    SCL: SclPin<TWI>,
    SDA: SdaPin<TWI>,
{
    type Error = Error;
}

impl<TWI, SCL, SDA> I2c for Twi<TWI, TwiPinset<TWI, SCL, SDA>>
where
    TWI: Instance,
    SCL: SclPin<TWI>,
    SDA: SdaPin<TWI>,
{
    fn transaction(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<(), Error> {
        // Detect Bus busy
        if self.twi.mstatus().read().busstate().is_busy() {
            return Err(Error::Busy);
        }

        if operations.is_empty() {
            return Ok(());
        }

        for operation in operations {
            match operation {
                Operation::Read(buffer) => {
                    // Write the address and read-bit
                    // This kicks off a START or repeated START condition on the bus
                    self.twi.maddr().write(|w| { w.bits(address << 1 | 1) });

                    // Wait for the bus state to transition into OWNED
                    wait_ownership!(self.twi);

                    // Wait for the address to be ACKed or NACKed
                    busy_wait!(self.twi, NackSource::Address);

                    // Special case for zero-length receive buffers
                    // Just set the ACK action to NACK. The next write to MADDR or
                    // the STOP action that is executed at the end of this function
                    // then performs the NACK and the appropriate action like a STOP or
                    // repeated START
                    self.twi.mctrlb().modify(|_, w| w.ackact().set_bit());

                    let mut it = buffer.iter_mut().peekable();
                    while let Some(b) = it.next() {
                        // Wait for data
                        busy_wait!(self.twi, NackSource::Data);

                        // Not the last byte we expect? ACK it, otherwise NACK it
                        // The following read from MDATA triggers the RECVTRANS action automatically
                        if it.peek().is_some() {
                            self.twi.mctrlb().modify(|_, w| w.ackact().clear_bit());
                        } else {
                            self.twi.mctrlb().modify(|_, w| w.ackact().set_bit());
                        }

                        // Read data and trigger ACK/NACK
                        *b = self.twi.mdata().read().bits();
                    }
                },

                Operation::Write(buffer) => {
                    // Write the address and ~read-bit
                    // This kicks off a START or repeated START condition on the bus
                    self.twi.maddr().write(|w| { w.bits(address << 1 | 0) });

                    // Wait for the bus state to transition into OWNED
                    wait_ownership!(self.twi);

                    // Wait for the address to be ACKed or NACKed
                    busy_wait!(self.twi, NackSource::Address);

                    // Send bytes in the buffer
                    // Should the sent byte be NACKed, the busy_wait! macro will
                    // return and issue a STOP condition on the bus
                    for b in buffer.iter() {
                        self.twi.mdata().write(|w| w.bits(*b));
                        busy_wait!(self.twi, NackSource::Data);
                    }
                }
            }
        }

        // Send the final STOP
        self.twi.mctrlb().modify(|_, w| w.mcmd().stop());

        Ok(())
    }
}

/// TWI instance
pub trait Instance:
    Deref<Target = RegisterBlock>
    + crate::private::Sealed
{
    #[doc(hidden)]
    fn clock(clocks: &Clocks) -> Hertz;
}



macro_rules! twi {
    ({
        instance: $TWI:ident,
        pins: [$(
            {
                scl: ($X_scl:ident/$x_scl:ident, $pin_scl:literal),
                sda: ($X_sda:ident/$x_sda:ident, $pin_sda:literal),
            },
        )+]
    }) => {
        use crate::pac::$TWI;

        impl Instance for crate::pac::$TWI {
            fn clock(clocks: &Clocks) -> Hertz {
                clocks.per()
            }
        }

        impl crate::private::Sealed for crate::pac::$TWI {}

        $(
            paste::paste! {
                impl SclPin<$TWI> for crate::gpio::[<port $x_scl>]::[<P $X_scl $pin_scl>]<Peripheral<$TWI>> {}
                impl SdaPin<$TWI> for crate::gpio::[<port $x_sda>]::[<P $X_sda $pin_sda>]<Peripheral<$TWI>> {}
            }
        )+
    };
}

use crate::gpio::Peripheral;

twi!({
    instance: TWI0,
    pins: [
        {
            scl: (B/b, 0),
            sda: (B/b, 1),
        },
        {
            scl: (A/a, 2),
            sda: (A/a, 1),
        },
    ]
});
