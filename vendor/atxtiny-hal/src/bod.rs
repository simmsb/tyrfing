//! # Brownout Detector

use crate::{
    Toggle,
    pac::{BOD, bod}
};

/// Sampling frequency
/// 
/// The configured sampling frequency is loaded from fusebits on reset.
#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamplingFrequency {
    _1KHz,
    _125KHz,
}

/// The brownout detector mode
#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// The brownout detector is disabled
    Disabled,

    /// The brownout detector is enabled continously
    Enabled,

    /// The brownout detector is enabled but samples the voltage at
    /// regular intervals as defined by [`SamplingFrequency`]
    Sampled,

    /// The brownout detector is continously enabled during Active mode and
    /// disabled in sleep modes. When a wake-up event occurs, the wake-up is
    /// halted until the the brownout detector signals that the power is good
    EnabledAndWakeupHaltedTillBODReady
}

impl From<Mode> for bod::ctrla::ACTIVE_A {
    fn from(value: Mode) -> Self {
        use bod::ctrla::ACTIVE_A::*;
        match value {
            Mode::Disabled => DIS,
            Mode::Enabled => ENABLED,
            Mode::Sampled => SAMPLED,
            Mode::EnabledAndWakeupHaltedTillBODReady => ENWAKE,
        }
    }
}

impl From<bod::ctrla::ACTIVE_A> for Mode {
    fn from(value: bod::ctrla::ACTIVE_A) -> Self {
        use bod::ctrla::ACTIVE_A::*;
        match value {
            DIS => Mode::Disabled,
            ENABLED => Mode::Enabled,
            SAMPLED => Mode::Sampled,
            ENWAKE => Mode::EnabledAndWakeupHaltedTillBODReady,
        }
    }
}

impl From<Mode> for bod::ctrla::SLEEP_A {
    fn from(value: Mode) -> Self {
        use bod::ctrla::SLEEP_A::*;
        match value {
            Mode::Disabled => DIS,
            Mode::Enabled => ENABLED,
            Mode::Sampled => SAMPLED,
            _ => unreachable!(),
        }
    }
}

impl From<bod::ctrla::SLEEP_A> for Mode {
    fn from(value: bod::ctrla::SLEEP_A) -> Self {
        use bod::ctrla::SLEEP_A::*;
        match value {
            DIS => Mode::Disabled,
            ENABLED => Mode::Enabled,
            SAMPLED => Mode::Sampled,
        }
    }
}

/// The brownout detector level
/// 
/// The configured level is loaded from fusebits on reset.
#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    /// 1.9V
    Level190V,

    /// 2.45V
    Level245V,

    /// 2.7V
    Level270V,

    /// 2.85V
    Level285V,

    /// 3.3V
    Level330V,

    /// 3.7V
    Level370V,

    /// 4V
    Level400V,

    /// 4.2V
    Level420V,
}

impl From<bod::ctrlb::LVL_A> for Level {
    fn from(value: bod::ctrlb::LVL_A) -> Self {
        use bod::ctrlb::LVL_A::*;
        match value {
            BODLEVEL0 => Level::Level190V,
            BODLEVEL1 => Level::Level245V,
            BODLEVEL2 => Level::Level270V,
            BODLEVEL3 => Level::Level285V,
            // BODLEVEL4 => Level::Level330V,
            // BODLEVEL5 => Level::Level370V,
            // BODLEVEL6 => Level::Level400V,
            // L7 => Level::Level420V,
        }
    }
}

/// The voltage level monitor threshold relative to the BOD threshold
#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoltageLevelThreshold {
    /// VLM threshold 5% above BOD threshold
    FivePercentAbove,

    /// VLM threshold 15% above BOD threshold
    FifteenPercentAbove,

    /// VLM threshold 25% above BOD threshold
    TwentyfivePercentAbove,
    OFF,
}

impl From<VoltageLevelThreshold> for bod::vlmctrla::VLMLVL_A {
    fn from(value: VoltageLevelThreshold) -> Self {
        use bod::vlmctrla::VLMLVL_A::*;
        match value {
            VoltageLevelThreshold::FivePercentAbove => _5ABOVE,
            VoltageLevelThreshold::FifteenPercentAbove => _15ABOVE,
            VoltageLevelThreshold::TwentyfivePercentAbove => _25ABOVE,
            VoltageLevelThreshold::OFF => OFF,
        }
    }
}

