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
    _reserved6: [u8; 0x05],
    rampz: RAMPZ,
    eind: EIND,
    _reserved8: [u8; 0x04],
    clkpr: CLKPR,
    _reserved9: [u8; 0x02],
    prr0: PRR0,
    prr1: PRR1,
    osccal: OSCCAL,
    rcctrl: RCCTRL,
    _reserved13: [u8; 0x5d],
    clksel0: CLKSEL0,
    clksel1: CLKSEL1,
    clksta: CLKSTA,
}
impl RegisterBlock {
    #[doc = "0x00 - General Purpose IO Register 0"]
    #[inline(always)]
    pub const fn gpior0(&self) -> &GPIOR0 {
        &self.gpior0
    }
    #[doc = "0x0c - General Purpose IO Register 1"]
    #[inline(always)]
    pub const fn gpior1(&self) -> &GPIOR1 {
        &self.gpior1
    }
    #[doc = "0x0d - General Purpose IO Register 2"]
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
    #[doc = "0x1d - Extended Z-pointer Register for ELPM/SPM"]
    #[inline(always)]
    pub const fn rampz(&self) -> &RAMPZ {
        &self.rampz
    }
    #[doc = "0x1e - Extended Indirect Register"]
    #[inline(always)]
    pub const fn eind(&self) -> &EIND {
        &self.eind
    }
    #[doc = "0x23 - No Description."]
    #[inline(always)]
    pub const fn clkpr(&self) -> &CLKPR {
        &self.clkpr
    }
    #[doc = "0x26 - Power Reduction Register0"]
    #[inline(always)]
    pub const fn prr0(&self) -> &PRR0 {
        &self.prr0
    }
    #[doc = "0x27 - Power Reduction Register1"]
    #[inline(always)]
    pub const fn prr1(&self) -> &PRR1 {
        &self.prr1
    }
    #[doc = "0x28 - Oscillator Calibration Value"]
    #[inline(always)]
    pub const fn osccal(&self) -> &OSCCAL {
        &self.osccal
    }
    #[doc = "0x29 - Oscillator Control Register"]
    #[inline(always)]
    pub const fn rcctrl(&self) -> &RCCTRL {
        &self.rcctrl
    }
    #[doc = "0x87 - No Description."]
    #[inline(always)]
    pub const fn clksel0(&self) -> &CLKSEL0 {
        &self.clksel0
    }
    #[doc = "0x88 - No Description."]
    #[inline(always)]
    pub const fn clksel1(&self) -> &CLKSEL1 {
        &self.clksel1
    }
    #[doc = "0x89 - No Description."]
    #[inline(always)]
    pub const fn clksta(&self) -> &CLKSTA {
        &self.clksta
    }
}
#[doc = "CLKPR (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkpr`]
module"]
pub type CLKPR = crate::Reg<clkpr::CLKPR_SPEC>;
#[doc = "No Description."]
pub mod clkpr;
#[doc = "CLKSEL0 (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel0`]
module"]
pub type CLKSEL0 = crate::Reg<clksel0::CLKSEL0_SPEC>;
#[doc = "No Description."]
pub mod clksel0;
#[doc = "CLKSEL1 (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel1`]
module"]
pub type CLKSEL1 = crate::Reg<clksel1::CLKSEL1_SPEC>;
#[doc = "No Description."]
pub mod clksel1;
#[doc = "CLKSTA (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksta`]
module"]
pub type CLKSTA = crate::Reg<clksta::CLKSTA_SPEC>;
#[doc = "No Description."]
pub mod clksta;
#[doc = "EIND (rw) register accessor: Extended Indirect Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eind::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eind::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eind`]
module"]
pub type EIND = crate::Reg<eind::EIND_SPEC>;
#[doc = "Extended Indirect Register"]
pub mod eind;
#[doc = "GPIOR0 (rw) register accessor: General Purpose IO Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpior0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpior0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpior0`]
module"]
pub type GPIOR0 = crate::Reg<gpior0::GPIOR0_SPEC>;
#[doc = "General Purpose IO Register 0"]
pub mod gpior0;
#[doc = "GPIOR1 (rw) register accessor: General Purpose IO Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpior1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpior1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpior1`]
module"]
pub type GPIOR1 = crate::Reg<gpior1::GPIOR1_SPEC>;
#[doc = "General Purpose IO Register 1"]
pub mod gpior1;
#[doc = "GPIOR2 (rw) register accessor: General Purpose IO Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpior2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpior2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpior2`]
module"]
pub type GPIOR2 = crate::Reg<gpior2::GPIOR2_SPEC>;
#[doc = "General Purpose IO Register 2"]
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
#[doc = "PRR0 (rw) register accessor: Power Reduction Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prr0`]
module"]
pub type PRR0 = crate::Reg<prr0::PRR0_SPEC>;
#[doc = "Power Reduction Register0"]
pub mod prr0;
#[doc = "PRR1 (rw) register accessor: Power Reduction Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prr1`]
module"]
pub type PRR1 = crate::Reg<prr1::PRR1_SPEC>;
#[doc = "Power Reduction Register1"]
pub mod prr1;
#[doc = "RAMPZ (rw) register accessor: Extended Z-pointer Register for ELPM/SPM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rampz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rampz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rampz`]
module"]
pub type RAMPZ = crate::Reg<rampz::RAMPZ_SPEC>;
#[doc = "Extended Z-pointer Register for ELPM/SPM"]
pub mod rampz;
#[doc = "RCCTRL (rw) register accessor: Oscillator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcctrl`]
module"]
pub type RCCTRL = crate::Reg<rcctrl::RCCTRL_SPEC>;
#[doc = "Oscillator Control Register"]
pub mod rcctrl;
#[doc = "SMCR (rw) register accessor: Sleep Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smcr`]
module"]
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
#[doc = "Sleep Mode Control Register"]
pub mod smcr;
