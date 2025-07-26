#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mclkctrla: MCLKCTRLA,
    mclkctrlb: MCLKCTRLB,
    mclkctrlc: MCLKCTRLC,
    mclkintctrl: MCLKINTCTRL,
    mclkintflags: MCLKINTFLAGS,
    mclkstatus: MCLKSTATUS,
    _reserved6: [u8; 0x02],
    oschfctrla: OSCHFCTRLA,
    oschftune: OSCHFTUNE,
    _reserved8: [u8; 0x06],
    pllctrla: PLLCTRLA,
    _reserved9: [u8; 0x07],
    osc32kctrla: OSC32KCTRLA,
    _reserved10: [u8; 0x03],
    xosc32kctrla: XOSC32KCTRLA,
    _reserved11: [u8; 0x03],
    xoschfctrla: XOSCHFCTRLA,
}
impl RegisterBlock {
    #[doc = "0x00 - MCLK Control A"]
    #[inline(always)]
    pub const fn mclkctrla(&self) -> &MCLKCTRLA {
        &self.mclkctrla
    }
    #[doc = "0x01 - MCLK Control B"]
    #[inline(always)]
    pub const fn mclkctrlb(&self) -> &MCLKCTRLB {
        &self.mclkctrlb
    }
    #[doc = "0x02 - MCLK Control C"]
    #[inline(always)]
    pub const fn mclkctrlc(&self) -> &MCLKCTRLC {
        &self.mclkctrlc
    }
    #[doc = "0x03 - MCLK Interrupt Control"]
    #[inline(always)]
    pub const fn mclkintctrl(&self) -> &MCLKINTCTRL {
        &self.mclkintctrl
    }
    #[doc = "0x04 - MCLK Interrupt Flags"]
    #[inline(always)]
    pub const fn mclkintflags(&self) -> &MCLKINTFLAGS {
        &self.mclkintflags
    }
    #[doc = "0x05 - MCLK Status"]
    #[inline(always)]
    pub const fn mclkstatus(&self) -> &MCLKSTATUS {
        &self.mclkstatus
    }
    #[doc = "0x08 - OSCHF Control A"]
    #[inline(always)]
    pub const fn oschfctrla(&self) -> &OSCHFCTRLA {
        &self.oschfctrla
    }
    #[doc = "0x09 - OSCHF Tune"]
    #[inline(always)]
    pub const fn oschftune(&self) -> &OSCHFTUNE {
        &self.oschftune
    }
    #[doc = "0x10 - PLL Control A"]
    #[inline(always)]
    pub const fn pllctrla(&self) -> &PLLCTRLA {
        &self.pllctrla
    }
    #[doc = "0x18 - OSC32K Control A"]
    #[inline(always)]
    pub const fn osc32kctrla(&self) -> &OSC32KCTRLA {
        &self.osc32kctrla
    }
    #[doc = "0x1c - XOSC32K Control A"]
    #[inline(always)]
    pub const fn xosc32kctrla(&self) -> &XOSC32KCTRLA {
        &self.xosc32kctrla
    }
    #[doc = "0x20 - XOSC HF Control A"]
    #[inline(always)]
    pub const fn xoschfctrla(&self) -> &XOSCHFCTRLA {
        &self.xoschfctrla
    }
}
#[doc = "MCLKCTRLA (rw) register accessor: MCLK Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclkctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclkctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclkctrla`]
module"]
pub type MCLKCTRLA = crate::Reg<mclkctrla::MCLKCTRLA_SPEC>;
#[doc = "MCLK Control A"]
pub mod mclkctrla;
#[doc = "MCLKCTRLB (rw) register accessor: MCLK Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclkctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclkctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclkctrlb`]
module"]
pub type MCLKCTRLB = crate::Reg<mclkctrlb::MCLKCTRLB_SPEC>;
#[doc = "MCLK Control B"]
pub mod mclkctrlb;
#[doc = "MCLKCTRLC (rw) register accessor: MCLK Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclkctrlc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclkctrlc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclkctrlc`]
module"]
pub type MCLKCTRLC = crate::Reg<mclkctrlc::MCLKCTRLC_SPEC>;
#[doc = "MCLK Control C"]
pub mod mclkctrlc;
#[doc = "MCLKINTCTRL (rw) register accessor: MCLK Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclkintctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclkintctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclkintctrl`]
module"]
pub type MCLKINTCTRL = crate::Reg<mclkintctrl::MCLKINTCTRL_SPEC>;
#[doc = "MCLK Interrupt Control"]
pub mod mclkintctrl;
#[doc = "MCLKINTFLAGS (rw) register accessor: MCLK Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclkintflags::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclkintflags::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclkintflags`]
module"]
pub type MCLKINTFLAGS = crate::Reg<mclkintflags::MCLKINTFLAGS_SPEC>;
#[doc = "MCLK Interrupt Flags"]
pub mod mclkintflags;
#[doc = "MCLKSTATUS (rw) register accessor: MCLK Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclkstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclkstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclkstatus`]
module"]
pub type MCLKSTATUS = crate::Reg<mclkstatus::MCLKSTATUS_SPEC>;
#[doc = "MCLK Status"]
pub mod mclkstatus;
#[doc = "OSC32KCTRLA (rw) register accessor: OSC32K Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc32kctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc32kctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc32kctrla`]
module"]
pub type OSC32KCTRLA = crate::Reg<osc32kctrla::OSC32KCTRLA_SPEC>;
#[doc = "OSC32K Control A"]
pub mod osc32kctrla;
#[doc = "OSCHFCTRLA (rw) register accessor: OSCHF Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oschfctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oschfctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oschfctrla`]
module"]
pub type OSCHFCTRLA = crate::Reg<oschfctrla::OSCHFCTRLA_SPEC>;
#[doc = "OSCHF Control A"]
pub mod oschfctrla;
#[doc = "OSCHFTUNE (rw) register accessor: OSCHF Tune\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oschftune::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oschftune::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oschftune`]
module"]
pub type OSCHFTUNE = crate::Reg<oschftune::OSCHFTUNE_SPEC>;
#[doc = "OSCHF Tune"]
pub mod oschftune;
#[doc = "PLLCTRLA (rw) register accessor: PLL Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllctrla`]
module"]
pub type PLLCTRLA = crate::Reg<pllctrla::PLLCTRLA_SPEC>;
#[doc = "PLL Control A"]
pub mod pllctrla;
#[doc = "XOSC32KCTRLA (rw) register accessor: XOSC32K Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xosc32kctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xosc32kctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xosc32kctrla`]
module"]
pub type XOSC32KCTRLA = crate::Reg<xosc32kctrla::XOSC32KCTRLA_SPEC>;
#[doc = "XOSC32K Control A"]
pub mod xosc32kctrla;
#[doc = "XOSCHFCTRLA (rw) register accessor: XOSC HF Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xoschfctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xoschfctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xoschfctrla`]
module"]
pub type XOSCHFCTRLA = crate::Reg<xoschfctrla::XOSCHFCTRLA_SPEC>;
#[doc = "XOSC HF Control A"]
pub mod xoschfctrla;
