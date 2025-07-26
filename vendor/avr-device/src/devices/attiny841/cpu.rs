#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    gpior0: GPIOR0,
    gpior1: GPIOR1,
    gpior2: GPIOR2,
    _reserved3: [u8; 0x1e],
    mcusr: MCUSR,
    mcucr: MCUCR,
    _reserved5: [u8; 0x01],
    spmcsr: SPMCSR,
    _reserved6: [u8; 0x18],
    prr: PRR,
    ccp: CCP,
    clkcr: CLKCR,
    clkpr: CLKPR,
    osccal0: OSCCAL0,
    osctcal0a: OSCTCAL0A,
    osctcal0b: OSCTCAL0B,
    osccal1: OSCCAL1,
}
impl RegisterBlock {
    #[doc = "0x00 - General Purpose I/O Register 0"]
    #[inline(always)]
    pub const fn gpior0(&self) -> &GPIOR0 {
        &self.gpior0
    }
    #[doc = "0x01 - General Purpose I/O Register 1"]
    #[inline(always)]
    pub const fn gpior1(&self) -> &GPIOR1 {
        &self.gpior1
    }
    #[doc = "0x02 - General Purpose I/O Register 2"]
    #[inline(always)]
    pub const fn gpior2(&self) -> &GPIOR2 {
        &self.gpior2
    }
    #[doc = "0x21 - MCU Status Register"]
    #[inline(always)]
    pub const fn mcusr(&self) -> &MCUSR {
        &self.mcusr
    }
    #[doc = "0x22 - MCU Control Register"]
    #[inline(always)]
    pub const fn mcucr(&self) -> &MCUCR {
        &self.mcucr
    }
    #[doc = "0x24 - Store Program Memory Control and Status Register"]
    #[inline(always)]
    pub const fn spmcsr(&self) -> &SPMCSR {
        &self.spmcsr
    }
    #[doc = "0x3d - Power Reduction Register"]
    #[inline(always)]
    pub const fn prr(&self) -> &PRR {
        &self.prr
    }
    #[doc = "0x3e - Configuration Change Protection"]
    #[inline(always)]
    pub const fn ccp(&self) -> &CCP {
        &self.ccp
    }
    #[doc = "0x3f - Clock Control Register"]
    #[inline(always)]
    pub const fn clkcr(&self) -> &CLKCR {
        &self.clkcr
    }
    #[doc = "0x40 - Clock Prescale Register"]
    #[inline(always)]
    pub const fn clkpr(&self) -> &CLKPR {
        &self.clkpr
    }
    #[doc = "0x41 - Oscillator Calibration Register 8MHz"]
    #[inline(always)]
    pub const fn osccal0(&self) -> &OSCCAL0 {
        &self.osccal0
    }
    #[doc = "0x42 - Oscillator Temperature Calibration Register A"]
    #[inline(always)]
    pub const fn osctcal0a(&self) -> &OSCTCAL0A {
        &self.osctcal0a
    }
    #[doc = "0x43 - Oscillator Temperature Calibration Register B"]
    #[inline(always)]
    pub const fn osctcal0b(&self) -> &OSCTCAL0B {
        &self.osctcal0b
    }
    #[doc = "0x44 - Oscillator Calibration Register 32kHz"]
    #[inline(always)]
    pub const fn osccal1(&self) -> &OSCCAL1 {
        &self.osccal1
    }
}
#[doc = "CCP (rw) register accessor: Configuration Change Protection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccp`]
module"]
pub type CCP = crate::Reg<ccp::CCP_SPEC>;
#[doc = "Configuration Change Protection"]
pub mod ccp;
#[doc = "CLKCR (rw) register accessor: Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcr`]
module"]
pub type CLKCR = crate::Reg<clkcr::CLKCR_SPEC>;
#[doc = "Clock Control Register"]
pub mod clkcr;
#[doc = "CLKPR (rw) register accessor: Clock Prescale Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkpr`]
module"]
pub type CLKPR = crate::Reg<clkpr::CLKPR_SPEC>;
#[doc = "Clock Prescale Register"]
pub mod clkpr;
#[doc = "GPIOR0 (rw) register accessor: General Purpose I/O Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpior0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpior0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpior0`]
module"]
pub type GPIOR0 = crate::Reg<gpior0::GPIOR0_SPEC>;
#[doc = "General Purpose I/O Register 0"]
pub mod gpior0;
#[doc = "GPIOR1 (rw) register accessor: General Purpose I/O Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpior1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpior1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpior1`]
module"]
pub type GPIOR1 = crate::Reg<gpior1::GPIOR1_SPEC>;
#[doc = "General Purpose I/O Register 1"]
pub mod gpior1;
#[doc = "GPIOR2 (rw) register accessor: General Purpose I/O Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpior2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpior2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpior2`]
module"]
pub type GPIOR2 = crate::Reg<gpior2::GPIOR2_SPEC>;
#[doc = "General Purpose I/O Register 2"]
pub mod gpior2;
#[doc = "MCUCR (rw) register accessor: MCU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcucr`]
module"]
pub type MCUCR = crate::Reg<mcucr::MCUCR_SPEC>;
#[doc = "MCU Control Register"]
pub mod mcucr;
#[doc = "MCUSR (rw) register accessor: MCU Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcusr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcusr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcusr`]
module"]
pub type MCUSR = crate::Reg<mcusr::MCUSR_SPEC>;
#[doc = "MCU Status Register"]
pub mod mcusr;
#[doc = "OSCCAL0 (rw) register accessor: Oscillator Calibration Register 8MHz\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccal0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osccal0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osccal0`]
module"]
pub type OSCCAL0 = crate::Reg<osccal0::OSCCAL0_SPEC>;
#[doc = "Oscillator Calibration Register 8MHz"]
pub mod osccal0;
#[doc = "OSCCAL1 (rw) register accessor: Oscillator Calibration Register 32kHz\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccal1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osccal1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osccal1`]
module"]
pub type OSCCAL1 = crate::Reg<osccal1::OSCCAL1_SPEC>;
#[doc = "Oscillator Calibration Register 32kHz"]
pub mod osccal1;
#[doc = "OSCTCAL0A (rw) register accessor: Oscillator Temperature Calibration Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osctcal0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osctcal0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osctcal0a`]
module"]
pub type OSCTCAL0A = crate::Reg<osctcal0a::OSCTCAL0A_SPEC>;
#[doc = "Oscillator Temperature Calibration Register A"]
pub mod osctcal0a;
#[doc = "OSCTCAL0B (rw) register accessor: Oscillator Temperature Calibration Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osctcal0b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osctcal0b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osctcal0b`]
module"]
pub type OSCTCAL0B = crate::Reg<osctcal0b::OSCTCAL0B_SPEC>;
#[doc = "Oscillator Temperature Calibration Register B"]
pub mod osctcal0b;
#[doc = "PRR (rw) register accessor: Power Reduction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prr`]
module"]
pub type PRR = crate::Reg<prr::PRR_SPEC>;
#[doc = "Power Reduction Register"]
pub mod prr;
#[doc = "SPMCSR (rw) register accessor: Store Program Memory Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spmcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spmcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spmcsr`]
module"]
pub type SPMCSR = crate::Reg<spmcsr::SPMCSR_SPEC>;
#[doc = "Store Program Memory Control and Status Register"]
pub mod spmcsr;
