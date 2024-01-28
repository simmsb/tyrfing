//! # Serial Peripheral Interface (SPI) bus

// TODO: support buffered mode
// TODO: support slave mode both buffered and unbuffered
// TODO: support dual role mode and switching between master and slave mode using the ~SS pin
// TODO: interrupts

use core::cmp::max;
use core::{
    ops::Deref,
    marker::PhantomData,
};

use enumset::{EnumSet, EnumSetType};

use crate::embedded_hal::spi::{SpiBus, ErrorType, MODE_0, MODE_1, MODE_2, MODE_3};

use crate::{
    pac::spi0::{RegisterBlock, ctrla::PRESC_A, ctrlb::MODE_A},
    time::*,
    clkctrl::Clocks,
};

pub mod config;
use self::config::DataOrder;

/// SPI error
#[derive(ufmt::derive::uDebug, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    // NOTE: only in buffered mode
    // /// Overrun occurred
    // Overrun,

    /// Write collision occurred
    WriteCollision,
}

impl crate::embedded_hal::spi::Error for Error {
    fn kind(&self) -> crate::embedded_hal::spi::ErrorKind {
        use crate::embedded_hal::spi::ErrorKind;
        match *self {
            Error::WriteCollision => ErrorKind::Other
        }
    }
}

/// SCK pin
pub trait SckPin<SPI>: crate::private::Sealed {}

/// MISO (Master In - Slave Out) pin
pub trait MisoPin<SPI>: crate::private::Sealed {}

/// MOSI (Master Out - Slave In) pin
pub trait MosiPin<SPI>: crate::private::Sealed {}

// Note: commented out because we don't support the slave mode right now
// /// ~SS (Slave-Select) pin
// pub trait SsPin<SPI>: crate::private::Sealed {}

// TODO: make some pins optional?
/// Pin set for the port multiplexer
pub struct SpiPinset<SPI, Sck: SckPin<SPI>, Miso: MisoPin<SPI>, Mosi: MosiPin<SPI>> {
    _spi: PhantomData<SPI>,
    sck: Sck,
    miso: Miso,
    mosi: Mosi,
}

// FIXME: implement buffered mode
// /// SPI in buffered mode (type state)
// pub struct Buffered;

/// SPI in unbuffered mode (type state)
pub struct Unbuffered;

pub trait ED {}
//impl ED for Buffered {}
impl ED for Unbuffered {}

impl<SPI, Sck, Miso, Mosi> SpiPinset<SPI, Sck, Miso, Mosi>
where
    Sck: SckPin<SPI>,
    Miso: MisoPin<SPI>,
    Mosi: MosiPin<SPI>,
{
    pub(crate) fn new(sck: Sck, miso: Miso, mosi: Mosi) -> Self {
        SpiPinset { _spi: PhantomData, sck, miso, mosi }
    }

    pub fn free(self) -> (Sck, Miso, Mosi) { 
        (self.sck, self.miso, self.mosi)
    }

    // TODO: allow retrieval of certain pins as inputs if we don't need them
    //       this might be handy when routing signals to CCL
}

/// Unbuffered status events.
#[derive(ufmt::derive::uDebug, Debug)]
#[cfg_attr(feature = "enumset", derive(EnumSetType))]
#[cfg_attr(not(feature = "enumset"), derive(Copy, Clone, PartialEq, Eq))]
pub enum UnbufferedEvent {
    /// Interrupt
    ///
    /// This flag is set when a serial transfer is complete, and one byte is
    /// completely shifted in/out of the SPIn.DATA register.
    /// 
    /// If SS is configured as input and is driven low when the SPI is in Host
    /// mode, this will also set this flag. The IF is cleared by hardware when
    /// executing the corresponding interrupt vector. Alternatively, the IF can
    /// be cleared by first reading the SPIn.INTFLAGS register when IF is set
    /// and then accessing the SPIn.DATA register
    #[doc(alias = "IF")]
    Interrupt,

    /// Write Collision Flag
    ///
    /// The WRCOL flag is set if the SPIn.DATA register is written before a
    /// complete byte has been shifted out. This flag is cleared by first
    /// reading the SPIn.INTFLAGS register when WRCOL is set and then accessing
    /// the SPIn.DATA register.
    #[doc(alias = "WRCOL")]
    WriteCollision,
}

