#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    eifr: EIFR,
    gimsk: GIMSK,
}
impl RegisterBlock {
    #[doc = "0x00 - Extended Interrupt Flag Register"]
    #[inline(always)]
    pub const fn eifr(&self) -> &EIFR {
        &self.eifr
    }
    #[doc = "0x01 - General Interrupt Mask Register"]
    #[inline(always)]
    pub const fn gimsk(&self) -> &GIMSK {
        &self.gimsk
    }
}
#[doc = "EIFR (rw) register accessor: Extended Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eifr`]
module"]
pub type EIFR = crate::Reg<eifr::EIFR_SPEC>;
#[doc = "Extended Interrupt Flag Register"]
pub mod eifr;
#[doc = "GIMSK (rw) register accessor: General Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gimsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gimsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gimsk`]
module"]
pub type GIMSK = crate::Reg<gimsk::GIMSK_SPEC>;
#[doc = "General Interrupt Mask Register"]
pub mod gimsk;
