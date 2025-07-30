//! # Voltage reference

// TODO: macros for different CPUs which have different peripherals
// FIXME: move this into the DAC and ADC modules? DAC and AC share the channel though

use core::hint::unreachable_unchecked;

use crate::{pac::VREF, Toggle};

/// Extension trait that constrains the [`VREF`] peripheral
pub trait VrefExt {
    /// Constrains the [`VREF`] peripheral.
    ///
    /// Consumes the [`pac::VREF`] peripheral and converts it to a [`HAL`] internal type
    /// constraining it's public access surface to fit the design of the `HAL`.
    ///
    /// [`pac::VREF`]: `crate::pac::VREF`
    /// [`HAL`]: `crate`
    fn constrain(self) -> Vref;
}

/// Constrained VREF peripheral
///
/// An instance of this struct is acquired by calling the [`constrain`](VrefExt::constrain) function
/// on the [`VREF`] struct.
///
/// ```
/// let dp = pac::Peripherals::take().unwrap();
/// let vref = dp.VREF.constrain();
/// ```
pub struct Vref {
    vref: VREF,
}

impl VrefExt for VREF {
    fn constrain(self) -> Vref {
        Vref { vref: self }
    }
}

/// Reference voltage for an ADC
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADCReferenceVoltage<const IDX: u8>;

/// Reference voltage for a DAC
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DACReferenceVoltage<const IDX: u8>;

impl<const IDX: u8> crate::private::Sealed for ADCReferenceVoltage<IDX> {}
impl<const IDX: u8> crate::private::Sealed for DACReferenceVoltage<IDX> {}

macro_rules! impl_reference_voltage {
    ($name:ident, $periphname:ident, $structret:ident, $refstruct:ty, $refvolttype:ty, $refselreg:ident, $refselbits:ident, $forceenreg:ident, $forceenbit:ident) => {
        impl Vref {
            #[doc = "Retrieve the reference voltage for the peripheral "]
            #[doc = stringify!($periphname)]
            pub fn $name(&mut self, voltage: $refvolttype) -> $refstruct {
                self.vref
                    .$refselreg()
                    .modify(|_, w| unsafe { w.$refselbits().bits(voltage as u8) });
                $structret
            }
        }

        #[doc = "The reference voltage for the peripheral "]
        #[doc = stringify!($periphname)]
        impl $refstruct {
            /// Set the reference voltage to the specified level.
            pub fn voltage(vref: &mut Vref, voltage: $refvolttype) {
                vref.vref
                    .$refselreg()
                    .modify(|_, w| unsafe { w.$refselbits().bits(voltage as u8) });
            }

            /// Force-enable the reference voltage.
            ///
            /// Usually the peripherals that use the reference voltage enable it
            /// automatically. Using this method it can be force-enabled.
            pub fn force(vref: &mut Vref, force: impl Into<Toggle>) {
                let force: Toggle = force.into();
                let force: bool = force.into();
                vref.vref
                    .$forceenreg()
                    .modify(|_, w| w.$forceenbit().bit(force));
            }
        }
    };
}

/// Reference Voltage.
#[derive(ufmt::derive::uDebug, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ReferenceVoltage {
    /// 1.024V
    _1V024 = 0x00,

    /// 2.048V
    _2V048 = 0x01,

    /// 4.096V
    _4V096 = 0x02,

    /// 2.5V
    _2V50 = 0x03,

    /// VDD
    VDD = 0x05,

    /// VRef
    VRef = 0x06,
}

impl ReferenceVoltage {
    pub const fn into_bits(self) -> u8 {
        self as _
    }

    pub const fn from_bits(value: u8) -> Self {
        match value {
            0x00 => Self::_1V024,
            0x01 => Self::_2V048,
            0x02 => Self::_4V096,
            0x03 => Self::_2V50,
            0x05 => Self::VDD,
            0x06 => Self::VRef,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl_reference_voltage!(
    adc0,
    ADC0,
    ADCReferenceVoltage,
    ADCReferenceVoltage<0>,
    ReferenceVoltage,
    adc0ref,
    refsel,
    adc0ref,
    alwayson
);

impl_reference_voltage!(
    dac0,
    DAC0,
    DACReferenceVoltage,
    DACReferenceVoltage<0>,
    ReferenceVoltage,
    dac0ref,
    refsel,
    dac0ref,
    alwayson
);