impl From<bod::vlmctrla::VLMLVL_A> for VoltageLevelThreshold {
    fn from(value: bod::vlmctrla::VLMLVL_A) -> Self {
        use bod::vlmctrla::VLMLVL_A::*;
        match value {
            _5ABOVE => VoltageLevelThreshold::FivePercentAbove,
            _15ABOVE => VoltageLevelThreshold::FifteenPercentAbove,
            _25ABOVE => VoltageLevelThreshold::TwentyfivePercentAbove,
            OFF => VoltageLevelThreshold::OFF,
        }
    }
}

/// The VLM (voltage level monitor) configuration
#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, PartialEq, Eq)]
pub enum VlmConfiguration {
    /// Voltage falls below the VDD threshold
    VoltageFallsBelowThreshold,

    /// Voltage rises above the VDD threshold
    VoltageRisesAboveThreshold,

    /// Voltage crosses the VDD threshold from either direction
    Cross,
}

impl From<VlmConfiguration> for bod::intctrl::VLMCFG_A {
    fn from(value: VlmConfiguration) -> Self {
        use bod::intctrl::VLMCFG_A::*;
        match value {
            VlmConfiguration::VoltageRisesAboveThreshold => RISING,
            VlmConfiguration::VoltageFallsBelowThreshold => FALLING,
            VlmConfiguration::Cross => BOTH,
        }
    }
}

impl From<bod::intctrl::VLMCFG_A> for VlmConfiguration {
    fn from(value: bod::intctrl::VLMCFG_A) -> Self {
        use bod::intctrl::VLMCFG_A::*;
        match value {
            RISING => VlmConfiguration::VoltageRisesAboveThreshold,
            FALLING => VlmConfiguration::VoltageFallsBelowThreshold,
            BOTH => VlmConfiguration::Cross,
        }
    }
}

/// Extension trait that constrains the [`BOD`] peripheral
pub trait BodExt {
    /// Constrains the [`BOD`] peripheral into a configurator.
    ///
    /// Consumes the [`pac::BOD`] peripheral and converts it to a [`HAL`] internal type
    /// constraining it's public access surface to fit the design of the `HAL`.
    /// Using the [`configurator`], the peripheral can be initially configured with
    /// a builder pattern. Afterwards the settings can be changed using the
    /// provided methods.
    ///
    /// [`pac::BOD`]: `crate::pac::BOD`
    /// [`HAL`]: `crate`
    /// [`configurator`]: `BrownoutDetectorConfigurator`
    fn constrain(self) -> BrownoutDetectorConfigurator;
}

/// Constrained BOD peripheral configurator
///
/// An instance of this struct is acquired by calling the [`constrain`](BodExt::constrain) function
/// on the [`BOD`] struct.
///
/// ```
/// let dp = pac::Peripherals::take().unwrap();
/// let bod_cfg = dp.BOD.constrain();
/// ```
pub struct BrownoutDetectorConfigurator {
    bod: BOD,
    sleep_mode: Option<Mode>,
    vlm_level: VoltageLevelThreshold,
    vlm_mode: VlmConfiguration,
    vlm_int: bool,
}

impl BodExt for BOD {
    fn constrain(self) -> BrownoutDetectorConfigurator {
        BrownoutDetectorConfigurator {
            bod: self,
            sleep_mode: None,
            vlm_level: VoltageLevelThreshold::FivePercentAbove,
            vlm_mode: VlmConfiguration::VoltageFallsBelowThreshold,
            vlm_int: false,
        }
    }
}

/// Configured BOD peripheral
///
/// An instance of this struct is acquired by calling the [`constrain`](BodExt::constrain) function
/// on the [`BOD`] struct and then [finishing the configuration](BrownoutDetectorConfigurator::configure)
/// on the constrained peripheral.
///
/// ```
/// let dp = pac::Peripherals::take().unwrap();
/// let bod_cfg = dp.BOD.constrain();
/// let bod = bod_cfg.configure();
/// ```
pub struct BrownoutDetector {
    bod: BOD,
}

