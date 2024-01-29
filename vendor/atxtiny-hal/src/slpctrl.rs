//! # Sleep Controller

use core::arch::asm;

use crate::pac::{slpctrl, SLPCTRL};

/// Extension trait that constrains the [`SLPCTRL`] peripheral
pub trait SlpctrlExt {
    /// Constrains the [`SLPCTRL`] peripheral.
    ///
    /// Consumes the [`pac::SLPCTRL`] peripheral and converts it to a [`HAL`] internal type
    /// constraining it's public access surface to fit the design of the `HAL`.
    ///
    /// [`pac::SLPCTRL`]: `crate::pac::SLPCTRL`
    /// [`HAL`]: `crate`
    fn constrain(self) -> Slpctrl;
}

/// Constrained Slpctrl peripheral
///
/// An instance of this struct is acquired by calling the [`constrain`](SlpctrlExt::constrain) function
/// on the [`SLPCTRL`] struct.
///
/// ```
/// let dp = pac::Peripherals::take().unwrap();
/// let slpctrl = dp.SLPCTRL.constrain();
/// ```
pub struct Slpctrl;

impl SlpctrlExt for SLPCTRL {
    fn constrain(self) -> Slpctrl {
        Slpctrl
    }
}

impl Slpctrl {
    /// Set the desired [sleep mode](SleepMode)
    pub fn set_sleep_mode(&mut self, mode: SleepMode) {
        let ctrla = unsafe { &(*SLPCTRL::ptr()).ctrla() };
        ctrla.modify(|_, w| w.smode().variant(mode.into()));
    }

    /// Enter the [previously configured](Slpctrl::set_sleep_mode) sleep mode
    /// This function sets the sleep-enable bit, performs the sleep and clears
    /// the enable bit once the CPU woke up again and yielded control back to
    /// the non-interrupt context.
    pub fn sleep(&mut self) {
        let ctrla = unsafe { &(*SLPCTRL::ptr()).ctrla() };
        ctrla.modify(|_, w| w.sen().set_bit());
        unsafe { asm!("sleep") };
        ctrla.modify(|_, w| w.sen().clear_bit());
    }
}

/// The desired sleep mode that is to be entered when calling
/// [`sleep`](Slpctrl::sleep)
pub enum SleepMode {
    Idle,
    Standby,
    PowerDown,
}

impl From<SleepMode> for slpctrl::ctrla::SMODE_A {
    fn from(value: SleepMode) -> Self {
        match value {
            SleepMode::Idle => slpctrl::ctrla::SMODE_A::IDLE,
            SleepMode::Standby => slpctrl::ctrla::SMODE_A::STANDBY,
            SleepMode::PowerDown => slpctrl::ctrla::SMODE_A::PDOWN,
        }
    }
}

impl From<slpctrl::ctrla::SMODE_A> for SleepMode {
    fn from(value: slpctrl::ctrla::SMODE_A) -> Self {
        match value {
            slpctrl::ctrla::SMODE_A::IDLE => SleepMode::Idle,
            slpctrl::ctrla::SMODE_A::STANDBY => SleepMode::Standby,
            slpctrl::ctrla::SMODE_A::PDOWN => SleepMode::PowerDown,
        }
    }
}
