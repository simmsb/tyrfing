//! Types for configuring a serial interface.

use crate::pac::usart0::ctrlc::{SBMODE_A, PMODE_A, CHSIZE_A};
use crate::time::*;

/// Stop Bit configuration parameter for serial.
///
/// Wrapper around [`SBMODE_A`]
#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopBits {
    /// 1 stop bit
    Stop1,
    /// 2 stop bit
    Stop2,
}

impl From<StopBits> for SBMODE_A {
    fn from(stopbit: StopBits) -> Self {
        match stopbit {
            StopBits::Stop1 => SBMODE_A::_1BIT,
            StopBits::Stop2 => SBMODE_A::_2BIT,
        }
    }
}

impl From<SBMODE_A> for StopBits {
    fn from(stopbit: SBMODE_A) -> Self {
        match stopbit {
            SBMODE_A::_1BIT => StopBits::Stop1,
            SBMODE_A::_2BIT => StopBits::Stop2,
        }
    }
}

/// Parity generation and checking. If odd or even parity is selected, the
/// underlying USART will be configured to send/receive the parity bit in
/// addtion to the data bits.
#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    /// No parity bit will be added/checked.
    None,
    /// The MSB transmitted/received will be generated/checked to have a
    /// even number of bits set.
    Even,
    /// The MSB transmitted/received will be generated/checked to have a
    /// odd number of bits set.
    Odd,
}

impl From<Parity> for PMODE_A {
    fn from(stopbit: Parity) -> Self {
        match stopbit {
            Parity::None => PMODE_A::DISABLED,
            Parity::Even => PMODE_A::EVEN,
            Parity::Odd => PMODE_A::ODD,
        }
    }
}

impl From<PMODE_A> for Parity {
    fn from(stopbit: PMODE_A) -> Self {
        match stopbit {
            PMODE_A::DISABLED => Parity::None,
            PMODE_A::EVEN => Parity::Even,
            PMODE_A::ODD => Parity::Odd,
        }
    }
}

/// Character size that the UART hardware sends and receives
#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterSize {
    Size5,
    Size6,
    Size7,
    Size8,
    // TODO: Add support
    //Size9_LSB,
    //Size9_MSB,
}

impl From<CharacterSize> for CHSIZE_A {
    fn from(chsize: CharacterSize) -> Self {
        match chsize {
            CharacterSize::Size5 => CHSIZE_A::_5BIT,
            CharacterSize::Size6 => CHSIZE_A::_6BIT,
            CharacterSize::Size7 => CHSIZE_A::_7BIT,
            CharacterSize::Size8 => CHSIZE_A::_8BIT,
        }
    }
}

impl From<CHSIZE_A> for CharacterSize {
    fn from(chsize: CHSIZE_A) -> Self {
        match chsize {
            CHSIZE_A::_5BIT => CharacterSize::Size5,
            CHSIZE_A::_6BIT => CharacterSize::Size6,
            CHSIZE_A::_7BIT => CharacterSize::Size7,
            CHSIZE_A::_8BIT => CharacterSize::Size8,
            _ => unimplemented!()
        }
    }
}

/// Configuration struct for [`Serial`](super::Serial) providing all
/// communication-related / parameters. [`Serial`](super::Serial) always uses eight data
/// bits plus the parity bit - if selected.
///
/// Create a configuration by using `default` in combination with the
/// builder methods. The following snippet shows creating a configuration
/// for 19,200 Baud, 8N1 by deriving it from the default value:
/// ```
/// # use crate::serial::config::*;
/// # use crate::time::Bps;
/// let config = Config::default().baudrate(19_200.bps());
///
/// assert!(config.baudrate == 19_200.bps());
/// assert!(config.parity == Parity::None);
/// assert!(config.stopbits == StopBits::STOP1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Serial interface baud rate
    pub baudrate: Bps,
    /// The number of data bits in a frame
    pub character_size: CharacterSize,
    /// Whether and how to generate/check a parity bit
    pub parity: Parity,
    /// The number of stop bits to follow the last data bit or the parity bit
    pub stopbits: StopBits,
}

impl Config {
    /// Sets the given baudrate.
    pub fn baudrate(mut self, baudrate: Bps) -> Self {
        self.baudrate = baudrate;
        self
    }

    /// Sets the given character size.
    pub fn character_size(mut self, character_size: CharacterSize) -> Self {
        self.character_size = character_size;
        self
    }

    /// Sets the given parity.
    pub fn parity(mut self, parity: Parity) -> Self {
        self.parity = parity;
        self
    }

    /// Sets the stop bits to `stopbits`.
    pub fn stopbits(mut self, stopbits: StopBits) -> Self {
        self.stopbits = stopbits;
        self
    }
}

impl Default for Config {
    /// Creates a new configuration with typically used parameters: 115,200
    /// Baud 8N1.
    fn default() -> Config {
        Config {
            baudrate: 115_200u32.bps(),
            character_size: CharacterSize::Size8,
            parity: Parity::None,
            stopbits: StopBits::Stop1,
        }
    }
}

impl From<Bps> for Config {
    fn from(b: Bps) -> Config {
        Config {
            baudrate: b,
            ..Default::default()
        }
    }
}
