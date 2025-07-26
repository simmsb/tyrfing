#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ccp: CCP,
}
impl RegisterBlock {
    #[doc = "0x00 - Configuration Change Protection"]
    #[inline(always)]
    pub const fn ccp(&self) -> &CCP {
        &self.ccp
    }
}
#[doc = "CCP (rw) register accessor: Configuration Change Protection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccp`]
module"]
pub type CCP = crate::Reg<ccp::CCP_SPEC>;
#[doc = "Configuration Change Protection"]
pub mod ccp;
