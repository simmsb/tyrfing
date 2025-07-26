#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    prr: PRR,
    bodcr: BODCR,
    _reserved2: [u8; 0x0b],
    gpior0: GPIOR0,
    gpior1: GPIOR1,
    gpior2: GPIOR2,
    _reserved5: [u8; 0x0d],
    gtccr: GTCCR,
    _reserved6: [u8; 0x02],
    clkpr: CLKPR,
    _reserved7: [u8; 0x0a],
    osccal: OSCCAL,
    _reserved8: [u8; 0x02],
    mcusr: MCUSR,
    mcucr: MCUCR,
    _reserved10: [u8; 0x01],
    spmcsr: SPMCSR,
    _reserved11: [u8; 0x05],
    spl: SPL,
}
impl RegisterBlock {
    #[doc = "0x00 - Power reduction register"]
    #[inline(always)]
    pub const fn prr(&self) -> &PRR {
        &self.prr
    }
    #[doc = "0x01 - BOD control register"]
    #[inline(always)]
    pub const fn bodcr(&self) -> &BODCR {
        &self.bodcr
    }
    #[doc = "0x0d - General Purpose I/O Register 0"]
    #[inline(always)]
    pub const fn gpior0(&self) -> &GPIOR0 {
        &self.gpior0
    }
    #[doc = "0x0e - General Purpose I/O Register 1"]
    #[inline(always)]
    pub const fn gpior1(&self) -> &GPIOR1 {
        &self.gpior1
    }
    #[doc = "0x0f - General Purpose I/O Register 2"]
    #[inline(always)]
    pub const fn gpior2(&self) -> &GPIOR2 {
        &self.gpior2
    }
    #[doc = "0x1d - General Timer Counter Control Register"]
    #[inline(always)]
    pub const fn gtccr(&self) -> &GTCCR {
        &self.gtccr
    }
    #[doc = "0x20 - Clock Prescale Register"]
    #[inline(always)]
    pub const fn clkpr(&self) -> &CLKPR {
        &self.clkpr
    }
    #[doc = "0x2b - Oscillator Calibration Register"]
    #[inline(always)]
    pub const fn osccal(&self) -> &OSCCAL {
        &self.osccal
    }
    #[doc = "0x2e - MCU Status register"]
    #[inline(always)]
    pub const fn mcusr(&self) -> &MCUSR {
        &self.mcusr
    }
    #[doc = "0x2f - MCU Control Register"]
    #[inline(always)]
    pub const fn mcucr(&self) -> &MCUCR {
        &self.mcucr
    }
    #[doc = "0x31 - Store Program Memory Control and Status register"]
    #[inline(always)]
    pub const fn spmcsr(&self) -> &SPMCSR {
        &self.spmcsr
    }
    #[doc = "0x37 - Stack Pointer Low Byte"]
    #[inline(always)]
    pub const fn spl(&self) -> &SPL {
        &self.spl
    }
}
#[doc = "BODCR (rw) register accessor: BOD control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bodcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bodcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bodcr`]
module"]
pub type BODCR = crate::Reg<bodcr::BODCR_SPEC>;
#[doc = "BOD control register"]
pub mod bodcr;
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
#[doc = "GTCCR (rw) register accessor: General Timer Counter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtccr`]
module"]
pub type GTCCR = crate::Reg<gtccr::GTCCR_SPEC>;
#[doc = "General Timer Counter Control Register"]
pub mod gtccr;
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
#[doc = "PRR (rw) register accessor: Power reduction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prr`]
module"]
pub type PRR = crate::Reg<prr::PRR_SPEC>;
#[doc = "Power reduction register"]
pub mod prr;
#[doc = "SPL (rw) register accessor: Stack Pointer Low Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spl`]
module"]
pub type SPL = crate::Reg<spl::SPL_SPEC>;
#[doc = "Stack Pointer Low Byte"]
pub mod spl;
#[doc = "SPMCSR (rw) register accessor: Store Program Memory Control and Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spmcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spmcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spmcsr`]
module"]
pub type SPMCSR = crate::Reg<spmcsr::SPMCSR_SPEC>;
#[doc = "Store Program Memory Control and Status register"]
pub mod spmcsr;
