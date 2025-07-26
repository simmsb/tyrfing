#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrla: CTRLA,
    ctrlb: CTRLB,
    ctrlc: CTRLC,
    ctrld: CTRLD,
    ctrle: CTRLE,
    _reserved5: [u8; 0x03],
    evctrla: EVCTRLA,
    evctrlb: EVCTRLB,
    _reserved7: [u8; 0x02],
    intctrl: INTCTRL,
    intflags: INTFLAGS,
    status: STATUS,
    _reserved10: [u8; 0x01],
    inputctrla: INPUTCTRLA,
    inputctrlb: INPUTCTRLB,
    faultctrl: FAULTCTRL,
    _reserved13: [u8; 0x01],
    dlyctrl: DLYCTRL,
    dlyval: DLYVAL,
    _reserved15: [u8; 0x02],
    ditctrl: DITCTRL,
    ditval: DITVAL,
    _reserved17: [u8; 0x04],
    dbgctrl: DBGCTRL,
    _reserved18: [u8; 0x03],
    capturea: CAPTUREA,
    captureb: CAPTUREB,
    _reserved20: [u8; 0x02],
    cmpaset: CMPASET,
    cmpaclr: CMPACLR,
    cmpbset: CMPBSET,
    cmpbclr: CMPBCLR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &CTRLA {
        &self.ctrla
    }
    #[doc = "0x01 - Control B"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &CTRLB {
        &self.ctrlb
    }
    #[doc = "0x02 - Control C"]
    #[inline(always)]
    pub const fn ctrlc(&self) -> &CTRLC {
        &self.ctrlc
    }
    #[doc = "0x03 - Control D"]
    #[inline(always)]
    pub const fn ctrld(&self) -> &CTRLD {
        &self.ctrld
    }
    #[doc = "0x04 - Control E"]
    #[inline(always)]
    pub const fn ctrle(&self) -> &CTRLE {
        &self.ctrle
    }
    #[doc = "0x08 - EVCTRLA"]
    #[inline(always)]
    pub const fn evctrla(&self) -> &EVCTRLA {
        &self.evctrla
    }
    #[doc = "0x09 - EVCTRLB"]
    #[inline(always)]
    pub const fn evctrlb(&self) -> &EVCTRLB {
        &self.evctrlb
    }
    #[doc = "0x0c - Interrupt Control"]
    #[inline(always)]
    pub const fn intctrl(&self) -> &INTCTRL {
        &self.intctrl
    }
    #[doc = "0x0d - Interrupt Flags"]
    #[inline(always)]
    pub const fn intflags(&self) -> &INTFLAGS {
        &self.intflags
    }
    #[doc = "0x0e - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x10 - Input Control A"]
    #[inline(always)]
    pub const fn inputctrla(&self) -> &INPUTCTRLA {
        &self.inputctrla
    }
    #[doc = "0x11 - Input Control B"]
    #[inline(always)]
    pub const fn inputctrlb(&self) -> &INPUTCTRLB {
        &self.inputctrlb
    }
    #[doc = "0x12 - Fault Control"]
    #[inline(always)]
    pub const fn faultctrl(&self) -> &FAULTCTRL {
        &self.faultctrl
    }
    #[doc = "0x14 - Delay Control"]
    #[inline(always)]
    pub const fn dlyctrl(&self) -> &DLYCTRL {
        &self.dlyctrl
    }
    #[doc = "0x15 - Delay value"]
    #[inline(always)]
    pub const fn dlyval(&self) -> &DLYVAL {
        &self.dlyval
    }
    #[doc = "0x18 - Dither Control A"]
    #[inline(always)]
    pub const fn ditctrl(&self) -> &DITCTRL {
        &self.ditctrl
    }
    #[doc = "0x19 - Dither value"]
    #[inline(always)]
    pub const fn ditval(&self) -> &DITVAL {
        &self.ditval
    }
    #[doc = "0x1e - Debug Control"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &DBGCTRL {
        &self.dbgctrl
    }
    #[doc = "0x22 - Capture A"]
    #[inline(always)]
    pub const fn capturea(&self) -> &CAPTUREA {
        &self.capturea
    }
    #[doc = "0x24 - Capture B"]
    #[inline(always)]
    pub const fn captureb(&self) -> &CAPTUREB {
        &self.captureb
    }
    #[doc = "0x28 - Compare A Set"]
    #[inline(always)]
    pub const fn cmpaset(&self) -> &CMPASET {
        &self.cmpaset
    }
    #[doc = "0x2a - Compare A Clear"]
    #[inline(always)]
    pub const fn cmpaclr(&self) -> &CMPACLR {
        &self.cmpaclr
    }
    #[doc = "0x2c - Compare B Set"]
    #[inline(always)]
    pub const fn cmpbset(&self) -> &CMPBSET {
        &self.cmpbset
    }
    #[doc = "0x2e - Compare B Clear"]
    #[inline(always)]
    pub const fn cmpbclr(&self) -> &CMPBCLR {
        &self.cmpbclr
    }
}
#[doc = "CAPTUREA (r) register accessor: Capture A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capturea::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capturea`]
module"]
pub type CAPTUREA = crate::Reg<capturea::CAPTUREA_SPEC>;
#[doc = "Capture A"]
pub mod capturea;
#[doc = "CAPTUREB (r) register accessor: Capture B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`captureb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@captureb`]
module"]
pub type CAPTUREB = crate::Reg<captureb::CAPTUREB_SPEC>;
#[doc = "Capture B"]
pub mod captureb;
#[doc = "CMPACLR (rw) register accessor: Compare A Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpaclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpaclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpaclr`]
module"]
pub type CMPACLR = crate::Reg<cmpaclr::CMPACLR_SPEC>;
#[doc = "Compare A Clear"]
pub mod cmpaclr;
#[doc = "CMPASET (rw) register accessor: Compare A Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpaset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpaset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpaset`]
module"]
pub type CMPASET = crate::Reg<cmpaset::CMPASET_SPEC>;
#[doc = "Compare A Set"]
pub mod cmpaset;
#[doc = "CMPBCLR (rw) register accessor: Compare B Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpbclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpbclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpbclr`]
module"]
pub type CMPBCLR = crate::Reg<cmpbclr::CMPBCLR_SPEC>;
#[doc = "Compare B Clear"]
pub mod cmpbclr;
#[doc = "CMPBSET (rw) register accessor: Compare B Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpbset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpbset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpbset`]
module"]
pub type CMPBSET = crate::Reg<cmpbset::CMPBSET_SPEC>;
#[doc = "Compare B Set"]
pub mod cmpbset;
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`]
module"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "CTRLC (rw) register accessor: Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlc`]
module"]
pub type CTRLC = crate::Reg<ctrlc::CTRLC_SPEC>;
#[doc = "Control C"]
pub mod ctrlc;
#[doc = "CTRLD (rw) register accessor: Control D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrld`]
module"]
pub type CTRLD = crate::Reg<ctrld::CTRLD_SPEC>;
#[doc = "Control D"]
pub mod ctrld;
#[doc = "CTRLE (rw) register accessor: Control E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrle::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrle::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrle`]
module"]
pub type CTRLE = crate::Reg<ctrle::CTRLE_SPEC>;
#[doc = "Control E"]
pub mod ctrle;
#[doc = "DBGCTRL (rw) register accessor: Debug Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "DITCTRL (rw) register accessor: Dither Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ditctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ditctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ditctrl`]
module"]
pub type DITCTRL = crate::Reg<ditctrl::DITCTRL_SPEC>;
#[doc = "Dither Control A"]
pub mod ditctrl;
#[doc = "DITVAL (rw) register accessor: Dither value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ditval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ditval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ditval`]
module"]
pub type DITVAL = crate::Reg<ditval::DITVAL_SPEC>;
#[doc = "Dither value"]
pub mod ditval;
#[doc = "DLYCTRL (rw) register accessor: Delay Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlyctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlyctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlyctrl`]
module"]
pub type DLYCTRL = crate::Reg<dlyctrl::DLYCTRL_SPEC>;
#[doc = "Delay Control"]
pub mod dlyctrl;
#[doc = "DLYVAL (rw) register accessor: Delay value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlyval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlyval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlyval`]
module"]
pub type DLYVAL = crate::Reg<dlyval::DLYVAL_SPEC>;
#[doc = "Delay value"]
pub mod dlyval;
#[doc = "EVCTRLA (rw) register accessor: EVCTRLA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrla`]
module"]
pub type EVCTRLA = crate::Reg<evctrla::EVCTRLA_SPEC>;
#[doc = "EVCTRLA"]
pub mod evctrla;
#[doc = "EVCTRLB (rw) register accessor: EVCTRLB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrlb`]
module"]
pub type EVCTRLB = crate::Reg<evctrlb::EVCTRLB_SPEC>;
#[doc = "EVCTRLB"]
pub mod evctrlb;
#[doc = "FAULTCTRL (rw) register accessor: Fault Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`faultctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`faultctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@faultctrl`]
module"]
pub type FAULTCTRL = crate::Reg<faultctrl::FAULTCTRL_SPEC>;
#[doc = "Fault Control"]
pub mod faultctrl;
#[doc = "INPUTCTRLA (rw) register accessor: Input Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inputctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inputctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inputctrla`]
module"]
pub type INPUTCTRLA = crate::Reg<inputctrla::INPUTCTRLA_SPEC>;
#[doc = "Input Control A"]
pub mod inputctrla;
#[doc = "INPUTCTRLB (rw) register accessor: Input Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inputctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inputctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inputctrlb`]
module"]
pub type INPUTCTRLB = crate::Reg<inputctrlb::INPUTCTRLB_SPEC>;
#[doc = "Input Control B"]
pub mod inputctrlb;
#[doc = "INTCTRL (rw) register accessor: Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intctrl`]
module"]
pub type INTCTRL = crate::Reg<intctrl::INTCTRL_SPEC>;
#[doc = "Interrupt Control"]
pub mod intctrl;
#[doc = "INTFLAGS (rw) register accessor: Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflags::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflags::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflags`]
module"]
pub type INTFLAGS = crate::Reg<intflags::INTFLAGS_SPEC>;
#[doc = "Interrupt Flags"]
pub mod intflags;
#[doc = "STATUS (rw) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
