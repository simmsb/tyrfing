#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrla: CTRLA,
    ctrlb: CTRLB,
    ctrlc: CTRLC,
    ctrld: CTRLD,
    ctrle: CTRLE,
    sampctrl: SAMPCTRL,
    muxpos: MUXPOS,
    _reserved7: [u8; 0x01],
    command: COMMAND,
    evctrl: EVCTRL,
    intctrl: INTCTRL,
    intflags: INTFLAGS,
    dbgctrl: DBGCTRL,
    temp: TEMP,
    _reserved13: [u8; 0x02],
    res: RES,
    winlt: WINLT,
    winht: WINHT,
    calib: CALIB,
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
    #[doc = "0x05 - Sample Control"]
    #[inline(always)]
    pub const fn sampctrl(&self) -> &SAMPCTRL {
        &self.sampctrl
    }
    #[doc = "0x06 - Positive mux input"]
    #[inline(always)]
    pub const fn muxpos(&self) -> &MUXPOS {
        &self.muxpos
    }
    #[doc = "0x08 - Command"]
    #[inline(always)]
    pub const fn command(&self) -> &COMMAND {
        &self.command
    }
    #[doc = "0x09 - Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &EVCTRL {
        &self.evctrl
    }
    #[doc = "0x0a - Interrupt Control"]
    #[inline(always)]
    pub const fn intctrl(&self) -> &INTCTRL {
        &self.intctrl
    }
    #[doc = "0x0b - Interrupt Flags"]
    #[inline(always)]
    pub const fn intflags(&self) -> &INTFLAGS {
        &self.intflags
    }
    #[doc = "0x0c - Debug Control"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &DBGCTRL {
        &self.dbgctrl
    }
    #[doc = "0x0d - Temporary Data"]
    #[inline(always)]
    pub const fn temp(&self) -> &TEMP {
        &self.temp
    }
    #[doc = "0x10 - ADC Accumulator Result"]
    #[inline(always)]
    pub const fn res(&self) -> &RES {
        &self.res
    }
    #[doc = "0x12 - Window comparator low threshold"]
    #[inline(always)]
    pub const fn winlt(&self) -> &WINLT {
        &self.winlt
    }
    #[doc = "0x14 - Window comparator high threshold"]
    #[inline(always)]
    pub const fn winht(&self) -> &WINHT {
        &self.winht
    }
    #[doc = "0x16 - Calibration"]
    #[inline(always)]
    pub const fn calib(&self) -> &CALIB {
        &self.calib
    }
}
#[doc = "CALIB (rw) register accessor: Calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calib::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calib::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calib`]
module"]
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
#[doc = "Calibration"]
pub mod calib;
#[doc = "COMMAND (rw) register accessor: Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`command::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command`]
module"]
pub type COMMAND = crate::Reg<command::COMMAND_SPEC>;
#[doc = "Command"]
pub mod command;
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
#[doc = "EVCTRL (rw) register accessor: Event Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`]
module"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
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
#[doc = "MUXPOS (rw) register accessor: Positive mux input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxpos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxpos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxpos`]
module"]
pub type MUXPOS = crate::Reg<muxpos::MUXPOS_SPEC>;
#[doc = "Positive mux input"]
pub mod muxpos;
#[doc = "RES (r) register accessor: ADC Accumulator Result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`res::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res`]
module"]
pub type RES = crate::Reg<res::RES_SPEC>;
#[doc = "ADC Accumulator Result"]
pub mod res;
#[doc = "SAMPCTRL (rw) register accessor: Sample Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sampctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sampctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sampctrl`]
module"]
pub type SAMPCTRL = crate::Reg<sampctrl::SAMPCTRL_SPEC>;
#[doc = "Sample Control"]
pub mod sampctrl;
#[doc = "TEMP (rw) register accessor: Temporary Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`temp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`temp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@temp`]
module"]
pub type TEMP = crate::Reg<temp::TEMP_SPEC>;
#[doc = "Temporary Data"]
pub mod temp;
#[doc = "WINHT (rw) register accessor: Window comparator high threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`winht::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`winht::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winht`]
module"]
pub type WINHT = crate::Reg<winht::WINHT_SPEC>;
#[doc = "Window comparator high threshold"]
pub mod winht;
#[doc = "WINLT (rw) register accessor: Window comparator low threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`winlt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`winlt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winlt`]
module"]
pub type WINLT = crate::Reg<winlt::WINLT_SPEC>;
#[doc = "Window comparator low threshold"]
pub mod winlt;
