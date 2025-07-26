#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    prr: PRR,
    clkpr: CLKPR,
    _reserved2: [u8; 0x07],
    dwdr: DWDR,
    _reserved3: [u8; 0x01],
    bodcr: BODCR,
    osccal: OSCCAL,
    _reserved5: [u8; 0x02],
    mcusr: MCUSR,
    mcucr: MCUCR,
    _reserved7: [u8; 0x01],
    spmcsr: SPMCSR,
    _reserved8: [u8; 0x05],
    spl: SPL,
}
impl RegisterBlock {
    #[doc = "0x00 - Power Reduction Register"]
    #[inline(always)]
    pub const fn prr(&self) -> &PRR {
        &self.prr
    }
    #[doc = "0x01 - Clock Prescale Register"]
    #[inline(always)]
    pub const fn clkpr(&self) -> &CLKPR {
        &self.clkpr
    }
    #[doc = "0x09 - Debug Wire Data Register"]
    #[inline(always)]
    pub const fn dwdr(&self) -> &DWDR {
        &self.dwdr
    }
    #[doc = "0x0b - BOD Control Register"]
    #[inline(always)]
    pub const fn bodcr(&self) -> &BODCR {
        &self.bodcr
    }
    #[doc = "0x0c - Oscillator Calibration Register"]
    #[inline(always)]
    pub const fn osccal(&self) -> &OSCCAL {
        &self.osccal
    }
    #[doc = "0x0f - MCU Status register"]
    #[inline(always)]
    pub const fn mcusr(&self) -> &MCUSR {
        &self.mcusr
    }
    #[doc = "0x10 - MCU Control Register"]
    #[inline(always)]
    pub const fn mcucr(&self) -> &MCUCR {
        &self.mcucr
    }
    #[doc = "0x12 - Store Program Memory Control and Status Register"]
    #[inline(always)]
    pub const fn spmcsr(&self) -> &SPMCSR {
        &self.spmcsr
    }
    #[doc = "0x18 - Stack Pointer Low Byte"]
    #[inline(always)]
    pub const fn spl(&self) -> &SPL {
        &self.spl
    }
}
#[doc = "BODCR (rw) register accessor: BOD Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bodcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bodcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bodcr`]
module"]
pub type BODCR = crate::Reg<bodcr::BODCR_SPEC>;
#[doc = "BOD Control Register"]
pub mod bodcr;
#[doc = "CLKPR (rw) register accessor: Clock Prescale Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkpr`]
module"]
pub type CLKPR = crate::Reg<clkpr::CLKPR_SPEC>;
#[doc = "Clock Prescale Register"]
pub mod clkpr;
#[doc = "DWDR (rw) register accessor: Debug Wire Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dwdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dwdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dwdr`]
module"]
pub type DWDR = crate::Reg<dwdr::DWDR_SPEC>;
#[doc = "Debug Wire Data Register"]
pub mod dwdr;
#[doc = "MCUCR (rw) register accessor: MCU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcucr`]
module"]
pub type MCUCR = crate::Reg<mcucr::MCUCR_SPEC>;
#[doc = "MCU Control Register"]
pub mod mcucr;
#[doc = "MCUSR (rw) register accessor: MCU Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcusr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcusr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcusr`]
module"]
pub type MCUSR = crate::Reg<mcusr::MCUSR_SPEC>;
#[doc = "MCU Status register"]
pub mod mcusr;
#[doc = "OSCCAL (rw) register accessor: Oscillator Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osccal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osccal`]
module"]
pub type OSCCAL = crate::Reg<osccal::OSCCAL_SPEC>;
#[doc = "Oscillator Calibration Register"]
pub mod osccal;
#[doc = "PRR (rw) register accessor: Power Reduction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prr`]
module"]
pub type PRR = crate::Reg<prr::PRR_SPEC>;
#[doc = "Power Reduction Register"]
pub mod prr;
#[doc = "SPL (rw) register accessor: Stack Pointer Low Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spl`]
module"]
pub type SPL = crate::Reg<spl::SPL_SPEC>;
#[doc = "Stack Pointer Low Byte"]
pub mod spl;
#[doc = "SPMCSR (rw) register accessor: Store Program Memory Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spmcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spmcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spmcsr`]
module"]
pub type SPMCSR = crate::Reg<spmcsr::SPMCSR_SPEC>;
#[doc = "Store Program Memory Control and Status Register"]
pub mod spmcsr;
