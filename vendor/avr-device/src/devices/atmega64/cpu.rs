#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mcucsr: MCUCSR,
    mcucr: MCUCR,
    _reserved2: [u8; 0x06],
    xdiv: XDIV,
    _reserved3: [u8; 0x0f],
    xmcrb: XMCRB,
    xmcra: XMCRA,
    _reserved5: [u8; 0x01],
    osccal: OSCCAL,
}
impl RegisterBlock {
    #[doc = "0x00 - MCU Control And Status Register"]
    #[inline(always)]
    pub const fn mcucsr(&self) -> &MCUCSR {
        &self.mcucsr
    }
    #[doc = "0x01 - MCU Control Register"]
    #[inline(always)]
    pub const fn mcucr(&self) -> &MCUCR {
        &self.mcucr
    }
    #[doc = "0x08 - XTAL Divide Control Register"]
    #[inline(always)]
    pub const fn xdiv(&self) -> &XDIV {
        &self.xdiv
    }
    #[doc = "0x18 - External Memory Control Register B"]
    #[inline(always)]
    pub const fn xmcrb(&self) -> &XMCRB {
        &self.xmcrb
    }
    #[doc = "0x19 - External Memory Control Register A"]
    #[inline(always)]
    pub const fn xmcra(&self) -> &XMCRA {
        &self.xmcra
    }
    #[doc = "0x1b - Oscillator Calibration Value"]
    #[inline(always)]
    pub const fn osccal(&self) -> &OSCCAL {
        &self.osccal
    }
}
#[doc = "MCUCR (rw) register accessor: MCU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcucr`]
module"]
pub type MCUCR = crate::Reg<mcucr::MCUCR_SPEC>;
#[doc = "MCU Control Register"]
pub mod mcucr;
#[doc = "MCUCSR (rw) register accessor: MCU Control And Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcucsr`]
module"]
pub type MCUCSR = crate::Reg<mcucsr::MCUCSR_SPEC>;
#[doc = "MCU Control And Status Register"]
pub mod mcucsr;
#[doc = "OSCCAL (rw) register accessor: Oscillator Calibration Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osccal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osccal`]
module"]
pub type OSCCAL = crate::Reg<osccal::OSCCAL_SPEC>;
#[doc = "Oscillator Calibration Value"]
pub mod osccal;
#[doc = "XDIV (rw) register accessor: XTAL Divide Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xdiv`]
module"]
pub type XDIV = crate::Reg<xdiv::XDIV_SPEC>;
#[doc = "XTAL Divide Control Register"]
pub mod xdiv;
#[doc = "XMCRA (rw) register accessor: External Memory Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xmcra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xmcra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xmcra`]
module"]
pub type XMCRA = crate::Reg<xmcra::XMCRA_SPEC>;
#[doc = "External Memory Control Register A"]
pub mod xmcra;
#[doc = "XMCRB (rw) register accessor: External Memory Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xmcrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xmcrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xmcrb`]
module"]
pub type XMCRB = crate::Reg<xmcrb::XMCRB_SPEC>;
#[doc = "External Memory Control Register B"]
pub mod xmcrb;
