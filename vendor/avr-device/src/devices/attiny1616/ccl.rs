#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrla: CTRLA,
    seqctrl: [SEQCTRL; 1],
    _reserved2: [u8; 0x03],
    lut: [LUT; 2],
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
    #[doc = "0x05..0x0d - CCL LUT configuration cluster"]
    #[inline(always)]
    pub const fn lut(&self, n: usize) -> &LUT {
        &self.lut[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x05..0x0d - CCL LUT configuration cluster"]
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
#[doc = "SEQCTRL (rw) register accessor: Sequential Control %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqctrl`]
module"]
pub type SEQCTRL = crate::Reg<seqctrl::SEQCTRL_SPEC>;
#[doc = "Sequential Control %s"]
pub mod seqctrl;