/// SPI abstraction in master mode
///
/// This is an abstraction of the SPI peripheral intended to be
/// used in master mode.
pub struct Spi<SPI, Mode, Pinset> {
    spi: SPI,
    pinset: Pinset,
    _mode: PhantomData<Mode>,
}

impl<SPI, SCK, MISO, MOSI> Spi<SPI, Unbuffered, SpiPinset<SPI, SCK, MISO, MOSI>>
where
    SPI: Instance,
    SCK: SckPin<SPI>,
    MISO: MisoPin<SPI>,
    MOSI: MosiPin<SPI>,
{
    /// Configures the SPI peripheral to work in unbuffered master mode
    pub fn new_unbuffered<Config>(
        spi: SPI,
        pinset: SpiPinset<SPI, SCK, MISO, MOSI>,
        config: Config,
        clocks: Clocks,
    ) -> Self
    where
        Config: Into<config::Config>,
    {
        let config = config.into();

        let mode = match config.mode {
            MODE_0 => MODE_A::_0,
            MODE_1 => MODE_A::_1,
            MODE_2 => MODE_A::_2,
            MODE_3 => MODE_A::_3,
        };

        let (clk2x, div) = Self::compute_baud_rate(clocks, config.frequency);

        // Disable the peripheral
        spi.ctrla().modify(|_, w| { w.enable().clear_bit() });

        // Configure SPI peripheral
        spi.ctrlb().modify(|_, w| { w
            .bufen().clear_bit()
            .bufwr().clear_bit()    // Disable buffer write mode (only valid for client mode anyway)
            .mode().variant(mode)
            .ssd().set_bit()        // Disable slave select
        });

        spi.ctrla().modify(|_, w| { w
            .dord().bit(config.order == DataOrder::LsbFirst)
            .enable().set_bit()     // Enable peripheral
            .master().set_bit()     // SPI Master

            .clk2x().bit(clk2x)     // Clock double speed
            .presc().variant(div)   // Clock prescaler
        });

        spi.intctrl().reset();

        Self { spi, pinset, _mode: PhantomData }
    }

    /// Enable the interrupt.
    #[inline]
    pub fn enable_interrupt(&mut self) {
        self.spi.intctrl().modify(|_, w| w.ie().set_bit());
    }

    /// Disable the interrupt.
    #[inline]
    pub fn disable_interrupt(&mut self) {
        self.spi.intctrl().modify(|_, w| w.ie().clear_bit());
    }

    /// Check if the interrupt is enabled.
    #[inline]
    pub fn is_interrupt_configured(&self) -> bool {
        self.spi.intctrl().read().ie().bit_is_set()
    }

    /// Check if an interrupt event happend.
    #[inline]
    pub fn is_event_triggered(&self, event: UnbufferedEvent) -> bool {
        let intflags = self.spi.intflags().read();
        match event {
            UnbufferedEvent::Interrupt => intflags.if_().bit(),
            UnbufferedEvent::WriteCollision => intflags.wrcol().bit(),
        }
    }

    /// Get an [`EnumSet`] of all fired interrupt events.
    #[cfg(feature = "enumset")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enumset")))]
    #[inline]
    pub fn triggered_events(&self) -> EnumSet<UnbufferedEvent> {
        let mut events = EnumSet::new();

        for event in EnumSet::<UnbufferedEvent>::all().iter() {
            if self.is_event_triggered(event) {
                events |= event;
            }
        }

        events
    }
}

