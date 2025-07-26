#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    acsrb: ACSRB,
    acsra: ACSRA,
}
impl RegisterBlock {
    #[doc = "0x00 - Analog Comparator Control And Status Register B"]
    #[inline(always)]
    pub const fn acsrb(&self) -> &ACSRB {
        &self.acsrb
    }
    #[doc = "0x01 - Analog Comparator Control And Status Register A"]
    #[inline(always)]
    pub const fn acsra(&self) -> &ACSRA {
        &self.acsra
    }
}
#[doc = "ACSRA (rw) register accessor: Analog Comparator Control And Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acsra`]
module"]
pub type ACSRA = crate::Reg<acsra::ACSRA_SPEC>;
#[doc = "Analog Comparator Control And Status Register A"]
pub mod acsra;
#[doc = "ACSRB (rw) register accessor: Analog Comparator Control And Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acsrb`]
module"]
pub type ACSRB = crate::Reg<acsrb::ACSRB_SPEC>;
#[doc = "Analog Comparator Control And Status Register B"]
pub mod acsrb;
