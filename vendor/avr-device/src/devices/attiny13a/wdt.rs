#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    wdtcr: WDTCR,
}
impl RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control Register"]
    #[inline(always)]
    pub const fn wdtcr(&self) -> &WDTCR {
        &self.wdtcr
    }
}
#[doc = "WDTCR (rw) register accessor: Watchdog Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtcr`]
module"]
pub type WDTCR = crate::Reg<wdtcr::WDTCR_SPEC>;
#[doc = "Watchdog Timer Control Register"]
pub mod wdtcr;
