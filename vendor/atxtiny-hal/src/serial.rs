//! # Serial
//!
//! Asynchronous serial communication using the interal USART peripherals
//!
//! The serial modules implement the [`Read`] and [`Write`] traits.
//!
//! [`Read`]: embedded_hal::serial::Read
//! [`Write`]: embedded_hal::serial::Write

use core::{
    fmt,
    ops::Deref,
    marker::PhantomData,
};

use crate::pac::usart0::{RegisterBlock, ctrlb::RXMODE_A};
use crate::embedded_io::{ErrorType as IoErrorType, Read as IoRead, Write as IoWrite};

use crate::{
    clkctrl::Clocks,
    time::*,
    Toggle,
};

#[cfg(feature = "enumset")]
use enumset::{EnumSet, EnumSetType};

pub mod config;

/// TX pin
pub trait TxPin<Usart>: crate::private::Sealed {}

/// RX pin
pub trait RxPin<Usart>: crate::private::Sealed {}

/// Pin set for the port multiplexer
pub struct UartPinset<Usart, Rx: RxPin<Usart>, Tx: TxPin<Usart>> {
    _usart: PhantomData<Usart>,
    rx: Rx,
    tx: Tx
}

impl<Usart, Rx, Tx> UartPinset<Usart, Rx, Tx>
where
    Rx: RxPin<Usart>,
    Tx: TxPin<Usart>,
{
    pub(crate) fn new(rx: Rx, tx: Tx) -> Self {
        UartPinset { _usart: PhantomData, rx, tx }
    }

    pub fn free(self) -> (Rx, Tx) { 
        (self.rx, self.tx)
    }
}

/// Status events.
///
/// All events can be cleared by [`Serial::clear_event`] or [`Serial::clear_events`].
/// Some events are also cleared on other conditions.
#[derive(ufmt::derive::uDebug, Debug)]
#[cfg_attr(feature = "enumset", derive(EnumSetType))]
#[cfg_attr(not(feature = "enumset"), derive(Copy, Clone, PartialEq, Eq))]
pub enum Event {
    /// Receive Complete Interrupt Flag
    ///
    /// This event is set by hardware when new data is present in the receive buffer.
    /// It is cleared by [`Serial`]s [`serial::Read::read()`] when reading data from the receive buffer.
    #[doc(alias = "RXCIF")]
    ReceiveComplete,

    /// Transmit Complete Interrupt Flag
    ///
    /// This event is set by hardware when the entire frame in the transmit shift register
    /// has been shifted out and there is no new data in the transmit buffer registers.
    #[doc(alias = "TXCIF")]
    TransmitComplete,

    /// Data Register Empty Interrupt Flag
    ///
    /// This event is set by hardware when the transmit data registers are empty and set
    /// when they contain data that has not yet been moved into the transmit shift register.
    #[doc(alias = "DREIF")]
    DataRegisterEmpty,

    /// Receive Start Interrupt Flag
    ///
    /// This event is set by hardware when the Start-Of-Frame detection is enabled, the device
    /// is in standby sleep mode and a valid start bit is detected.
    #[doc(alias = "RXSIF")]
    ReceiveStart,

    /// Inconsistent Synchronization Field Flag
    /// 
    /// This flag is set if an auto-baud mode is enabled, and the synchronization field is
    /// too short or too long to give a valid baud setting. It will also be set when USART
    /// is set to LINAUTO mode, and the SYNC character differs from data value 0x55.
    #[doc(alias = "ISFIF")]
    InconsistentSynchronizationField,

    /// Break Detected Flag
    ///
    /// This event is set if an auto-baud mode is enabled and a valid break and synchronization
    /// character is detected, and is cleared when the next data are received. 
    #[doc(alias = "BDF")]
    BreakDetected,
}

/// Interrupts.
///
/// Interrupts that can be enabled or disabled by [`Serial::enable_interrupt`]
/// or [`Serial::disable_interrupt`].
/// When an interrupt occurs, [`Event`] flags in status registers are set which can be read by
/// [`Serial::is_event_triggered`] and cleared by [`Serial::clear_event`].
#[derive(ufmt::derive::uDebug, Debug)]
#[cfg_attr(feature = "enumset", derive(EnumSetType))]
#[cfg_attr(not(feature = "enumset"), derive(Copy, Clone, PartialEq, Eq))]
pub enum Interrupt {
    /// Receive Complete Interrupt Enable
    ///
    /// This event is set by hardware when new data is present in the receive buffer.
    /// It is cleared by [`Serial`]s [`serial::Read::read()`] when reading data from the receive buffer.
    #[doc(alias = "RXCIE")]
    ReceiveComplete,

