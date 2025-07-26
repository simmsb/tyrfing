#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pinl: PINL,
    ddrl: DDRL,
    portl: PORTL,
}
impl RegisterBlock {
    #[doc = "0x00 - PORT L Input Pins"]
    #[inline(always)]
    pub const fn pinl(&self) -> &PINL {
        &self.pinl
    }
    #[doc = "0x01 - PORT L Data Direction Register"]
    #[inline(always)]
    pub const fn ddrl(&self) -> &DDRL {
        &self.ddrl
    }
    #[doc = "0x02 - PORT L Data Register"]
    #[inline(always)]
    pub const fn portl(&self) -> &PORTL {
        &self.portl
    }
}
#[doc = "DDRL (rw) register accessor: PORT L Data Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrl`]
module"]
pub type DDRL = crate::Reg<ddrl::DDRL_SPEC>;
#[doc = "PORT L Data Direction Register"]
pub mod ddrl;
#[doc = "PINL (rw) register accessor: PORT L Input Pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinl`]
module"]
pub type PINL = crate::Reg<pinl::PINL_SPEC>;
#[doc = "PORT L Input Pins"]
pub mod pinl;
#[doc = "PORTL (rw) register accessor: PORT L Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portl`]
module"]
pub type PORTL = crate::Reg<portl::PORTL_SPEC>;
#[doc = "PORT L Data Register"]
pub mod portl;
