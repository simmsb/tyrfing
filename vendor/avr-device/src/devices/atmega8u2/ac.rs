#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    acsr: ACSR,
    _reserved1: [u8; 0x2c],
    acmux: ACMUX,
    _reserved2: [u8; 0x01],
    didr1: DIDR1,
}
impl RegisterBlock {
    #[doc = "0x00 - Analog Comparator Control And Status Register"]
    #[inline(always)]
    pub const fn acsr(&self) -> &ACSR {
        &self.acsr
    }
    #[doc = "0x2d - Analog Comparator Input Multiplexer"]
    #[inline(always)]
    pub const fn acmux(&self) -> &ACMUX {
        &self.acmux
    }
    #[doc = "0x2f - No Description."]
    #[inline(always)]
    pub const fn didr1(&self) -> &DIDR1 {
        &self.didr1
    }
}
#[doc = "ACMUX (rw) register accessor: Analog Comparator Input Multiplexer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acmux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acmux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acmux`]
module"]
pub type ACMUX = crate::Reg<acmux::ACMUX_SPEC>;
#[doc = "Analog Comparator Input Multiplexer"]
pub mod acmux;
#[doc = "ACSR (rw) register accessor: Analog Comparator Control And Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acsr`]
module"]
pub type ACSR = crate::Reg<acsr::ACSR_SPEC>;
#[doc = "Analog Comparator Control And Status Register"]
pub mod acsr;
#[doc = "DIDR1 (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`didr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`didr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@didr1`]
module"]
pub type DIDR1 = crate::Reg<didr1::DIDR1_SPEC>;
#[doc = "No Description."]
pub mod didr1;