    /// Transmit Complete Interrupt Enable
    ///
    /// This event is set by hardware when the entire frame in the transmit shift register
    /// has been shifted out and there is no new data in the transmit buffer registers.
    #[doc(alias = "TXCIE")]
    TransmitComplete,

    /// Data Register Empty Interrupt Enable
    ///
    /// This event is set by hardware when the transmit data registers are empty and set
    /// when they contain data that has not yet been moved into the transmit shift register.
    #[doc(alias = "DREIE")]
    DataRegisterEmpty,

    /// Receive Start Interrupt Enable
    ///
    /// This event is set by hardware when the Start-Of-Frame detection is enabled, the device
    /// is in standby sleep mode and a valid start bit is detected.
    #[doc(alias = "RXSIE")]
    ReceiveStart,

    /// Auto-Baud Error Interrupt Enable
    ///
    /// This event is set if an auto-baud mode is enabled and no valid break and synchronization
    /// character is detected.
    #[doc(alias = "ABEIE")]
    AutoBaudError,
}

/// Serial error
///
/// As these are status events, they can be converted to [`Event`]s, via [`Into`].
#[derive(ufmt::derive::uDebug, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    /// Framing error
    ///
    /// This error is thrown by hardware when a de-synchronization, excessive noise or a break
    /// character is detected.
    Framing,
    /// RX buffer overrun
    ///
    /// # Cause
    ///
    /// An overrun error occurs when a character is received when the receive buffer is full,
    /// the receive shift register contains new data and a start bit is detected on the RX line.
    ///
    /// # Behavior
    ///
    /// - The received data is lost
    /// - The shift receive buffer is flushed until there is no new data available
    Overrun,
    /// Parity check error
    ///
    /// This error is thrown by hardware when a parity error occurs in receiver mode.
    Parity,
}

impl crate::embedded_io::Error for Error {
    fn kind(&self) -> crate::embedded_io::ErrorKind {
        use crate::embedded_io::ErrorKind;
        match *self {
            Error::Framing => ErrorKind::Other,
            Error::Overrun => ErrorKind::Other,
            Error::Parity =>  ErrorKind::Other,
        }
    }
}

/// An error that can be returned by the [`ufmt::uWrite`] trait
#[allow(non_camel_case_types)]
#[derive(ufmt::derive::uDebug, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct uWriteError;

/// Serial abstraction
///
/// This is an abstraction of the UART peripheral intended to be
/// used for standard duplex serial communication.
pub struct Serial<Usart, Pinset> {
    usart: Usart,
    pinset: Pinset,
}

mod split {
    use super::Instance;

    /// Serial receiver
    #[derive(ufmt::derive::uDebug, Debug)]
    pub struct Rx<Usart, Pin> {
        usart: Usart,
        pub(crate) pin: Pin,
    }

    /// Serial transmitter
    #[derive(ufmt::derive::uDebug, Debug)]
    pub struct Tx<Usart, Pin> {
        usart: Usart,
        pub(crate) pin: Pin,
    }

    impl<Usart, Pin> Tx<Usart, Pin>
    where
        Usart: Instance,
        Pin: super::TxPin<Usart>,
    {
        pub(crate) fn new(usart: Usart, pin: Pin) -> Self {
            Tx { usart, pin }
        }

        /// Destruct [`Tx`] to regain access to underlying USART and pin.
        pub(crate) fn free(self) -> (Usart, Pin) {
            (self.usart, self.pin)
        }
    }

