#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrla: CTRLA,
    status: STATUS,
    intctrl: INTCTRL,
    intflags: INTFLAGS,
    temp: TEMP,
    dbgctrl: DBGCTRL,
    _reserved6: [u8; 0x01],
    clksel: CLKSEL,
    cnt: CNT,
    per: PER,
    cmp: CMP,
    _reserved10: [u8; 0x02],
    pitctrla: PITCTRLA,
    pitstatus: PITSTATUS,
    pitintctrl: PITINTCTRL,
    pitintflags: PITINTFLAGS,
    _reserved14: [u8; 0x01],
    pitdbgctrl: PITDBGCTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &CTRLA {
        &self.ctrla
    }
    #[doc = "0x01 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x02 - Interrupt Control"]
    #[inline(always)]
    pub const fn intctrl(&self) -> &INTCTRL {
        &self.intctrl
    }
    #[doc = "0x03 - Interrupt Flags"]
    #[inline(always)]
    pub const fn intflags(&self) -> &INTFLAGS {
        &self.intflags
    }
    #[doc = "0x04 - Temporary"]
    #[inline(always)]
    pub const fn temp(&self) -> &TEMP {
        &self.temp
    }
    #[doc = "0x05 - Debug control"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &DBGCTRL {
        &self.dbgctrl
    }
    #[doc = "0x07 - Clock Select"]
    #[inline(always)]
    pub const fn clksel(&self) -> &CLKSEL {
        &self.clksel
    }
    #[doc = "0x08 - Counter"]
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    #[doc = "0x0a - Period"]
    #[inline(always)]
    pub const fn per(&self) -> &PER {
        &self.per
    }
    #[doc = "0x0c - Compare"]
    #[inline(always)]
    pub const fn cmp(&self) -> &CMP {
        &self.cmp
    }
    #[doc = "0x10 - PIT Control A"]
    #[inline(always)]
    pub const fn pitctrla(&self) -> &PITCTRLA {
        &self.pitctrla
    }
    #[doc = "0x11 - PIT Status"]
    #[inline(always)]
    pub const fn pitstatus(&self) -> &PITSTATUS {
        &self.pitstatus
    }
    #[doc = "0x12 - PIT Interrupt Control"]
    #[inline(always)]
    pub const fn pitintctrl(&self) -> &PITINTCTRL {
        &self.pitintctrl
    }
    #[doc = "0x13 - PIT Interrupt Flags"]
    #[inline(always)]
    pub const fn pitintflags(&self) -> &PITINTFLAGS {
        &self.pitintflags
    }
    #[doc = "0x15 - PIT Debug control"]
    #[inline(always)]
    pub const fn pitdbgctrl(&self) -> &PITDBGCTRL {
        &self.pitdbgctrl
    }
}
#[doc = "CLKSEL (rw) register accessor: Clock Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel`]
module"]
pub type CLKSEL = crate::Reg<clksel::CLKSEL_SPEC>;
#[doc = "Clock Select"]
pub mod clksel;
#[doc = "CMP (rw) register accessor: Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp`]
module"]
pub type CMP = crate::Reg<cmp::CMP_SPEC>;
#[doc = "Compare"]
pub mod cmp;
#[doc = "CNT (rw) register accessor: Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter"]
pub mod cnt;
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "DBGCTRL (rw) register accessor: Debug control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug control"]
pub mod dbgctrl;
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
#[doc = "PER (rw) register accessor: Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`per::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`per::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per`]
module"]
pub type PER = crate::Reg<per::PER_SPEC>;
#[doc = "Period"]
pub mod per;
#[doc = "PITCTRLA (rw) register accessor: PIT Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pitctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pitctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pitctrla`]
module"]
pub type PITCTRLA = crate::Reg<pitctrla::PITCTRLA_SPEC>;
#[doc = "PIT Control A"]
pub mod pitctrla;
#[doc = "PITDBGCTRL (rw) register accessor: PIT Debug control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pitdbgctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pitdbgctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pitdbgctrl`]
module"]
pub type PITDBGCTRL = crate::Reg<pitdbgctrl::PITDBGCTRL_SPEC>;
#[doc = "PIT Debug control"]
pub mod pitdbgctrl;
#[doc = "PITINTCTRL (rw) register accessor: PIT Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pitintctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pitintctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pitintctrl`]
module"]
pub type PITINTCTRL = crate::Reg<pitintctrl::PITINTCTRL_SPEC>;
#[doc = "PIT Interrupt Control"]
pub mod pitintctrl;
#[doc = "PITINTFLAGS (rw) register accessor: PIT Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pitintflags::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pitintflags::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pitintflags`]
module"]
pub type PITINTFLAGS = crate::Reg<pitintflags::PITINTFLAGS_SPEC>;
#[doc = "PIT Interrupt Flags"]
pub mod pitintflags;
#[doc = "PITSTATUS (r) register accessor: PIT Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pitstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pitstatus`]
module"]
pub type PITSTATUS = crate::Reg<pitstatus::PITSTATUS_SPEC>;
#[doc = "PIT Status"]
pub mod pitstatus;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "TEMP (rw) register accessor: Temporary\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`temp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`temp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@temp`]
module"]
pub type TEMP = crate::Reg<temp::TEMP_SPEC>;
#[doc = "Temporary"]
pub mod temp;
