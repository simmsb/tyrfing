#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    gpior0: GPIOR0,
    gpior1: GPIOR1,
    gpior2: GPIOR2,
    _reserved3: [u8; 0x0c],
    prr: PRR,
    _reserved4: [u8; 0x01],
    dwdr: DWDR,
    _reserved5: [u8; 0x03],
    clkpr: CLKPR,
    pllcsr: PLLCSR,
    _reserved7: [u8; 0x09],
    osccal: OSCCAL,
    _reserved8: [u8; 0x02],
    mcusr: MCUSR,
    mcucr: MCUCR,
}
impl RegisterBlock {
    #[doc = "0x00 - General purpose register 0"]
    #[inline(always)]
    pub const fn gpior0(&self) -> &GPIOR0 {
        &self.gpior0
    }
    #[doc = "0x01 - General Purpose register 1"]
    #[inline(always)]
    pub const fn gpior1(&self) -> &GPIOR1 {
        &self.gpior1
    }
    #[doc = "0x02 - General Purpose IO register 2"]
    #[inline(always)]
    pub const fn gpior2(&self) -> &GPIOR2 {
        &self.gpior2
    }
    #[doc = "0x0f - Power Reduction Register"]
    #[inline(always)]
    pub const fn prr(&self) -> &PRR {
        &self.prr
    }
    #[doc = "0x11 - debugWire data register"]
    #[inline(always)]
    pub const fn dwdr(&self) -> &DWDR {
        &self.dwdr
    }
    #[doc = "0x15 - Clock Prescale Register"]
    #[inline(always)]
    pub const fn clkpr(&self) -> &CLKPR {
        &self.clkpr
    }
    #[doc = "0x16 - PLL Control and status register"]
    #[inline(always)]
    pub const fn pllcsr(&self) -> &PLLCSR {
        &self.pllcsr
    }
    #[doc = "0x20 - Oscillator Calibration Register"]
    #[inline(always)]
    pub const fn osccal(&self) -> &OSCCAL {
        &self.osccal
    }
    #[doc = "0x23 - MCU Status register"]
    #[inline(always)]
    pub const fn mcusr(&self) -> &MCUSR {
        &self.mcusr
    }
    #[doc = "0x24 - MCU Control Register"]
    #[inline(always)]
    pub const fn mcucr(&self) -> &MCUCR {
        &self.mcucr
    }
}
#[doc = "CLKPR (rw) register accessor: Clock Prescale Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkpr`]
module"]
pub type CLKPR = crate::Reg<clkpr::CLKPR_SPEC>;
#[doc = "Clock Prescale Register"]
pub mod clkpr;
#[doc = "DWDR (rw) register accessor: debugWire data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dwdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dwdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dwdr`]
module"]
pub type DWDR = crate::Reg<dwdr::DWDR_SPEC>;
#[doc = "debugWire data register"]
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
#[doc = "PLLCSR (rw) register accessor: PLL Control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcsr`]
module"]
pub type PLLCSR = crate::Reg<pllcsr::PLLCSR_SPEC>;
#[doc = "PLL Control and status register"]
pub mod pllcsr;
#[doc = "PRR (rw) register accessor: Power Reduction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prr`]
module"]
pub type PRR = crate::Reg<prr::PRR_SPEC>;
#[doc = "Power Reduction Register"]
pub mod prr;