    impl<Usart, Pin> Tx<Usart, Pin>
    where
        Usart: Instance,
    {
        /// Get a reference to internal usart peripheral
        ///
        /// # Safety
        ///
        /// This is unsafe, because the creation of this struct
        /// is only possible by splitting the the USART peripheral
        /// into Tx and Rx, which are internally both pointing
        /// to the same peripheral.
        ///
        /// Therefor, if getting a mutuable reference to the peripheral
        /// or changing any of it's configuration, the exclusivity
        /// is no longer guaranteed by the type system.
        ///
        /// Ensure that the Tx and Rx implemtation only do things with
        /// the peripheral, which do not effect the other.
        pub(crate) unsafe fn usart(&self) -> &Usart {
            &self.usart
        }

        /// Get a reference to internal usart peripheral
        ///
        /// # Saftey
        ///
        /// Same as in [`Self::usart()`].
        #[allow(dead_code)]
        pub(crate) unsafe fn usart_mut(&mut self) -> &mut Usart {
            &mut self.usart
        }
    }

    impl<Usart, Pin> Rx<Usart, Pin>
    where
        Usart: Instance,
        Pin: super::RxPin<Usart>,
    {
        pub(crate) fn new(usart: Usart, pin: Pin) -> Self {
            Rx { usart, pin }
        }

        /// Destruct [`Rx`] to regain access to the underlying pin.
        ///
        /// The USART is omitted, as it is returnend from Tx already to avoid
        /// beeing able to crate a duplicate reference to the same underlying
        /// peripheral.
        pub(crate) fn free(self) -> Pin {
            self.pin
        }
    }

    impl<Usart, Pin> Rx<Usart, Pin>
    where
        Usart: Instance,
    {
        // /// Get a reference to internal usart peripheral
        // ///
        // /// # Safety
        // ///
        // /// This is unsafe, because the creation of this struct
        // /// is only possible by splitting the the USART peripheral
        // /// into Tx and Rx, which are internally both pointing
        // /// to the same peripheral.
        // ///
        // /// Therefor, if getting a mutuable reference to the peripheral
        // /// or changing any of it's configuration, the exclusivity
        // /// is no longer guaranteed by the type system.
        // ///
        // /// Ensure that the Tx and Rx implemtation only do things with
        // /// the peripheral, which do not effect the other.
        // pub(crate) unsafe fn usart(&self) -> &Usart {
        //     &self.usart
        // }

        /// Get a reference to internal usart peripheral
        ///
        /// # Saftey
        ///
        /// Same as in [`Self::usart()`].
        pub(crate) unsafe fn usart_mut(&mut self) -> &mut Usart {
            &mut self.usart
        }
    }
}

pub use split::{Rx, Tx};

