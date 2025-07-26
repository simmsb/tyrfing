#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mclkctrla: MCLKCTRLA,
    mclkctrlb: MCLKCTRLB,
    mclklock: MCLKLOCK,
    mclkstatus: MCLKSTATUS,
    _reserved4: [u8; 0x0c],
    osc20mctrla: OSC20MCTRLA,
    osc20mcaliba: OSC20MCALIBA,
    osc20mcalibb: OSC20MCALIBB,
    _reserved7: [u8; 0x05],
    osc32kctrla: OSC32KCTRLA,
    _reserved8: [u8; 0x03],
    xosc32kctrla: XOSC32KCTRLA,
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
    #[doc = "0x02 - MCLK Lock"]
    #[inline(always)]
    pub const fn mclklock(&self) -> &MCLKLOCK {
        &self.mclklock
    }
    #[doc = "0x03 - MCLK Status"]
    #[inline(always)]
    pub const fn mclkstatus(&self) -> &MCLKSTATUS {
        &self.mclkstatus
    }
    #[doc = "0x10 - OSC20M Control A"]
    #[inline(always)]
    pub const fn osc20mctrla(&self) -> &OSC20MCTRLA {
        &self.osc20mctrla
    }
    #[doc = "0x11 - OSC20M Calibration A"]
    #[inline(always)]
    pub const fn osc20mcaliba(&self) -> &OSC20MCALIBA {
        &self.osc20mcaliba
    }
    #[doc = "0x12 - OSC20M Calibration B"]
    #[inline(always)]
    pub const fn osc20mcalibb(&self) -> &OSC20MCALIBB {
        &self.osc20mcalibb
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
#[doc = "MCLKLOCK (rw) register accessor: MCLK Lock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclklock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclklock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclklock`]
module"]
pub type MCLKLOCK = crate::Reg<mclklock::MCLKLOCK_SPEC>;
#[doc = "MCLK Lock"]
pub mod mclklock;
#[doc = "MCLKSTATUS (r) register accessor: MCLK Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclkstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclkstatus`]
module"]
pub type MCLKSTATUS = crate::Reg<mclkstatus::MCLKSTATUS_SPEC>;
#[doc = "MCLK Status"]
pub mod mclkstatus;
#[doc = "OSC20MCALIBA (rw) register accessor: OSC20M Calibration A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc20mcaliba::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc20mcaliba::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc20mcaliba`]
module"]
pub type OSC20MCALIBA = crate::Reg<osc20mcaliba::OSC20MCALIBA_SPEC>;
#[doc = "OSC20M Calibration A"]
pub mod osc20mcaliba;
#[doc = "OSC20MCALIBB (rw) register accessor: OSC20M Calibration B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc20mcalibb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc20mcalibb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc20mcalibb`]
module"]
pub type OSC20MCALIBB = crate::Reg<osc20mcalibb::OSC20MCALIBB_SPEC>;
#[doc = "OSC20M Calibration B"]
pub mod osc20mcalibb;
#[doc = "OSC20MCTRLA (rw) register accessor: OSC20M Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc20mctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc20mctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc20mctrla`]
module"]
pub type OSC20MCTRLA = crate::Reg<osc20mctrla::OSC20MCTRLA_SPEC>;
#[doc = "OSC20M Control A"]
pub mod osc20mctrla;
#[doc = "OSC32KCTRLA (rw) register accessor: OSC32K Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc32kctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc32kctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc32kctrla`]
module"]
pub type OSC32KCTRLA = crate::Reg<osc32kctrla::OSC32KCTRLA_SPEC>;
#[doc = "OSC32K Control A"]
pub mod osc32kctrla;
#[doc = "XOSC32KCTRLA (rw) register accessor: XOSC32K Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xosc32kctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xosc32kctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xosc32kctrla`]
module"]
pub type XOSC32KCTRLA = crate::Reg<xosc32kctrla::XOSC32KCTRLA_SPEC>;
#[doc = "XOSC32K Control A"]
pub mod xosc32kctrla;