impl<SPI, MODE, SCK, MISO, MOSI> Spi<SPI, MODE, SpiPinset<SPI, SCK, MISO, MOSI>>
where
    SPI: Instance,
    SCK: SckPin<SPI>,
    MISO: MisoPin<SPI>,
    MOSI: MosiPin<SPI>,
    MODE: ED
{
    fn compute_baud_rate(clocks: Clocks, freq: Hertz) -> (bool, PRESC_A) {
        match SPI::clock(&clocks).raw() / freq.raw() {
            0 => unreachable!(),
            1..=2 => (true, PRESC_A::DIV4),         // DIV_2
            3..=5 => (false, PRESC_A::DIV4),        // DIV_4
            6..=11 => (true, PRESC_A::DIV16),       // DIV_8
            12..=23 => (false, PRESC_A::DIV16),     // DIV_16
            24..=39 => (true, PRESC_A::DIV64),      // DIV_32
            40..=95 => (false, PRESC_A::DIV64),     // DIV_64
            //96..=191 => (false, PRESC_A::DIV128),   // DIV_128
            _ => (false, PRESC_A::DIV128),
        }
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
    pub unsafe fn peripheral(&mut self) -> &mut SPI {
        &mut self.spi
    }

    /// Releases the SPI peripheral and associated pins
    pub fn free(self) -> (SPI, SpiPinset<SPI, SCK, MISO, MOSI>) {
        (self.spi, self.pinset)
    }

    fn transfer_byte(&mut self, tx: u8) -> Result<u8, Error> {
        self.spi.data().write(|w| w.bits(tx));
        while self.spi.intflags().read().if_().bit_is_clear() {}
        Ok(self.spi.data().read().bits())
    }
}

impl<SPI, MODE, SCK, MISO, MOSI> ErrorType for Spi<SPI, MODE, SpiPinset<SPI, SCK, MISO, MOSI>>
where
    SPI: Instance,
    SCK: SckPin<SPI>,
    MISO: MisoPin<SPI>,
    MOSI: MosiPin<SPI>,
    MODE: ED,
{
    type Error = Error;
}


impl<SPI, MODE, SCK, MISO, MOSI> SpiBus for Spi<SPI, MODE, SpiPinset<SPI, SCK, MISO, MOSI>>
where
    SPI: Instance,
    SCK: SckPin<SPI>,
    MISO: MisoPin<SPI>,
    MOSI: MosiPin<SPI>,
    MODE: ED,
{
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        for w in words.iter_mut() {
            *w = self.transfer_byte(0xff)?;
        }

        Ok(())
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        for w in words.iter() {
            self.transfer_byte(*w)?;
        }

        Ok(())
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        for i in 0..max(read.len(), write.len()) {
            let tx_byte = if i < write.len() { write[i] } else { 0xff };
            let rx_byte = self.transfer_byte(tx_byte)?;
            if i < read.len() { read[i] = rx_byte };
        }

        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        for w in words.iter_mut() {
            let d = self.transfer_byte(*w)?;
            *w = d;
        }

        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        //while self.spi.intflags().read().if_().bit_is_clear() {}
        //let _ = self.spi.data().read().bits();
        Ok(())
    }
}

/// SPI instance
pub trait Instance:
    Deref<Target = RegisterBlock>
    + crate::private::Sealed
{
    #[doc(hidden)]
    fn clock(clocks: &Clocks) -> Hertz;
}



macro_rules! spi {
    ({
        instance: $SPI:ident,
        pins: [$(
            {
                sck: ($X_sck:ident/$x_sck:ident, $pin_sck:literal),
                miso: ($X_miso:ident/$x_miso:ident, $pin_miso:literal),
                mosi: ($X_mosi:ident/$x_mosi:ident, $pin_mosi:literal),
                //ss: ($X_ss:ident/$x_ss:ident, $pin_ss:literal),
            },
        )+]
    }) => {
        use crate::pac::$SPI;

        impl Instance for crate::pac::$SPI {
            fn clock(clocks: &Clocks) -> Hertz {
                clocks.per()
            }
        }

        impl crate::private::Sealed for crate::pac::$SPI {}

        $(
            paste::paste! {
                impl SckPin<$SPI> for crate::gpio::[<port $x_sck>]::[<P $X_sck $pin_sck>]<Output<Stateless>> {}
                impl MisoPin<$SPI> for crate::gpio::[<port $x_miso>]::[<P $X_miso $pin_miso>]<Input> {}
                impl MosiPin<$SPI> for crate::gpio::[<port $x_mosi>]::[<P $X_mosi $pin_mosi>]<Output<Stateless>> {}
                // NOTE: should we ever use that pin, it is an output in master mode, but it needs to be an
                //       input in slave mode, or when you want to dynamically switch between master and slave mode
                //impl SsPin<$SPI> for crate::gpio::[<port $x_ss>]::[<P $X_ss $pin_ss>]<Output<Stateless>> {}
            }
        )+
    };
}

use crate::gpio::{Input, Output, Stateless};

spi!({
    instance: SPI0,
    pins: [
        {
            sck: (A/a, 3),
            miso: (A/a, 2),
            mosi: (A/a, 1),
            //ss: (A/a, 4),
        },
        {
            sck: (C/c, 0),
            miso: (C/c, 1),
            mosi: (C/c, 2),
            //ss: (C/c, 3),
        },
    ]
});