impl<Usart, RX, TX> Serial<Usart, UartPinset<Usart, RX, TX>>
where
    Usart: Instance,
    RX: RxPin<Usart>,
    TX: TxPin<Usart>,
{
    /// Configures a USART peripheral to provide serial communication
    pub fn new<Config>(
        usart: Usart,
        pinset: UartPinset<Usart, RX, TX>,
        config: Config,
        clocks: Clocks,
    ) -> Self
    where
        Config: Into<config::Config>,
    {
        let config = config.into();

        // Disable the transmitter and receiver
        usart.ctrlb().modify(|_, w| w
            .rxen().clear_bit()
            .txen().clear_bit()
        );

        // Calculate and set the baud rate
        let baudrate = config.baudrate.0;
        let f_per = Usart::clock(&clocks).raw();

        let (rxmode, brr) = if baudrate > (f_per / 16) {
            (RXMODE_A::CLK2X, (4 * f_per) / (baudrate / 2))
        } else {
            (RXMODE_A::NORMAL, (4 * f_per) / baudrate)
        };

        // FIXME: return error
        assert!(brr >= 64, "impossible baud rate");

        // FIXME: does the 16 bit write work correctly on the AVR mega cores?
        usart.baud().write(|w| { w.bits(brr as u16) });

        // Asynchronous mode, Parity, Stopbits and character size according to config
        usart.ctrlc().write(|w| w
            .cmode().asynchronous()
            .pmode().variant(config.parity.into())
            .sbmode().variant(config.stopbits.into())
            .chsize().variant(config.character_size.into())
        );

        // Disable all interrupts for now
        usart.ctrla().write(|w| w
            .rxcie().clear_bit()        // Receive Complete Interrupt Enable
            .txcie().clear_bit()        // Transmit Complete Interrupt Enable
            .dreie().clear_bit()        // Data Register Empty Interrupt Enable
            .rxsie().clear_bit()        // Receiver Start Frame Interrupt Enable
            .lbme().clear_bit()         // Loop-Back Mode Enable
            .abeie().clear_bit()        // Auto-Baud Error Interrupt Enable
            .rs485().off()              // RS-485 Mode
        );

        usart.ctrlb().write(|w| w
            .rxen().set_bit()           // Enable receiver
            .txen().set_bit()           // Enable transmitter
            .sfden().clear_bit()        // Disable start-of-frame detection
            .odme().clear_bit()         // Disable open-drain mode
            .rxmode().variant(rxmode)   // Set the baudrate generator mode
        );

        Self { usart, pinset }
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
    pub unsafe fn peripheral(&mut self) -> &mut Usart {
        &mut self.usart
    }

    /// Releases the USART peripheral and associated pinset
    pub fn free(self) -> (Usart, UartPinset<Usart, RX, TX>) {
        self.usart.ctrlb().modify(|_, w| w
            .rxen().clear_bit()
            .txen().clear_bit()
        );
        (self.usart, self.pinset)
    }

    /// Joins previously [`Serial::split()`] serial.
    ///
    /// This is often needed to access methods only implemented for [`Serial`]
    /// but not for [`Tx`] nor [`Rx`].
    ///
    /// # Example
    ///
    /// ```
    /// let dp = pac::Peripherals::take().unwrap();
    ///
    /// (tx, rx) = Serial::new(dp.USART1, ...).split();
    ///
    /// // Do something with tx and rx
    ///
    /// serial = Serial::join(tx, rx);
    /// ```
    pub fn join(rx: split::Rx<Usart, RX>, tx: split::Tx<Usart, TX>) -> Self
    where
        RX: RxPin<Usart>,
        TX: TxPin<Usart>,
    {
        let (usart, tx_pin) = tx.free();
        let rx_pin = rx.free();
        Self {
            usart,
            pinset: UartPinset::new(rx_pin, tx_pin),
        }
    }
}

impl<Usart, RX, TX> Serial<Usart, UartPinset<Usart, RX, TX>>
where
    Usart: Instance,
    RX: RxPin<Usart>,
    TX: TxPin<Usart>,
{
    /// Serial read out of the read register
    ///
    /// No error handling and no additional side-effects, besides the implied
    /// side-effects when reading out the RXDATAL register.
    /// Handling errors has to be done manually. This can be done, by checking
    /// the triggered events via [`Serial::triggered_events`].
    ///
    /// Returns `None` if there is no new data available.
    ///
    /// ## Embedded HAL
    ///
    /// To have a more managed way to read from the serial use the [`embedded_hal::serial::Read`]
    /// trait implementation.
    #[doc(alias = "RDR")]
    pub fn read_data_register(&self) -> Option<u8> {
        if self.usart.rxdatah().read().rxcif().bit_is_clear() {
            return None
        }

        Some(self.usart.rxdatal().read().bits() as u8)
    }

    /// Serial read out of the read register including the 8th bit
    ///
    /// Functionally this is equivalent to [`Serial::read_data_register`] but
    /// including the 8th bit from the RXDATAH register
    #[doc(alias = "RDR")]
    pub fn read_data_register_9bit(&self) -> Option<u16> {
        if self.usart.rxdatah().read().rxcif().bit_is_clear() {
            return None
        }

        let bit9 = self.usart.rxdatah().read().data8().bit() as u16;
        Some(bit9 << 8 | (self.usart.rxdatal().read().bits() as u16))
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
            Interrupt::ReceiveComplete => self.usart.ctrla().modify(|_, w| w.rxcie().bit(enable)),
            Interrupt::TransmitComplete => self.usart.ctrla().modify(|_, w| w.txcie().bit(enable)),
            Interrupt::DataRegisterEmpty => self.usart.ctrla().modify(|_, w| w.dreie().bit(enable)),
            Interrupt::ReceiveStart => self.usart.ctrla().modify(|_, w| w.rxsie().bit(enable)),
            Interrupt::AutoBaudError => self.usart.ctrla().modify(|_, w| w.abeie().bit(enable)),
        };
    }

    /// Enable or disable interrupt for the specified [`Interrupt`]s.
    ///
    /// Like [`Serial::configure_interrupt`], but instead using an enumset. The corresponding
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
            Interrupt::ReceiveComplete => self.usart.ctrla().read().rxcie().bit_is_set(),
            Interrupt::TransmitComplete => self.usart.ctrla().read().txcie().bit_is_set(),
            Interrupt::DataRegisterEmpty => self.usart.ctrla().read().dreie().bit_is_set(),
            Interrupt::ReceiveStart => self.usart.ctrla().read().rxsie().bit_is_set(),
            Interrupt::AutoBaudError => self.usart.ctrla().read().abeie().bit_is_set(),
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

    /// Check if an interrupt event happend.
    #[inline]
    pub fn is_event_triggered(&self, event: Event) -> bool {
        let isr = self.usart.status().read();
        match event {
            Event::ReceiveComplete => isr.rxcif().bit(),
            Event::TransmitComplete => isr.txcif().bit(),
            Event::DataRegisterEmpty => isr.dreif().bit(),
            Event::ReceiveStart => isr.rxsif().bit(),
            Event::InconsistentSynchronizationField => isr.isfif().bit(),
            Event::BreakDetected => isr.bdf().bit(),
        }
    }

    /// Get an [`EnumSet`] of all fired interrupt events.
    ///
    /// # Examples
    ///
    /// This allows disabling all fired event at once, via the enum set abstraction, like so
    ///
    /// ```rust
    /// for event in serial.triggered_events() {
    ///     serial.configure_interrupt(event, false);
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

    /// Clear the given interrupt event flag.
    #[inline]
    pub fn clear_event(&mut self, event: Event) {
        self.usart.status().write(|w| match event {
            Event::ReceiveComplete => w,                    // Not clearable
            Event::TransmitComplete => w.txcif().set_bit(),
            Event::DataRegisterEmpty => w,                  // Not clearable
            Event::ReceiveStart => w.rxsif().set_bit(),
            Event::InconsistentSynchronizationField => w.isfif().set_bit(),
            Event::BreakDetected => w.bdf().set_bit(),
        });
    }

    /// Clear **all** interrupt events.
    #[inline]
    pub fn clear_events(&mut self) {
        // SAFETY: This atomic write clears all flags and ignores the reserverd bit fields.
        self.usart.status().write(|w| unsafe { w.bits(u8::MAX) });
    }

    /// Set the baud rate generator mode to automatic baud rate generation
    #[inline]
    pub fn enable_autobaud(&mut self) {
        self.usart.ctrlb().write(|w| w.rxmode().genauto());
    }

    /// Enable or disable the interrupt for the break field detection.
    #[inline]
    pub fn set_wait_for_break(&mut self, enable: impl Into<Toggle>) {
        // Do a round way trip to be convert Into<Toggle> -> bool
        let enable: Toggle = enable.into();
        let enable: bool = enable.into();

        self.usart.status().write(|w| w.wfb().bit(enable) );
    }

    /// Enable or disable the multiprocessor communication mode.
    #[inline]
    pub fn set_multiprocessor_communication(&mut self, enable: impl Into<Toggle>) {
        // Do a round way trip to be convert Into<Toggle> -> bool
        let enable: Toggle = enable.into();
        let enable: bool = enable.into();

        self.usart.ctrlb().write(|w| w.mpcm().bit(enable) );
    }
}

