//! # Watchdog

use core::fmt;
use crate::pac::{WDT, wdt::ctrla::{PERIOD_A, WINDOW_A}};

use avr_device::ccp::ProtectedWritable;

/// The timeout how long it should take for the watchdog take to expire when
/// it's not fed by calling [`feed`]
/// 
/// [`feed`]: `crate::hal::watchdog::Watchdog::feed`
#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchdogTimeout {
    Disabled,
    Ms8,
    Ms16,
    Ms31,
    Ms63,
    Ms125,
    Ms250,
    Ms500,
    S1,
    S2,
    S4,
    S8,
}

impl Into<PERIOD_A> for WatchdogTimeout {
    fn into(self) -> PERIOD_A {
        match self {
            Self::Disabled => PERIOD_A::OFF,
            Self::Ms8 => PERIOD_A::_8CLK,
            Self::Ms16 => PERIOD_A::_16CLK,
            Self::Ms31 => PERIOD_A::_32CLK,
            Self::Ms63 => PERIOD_A::_64CLK,
            Self::Ms125 => PERIOD_A::_128CLK,
            Self::Ms250 => PERIOD_A::_256CLK,
            Self::Ms500 => PERIOD_A::_512CLK,
            Self::S1 => PERIOD_A::_1KCLK,
            Self::S2 => PERIOD_A::_2KCLK,
            Self::S4 => PERIOD_A::_4KCLK,
            Self::S8 => PERIOD_A::_8KCLK,
        }
    }
}

impl Into<WINDOW_A> for WatchdogTimeout {
    fn into(self) -> WINDOW_A {
        match self {
            Self::Disabled => WINDOW_A::OFF,
            Self::Ms8 => WINDOW_A::_8CLK,
            Self::Ms16 => WINDOW_A::_16CLK,
            Self::Ms31 => WINDOW_A::_32CLK,
            Self::Ms63 => WINDOW_A::_64CLK,
            Self::Ms125 => WINDOW_A::_128CLK,
            Self::Ms250 => WINDOW_A::_256CLK,
            Self::Ms500 => WINDOW_A::_512CLK,
            Self::S1 => WINDOW_A::_1KCLK,
            Self::S2 => WINDOW_A::_2KCLK,
            Self::S4 => WINDOW_A::_4KCLK,
            Self::S8 => WINDOW_A::_8KCLK,
        }
    }
}

/// Extension trait that constrains the [`WDT`] peripheral
pub trait WdtExt: crate::private::Sealed {
    /// Constrains the [`WDT`] peripheral.
    ///
    /// Consumes the [`pac::WDT`] peripheral and converts it to a [`HAL`] internal type
    /// constraining it's public access surface to fit the design of the `HAL`.
    ///
    /// [`pac::WDT`]: `crate::pac::WDT`
    /// [`HAL`]: `crate`
    fn constrain(self) -> WatchdogTimer;
}

impl crate::private::Sealed for WDT {}

impl WdtExt for WDT {
    fn constrain(self) -> WatchdogTimer {
        WatchdogTimer { wdt: self }
    }
}

/// Constrained Watchdog peripheral
///
/// An instance of this struct is acquired by calling the [`constrain`](WdtExt::constrain) function
/// on the [`WDT`] struct.
///
/// ```
/// let dp = pac::Peripherals::take().unwrap();
/// let watchdog = dp.WDT.constrain();
/// ```
pub struct WatchdogTimer {
    wdt: WDT,
}

impl fmt::Debug for WatchdogTimer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WatchdogTimer")
            .field("wdt", &"WDT")
            .finish()
    }
}

impl WatchdogTimer {
    /// Write the timeout and window values into the CTRLA register
    fn setup(&self, timeout: WatchdogTimeout, window: Option<WatchdogTimeout>) {
        let window = window.unwrap_or(WatchdogTimeout::Disabled);

        self.wdt.ctrla().write_protected(|w| { w
            .period().variant(timeout.into())
            .window().variant(window.into())
        });
    }

    /// Lock the watchdog peripheral.
    /// 
    /// Once this function has been called, it cannot be reconfigured anymore
    pub fn lock(&self) {
        self.wdt.status().write_protected(|w| { w
            .lock().set_bit()
        });
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
    pub unsafe fn peripheral(&mut self) -> &mut WDT {
        &mut self.wdt
    }

    /// Start the watchdog with the supplied timeout period
    ///
    /// NOTE: This was an Embedded-HAL trait method once which was removed and
    /// will be added back at a later time
    pub fn start(&mut self, period: WatchdogTimeout) {
        self.setup(period, None);
    }

    /// Feed the watchdog and prevent it from expiring
    ///
    /// NOTE: This was an Embedded-HAL trait method once which was removed and
    /// will be added back at a later time
    #[inline(always)]
    pub fn feed(&mut self) {
        avr_device::asm::wdr()
    }
}
