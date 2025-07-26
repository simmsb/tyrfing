#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    portcr: PORTCR,
    _reserved1: [u8; 0x0b],
    gpior0: GPIOR0,
    _reserved2: [u8; 0x0b],
    gpior1: GPIOR1,
    gpior2: GPIOR2,
    _reserved4: [u8; 0x05],
    dwdr: DWDR,
    _reserved5: [u8; 0x01],
    smcr: SMCR,
    mcusr: MCUSR,
    mcucr: MCUCR,
    _reserved8: [u8; 0x0b],
    clkpr: CLKPR,
    clkcsr: CLKCSR,
    clkselr: CLKSELR,
    prr: PRR,
    _reserved12: [u8; 0x01],
    osccal: OSCCAL,
}
impl RegisterBlock {
    #[doc = "0x00 - Port Control Register"]
    #[inline(always)]
    pub const fn portcr(&self) -> &PORTCR {
        &self.portcr
    }
    #[doc = "0x0c - General purpose register 0"]
    #[inline(always)]
    pub const fn gpior0(&self) -> &GPIOR0 {
        &self.gpior0
    }
    #[doc = "0x18 - General Purpose register 1"]
    #[inline(always)]
    pub const fn gpior1(&self) -> &GPIOR1 {
        &self.gpior1
    }
    #[doc = "0x19 - General Purpose IO register 2"]
    #[inline(always)]
    pub const fn gpior2(&self) -> &GPIOR2 {
        &self.gpior2
    }
    #[doc = "0x1f - DebugWire data register"]
    #[inline(always)]
    pub const fn dwdr(&self) -> &DWDR {
        &self.dwdr
    }
    #[doc = "0x21 - Sleep Mode Control Register"]
    #[inline(always)]
    pub const fn smcr(&self) -> &SMCR {
        &self.smcr
    }
    #[doc = "0x22 - MCU Status register"]
    #[inline(always)]
    pub const fn mcusr(&self) -> &MCUSR {
        &self.mcusr
    }
    #[doc = "0x23 - MCU Control Register"]
    #[inline(always)]
    pub const fn mcucr(&self) -> &MCUCR {
        &self.mcucr
    }
    #[doc = "0x2f - Clock Prescale Register"]
    #[inline(always)]
    pub const fn clkpr(&self) -> &CLKPR {
        &self.clkpr
    }
    #[doc = "0x30 - Clock Control &amp; Status Register"]
    #[inline(always)]
    pub const fn clkcsr(&self) -> &CLKCSR {
        &self.clkcsr
    }
    #[doc = "0x31 - Clock Selection Register"]
    #[inline(always)]
    pub const fn clkselr(&self) -> &CLKSELR {
        &self.clkselr
    }
    #[doc = "0x32 - Power Reduction Register"]
    #[inline(always)]
    pub const fn prr(&self) -> &PRR {
        &self.prr
    }
    #[doc = "0x34 - Oscillator Calibration Register"]
    #[inline(always)]
    pub const fn osccal(&self) -> &OSCCAL {
        &self.osccal
    }
}
#[doc = "CLKCSR (rw) register accessor: Clock Control &amp; Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcsr`]
module"]
pub type CLKCSR = crate::Reg<clkcsr::CLKCSR_SPEC>;
#[doc = "Clock Control &amp; Status Register"]
pub mod clkcsr;
#[doc = "CLKPR (rw) register accessor: Clock Prescale Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkpr`]
module"]
pub type CLKPR = crate::Reg<clkpr::CLKPR_SPEC>;
#[doc = "Clock Prescale Register"]
pub mod clkpr;
#[doc = "CLKSELR (rw) register accessor: Clock Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkselr`]
module"]
pub type CLKSELR = crate::Reg<clkselr::CLKSELR_SPEC>;
#[doc = "Clock Selection Register"]
pub mod clkselr;
#[doc = "DWDR (rw) register accessor: DebugWire data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dwdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dwdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dwdr`]
module"]
pub type DWDR = crate::Reg<dwdr::DWDR_SPEC>;
#[doc = "DebugWire data register"]
pub mod dwdr;
#[doc = "GPIOR0 (rw) register accessor: General purpose register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpior0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpior0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpior0`]
module"]
pub type GPIOR0 = crate::Reg<gpior0::GPIOR0_SPEC>;
#[doc = "General purpose register 0"]
pub mod gpior0;
#[doc = "GPIOR1 (rw) register accessor: General Purpose register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpior1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpior1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpior1`]
module"]
pub type GPIOR1 = crate::Reg<gpior1::GPIOR1_SPEC>;
#[doc = "General Purpose register 1"]
pub mod gpior1;
#[doc = "GPIOR2 (rw) register accessor: General Purpose IO register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpior2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpior2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpior2`]
module"]
pub type GPIOR2 = crate::Reg<gpior2::GPIOR2_SPEC>;
#[doc = "General Purpose IO register 2"]
pub mod gpior2;
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
#[doc = "PORTCR (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portcr`]
module"]
pub type PORTCR = crate::Reg<portcr::PORTCR_SPEC>;
#[doc = "Port Control Register"]
pub mod portcr;
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
