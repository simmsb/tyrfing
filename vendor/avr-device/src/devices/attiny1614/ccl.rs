#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrla: CTRLA,
    seqctrl0: SEQCTRL0,
    _reserved2: [u8; 0x03],
    lut0ctrla: LUT0CTRLA,
    lut0ctrlb: LUT0CTRLB,
    lut0ctrlc: LUT0CTRLC,
    truth0: TRUTH0,
    lut1ctrla: LUT1CTRLA,
    lut1ctrlb: LUT1CTRLB,
    lut1ctrlc: LUT1CTRLC,
    truth1: TRUTH1,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &CTRLA {
        &self.ctrla
    }
    #[doc = "0x01 - Sequential Control 0"]
    #[inline(always)]
    pub const fn seqctrl0(&self) -> &SEQCTRL0 {
        &self.seqctrl0
    }
    #[doc = "0x05 - LUT Control 0 A"]
    #[inline(always)]
    pub const fn lut0ctrla(&self) -> &LUT0CTRLA {
        &self.lut0ctrla
    }
    #[doc = "0x06 - LUT Control 0 B"]
    #[inline(always)]
    pub const fn lut0ctrlb(&self) -> &LUT0CTRLB {
        &self.lut0ctrlb
    }
    #[doc = "0x07 - LUT Control 0 C"]
    #[inline(always)]
    pub const fn lut0ctrlc(&self) -> &LUT0CTRLC {
        &self.lut0ctrlc
    }
    #[doc = "0x08 - Truth 0"]
    #[inline(always)]
    pub const fn truth0(&self) -> &TRUTH0 {
        &self.truth0
    }
    #[doc = "0x09 - LUT Control 1 A"]
    #[inline(always)]
    pub const fn lut1ctrla(&self) -> &LUT1CTRLA {
        &self.lut1ctrla
    }
    #[doc = "0x0a - LUT Control 1 B"]
    #[inline(always)]
    pub const fn lut1ctrlb(&self) -> &LUT1CTRLB {
        &self.lut1ctrlb
    }
    #[doc = "0x0b - LUT Control 1 C"]
    #[inline(always)]
    pub const fn lut1ctrlc(&self) -> &LUT1CTRLC {
        &self.lut1ctrlc
    }
    #[doc = "0x0c - Truth 1"]
    #[inline(always)]
    pub const fn truth1(&self) -> &TRUTH1 {
        &self.truth1
    }
}
#[doc = "CTRLA (rw) register accessor: Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control Register A"]
pub mod ctrla;
#[doc = "LUT0CTRLA (rw) register accessor: LUT Control 0 A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut0ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut0ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut0ctrla`]
module"]
pub type LUT0CTRLA = crate::Reg<lut0ctrla::LUT0CTRLA_SPEC>;
#[doc = "LUT Control 0 A"]
pub mod lut0ctrla;
#[doc = "LUT0CTRLB (rw) register accessor: LUT Control 0 B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut0ctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut0ctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut0ctrlb`]
module"]
pub type LUT0CTRLB = crate::Reg<lut0ctrlb::LUT0CTRLB_SPEC>;
#[doc = "LUT Control 0 B"]
pub mod lut0ctrlb;
#[doc = "LUT0CTRLC (rw) register accessor: LUT Control 0 C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut0ctrlc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut0ctrlc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut0ctrlc`]
module"]
pub type LUT0CTRLC = crate::Reg<lut0ctrlc::LUT0CTRLC_SPEC>;
#[doc = "LUT Control 0 C"]
pub mod lut0ctrlc;
#[doc = "LUT1CTRLA (rw) register accessor: LUT Control 1 A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut1ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut1ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut1ctrla`]
module"]
pub type LUT1CTRLA = crate::Reg<lut1ctrla::LUT1CTRLA_SPEC>;
#[doc = "LUT Control 1 A"]
pub mod lut1ctrla;
#[doc = "LUT1CTRLB (rw) register accessor: LUT Control 1 B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut1ctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut1ctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut1ctrlb`]
module"]
pub type LUT1CTRLB = crate::Reg<lut1ctrlb::LUT1CTRLB_SPEC>;
#[doc = "LUT Control 1 B"]
pub mod lut1ctrlb;
#[doc = "LUT1CTRLC (rw) register accessor: LUT Control 1 C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut1ctrlc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut1ctrlc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut1ctrlc`]
module"]
pub type LUT1CTRLC = crate::Reg<lut1ctrlc::LUT1CTRLC_SPEC>;
#[doc = "LUT Control 1 C"]
pub mod lut1ctrlc;
#[doc = "SEQCTRL0 (rw) register accessor: Sequential Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqctrl0`]
module"]
pub type SEQCTRL0 = crate::Reg<seqctrl0::SEQCTRL0_SPEC>;
#[doc = "Sequential Control 0"]
pub mod seqctrl0;
#[doc = "TRUTH0 (rw) register accessor: Truth 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`truth0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`truth0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@truth0`]
module"]
pub type TRUTH0 = crate::Reg<truth0::TRUTH0_SPEC>;
#[doc = "Truth 0"]
pub mod truth0;
#[doc = "TRUTH1 (rw) register accessor: Truth 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`truth1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`truth1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@truth1`]
module"]
pub type TRUTH1 = crate::Reg<truth1::TRUTH1_SPEC>;
#[doc = "Truth 1"]
pub mod truth1;
