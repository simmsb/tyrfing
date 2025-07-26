#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ccp: CCP,
    _reserved1: [u8; 0x08],
    spl: SPL,
    sph: SPH,
}
impl RegisterBlock {
    #[doc = "0x00 - Configuration Change Protection"]
    #[inline(always)]
    pub const fn ccp(&self) -> &CCP {
        &self.ccp
    }
    #[doc = "0x09 - Stack Pointer Low"]
    #[inline(always)]
    pub const fn spl(&self) -> &SPL {
        &self.spl
    }
    #[doc = "0x0a - Stack Pointer High"]
    #[inline(always)]
    pub const fn sph(&self) -> &SPH {
        &self.sph
    }
}
#[doc = "CCP (rw) register accessor: Configuration Change Protection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccp`]
module"]
pub type CCP = crate::Reg<ccp::CCP_SPEC>;
#[doc = "Configuration Change Protection"]
pub mod ccp;
#[doc = "SPH (rw) register accessor: Stack Pointer High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sph::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sph::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sph`]
module"]
pub type SPH = crate::Reg<sph::SPH_SPEC>;
#[doc = "Stack Pointer High"]
pub mod sph;
#[doc = "SPL (rw) register accessor: Stack Pointer Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spl`]
module"]
pub type SPL = crate::Reg<spl::SPL_SPEC>;
#[doc = "Stack Pointer Low"]
pub mod spl;
