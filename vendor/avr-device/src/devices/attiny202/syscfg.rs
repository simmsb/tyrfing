#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    revid: REVID,
    extbrk: EXTBRK,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision ID"]
    #[inline(always)]
    pub const fn revid(&self) -> &REVID {
        &self.revid
    }
    #[doc = "0x01 - External Break"]
    #[inline(always)]
    pub const fn extbrk(&self) -> &EXTBRK {
        &self.extbrk
    }
}
#[doc = "EXTBRK (rw) register accessor: External Break\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extbrk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extbrk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extbrk`]
module"]
pub type EXTBRK = crate::Reg<extbrk::EXTBRK_SPEC>;
#[doc = "External Break"]
pub mod extbrk;
#[doc = "REVID (rw) register accessor: Revision ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`revid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revid`]
module"]
pub type REVID = crate::Reg<revid::REVID_SPEC>;
#[doc = "Revision ID"]
pub mod revid;
