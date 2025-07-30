#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrla: CTRLA,
    seqctrl: [SEQCTRL; 2],
    _reserved2: [u8; 0x02],
    intctrl0: INTCTRL0,
    _reserved3: [u8; 0x01],
    intflags: INTFLAGS,
    lut: [LUT; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &CTRLA {
        &self.ctrla
    }
    #[doc = "0x01 - Sequential Control %s"]
    #[inline(always)]
    pub const fn seqctrl(&self, n: usize) -> &SEQCTRL {
        &self.seqctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x01 - Sequential Control %s"]
    #[inline(always)]
    pub fn seqctrl_iter(&self) -> impl Iterator<Item = &SEQCTRL> {
        self.seqctrl.iter()
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
    #[doc = "0x08..0x18 - CCL LUT configuration cluster"]
    #[inline(always)]
    pub const fn lut(&self, n: usize) -> &LUT {
        &self.lut[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x18 - CCL LUT configuration cluster"]
    #[inline(always)]
    pub fn lut_iter(&self) -> impl Iterator<Item = &LUT> {
        self.lut.iter()
    }
}
#[doc = "CCL LUT configuration cluster"]
pub use self::lut::LUT;
#[doc = r"Cluster"]
#[doc = "CCL LUT configuration cluster"]
pub mod lut;
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
#[doc = "SEQCTRL (rw) register accessor: Sequential Control %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqctrl`]
module"]
pub type SEQCTRL = crate::Reg<seqctrl::SEQCTRL_SPEC>;
#[doc = "Sequential Control %s"]
pub mod seqctrl;
