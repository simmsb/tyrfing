#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    gpior0: GPIOR0,
    _reserved1: [u8; 0x0b],
    gpior1: GPIOR1,
    gpior2: GPIOR2,
    _reserved3: [u8; 0x07],
    smcr: SMCR,
    mcusr: MCUSR,
    mcucr: MCUCR,
    _reserved6: [u8; 0x01],
    spmcsr: SPMCSR,
    _reserved7: [u8; 0x09],
    clkpr: CLKPR,
    _reserved8: [u8; 0x02],
    prr: PRR,
    _reserved9: [u8; 0x01],
    osccal: OSCCAL,
}
impl RegisterBlock {
    #[doc = "0x00 - General Purpose I/O Register 0"]
    #[inline(always)]
    pub const fn gpior0(&self) -> &GPIOR0 {
        &self.gpior0
    }
    #[doc = "0x0c - General Purpose I/O Register 1"]
    #[inline(always)]
    pub const fn gpior1(&self) -> &GPIOR1 {
        &self.gpior1
    }
    #[doc = "0x0d - General Purpose I/O Register 2"]
    #[inline(always)]
    pub const fn gpior2(&self) -> &GPIOR2 {
        &self.gpior2
    }
    #[doc = "0x15 - Sleep Mode Control Register"]
    #[inline(always)]
    pub const fn smcr(&self) -> &SMCR {
        &self.smcr
    }
    #[doc = "0x16 - MCU Status Register"]
    #[inline(always)]
    pub const fn mcusr(&self) -> &MCUSR {
        &self.mcusr
    }
    #[doc = "0x17 - MCU Control Register"]
    #[inline(always)]
    pub const fn mcucr(&self) -> &MCUCR {
        &self.mcucr
    }
    #[doc = "0x19 - Store Program Memory Control and Status Register"]
    #[inline(always)]
    pub const fn spmcsr(&self) -> &SPMCSR {
        &self.spmcsr
    }
    #[doc = "0x23 - Clock Prescale Register"]
    #[inline(always)]
    pub const fn clkpr(&self) -> &CLKPR {
        &self.clkpr
    }
    #[doc = "0x26 - Power Reduction Register"]
    #[inline(always)]
    pub const fn prr(&self) -> &PRR {
        &self.prr
    }
    #[doc = "0x28 - Oscillator Calibration Value"]
    #[inline(always)]
    pub const fn osccal(&self) -> &OSCCAL {
        &self.osccal
    }
}
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
#[doc = "OSCCAL (rw) register accessor: Oscillator Calibration Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osccal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osccal`]
module"]
pub type OSCCAL = crate::Reg<osccal::OSCCAL_SPEC>;
#[doc = "Oscillator Calibration Value"]
pub mod osccal;
#[doc = "PRR (rw) register accessor: Power Reduction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prr`]
module"]
pub type PRR = crate::Reg<prr::PRR_SPEC>;
#[doc = "Power Reduction Register"]
pub mod prr;
#[doc = "SMCR (rw) register accessor: Sleep Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smcr`]
module"]
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
#[doc = "Sleep Mode Control Register"]
pub mod smcr;
#[doc = "SPMCSR (rw) register accessor: Store Program Memory Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spmcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spmcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spmcsr`]
module"]
pub type SPMCSR = crate::Reg<spmcsr::SPMCSR_SPEC>;
#[doc = "Store Program Memory Control and Status Register"]
pub mod spmcsr;
