#[doc = r"Register block"]
#[repr(C)]
pub struct LUT {
    lutctrla: LUTCTRLA,
    lutctrlb: LUTCTRLB,
    lutctrlc: LUTCTRLC,
    truth: TRUTH,
}
impl LUT {
    #[doc = "0x00 - LUT Control 0 A"]
    #[inline(always)]
    pub const fn lutctrla(&self) -> &LUTCTRLA {
        &self.lutctrla
    }
    #[doc = "0x01 - LUT Control 0 B"]
    #[inline(always)]
    pub const fn lutctrlb(&self) -> &LUTCTRLB {
        &self.lutctrlb
    }
    #[doc = "0x02 - LUT Control 0 C"]
    #[inline(always)]
    pub const fn lutctrlc(&self) -> &LUTCTRLC {
        &self.lutctrlc
    }
    #[doc = "0x03 - Truth 0"]
    #[inline(always)]
    pub const fn truth(&self) -> &TRUTH {
        &self.truth
    }
}
#[doc = "LUTCTRLA (rw) register accessor: LUT Control 0 A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lutctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lutctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lutctrla`]
module"]
pub type LUTCTRLA = crate::Reg<lutctrla::LUTCTRLA_SPEC>;
#[doc = "LUT Control 0 A"]
pub mod lutctrla;
#[doc = "LUTCTRLB (rw) register accessor: LUT Control 0 B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lutctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lutctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lutctrlb`]
module"]
pub type LUTCTRLB = crate::Reg<lutctrlb::LUTCTRLB_SPEC>;
#[doc = "LUT Control 0 B"]
pub mod lutctrlb;
#[doc = "LUTCTRLC (rw) register accessor: LUT Control 0 C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lutctrlc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lutctrlc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lutctrlc`]
module"]
pub type LUTCTRLC = crate::Reg<lutctrlc::LUTCTRLC_SPEC>;
#[doc = "LUT Control 0 C"]
pub mod lutctrlc;
#[doc = "TRUTH (rw) register accessor: Truth 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`truth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`truth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@truth`]
module"]
pub type TRUTH = crate::Reg<truth::TRUTH_SPEC>;
#[doc = "Truth 0"]
pub mod truth;
