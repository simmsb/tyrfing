//! # Reset Controller

use enumset::{EnumSet, EnumSetType};
use crate::pac::RSTCTRL;

/// Reset Flags.
/// 
/// Depending on how the system was reset, one or more of these flags are set in
/// the reset controller.
#[derive(ufmt::derive::uDebug, Debug)]
#[cfg_attr(feature = "enumset", derive(EnumSetType))]
#[cfg_attr(not(feature = "enumset"), derive(Copy, Clone, PartialEq, Eq))]
pub enum ResetReason {
    /// UPDI Reset Flag
    ///
    /// This flag is set when the system was reset through the UPDI
    /// debug infrastructure.
    #[doc(alias = "UPDIRF")]
    UPDI,

    /// Software Reset Flag
    ///
    /// This flag is set when the system was reset through a software reset.
    /// A software reset can be executed by calling `[Rstctrl::software_reset]`.
    #[doc(alias = "SWRF")]
    Software,

    /// Watchdog Reset Flag
    ///
    /// This flag is set when the system was reset through by an expired.
    /// watchdog timer. The watchdog timer can be configured using the `[WatchdogTimer]`
    /// peripheral.
    #[doc(alias = "WDRF")]
    Watchdog,

    /// External Reset Flag
    ///
    /// This flag is set when the system was reset using the external reset pin.
    #[doc(alias = "EXTRF")]
    External,

    /// Brownout Reset Flag
    /// 
    /// This flag is set when the system was reset by the Brownount detector.
    /// The brownout detector can be configured using the `[BrownoutDetector]`
    /// peripheral.
    #[doc(alias = "BORF")]
    Brownout,

    /// Power-On Reset Flag
    ///
    /// This flag is set when the system powered up initially.
    #[doc(alias = "PORF")]
    PowerOn,
}

/// Extension trait that constrains the [`RSTCTRL`] peripheral
pub trait RstctrlExt {
    /// Constrains the [`RSTCTRL`] peripheral.
    ///
    /// Consumes the [`pac::RSTCTRL`] peripheral and converts it to a [`HAL`] internal type
    /// constraining it's public access surface to fit the design of the `HAL`.
    ///
    /// [`pac::RSTCTRL`]: `crate::pac::RSTCTRL`
    /// [`HAL`]: `crate`
    fn constrain(self) -> Rstctrl;
}

/// Constrained Rstctrl peripheral
///
/// An instance of this struct is acquired by calling the [`constrain`](RstctrlExt::constrain) function
/// on the [`RSTCTRL`] struct.
///
/// ```
/// let dp = pac::Peripherals::take().unwrap();
/// let rstctrl = dp.RSTCTRL.constrain();
/// ```
pub struct Rstctrl {
    rstctrl: RSTCTRL
}

impl RstctrlExt for RSTCTRL {
    fn constrain(self) -> Rstctrl {
        Rstctrl { rstctrl: self }
    }
}

impl Rstctrl {
    /// Perform a software reset of the system
    #[inline]
    pub fn software_reset(&mut self) {
        self.rstctrl.swrr().modify(|_, w| w.swrst().set_bit());
    }

    /// Check for a reset reason.
    #[inline]
    pub fn is_reset_reason(&self, reset: ResetReason) -> bool {
        let rstfr = self.rstctrl.rstfr().read();
        match reset {
            ResetReason::UPDI => rstfr.updirf().bit_is_set(),
            ResetReason::Software => rstfr.swrf().bit_is_set(),
            ResetReason::Watchdog => rstfr.wdrf().bit_is_set(),
            ResetReason::External => rstfr.extrf().bit_is_set(),
            ResetReason::Brownout => rstfr.borf().bit_is_set(),
            ResetReason::PowerOn => rstfr.porf().bit_is_set(),
        }
    }

    /// Get all reset reasons.
    #[cfg(feature = "enumset")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enumset")))]
    #[inline]
    pub fn reset_reasons(&mut self) -> EnumSet<ResetReason> {
        let mut reasons = EnumSet::new();

        for reason in EnumSet::<ResetReason>::all().iter() {
            if self.is_reset_reason(reason) {
                reasons |= reason;
            }
        }

        reasons
    }

    /// Clear the given reset reason in the flag register.
    #[inline]
    pub fn clear_reason(&mut self, reason: ResetReason) {
        self.rstctrl.rstfr().write(|w| match reason {
            ResetReason::UPDI => w.updirf().set_bit(),
            ResetReason::Software => w.swrf().set_bit(),
            ResetReason::Watchdog => w.wdrf().set_bit(),
            ResetReason::External => w.extrf().set_bit(),
            ResetReason::Brownout => w.borf().set_bit(),
            ResetReason::PowerOn => w.porf().set_bit(),
        });
    }

    /// Clear **all** reset flags.
    #[inline]
    pub fn clear_reasons(&mut self) {
        // SAFETY: This atomic write clears all flags and ignores the reserverd bit fields.
        self.rstctrl.rstfr().write(|w| unsafe { w.bits(u8::MAX) });
    }
}
