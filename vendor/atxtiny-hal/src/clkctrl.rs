//! # Clock Controller

use crate::pac::{
    clkctrl::{mclkctrla, mclkctrlb},
    CLKCTRL
};
use crate::time::*;

use avr_device::ccp::ProtectedWritable;

impl crate::private::Sealed for CLKCTRL {}

// FIXME: stop using from_raw now with fugit?
// FIXME: allow to configure 32khz clock either from internal source, external crystal or external clock
// TODO: allow config of RUNSTDBY in different oscillators

pub trait CLKCTRLExt: crate::private::Sealed {
    /// Constrains the [`CLKCTRL`] peripheral.
    ///
    /// Consumes the [`pac::CLKCTRL`] peripheral and converts it to a [`HAL`] internal type
    /// constraining it's public access surface to fit the design of the `HAL`.
    ///
    /// [`pac::CLKCTRL`]: `crate::pac::CLKCTRL`
    /// [`HAL`]: `crate`
    fn constrain(self) -> ClkCtrl;
}

impl CLKCTRLExt for CLKCTRL {
    fn constrain(self) -> ClkCtrl {
        ClkCtrl::default()
    }
}

/// The possible main clock sources for the chip
/// 
/// This clock source gets divided down by the clock controller and is passed
/// to further blocks like memory, CPU, peripherals etc.
#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainClkSrc {
    Osc20M,
    OscUlp32K,
    XOsc32K,
    ExtClk,
}

fn into_clksel(src: MainClkSrc) -> mclkctrla::CLKSEL_A {
    match src {
        MainClkSrc::Osc20M => mclkctrla::CLKSEL_A::OSC20M,
        MainClkSrc::OscUlp32K => mclkctrla::CLKSEL_A::OSCULP32K,
        MainClkSrc::XOsc32K => mclkctrla::CLKSEL_A::XOSC32K,
        MainClkSrc::ExtClk => mclkctrla::CLKSEL_A::EXTCLK,
    }
}

fn into_pdiv(div: u32) -> Option<mclkctrlb::PDIV_A> {
    match div {
        2 => Some(mclkctrlb::PDIV_A::_2X),
        4 => Some(mclkctrlb::PDIV_A::_4X),
        6 => Some(mclkctrlb::PDIV_A::_6X),
        8 => Some(mclkctrlb::PDIV_A::_8X),
        10 => Some(mclkctrlb::PDIV_A::_10X),
        12 => Some(mclkctrlb::PDIV_A::_12X),
        16 => Some(mclkctrlb::PDIV_A::_16X),
        24 => Some(mclkctrlb::PDIV_A::_24X),
        32 => Some(mclkctrlb::PDIV_A::_32X),
        48 => Some(mclkctrlb::PDIV_A::_48X),
        64 => Some(mclkctrlb::PDIV_A::_64X),
        _ => None,
    }
}

/// Clock controller abstraction
/// 
/// This is an abstraction of the CLKCTRL peripheral used to configure the
/// system clock tree.
pub struct ClkCtrl {
    main_osc: u32,
    main_clk_src: MainClkSrc,
    enable_clkout: bool,
    per_clk: Option<u32>,
}

impl Default for ClkCtrl {
    fn default() -> Self {
        Self {
            main_osc:  Hertz::from_raw(20_000_000).raw(),
            main_clk_src: MainClkSrc::Osc20M,
            enable_clkout: false,
            per_clk: None,
        }
    }
}

impl ClkCtrl {
    /// Set the main oscillator frequency.
    /// 
    /// This is the frequency of the main oscillator which gets divided down
    /// into fractions.
    /// 
    /// The frequency depends on the selection of the main clock source by
    /// calling [`ClkCtrl::clk_src_main`].
    /// 
    /// This is usually 16 or 20MHz, depending on the `OSCCFG` fuse bit when
    /// the [`MainClkSrc::Osc20M`].
    /// For [`MainClkSrc::OscUlp32K`]/[`MainClkSrc::XOsc32K`] it's 32KHz.
    /// And for the [`MainClkSrc::ExtClk`] it is whatever the supplied external
    /// clock frequency is.
    pub fn main_osc_freq(mut self, freq: Hertz) -> Self {
        self.main_osc = freq.raw();
        self
    }

