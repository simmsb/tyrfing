use core::fmt;

use crate::time::*;
use crate::embedded_hal::spi::{self, Mode};
use crate::pac::spi0::ctrla::DORD_A;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// SPI bus clock frequency
    pub frequency: Hertz,
    /// Operation Mode as defined by the [`embedded-hal`]
    pub mode: Mode,
    /// The data order of transmissions
    pub order: DataOrder,
}

#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataOrder {
    /// Transmit the most significant bit first
    MsbFirst,
    /// Transmit the least significant bit first
    LsbFirst,
}

impl From<DataOrder> for DORD_A {
    fn from(order: DataOrder) -> Self {
        match order {
            DataOrder::MsbFirst => DORD_A::MSB_FIRST,
            DataOrder::LsbFirst => DORD_A::LSB_FIRST,
        }
    }
}

impl From<DORD_A> for DataOrder {
    fn from(order: DORD_A) -> Self {
        match order {
            DORD_A::MSB_FIRST => DataOrder::MsbFirst,
            DORD_A::LSB_FIRST => DataOrder::LsbFirst,
        }
    }
}

impl Config {
    /// Set the operating frequency of the SPI
    pub fn frequency(mut self, frequency: impl Into<Hertz>) -> Self {
        self.frequency = frequency.into();
        self
    }

    /// Set the Operation Mode
    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    /// Set the data order
    pub fn data_order(mut self, order: DataOrder) -> Self {
        self.order = order;
        self
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mode = if self.mode == spi::MODE_0 {
            0
        } else if self.mode == spi::MODE_1 {
            1
        } else if self.mode == spi::MODE_2 {
            2
        } else {
            3
        };

        f.debug_struct("Config")
            .field("frequency", &format_args!("{:?}", self.frequency))
            .field("mode", &format_args!("MODE_{}", mode))
            .field("order", &format_args!("{:?}", self.order))
            .finish()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            frequency: 1_000_000.Hz(),
            mode: spi::MODE_0,
            order: DataOrder::MsbFirst,
        }
    }
}

impl<F: Into<Hertz>> From<F> for Config {
    fn from(f: F) -> Config {
        Config {
            frequency: f.into(),
            ..Default::default()
        }
    }
}
