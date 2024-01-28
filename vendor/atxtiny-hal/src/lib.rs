#![no_std]
#![feature(asm_const)]
#![feature(asm_experimental_arch)]
#![feature(panic_info_message)]
#![feature(associated_type_defaults)]
#![feature(associated_const_equality)]

// #![deny(rustdoc::broken_intra_doc_links)]

pub use embedded_hal;
pub use embedded_io;
pub use embedded_hal_bus;

mod private {
    /// Private sealed trait to seal all GPIO implementations
    /// which do implement peripheral functionalities.
    pub trait Sealed {}
}

pub mod time;
pub mod prelude;
pub mod panic_serial;

pub use avr_device;

#[cfg(feature = "attiny817")]
pub use avr_device::attiny817 as pac;
#[cfg(feature = "attiny1616")]
pub use avr_device::attiny1616 as pac;

pub mod clkctrl;
pub mod gpio;
pub mod portmux;
pub mod serial;
pub mod twi;
pub mod spi;
pub mod watchdog;
pub mod nvmctrl;
pub mod ccl;
pub mod slpctrl;
pub mod rstctrl;
pub mod bod;
pub mod vref;
pub mod dac;
pub mod ac;
pub mod timer;
pub mod cpuint;
pub mod syscfg;
pub mod evsys;
pub mod evout;

/// Toggle something on or off.
///
/// Convenience enum and wrapper around a bool, which more explicit about the intention to enable
/// or disable something, in comparison to `true` or `false`.
#[derive(ufmt::derive::uDebug, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Toggle {
    /// Toggle something on / enable a thing.
    On,
    /// Toggle something off / disable a thing.
    Off,
}

impl From<Toggle> for bool {
    fn from(toggle: Toggle) -> Self {
        matches!(toggle, Toggle::On)
    }
}

impl From<bool> for Toggle {
    fn from(b: bool) -> Self {
        match b {
            true => Toggle::On,
            false => Toggle::Off,
        }
    }
}