impl BrownoutDetectorConfigurator {
    /// Set the brownout detection mode when the CPU is in a sleep state
    pub fn sleep_mode(mut self, sleep_mode: Mode) -> Self {
        self.sleep_mode = Some(sleep_mode);
        self
    }

    /// Configure the voltage level monitor
    pub fn voltage_level_monitor(mut self, level: VoltageLevelThreshold, mode: VlmConfiguration, int: bool) -> Self {
        self.vlm_level = level;
        self.vlm_mode = mode;
        self.vlm_int = int;
        self
    }

    /// Apply the configuration and return a configured [`BrownoutDetector`]
    pub fn configure(self) -> BrownoutDetector {
        let mut bod = BrownoutDetector { bod: self.bod };

        if let Some(sleep_mode) = self.sleep_mode {
            bod.set_sleep_mode(sleep_mode);
        }

        bod.set_voltage_monitor_threshold(self.vlm_level);
        bod.configure_interrupt(self.vlm_int, self.vlm_mode);

        bod
    }
}

impl BrownoutDetector {
    /// Get the configured sampling frequency for the brownout detection
    /// 
    /// This setting is loaded from fusebits during reset and can not be changed
    /// during runtime
    pub fn get_sampling_frequency(&self) -> SamplingFrequency {
        if self.bod.ctrla().read().sampfreq().bit_is_set() {
            SamplingFrequency::_125KHz
        } else {
            SamplingFrequency::_1KHz
        }
    }

    /// Get the configured active brownout detection mode
    /// 
    /// This mode is loaded from fusebits during reset and can not be changed
    /// during runtime
    #[inline]
    pub fn get_active_mode(&self) -> Mode {
        self.bod.ctrla().read().active().variant().into()
    }

    /// Get the configured sleep brownout detection mode
    #[inline]
    pub fn get_sleep_mode(&self) -> Mode {
        self.bod.ctrla().read().sleep().variant().unwrap().into()
    }

    /// Set the configured sleep brownout detection mode
    #[inline]
    pub fn set_sleep_mode(&mut self, mode: Mode) {
        self.bod.ctrla().modify(|_, w| w.sleep().variant(mode.into()));
    }

    /// Get the configured sleep brownout detection mode
    /// 
    /// This setting is loaded from fusebits during reset and can not be changed
    /// during runtime
    #[inline]
    pub fn get_brownout_detection_level(&self) -> Level {
        self.bod.ctrlb().read().lvl().variant().unwrap().into()
    }

    /// Set the current monitor threshold for the voltage level monitor.
    #[inline]
    pub fn set_voltage_monitor_threshold(&mut self, level: VoltageLevelThreshold) {
        self.bod.vlmctrla().modify(|_, w| w.vlmlvl().variant(level.into()));
    }

    /// Get the current monitor threshold for the voltage level monitor.
    #[inline]
    pub fn get_voltage_monitor_threshold(&self) -> VoltageLevelThreshold {
        self.bod.vlmctrla().read().vlmlvl().variant().into()
    }

    /// Enable or disable the voltage level monitor interrupt.
    /// 
    /// The passed [`VlmConfiguration`] configures when an interrupt is triggered.
    #[inline]
    pub fn configure_interrupt(&mut self, enable: impl Into<Toggle>, config: VlmConfiguration) {
        let enable: Toggle = enable.into();
        let enable: bool = enable.into();

        self.bod.intctrl().modify(|_, w| w
            .vlmcfg().variant(config.into())
            .vlmie().bit(enable)
        );
    }

    /// Enable the voltage level monitor interrupt.
    #[inline]
    pub fn enable_interrupt(&mut self) {
        self.bod.intctrl().modify(|_, w| w
            .vlmie().set_bit()
        );
    }

    /// Disable the voltage level monitor interrupt.
    #[inline]
    pub fn disable_interrupt(&mut self) {
        self.bod.intctrl().modify(|_, w| w
            .vlmie().clear_bit()
        );
    }

    /// Check if the voltage level monitoring interrupt event happend.
    #[inline]
    pub fn is_event_triggered(&self) -> bool {
        self.bod.intflags().read().vlmif().bit_is_set()
    }

    /// Clear the voltage level monitoring interrupt event.
    #[inline]
    pub fn clear_event(&mut self) {
        self.bod.intflags().modify(|_, w| w.vlmif().set_bit());
    }
}
