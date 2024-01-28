//! # CPU Interrupt Controller

use enumset::{EnumSet, EnumSetType};

use crate::pac::{CPUINT, cpuint::*};
use avr_device::ccp::ProtectedWritable;

/// Status Flags.
/// 
/// Depending on what kind of interrupts fired, one or more of these flags are set in
/// the interrupt controller.
#[derive(ufmt::derive::uDebug, Debug)]
#[cfg_attr(feature = "enumset", derive(EnumSetType))]
#[cfg_attr(not(feature = "enumset"), derive(Copy, Clone, PartialEq, Eq))]
pub enum InterruptControllerStatus {
    /// Non-Maskable Interrupt Executing Flag
    ///
    /// This flag is set if a non-maskable interrupt is executing.
    /// The flag is cleared when returning (RETI) from the interrupt handler.
    #[doc(alias = "NMIEX")]
    NMI,

    /// Level 1 Interrupt Executing Flag
    ///
    /// This flag is set when a priority level 1 interrupt is executing,
    /// or when the interrupt handler has been interrupted by an NMI.
    /// The flag is cleared when returning (RETI) from the interrupt handler.
    #[doc(alias = "LVL1EX")]
    LVL1,

    /// Level 0 Interrupt Executing Flag
    ///
    /// This flag is set when a priority level 0 interrupt is executing,
    /// or when the interrupt handler has been interrupted by a priority level 1
    /// interrupt or an NMI.
    /// The flag is cleared when returning (RETI) from the interrupt handler.
    #[doc(alias = "LVL0EX")]
    LVL0,
}

impl crate::private::Sealed for CPUINT {}

pub trait CPUINTExt: crate::private::Sealed {
    /// Constrains the [`CPUINT`] peripheral.
    ///
    /// Consumes the [`pac::CPUINT`] peripheral and converts it to a [`HAL`] internal type
    /// constraining it's public access surface to fit the design of the `HAL`.
    ///
    /// [`pac::CPUINT`]: `crate::pac::CPUINT`
    /// [`HAL`]: `crate`
    fn constrain(self) -> CpuInt;
}

#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptVectorSelect {
    AfterBootSection,
    StartOfBootSection,
}

fn into_ivsel(ivsel: InterruptVectorSelect) -> ctrla::IVSEL_A {
    match ivsel {
        InterruptVectorSelect::AfterBootSection => ctrla::IVSEL_A::AFTERBOOT,
        InterruptVectorSelect::StartOfBootSection => ctrla::IVSEL_A::INBOOT,
    }
}

fn into_cvt(cvt: bool) -> ctrla::CVT_A {
    match cvt {
        false => ctrla::CVT_A::NORMAL,
        true => ctrla::CVT_A::COMPACT,
    }
}

fn into_lvl0rr(lvl0rr: bool) -> ctrla::LVL0RR_A {
    match lvl0rr {
        false => ctrla::LVL0RR_A::FIXED,
        true => ctrla::LVL0RR_A::ROUNDROBIN,
    }
}

impl CPUINTExt for CPUINT {
    fn constrain(self) -> CpuInt {
        CpuInt {
            cpuint: self,
            ivsel: InterruptVectorSelect::AfterBootSection,
            cvt: false,
            lvl0rr:false
        }
    }
}

pub struct CpuInt {
    cpuint: CPUINT,
    ivsel: InterruptVectorSelect,
    cvt: bool,
    lvl0rr: bool,
}

pub struct CpuIntConfigured {
    cpuint: CPUINT
}

impl CpuInt {
    pub fn interrupt_vector_select(mut self, ivsel: InterruptVectorSelect) -> Self {
        self.ivsel = ivsel;
        self
    }

    pub fn compact_vector_table(mut self, cvt: bool) -> Self {
        self.cvt = cvt;
        self
    }

    pub fn lvl0_round_robin(mut self, lvl0rr: bool) -> Self {
        self.lvl0rr = lvl0rr;
        self
    }

    pub fn configure(self) -> CpuIntConfigured {
        self.cpuint.ctrla().write_protected(|w| { w
            .ivsel().variant(into_ivsel(self.ivsel))
            .cvt().variant(into_cvt(self.cvt))
            .lvl0rr().variant(into_lvl0rr(self.lvl0rr))
        });

        self.cpuint.lvl0pri().write(|w| w.bits(0));
        self.cpuint.lvl1vec().write(|w| w.bits(0));

        CpuIntConfigured { cpuint: self.cpuint }
    }
}

impl CpuIntConfigured {
    #[inline]
    pub fn get_lvl0_priority(&self) -> u8 {
        self.cpuint.lvl0pri().read().bits()
    }

    #[inline]
    pub fn set_lvl0_priority(&mut self, level: u8) {
        self.cpuint.lvl0pri().write(|w| w.bits(level))
    }

    #[inline]
    pub fn get_lvl1_vector(&self) -> u8 {
        self.cpuint.lvl1vec().read().bits()
    }

    #[inline]
    pub fn set_lvl1_vector(&mut self, vector: u8) {
        self.cpuint.lvl1vec().write(|w| w.bits(vector))
    }

    /// Check for a status.
    #[inline]
    pub fn is_status(&self, status: InterruptControllerStatus) -> bool {
        let status_reg = self.cpuint.status().read();
        match status {
            InterruptControllerStatus::NMI => status_reg.nmiex().bit_is_set(),
            InterruptControllerStatus::LVL1 => status_reg.lvl1ex().bit_is_set(),
            InterruptControllerStatus::LVL0 => status_reg.lvl0ex().bit_is_set(),
        }
    }

    /// Get all status flags.
    #[cfg(feature = "enumset")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enumset")))]
    #[inline]
    pub fn status(&mut self) -> EnumSet<InterruptControllerStatus> {
        let mut status_set = EnumSet::new();

        for status in EnumSet::<InterruptControllerStatus>::all().iter() {
            if self.is_status(status) {
                status_set |= status;
            }
        }

        status_set
    }
}
