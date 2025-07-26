#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    sccr0: SCCR0,
    sccr1: SCCR1,
    scsr: SCSR,
    scirqm: SCIRQM,
    scirqs: SCIRQS,
    sccntll: SCCNTLL,
    sccntlh: SCCNTLH,
    sccnthl: SCCNTHL,
    sccnthh: SCCNTHH,
    scbtsrll: SCBTSRLL,
    scbtsrlh: SCBTSRLH,
    scbtsrhl: SCBTSRHL,
    scbtsrhh: SCBTSRHH,
    sctsrll: SCTSRLL,
    sctsrlh: SCTSRLH,
    sctsrhl: SCTSRHL,
    sctsrhh: SCTSRHH,
    scocr3ll: SCOCR3LL,
    scocr3lh: SCOCR3LH,
    scocr3hl: SCOCR3HL,
    scocr3hh: SCOCR3HH,
    scocr2ll: SCOCR2LL,
    scocr2lh: SCOCR2LH,
    scocr2hl: SCOCR2HL,
    scocr2hh: SCOCR2HH,
    scocr1ll: SCOCR1LL,
    scocr1lh: SCOCR1LH,
    scocr1hl: SCOCR1HL,
    scocr1hh: SCOCR1HH,
}
impl RegisterBlock {
    #[doc = "0x00 - Symbol Counter Control Register 0"]
    #[inline(always)]
    pub const fn sccr0(&self) -> &SCCR0 {
        &self.sccr0
    }
    #[doc = "0x01 - Symbol Counter Control Register 1"]
    #[inline(always)]
    pub const fn sccr1(&self) -> &SCCR1 {
        &self.sccr1
    }
    #[doc = "0x02 - Symbol Counter Status Register"]
    #[inline(always)]
    pub const fn scsr(&self) -> &SCSR {
        &self.scsr
    }
    #[doc = "0x03 - Symbol Counter Interrupt Mask Register"]
    #[inline(always)]
    pub const fn scirqm(&self) -> &SCIRQM {
        &self.scirqm
    }
    #[doc = "0x04 - Symbol Counter Interrupt Status Register"]
    #[inline(always)]
    pub const fn scirqs(&self) -> &SCIRQS {
        &self.scirqs
    }
    #[doc = "0x05 - Symbol Counter Register LL-Byte"]
    #[inline(always)]
    pub const fn sccntll(&self) -> &SCCNTLL {
        &self.sccntll
    }
    #[doc = "0x06 - Symbol Counter Register LH-Byte"]
    #[inline(always)]
    pub const fn sccntlh(&self) -> &SCCNTLH {
        &self.sccntlh
    }
    #[doc = "0x07 - Symbol Counter Register HL-Byte"]
    #[inline(always)]
    pub const fn sccnthl(&self) -> &SCCNTHL {
        &self.sccnthl
    }
    #[doc = "0x08 - Symbol Counter Register HH-Byte"]
    #[inline(always)]
    pub const fn sccnthh(&self) -> &SCCNTHH {
        &self.sccnthh
    }
    #[doc = "0x09 - Symbol Counter Beacon Timestamp Register LL-Byte"]
    #[inline(always)]
    pub const fn scbtsrll(&self) -> &SCBTSRLL {
        &self.scbtsrll
    }
    #[doc = "0x0a - Symbol Counter Beacon Timestamp Register LH-Byte"]
    #[inline(always)]
    pub const fn scbtsrlh(&self) -> &SCBTSRLH {
        &self.scbtsrlh
    }
    #[doc = "0x0b - Symbol Counter Beacon Timestamp Register HL-Byte"]
    #[inline(always)]
    pub const fn scbtsrhl(&self) -> &SCBTSRHL {
        &self.scbtsrhl
    }
    #[doc = "0x0c - Symbol Counter Beacon Timestamp Register HH-Byte"]
    #[inline(always)]
    pub const fn scbtsrhh(&self) -> &SCBTSRHH {
        &self.scbtsrhh
    }
    #[doc = "0x0d - Symbol Counter Frame Timestamp Register LL-Byte"]
    #[inline(always)]
    pub const fn sctsrll(&self) -> &SCTSRLL {
        &self.sctsrll
    }
    #[doc = "0x0e - Symbol Counter Frame Timestamp Register LH-Byte"]
    #[inline(always)]
    pub const fn sctsrlh(&self) -> &SCTSRLH {
        &self.sctsrlh
    }
    #[doc = "0x0f - Symbol Counter Frame Timestamp Register HL-Byte"]
    #[inline(always)]
    pub const fn sctsrhl(&self) -> &SCTSRHL {
        &self.sctsrhl
    }
    #[doc = "0x10 - Symbol Counter Frame Timestamp Register HH-Byte"]
    #[inline(always)]
    pub const fn sctsrhh(&self) -> &SCTSRHH {
        &self.sctsrhh
    }
    #[doc = "0x11 - Symbol Counter Output Compare Register 3 LL-Byte"]
    #[inline(always)]
    pub const fn scocr3ll(&self) -> &SCOCR3LL {
        &self.scocr3ll
    }
    #[doc = "0x12 - Symbol Counter Output Compare Register 3 LH-Byte"]
    #[inline(always)]
    pub const fn scocr3lh(&self) -> &SCOCR3LH {
        &self.scocr3lh
    }
    #[doc = "0x13 - Symbol Counter Output Compare Register 3 HL-Byte"]
    #[inline(always)]
    pub const fn scocr3hl(&self) -> &SCOCR3HL {
        &self.scocr3hl
    }
    #[doc = "0x14 - Symbol Counter Output Compare Register 3 HH-Byte"]
    #[inline(always)]
    pub const fn scocr3hh(&self) -> &SCOCR3HH {
        &self.scocr3hh
    }
    #[doc = "0x15 - Symbol Counter Output Compare Register 2 LL-Byte"]
    #[inline(always)]
    pub const fn scocr2ll(&self) -> &SCOCR2LL {
        &self.scocr2ll
    }
    #[doc = "0x16 - Symbol Counter Output Compare Register 2 LH-Byte"]
    #[inline(always)]
    pub const fn scocr2lh(&self) -> &SCOCR2LH {
        &self.scocr2lh
    }
    #[doc = "0x17 - Symbol Counter Output Compare Register 2 HL-Byte"]
    #[inline(always)]
    pub const fn scocr2hl(&self) -> &SCOCR2HL {
        &self.scocr2hl
    }
    #[doc = "0x18 - Symbol Counter Output Compare Register 2 HH-Byte"]
    #[inline(always)]
    pub const fn scocr2hh(&self) -> &SCOCR2HH {
        &self.scocr2hh
    }
    #[doc = "0x19 - Symbol Counter Output Compare Register 1 LL-Byte"]
    #[inline(always)]
    pub const fn scocr1ll(&self) -> &SCOCR1LL {
        &self.scocr1ll
    }
    #[doc = "0x1a - Symbol Counter Output Compare Register 1 LH-Byte"]
    #[inline(always)]
    pub const fn scocr1lh(&self) -> &SCOCR1LH {
        &self.scocr1lh
    }
    #[doc = "0x1b - Symbol Counter Output Compare Register 1 HL-Byte"]
    #[inline(always)]
    pub const fn scocr1hl(&self) -> &SCOCR1HL {
        &self.scocr1hl
    }
    #[doc = "0x1c - Symbol Counter Output Compare Register 1 HH-Byte"]
    #[inline(always)]
    pub const fn scocr1hh(&self) -> &SCOCR1HH {
        &self.scocr1hh
    }
}
#[doc = "SCBTSRHH (rw) register accessor: Symbol Counter Beacon Timestamp Register HH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scbtsrhh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scbtsrhh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scbtsrhh`]
module"]
pub type SCBTSRHH = crate::Reg<scbtsrhh::SCBTSRHH_SPEC>;
#[doc = "Symbol Counter Beacon Timestamp Register HH-Byte"]
pub mod scbtsrhh;
#[doc = "SCBTSRHL (rw) register accessor: Symbol Counter Beacon Timestamp Register HL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scbtsrhl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scbtsrhl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scbtsrhl`]
module"]
pub type SCBTSRHL = crate::Reg<scbtsrhl::SCBTSRHL_SPEC>;
#[doc = "Symbol Counter Beacon Timestamp Register HL-Byte"]
pub mod scbtsrhl;
#[doc = "SCBTSRLH (rw) register accessor: Symbol Counter Beacon Timestamp Register LH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scbtsrlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scbtsrlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scbtsrlh`]
module"]
pub type SCBTSRLH = crate::Reg<scbtsrlh::SCBTSRLH_SPEC>;
#[doc = "Symbol Counter Beacon Timestamp Register LH-Byte"]
pub mod scbtsrlh;
#[doc = "SCBTSRLL (rw) register accessor: Symbol Counter Beacon Timestamp Register LL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scbtsrll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scbtsrll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scbtsrll`]
module"]
pub type SCBTSRLL = crate::Reg<scbtsrll::SCBTSRLL_SPEC>;
#[doc = "Symbol Counter Beacon Timestamp Register LL-Byte"]
pub mod scbtsrll;
#[doc = "SCCNTHH (rw) register accessor: Symbol Counter Register HH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sccnthh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccnthh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccnthh`]
module"]
pub type SCCNTHH = crate::Reg<sccnthh::SCCNTHH_SPEC>;
#[doc = "Symbol Counter Register HH-Byte"]
pub mod sccnthh;
#[doc = "SCCNTHL (rw) register accessor: Symbol Counter Register HL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sccnthl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccnthl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccnthl`]
module"]
pub type SCCNTHL = crate::Reg<sccnthl::SCCNTHL_SPEC>;
#[doc = "Symbol Counter Register HL-Byte"]
pub mod sccnthl;
#[doc = "SCCNTLH (rw) register accessor: Symbol Counter Register LH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sccntlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccntlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccntlh`]
module"]
pub type SCCNTLH = crate::Reg<sccntlh::SCCNTLH_SPEC>;
#[doc = "Symbol Counter Register LH-Byte"]
pub mod sccntlh;
#[doc = "SCCNTLL (rw) register accessor: Symbol Counter Register LL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sccntll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccntll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccntll`]
module"]
pub type SCCNTLL = crate::Reg<sccntll::SCCNTLL_SPEC>;
#[doc = "Symbol Counter Register LL-Byte"]
pub mod sccntll;
#[doc = "SCCR0 (rw) register accessor: Symbol Counter Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sccr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccr0`]
module"]
pub type SCCR0 = crate::Reg<sccr0::SCCR0_SPEC>;
#[doc = "Symbol Counter Control Register 0"]
pub mod sccr0;
#[doc = "SCCR1 (rw) register accessor: Symbol Counter Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccr1`]
module"]
pub type SCCR1 = crate::Reg<sccr1::SCCR1_SPEC>;
#[doc = "Symbol Counter Control Register 1"]
pub mod sccr1;
#[doc = "SCIRQM (rw) register accessor: Symbol Counter Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scirqm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scirqm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scirqm`]
module"]
pub type SCIRQM = crate::Reg<scirqm::SCIRQM_SPEC>;
#[doc = "Symbol Counter Interrupt Mask Register"]
pub mod scirqm;
#[doc = "SCIRQS (rw) register accessor: Symbol Counter Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scirqs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scirqs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scirqs`]
module"]
pub type SCIRQS = crate::Reg<scirqs::SCIRQS_SPEC>;
#[doc = "Symbol Counter Interrupt Status Register"]
pub mod scirqs;
#[doc = "SCOCR1HH (rw) register accessor: Symbol Counter Output Compare Register 1 HH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr1hh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr1hh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scocr1hh`]
module"]
pub type SCOCR1HH = crate::Reg<scocr1hh::SCOCR1HH_SPEC>;
#[doc = "Symbol Counter Output Compare Register 1 HH-Byte"]
pub mod scocr1hh;
#[doc = "SCOCR1HL (rw) register accessor: Symbol Counter Output Compare Register 1 HL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr1hl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr1hl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scocr1hl`]
module"]
pub type SCOCR1HL = crate::Reg<scocr1hl::SCOCR1HL_SPEC>;
#[doc = "Symbol Counter Output Compare Register 1 HL-Byte"]
pub mod scocr1hl;
#[doc = "SCOCR1LH (rw) register accessor: Symbol Counter Output Compare Register 1 LH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr1lh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr1lh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scocr1lh`]
module"]
pub type SCOCR1LH = crate::Reg<scocr1lh::SCOCR1LH_SPEC>;
#[doc = "Symbol Counter Output Compare Register 1 LH-Byte"]
pub mod scocr1lh;
#[doc = "SCOCR1LL (rw) register accessor: Symbol Counter Output Compare Register 1 LL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr1ll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr1ll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scocr1ll`]
module"]
pub type SCOCR1LL = crate::Reg<scocr1ll::SCOCR1LL_SPEC>;
#[doc = "Symbol Counter Output Compare Register 1 LL-Byte"]
pub mod scocr1ll;
#[doc = "SCOCR2HH (rw) register accessor: Symbol Counter Output Compare Register 2 HH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr2hh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr2hh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scocr2hh`]
module"]
pub type SCOCR2HH = crate::Reg<scocr2hh::SCOCR2HH_SPEC>;
#[doc = "Symbol Counter Output Compare Register 2 HH-Byte"]
pub mod scocr2hh;
#[doc = "SCOCR2HL (rw) register accessor: Symbol Counter Output Compare Register 2 HL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr2hl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr2hl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scocr2hl`]
module"]
pub type SCOCR2HL = crate::Reg<scocr2hl::SCOCR2HL_SPEC>;
#[doc = "Symbol Counter Output Compare Register 2 HL-Byte"]
pub mod scocr2hl;
#[doc = "SCOCR2LH (rw) register accessor: Symbol Counter Output Compare Register 2 LH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr2lh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr2lh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scocr2lh`]
module"]
pub type SCOCR2LH = crate::Reg<scocr2lh::SCOCR2LH_SPEC>;
#[doc = "Symbol Counter Output Compare Register 2 LH-Byte"]
pub mod scocr2lh;
#[doc = "SCOCR2LL (rw) register accessor: Symbol Counter Output Compare Register 2 LL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr2ll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr2ll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scocr2ll`]
module"]
pub type SCOCR2LL = crate::Reg<scocr2ll::SCOCR2LL_SPEC>;
#[doc = "Symbol Counter Output Compare Register 2 LL-Byte"]
pub mod scocr2ll;
#[doc = "SCOCR3HH (rw) register accessor: Symbol Counter Output Compare Register 3 HH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr3hh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr3hh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scocr3hh`]
module"]
pub type SCOCR3HH = crate::Reg<scocr3hh::SCOCR3HH_SPEC>;
#[doc = "Symbol Counter Output Compare Register 3 HH-Byte"]
pub mod scocr3hh;
#[doc = "SCOCR3HL (rw) register accessor: Symbol Counter Output Compare Register 3 HL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr3hl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr3hl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scocr3hl`]
module"]
pub type SCOCR3HL = crate::Reg<scocr3hl::SCOCR3HL_SPEC>;
#[doc = "Symbol Counter Output Compare Register 3 HL-Byte"]
pub mod scocr3hl;
#[doc = "SCOCR3LH (rw) register accessor: Symbol Counter Output Compare Register 3 LH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr3lh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr3lh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scocr3lh`]
module"]
pub type SCOCR3LH = crate::Reg<scocr3lh::SCOCR3LH_SPEC>;
#[doc = "Symbol Counter Output Compare Register 3 LH-Byte"]
pub mod scocr3lh;
#[doc = "SCOCR3LL (rw) register accessor: Symbol Counter Output Compare Register 3 LL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr3ll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr3ll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scocr3ll`]
module"]
pub type SCOCR3LL = crate::Reg<scocr3ll::SCOCR3LL_SPEC>;
#[doc = "Symbol Counter Output Compare Register 3 LL-Byte"]
pub mod scocr3ll;
#[doc = "SCSR (rw) register accessor: Symbol Counter Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsr`]
module"]
pub type SCSR = crate::Reg<scsr::SCSR_SPEC>;
#[doc = "Symbol Counter Status Register"]
pub mod scsr;
#[doc = "SCTSRHH (rw) register accessor: Symbol Counter Frame Timestamp Register HH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctsrhh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctsrhh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctsrhh`]
module"]
pub type SCTSRHH = crate::Reg<sctsrhh::SCTSRHH_SPEC>;
#[doc = "Symbol Counter Frame Timestamp Register HH-Byte"]
pub mod sctsrhh;
#[doc = "SCTSRHL (rw) register accessor: Symbol Counter Frame Timestamp Register HL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctsrhl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctsrhl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctsrhl`]
module"]
pub type SCTSRHL = crate::Reg<sctsrhl::SCTSRHL_SPEC>;
#[doc = "Symbol Counter Frame Timestamp Register HL-Byte"]
pub mod sctsrhl;
#[doc = "SCTSRLH (rw) register accessor: Symbol Counter Frame Timestamp Register LH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctsrlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctsrlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctsrlh`]
module"]
pub type SCTSRLH = crate::Reg<sctsrlh::SCTSRLH_SPEC>;
#[doc = "Symbol Counter Frame Timestamp Register LH-Byte"]
pub mod sctsrlh;
#[doc = "SCTSRLL (rw) register accessor: Symbol Counter Frame Timestamp Register LL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctsrll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctsrll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctsrll`]
module"]
pub type SCTSRLL = crate::Reg<sctsrll::SCTSRLL_SPEC>;
#[doc = "Symbol Counter Frame Timestamp Register LL-Byte"]
pub mod sctsrll;
