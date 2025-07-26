#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrla: CTRLA,
    seqctrl0: SEQCTRL0,
    seqctrl1: SEQCTRL1,
    _reserved3: [u8; 0x02],
    intctrl0: INTCTRL0,
    _reserved4: [u8; 0x01],
    intflags: INTFLAGS,
    lut0ctrla: LUT0CTRLA,
    lut0ctrlb: LUT0CTRLB,
    lut0ctrlc: LUT0CTRLC,
    truth0: TRUTH0,
    lut1ctrla: LUT1CTRLA,
    lut1ctrlb: LUT1CTRLB,
    lut1ctrlc: LUT1CTRLC,
    truth1: TRUTH1,
    lut2ctrla: LUT2CTRLA,
    lut2ctrlb: LUT2CTRLB,
    lut2ctrlc: LUT2CTRLC,
    truth2: TRUTH2,
    lut3ctrla: LUT3CTRLA,
    lut3ctrlb: LUT3CTRLB,
    lut3ctrlc: LUT3CTRLC,
    truth3: TRUTH3,
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
    #[doc = "0x02 - Sequential Control 1"]
    #[inline(always)]
    pub const fn seqctrl1(&self) -> &SEQCTRL1 {
        &self.seqctrl1
    }
    #[doc = "0x05 - Interrupt Control 0"]
    #[inline(always)]
    pub const fn intctrl0(&self) -> &INTCTRL0 {
        &self.intctrl0
    }
    #[doc = "0x07 - Interrupt Flags"]
    #[inline(always)]
    pub const fn intflags(&self) -> &INTFLAGS {
        &self.intflags
    }
    #[doc = "0x08 - LUT 0 Control A"]
    #[inline(always)]
    pub const fn lut0ctrla(&self) -> &LUT0CTRLA {
        &self.lut0ctrla
    }
    #[doc = "0x09 - LUT 0 Control B"]
    #[inline(always)]
    pub const fn lut0ctrlb(&self) -> &LUT0CTRLB {
        &self.lut0ctrlb
    }
    #[doc = "0x0a - LUT 0 Control C"]
    #[inline(always)]
    pub const fn lut0ctrlc(&self) -> &LUT0CTRLC {
        &self.lut0ctrlc
    }
    #[doc = "0x0b - Truth 0"]
    #[inline(always)]
    pub const fn truth0(&self) -> &TRUTH0 {
        &self.truth0
    }
    #[doc = "0x0c - LUT 1 Control A"]
    #[inline(always)]
    pub const fn lut1ctrla(&self) -> &LUT1CTRLA {
        &self.lut1ctrla
    }
    #[doc = "0x0d - LUT 1 Control B"]
    #[inline(always)]
    pub const fn lut1ctrlb(&self) -> &LUT1CTRLB {
        &self.lut1ctrlb
    }
    #[doc = "0x0e - LUT 1 Control C"]
    #[inline(always)]
    pub const fn lut1ctrlc(&self) -> &LUT1CTRLC {
        &self.lut1ctrlc
    }
    #[doc = "0x0f - Truth 1"]
    #[inline(always)]
    pub const fn truth1(&self) -> &TRUTH1 {
        &self.truth1
    }
    #[doc = "0x10 - LUT 2 Control A"]
    #[inline(always)]
    pub const fn lut2ctrla(&self) -> &LUT2CTRLA {
        &self.lut2ctrla
    }
    #[doc = "0x11 - LUT 2 Control B"]
    #[inline(always)]
    pub const fn lut2ctrlb(&self) -> &LUT2CTRLB {
        &self.lut2ctrlb
    }
    #[doc = "0x12 - LUT 2 Control C"]
    #[inline(always)]
    pub const fn lut2ctrlc(&self) -> &LUT2CTRLC {
        &self.lut2ctrlc
    }
    #[doc = "0x13 - Truth 2"]
    #[inline(always)]
    pub const fn truth2(&self) -> &TRUTH2 {
        &self.truth2
    }
    #[doc = "0x14 - LUT 3 Control A"]
    #[inline(always)]
    pub const fn lut3ctrla(&self) -> &LUT3CTRLA {
        &self.lut3ctrla
    }
    #[doc = "0x15 - LUT 3 Control B"]
    #[inline(always)]
    pub const fn lut3ctrlb(&self) -> &LUT3CTRLB {
        &self.lut3ctrlb
    }
    #[doc = "0x16 - LUT 3 Control C"]
    #[inline(always)]
    pub const fn lut3ctrlc(&self) -> &LUT3CTRLC {
        &self.lut3ctrlc
    }
    #[doc = "0x17 - Truth 3"]
    #[inline(always)]
    pub const fn truth3(&self) -> &TRUTH3 {
        &self.truth3
    }
}
#[doc = "CTRLA (rw) register accessor: Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control Register A"]
pub mod ctrla;
#[doc = "INTCTRL0 (rw) register accessor: Interrupt Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intctrl0`]
module"]
pub type INTCTRL0 = crate::Reg<intctrl0::INTCTRL0_SPEC>;
#[doc = "Interrupt Control 0"]
pub mod intctrl0;
#[doc = "INTFLAGS (rw) register accessor: Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflags::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflags::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflags`]
module"]
pub type INTFLAGS = crate::Reg<intflags::INTFLAGS_SPEC>;
#[doc = "Interrupt Flags"]
pub mod intflags;
#[doc = "LUT0CTRLA (rw) register accessor: LUT 0 Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut0ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut0ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut0ctrla`]
module"]
pub type LUT0CTRLA = crate::Reg<lut0ctrla::LUT0CTRLA_SPEC>;
#[doc = "LUT 0 Control A"]
pub mod lut0ctrla;
#[doc = "LUT0CTRLB (rw) register accessor: LUT 0 Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut0ctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut0ctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut0ctrlb`]
module"]
pub type LUT0CTRLB = crate::Reg<lut0ctrlb::LUT0CTRLB_SPEC>;
#[doc = "LUT 0 Control B"]
pub mod lut0ctrlb;
#[doc = "LUT0CTRLC (rw) register accessor: LUT 0 Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut0ctrlc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut0ctrlc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut0ctrlc`]
module"]
pub type LUT0CTRLC = crate::Reg<lut0ctrlc::LUT0CTRLC_SPEC>;
#[doc = "LUT 0 Control C"]
pub mod lut0ctrlc;
#[doc = "LUT1CTRLA (rw) register accessor: LUT 1 Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut1ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut1ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut1ctrla`]
module"]
pub type LUT1CTRLA = crate::Reg<lut1ctrla::LUT1CTRLA_SPEC>;
#[doc = "LUT 1 Control A"]
pub mod lut1ctrla;
#[doc = "LUT1CTRLB (rw) register accessor: LUT 1 Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut1ctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut1ctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut1ctrlb`]
module"]
pub type LUT1CTRLB = crate::Reg<lut1ctrlb::LUT1CTRLB_SPEC>;
#[doc = "LUT 1 Control B"]
pub mod lut1ctrlb;
#[doc = "LUT1CTRLC (rw) register accessor: LUT 1 Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut1ctrlc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut1ctrlc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut1ctrlc`]
module"]
pub type LUT1CTRLC = crate::Reg<lut1ctrlc::LUT1CTRLC_SPEC>;
#[doc = "LUT 1 Control C"]
pub mod lut1ctrlc;
#[doc = "LUT2CTRLA (rw) register accessor: LUT 2 Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut2ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut2ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut2ctrla`]
module"]
pub type LUT2CTRLA = crate::Reg<lut2ctrla::LUT2CTRLA_SPEC>;
#[doc = "LUT 2 Control A"]
pub mod lut2ctrla;
#[doc = "LUT2CTRLB (rw) register accessor: LUT 2 Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut2ctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut2ctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut2ctrlb`]
module"]
pub type LUT2CTRLB = crate::Reg<lut2ctrlb::LUT2CTRLB_SPEC>;
#[doc = "LUT 2 Control B"]
pub mod lut2ctrlb;
#[doc = "LUT2CTRLC (rw) register accessor: LUT 2 Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut2ctrlc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut2ctrlc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut2ctrlc`]
module"]
pub type LUT2CTRLC = crate::Reg<lut2ctrlc::LUT2CTRLC_SPEC>;
#[doc = "LUT 2 Control C"]
pub mod lut2ctrlc;
#[doc = "LUT3CTRLA (rw) register accessor: LUT 3 Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut3ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut3ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut3ctrla`]
module"]
pub type LUT3CTRLA = crate::Reg<lut3ctrla::LUT3CTRLA_SPEC>;
#[doc = "LUT 3 Control A"]
pub mod lut3ctrla;
#[doc = "LUT3CTRLB (rw) register accessor: LUT 3 Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut3ctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut3ctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut3ctrlb`]
module"]
pub type LUT3CTRLB = crate::Reg<lut3ctrlb::LUT3CTRLB_SPEC>;
#[doc = "LUT 3 Control B"]
pub mod lut3ctrlb;
#[doc = "LUT3CTRLC (rw) register accessor: LUT 3 Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut3ctrlc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut3ctrlc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut3ctrlc`]
module"]
pub type LUT3CTRLC = crate::Reg<lut3ctrlc::LUT3CTRLC_SPEC>;
#[doc = "LUT 3 Control C"]
pub mod lut3ctrlc;
#[doc = "SEQCTRL0 (rw) register accessor: Sequential Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqctrl0`]
module"]
pub type SEQCTRL0 = crate::Reg<seqctrl0::SEQCTRL0_SPEC>;
#[doc = "Sequential Control 0"]
pub mod seqctrl0;
#[doc = "SEQCTRL1 (rw) register accessor: Sequential Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqctrl1`]
module"]
pub type SEQCTRL1 = crate::Reg<seqctrl1::SEQCTRL1_SPEC>;
#[doc = "Sequential Control 1"]
pub mod seqctrl1;
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
#[doc = "TRUTH2 (rw) register accessor: Truth 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`truth2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`truth2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@truth2`]
module"]
pub type TRUTH2 = crate::Reg<truth2::TRUTH2_SPEC>;
#[doc = "Truth 2"]
pub mod truth2;
#[doc = "TRUTH3 (rw) register accessor: Truth 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`truth3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`truth3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@truth3`]
module"]
pub type TRUTH3 = crate::Reg<truth3::TRUTH3_SPEC>;
#[doc = "Truth 3"]
pub mod truth3;
