#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    bgcr: BGCR,
    _reserved1: [u8; 0x0d],
    nemcr: NEMCR,
}
impl RegisterBlock {
    #[doc = "0x00 - Reference Voltage Calibration Register"]
    #[inline(always)]
    pub const fn bgcr(&self) -> &BGCR {
        &self.bgcr
    }
    #[doc = "0x0e - Flash Extended-Mode Control-Register"]
    #[inline(always)]
    pub const fn nemcr(&self) -> &NEMCR {
        &self.nemcr
    }
}
#[doc = "BGCR (rw) register accessor: Reference Voltage Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgcr`]
module"]
pub type BGCR = crate::Reg<bgcr::BGCR_SPEC>;
#[doc = "Reference Voltage Calibration Register"]
pub mod bgcr;
#[doc = "NEMCR (rw) register accessor: Flash Extended-Mode Control-Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nemcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nemcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nemcr`]
module"]
pub type NEMCR = crate::Reg<nemcr::NEMCR_SPEC>;
#[doc = "Flash Extended-Mode Control-Register"]
pub mod nemcr;
