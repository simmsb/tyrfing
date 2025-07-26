#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    acsrb: ACSRB,
    _reserved_1_acsr: [u8; 0x01],
    _reserved2: [u8; 0x2e],
    didr1: DIDR1,
}
impl RegisterBlock {
    #[doc = "0x00 - Analog Comparator Control And Status Register-B"]
    #[inline(always)]
    pub const fn acsrb(&self) -> &ACSRB {
        &self.acsrb
    }
    #[doc = "0x01 - Analog Comparator Control And Status Register-A"]
    #[inline(always)]
    pub const fn acsra(&self) -> &ACSRA {
        unsafe { &*(self as *const Self).cast::<u8>().add(1).cast() }
    }
    #[doc = "0x01 - Analog Comparator Control And Status Register"]
    #[inline(always)]
    pub const fn acsr(&self) -> &ACSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(1).cast() }
    }
    #[doc = "0x30 - Digital Input Disable Register 1"]
    #[inline(always)]
    pub const fn didr1(&self) -> &DIDR1 {
        &self.didr1
    }
}
#[doc = "ACSR (rw) register accessor: Analog Comparator Control And Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acsr`]
module"]
pub type ACSR = crate::Reg<acsr::ACSR_SPEC>;
#[doc = "Analog Comparator Control And Status Register"]
pub mod acsr;
#[doc = "ACSRA (rw) register accessor: Analog Comparator Control And Status Register-A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acsra`]
module"]
pub type ACSRA = crate::Reg<acsra::ACSRA_SPEC>;
#[doc = "Analog Comparator Control And Status Register-A"]
pub mod acsra;
#[doc = "ACSRB (rw) register accessor: Analog Comparator Control And Status Register-B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acsrb`]
module"]
pub type ACSRB = crate::Reg<acsrb::ACSRB_SPEC>;
#[doc = "Analog Comparator Control And Status Register-B"]
pub mod acsrb;
#[doc = "DIDR1 (rw) register accessor: Digital Input Disable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`didr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`didr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@didr1`]
module"]
pub type DIDR1 = crate::Reg<didr1::DIDR1_SPEC>;
#[doc = "Digital Input Disable Register 1"]
pub mod didr1;