/// Implementation of the [`embedded_hal::serial::Read`] trait
/// shared between [`Rx::read()`] and [`Serial::read()`]
fn eh_read<Usart>(usart: &mut Usart) -> Result<Option<u8>, Error>
where
    Usart: Instance,
{
    let rxdatah = usart.rxdatah().read();

    if rxdatah.perr().bit_is_set() {
        // Read and remove the data that caused this error
        let _ = usart.rxdatal().read();
        Err(Error::Parity)
    } else if rxdatah.ferr().bit_is_set() {
        // Read and remove the data that caused this error
        let _ = usart.rxdatal().read();
        Err(Error::Framing)
    } else if rxdatah.bufovf().bit_is_set() {
        // Flush the receive data
        //
        // Imagine a case of an overrun, where 2 or more bytes have been received by the hardware
        // but haven't been read out yet: An overrun is signaled!
        //
        // The current state is: One byte is in the RDR (read data register) one one byte is still
        // in the hardware pipeline (shift register).
        //
        // With this implementation, the overrun flag would be cleared but the data would not be
        // read out, so there are still to bytes waiting in the pipeline.
        //
        // In case the flush wasn't called: The next read would then be successful, as the RDR is
        // cleared, but the read after that would again report an overrun error, because the byte
        // still in the hardware shift register would signal it.
        //
        // This means, that the overrun error is not completely handled by this read()
        // implementation and leads to surprising behavior, if one would explicitly check for
        // Error::Overrun and think, that this error would than be handled, which would not be the
        // case.
        //
        // This is because, with this function signature, the data can not be returned
        // simultainously with the error.
        //
        // To mitigate this and have an implementation without these surprises flush the RDR
        // register. This leads to loosing a theoretically still receivable data byte! But at least
        // no cleanup is needed, after an overrun is called.
        while usart.rxdatah().read().rxcif().bit_is_set() {
            let _ = usart.rxdatal().read();
        }
        Err(Error::Overrun)
    } else if rxdatah.rxcif().bit_is_set() {
        Ok(Some(usart.rxdatal().read().bits() as u8))
    } else {
        Ok(None)
    }
}

