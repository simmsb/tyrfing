#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pinf: PINF,
    ddrf: DDRF,
    portf: PORTF,
}
impl RegisterBlock {
    #[doc = "0x00 - Port F Input Pins Address"]
    #[inline(always)]
    pub const fn pinf(&self) -> &PINF {
        &self.pinf
    }
    #[doc = "0x01 - Port F Data Direction Register"]
    #[inline(always)]
    pub const fn ddrf(&self) -> &DDRF {
        &self.ddrf
    }
    #[doc = "0x02 - Port F Data Register"]
    #[inline(always)]
    pub const fn portf(&self) -> &PORTF {
        &self.portf
    }
}
#[doc = "DDRF (rw) register accessor: Port F Data Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrf`]
module"]
pub type DDRF = crate::Reg<ddrf::DDRF_SPEC>;
#[doc = "Port F Data Direction Register"]
pub mod ddrf;
#[doc = "PINF (rw) register accessor: Port F Input Pins Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinf`]
module"]
pub type PINF = crate::Reg<pinf::PINF_SPEC>;
#[doc = "Port F Input Pins Address"]
pub mod pinf;
#[doc = "PORTF (rw) register accessor: Port F Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portf`]
module"]
pub type PORTF = crate::Reg<portf::PORTF_SPEC>;
#[doc = "Port F Data Register"]
pub mod portf;
