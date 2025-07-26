#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    didr: DIDR,
    _reserved1: [u8; 0x06],
    acsr: ACSR,
}
impl RegisterBlock {
    #[doc = "0x00 - Digital Input Disable Register 1"]
    #[inline(always)]
    pub const fn didr(&self) -> &DIDR {
        &self.didr
    }
    #[doc = "0x07 - Analog Comparator Control And Status Register"]
    #[inline(always)]
    pub const fn acsr(&self) -> &ACSR {
        &self.acsr
    }
}
#[doc = "ACSR (rw) register accessor: Analog Comparator Control And Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acsr`]
module"]
pub type ACSR = crate::Reg<acsr::ACSR_SPEC>;
#[doc = "Analog Comparator Control And Status Register"]
pub mod acsr;
#[doc = "DIDR (rw) register accessor: Digital Input Disable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`didr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`didr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@didr`]
module"]
pub type DIDR = crate::Reg<didr::DIDR_SPEC>;
#[doc = "Digital Input Disable Register 1"]
pub mod didr;