impl<Usart, RX, TX> IoErrorType for Serial<Usart, UartPinset<Usart, RX, TX>>
where
    Usart: Instance,
    RX: RxPin<Usart>,
    TX: TxPin<Usart>,
{
    type Error = Error;
}

impl<Usart, RX, TX> IoRead for Serial<Usart, UartPinset<Usart, RX, TX>>
where
    Usart: Instance,
    RX: RxPin<Usart>,
    TX: TxPin<Usart>,
{
    /// Getting back an error means that the error is defined as "handled":
    ///
    /// This implementation has the side effect for error handling, that the [`Event`] flag of the returned
    /// [`Error`] is cleared.
    ///
    /// This might be a problem, because if an interrupt is enabled for this particular flag, the
    /// interrupt handler might not have the chance to find out from which flag the interrupt
    /// originated.
    ///
    /// So this function is only intended to be used for direct error handling and not leaving it
    /// up to the interrupt handler.
    ///
    /// To read out the content of the read register without internal error handling, use
    /// [`Serial::read_data_register(&mut self)`].
    /// ...
    // -> According to this API it should be skipped.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        loop {
            if let Some(b) = eh_read(&mut self.usart)? {
                buf[0] = b;
                return Ok(1);
            }
        }
    }
}

impl<Usart, Pin> IoErrorType for Rx<Usart, Pin>
where
    Usart: Instance,
    Pin: RxPin<Usart>,
{
    type Error = Error;
}

impl<Usart, Pin> IoRead for Rx<Usart, Pin>
where
    Usart: Instance,
    Pin: RxPin<Usart>,
{

    /// This implementation shares the same effects as the [`Serial`]s [`serial::Read`] implemenation.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        loop {
            if let Some(b) = eh_read(unsafe { self.usart_mut() })? {
                buf[0] = b;
                return Ok(1);
            }
        }
    }
}

impl<Usart, RX, TX> IoWrite for Serial<Usart, UartPinset<Usart, RX, TX>>
where
    Usart: Instance,
    RX: RxPin<Usart>,
    TX: TxPin<Usart>,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        for b in buf {
            while self.usart.status().read().dreif().bit_is_clear() {}
            self.usart.txdatal().write(|w| w.bits(*b));
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        while self.usart.status().read().txcif().bit_is_clear() {}
        self.usart.status().write(|w| w.txcif().clear_bit());
        Ok(())
    }
}

impl<Usart, RX, TX> fmt::Write for Serial<Usart, UartPinset<Usart, RX, TX>>
where
    Serial<Usart, UartPinset<Usart, RX, TX>>: IoWrite,
    RX: RxPin<Usart>,
    TX: TxPin<Usart>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s.as_bytes()).map(|_x| ()).map_err(|_| fmt::Error)
    }
}

impl<Usart, RX, TX> ufmt::uWrite for Serial<Usart, UartPinset<Usart, RX, TX>>
where
    Serial<Usart, UartPinset<Usart, RX, TX>>: IoWrite,
    RX: RxPin<Usart>,
    TX: TxPin<Usart>,
{
    type Error = uWriteError;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.write(s.as_bytes()).map(|_x| ()).map_err(|_| uWriteError)
    }

    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        self.write(&[c as u8]).map(|_x| ()).map_err(|_| uWriteError)
    }
}

