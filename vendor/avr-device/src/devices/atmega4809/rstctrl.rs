#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    rstfr: RSTFR,
    swrr: SWRR,
}
impl RegisterBlock {
    #[doc = "0x00 - Reset Flags"]
    #[inline(always)]
    pub const fn rstfr(&self) -> &RSTFR {
        &self.rstfr
    }
    #[doc = "0x01 - Software Reset"]
    #[inline(always)]
    pub const fn swrr(&self) -> &SWRR {
        &self.swrr
    }
}
#[doc = "RSTFR (rw) register accessor: Reset Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstfr`]
module"]
pub type RSTFR = crate::Reg<rstfr::RSTFR_SPEC>;
#[doc = "Reset Flags"]
pub mod rstfr;
#[doc = "SWRR (rw) register accessor: Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrr`]
module"]
pub type SWRR = crate::Reg<swrr::SWRR_SPEC>;
#[doc = "Software Reset"]
pub mod swrr;
