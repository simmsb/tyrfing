#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    revid: REVID,
    extbrk: EXTBRK,
    _reserved2: [u8; 0x15],
    ocdm: OCDM,
    ocdms: OCDMS,
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
    #[doc = "0x17 - OCD Message Register"]
    #[inline(always)]
    pub const fn ocdm(&self) -> &OCDM {
        &self.ocdm
    }
    #[doc = "0x18 - OCD Message Status"]
    #[inline(always)]
    pub const fn ocdms(&self) -> &OCDMS {
        &self.ocdms
    }
}
#[doc = "EXTBRK (rw) register accessor: External Break\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extbrk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extbrk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extbrk`]
module"]
pub type EXTBRK = crate::Reg<extbrk::EXTBRK_SPEC>;
#[doc = "External Break"]
pub mod extbrk;
#[doc = "OCDM (rw) register accessor: OCD Message Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocdm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocdm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocdm`]
module"]
pub type OCDM = crate::Reg<ocdm::OCDM_SPEC>;
#[doc = "OCD Message Register"]
pub mod ocdm;
#[doc = "OCDMS (rw) register accessor: OCD Message Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocdms::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocdms::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocdms`]
module"]
pub type OCDMS = crate::Reg<ocdms::OCDMS_SPEC>;
#[doc = "OCD Message Status"]
pub mod ocdms;
#[doc = "REVID (rw) register accessor: Revision ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`revid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revid`]
module"]
pub type REVID = crate::Reg<revid::REVID_SPEC>;
#[doc = "Revision ID"]
pub mod revid;