impl<Usart, Pin> IoErrorType for Tx<Usart, Pin>
where
    Usart: Instance,
    Pin: TxPin<Usart>,
{
    type Error = Error;
}

impl<Usart, Pin> IoWrite for Tx<Usart, Pin>
where
    Usart: Instance,
    Pin: TxPin<Usart>,
{

    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        for b in buf {
            unsafe {
                while self.usart().status().read().dreif().bit_is_clear() {}
                self.usart().txdatal().write(|w| w.bits(*b));
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        unsafe {
            while self.usart().status().read().txcif().bit_is_clear() {}
            self.usart().status().write(|w| w.txcif().clear_bit());
        }
        Ok(())
    }
}

impl<Usart, Pin> fmt::Write for Tx<Usart, Pin>
where
    Tx<Usart, Pin>: IoWrite,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s.as_bytes()).map(|_x| ()).map_err(|_| fmt::Error)
    }
}

impl<Usart, Pin> ufmt::uWrite for Tx<Usart, Pin>
where
    Tx<Usart, Pin>: IoWrite,
{
    type Error = uWriteError;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.write(s.as_bytes()).map(|_x| ()).map_err(|_| uWriteError)
    }

    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        self.write(&[c as u8]).map(|_x| ()).map_err(|_| uWriteError)
    }
}



/// UART instance
pub trait Instance:
    Deref<Target = RegisterBlock>
    + crate::private::Sealed
{
    #[doc(hidden)]
    fn clock(clocks: &Clocks) -> Hertz;
}



macro_rules! uart {
    ({
        instance: $USART:ident,
        pins: [$(
            {
                tx: ($X_tx:ident/$x_tx:ident, $pin_tx:literal),
                rx: ($X_rx:ident/$x_rx:ident, $pin_rx:literal),
            },
        )+]
    }) => {
        use crate::pac::$USART;

        impl Instance for crate::pac::$USART {
            fn clock(clocks: &Clocks) -> Hertz {
                clocks.per()
            }
        }

        impl crate::private::Sealed for crate::pac::$USART {}

        impl<RX, TX> Serial<crate::pac::$USART, UartPinset<crate::pac::$USART, RX, TX>>
        where
            RX: RxPin<crate::pac::$USART>,
            TX: TxPin<crate::pac::$USART>,
        {
            /// Splits the [`Serial`] abstraction into a transmitter and a receiver half.
            ///
            /// This allows using [`Tx`] and [`Rx`] related actions to
            /// be handled independently and even use these safely in different
            /// contexts (like interrupt routines) without needing to do synchronization work
            /// between them.
            pub fn split(self) -> (split::Rx<crate::pac::$USART, RX>, split::Tx<crate::pac::$USART, TX>) {
                // NOTE(unsafe): This essentially duplicates the USART peripheral
                //
                // As RX and TX both do have direct access to the peripheral,
                // they must guarantee to only do atomic operations on the peripheral
                // registers to avoid data races.
                //
                // Tx and Rx won't access the same registers anyways,
                // as they have independent responsibilities, which are NOT represented
                // in the type system.
                let (rx, tx) = unsafe {
                    (
                        crate::pac::Peripherals::steal().$USART,
                        crate::pac::Peripherals::steal().$USART,
                    )
                };
                (split::Rx::new(rx, self.pinset.rx), split::Tx::new(tx, self.pinset.tx))
            }
        }

        $(
            paste::paste! {
                impl TxPin<$USART> for crate::gpio::[<port $x_tx>]::[<P $X_tx $pin_tx>]<Output<Stateless>> {}
                impl RxPin<$USART> for crate::gpio::[<port $x_rx>]::[<P $X_rx $pin_rx>]<Input> {}
            }
        )+
    };
}

use crate::gpio::{Input, Output, Stateless};

uart!({
    instance: USART0,
    pins: [
        {
            tx: (B/b, 2),
            rx: (B/b, 3),
        },
        {
            tx: (A/a, 1),
            rx: (A/a, 2),
        },
    ]
});
