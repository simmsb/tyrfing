#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    wdtcsr: WDTCSR,
    _reserved1: [u8; 0x01],
    wdtckd: WDTCKD,
}
impl RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control Register"]
    #[inline(always)]
    pub const fn wdtcsr(&self) -> &WDTCSR {
        &self.wdtcsr
    }
    #[doc = "0x02 - Watchdog Timer Clock Divider"]
    #[inline(always)]
    pub const fn wdtckd(&self) -> &WDTCKD {
        &self.wdtckd
    }
}
#[doc = "WDTCKD (rw) register accessor: Watchdog Timer Clock Divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtckd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtckd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtckd`]
module"]
pub type WDTCKD = crate::Reg<wdtckd::WDTCKD_SPEC>;
#[doc = "Watchdog Timer Clock Divider"]
pub mod wdtckd;
#[doc = "WDTCSR (rw) register accessor: Watchdog Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtcsr`]
module"]
pub type WDTCSR = crate::Reg<wdtcsr::WDTCSR_SPEC>;
#[doc = "Watchdog Timer Control Register"]
pub mod wdtcsr;
