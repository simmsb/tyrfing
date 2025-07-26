#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    revid: REVID,
    _reserved1: [u8; 0x02],
    ocdmctrl: OCDMCTRL,
    ocdmstatus: OCDMSTATUS,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision ID"]
    #[inline(always)]
    pub const fn revid(&self) -> &REVID {
        &self.revid
    }
    #[doc = "0x03 - OCD Message Control"]
    #[inline(always)]
    pub const fn ocdmctrl(&self) -> &OCDMCTRL {
        &self.ocdmctrl
    }
    #[doc = "0x04 - OCD Message Status"]
    #[inline(always)]
    pub const fn ocdmstatus(&self) -> &OCDMSTATUS {
        &self.ocdmstatus
    }
}
#[doc = "OCDMCTRL (rw) register accessor: OCD Message Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocdmctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocdmctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocdmctrl`]
module"]
pub type OCDMCTRL = crate::Reg<ocdmctrl::OCDMCTRL_SPEC>;
#[doc = "OCD Message Control"]
pub mod ocdmctrl;
#[doc = "OCDMSTATUS (rw) register accessor: OCD Message Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocdmstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocdmstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocdmstatus`]
module"]
pub type OCDMSTATUS = crate::Reg<ocdmstatus::OCDMSTATUS_SPEC>;
#[doc = "OCD Message Status"]
pub mod ocdmstatus;
#[doc = "REVID (rw) register accessor: Revision ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`revid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revid`]
module"]
pub type REVID = crate::Reg<revid::REVID_SPEC>;
#[doc = "Revision ID"]
pub mod revid;