    /// Set the main clock source.
    ///
    /// The clock source can be the internal 16/20MHz oscillator, an external
    /// clock or internal/external 32kHz oscillators.
    /// 
    /// The appropriate frequency needs to be set using [`ClkCtrl::main_osc_freq`].
    pub fn clk_src_main(mut self, src: MainClkSrc) -> Self {
        self.main_clk_src = src;
        self
    }

    /// Enable the clock output pin.
    pub fn enable_clkout(mut self) -> Self {
        self.enable_clkout = true;
        self
    }

    /// Set the desired `PER_CLK`` peripheral clock.
    /// 
    /// This clock is divided down from the main clock. The prescaler also
    /// affects the CPU clock `CPU_CLK` which is also fed from the same divided
    /// main clock.
    /// 
    /// NOTE: not all combinations of main clocks and CPU/peripheral clocks are
    /// possible. The prescaler must be in a list of supported prescalers which
    /// can cause unsupported combinations.
    pub fn per_clk_freq(mut self, freq: Hertz) -> Self {
        // NOTE: I used to use Megahertz here and then convert it. This
        //       introduced about 3 Kilobytes of code doing multiplication and
        //       division. No thanks...
        //let freq: Hertz = freq.try_into().expect("ConversionError");
        self.per_clk = Some(freq.raw());
        self
    }

    // FIXME: return Error for impossible dividers and clock rates?
    /// Configure the clock controller as desired.
    /// 
    /// The returned [`Clocks`] struct contains the resulting clock frequencies.
    pub fn freeze(self) -> Clocks {
        assert!(self.main_osc <= 20_000_000);

        let clkctrl = unsafe { &*CLKCTRL::ptr() };
        let clksel = into_clksel(self.main_clk_src);

        // Wait for the selected clock to stabilize
        match clksel {
           mclkctrla::CLKSEL_A::EXTCLK => while clkctrl.mclkstatus().read().exts().bit_is_clear() {},
           mclkctrla::CLKSEL_A::OSC20M => while clkctrl.mclkstatus().read().osc20ms().bit_is_clear() {},
           mclkctrla::CLKSEL_A::OSCULP32K => while clkctrl.mclkstatus().read().osc32ks().bit_is_clear() {},
           mclkctrla::CLKSEL_A::XOSC32K => while clkctrl.mclkstatus().read().xosc32ks().bit_is_clear() {},
        };

        // Set main clock source
        clkctrl.mclkctrla().write_protected(|w| { w
            .clksel().variant(clksel)
            .clkout().bit(self.enable_clkout)
        });

        // Set per_clk divider
        let desired_per_clk = self.per_clk.unwrap_or(self.main_osc);
        assert!(desired_per_clk <= 20_000_000);
        let divider = self.main_osc / desired_per_clk;

        if divider > 1 {
            let pdiv = into_pdiv(divider).expect("Impossible clock divider");
            clkctrl.mclkctrlb().write_protected(|w| { w
                .pdiv().variant(pdiv)
                .pen().set_bit()
            });
        } else {
            clkctrl.mclkctrlb().write_protected(|w| { w
                .pen().clear_bit()
            });
        }

        // Wait for the clock change to the new source
        while clkctrl.mclkstatus().read().sosc().bit_is_set() {}

        Clocks {
            main: Hertz::from_raw(self.main_osc),
            per: Hertz::from_raw(self.main_osc / divider),
            main_prescaler: divider as u8,
            bod_wdt: (32768u32/1024).Hz()
        }
    }
}

/// The resulting clock frequencies that a configured [`ClkCtrl`] generates
#[derive(Debug, Clone, Copy)]
pub struct Clocks {
    main: Hertz,
    per: Hertz,
    main_prescaler: u8,
    bod_wdt: Hertz,
}

impl Clocks {
    /// Returns the frequency of the main oscillator
    pub fn main(&self) -> Hertz {
        self.main
    }

    /// Returns the clock frequency of the CPU and Peripherals
    pub fn per(&self) -> Hertz {
        self.per
    }

    /// Returns the frequency of the BOD and WDT oscillator derived from
    /// OSCULP32K which is divided by 32 in hardware
    pub fn bod_wdt(&self) -> Hertz {
        self.bod_wdt
    }

    /// Returns the main prescaler which divides CLK_MAIN down to CLK_PER
    pub fn main_prescaler(&self) -> u8 {
        self.main_prescaler
    }
}
