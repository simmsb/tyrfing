#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    dir: DIR,
    dirset: DIRSET,
    dirclr: DIRCLR,
    dirtgl: DIRTGL,
    out: OUT,
    outset: OUTSET,
    outclr: OUTCLR,
    outtgl: OUTTGL,
    in_: IN,
    intflags: INTFLAGS,
    portctrl: PORTCTRL,
    pinconfig: PINCONFIG,
    pinctrlupd: PINCTRLUPD,
    pinctrlset: PINCTRLSET,
    pinctrlclr: PINCTRLCLR,
    _reserved15: [u8; 0x01],
    pinctrl: [PINCTRL; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - Data Direction"]
    #[inline(always)]
    pub const fn dir(&self) -> &DIR {
        &self.dir
    }
    #[doc = "0x01 - Data Direction Set"]
    #[inline(always)]
    pub const fn dirset(&self) -> &DIRSET {
        &self.dirset
    }
    #[doc = "0x02 - Data Direction Clear"]
    #[inline(always)]
    pub const fn dirclr(&self) -> &DIRCLR {
        &self.dirclr
    }
    #[doc = "0x03 - Data Direction Toggle"]
    #[inline(always)]
    pub const fn dirtgl(&self) -> &DIRTGL {
        &self.dirtgl
    }
    #[doc = "0x04 - Output Value"]
    #[inline(always)]
    pub const fn out(&self) -> &OUT {
        &self.out
    }
    #[doc = "0x05 - Output Value Set"]
    #[inline(always)]
    pub const fn outset(&self) -> &OUTSET {
        &self.outset
    }
    #[doc = "0x06 - Output Value Clear"]
    #[inline(always)]
    pub const fn outclr(&self) -> &OUTCLR {
        &self.outclr
    }
    #[doc = "0x07 - Output Value Toggle"]
    #[inline(always)]
    pub const fn outtgl(&self) -> &OUTTGL {
        &self.outtgl
    }
    #[doc = "0x08 - Input Value"]
    #[inline(always)]
    pub const fn in_(&self) -> &IN {
        &self.in_
    }
    #[doc = "0x09 - Interrupt Flags"]
    #[inline(always)]
    pub const fn intflags(&self) -> &INTFLAGS {
        &self.intflags
    }
    #[doc = "0x0a - Port Control"]
    #[inline(always)]
    pub const fn portctrl(&self) -> &PORTCTRL {
        &self.portctrl
    }
    #[doc = "0x0b - Pin Control Config"]
    #[inline(always)]
    pub const fn pinconfig(&self) -> &PINCONFIG {
        &self.pinconfig
    }
    #[doc = "0x0c - Pin Control Update"]
    #[inline(always)]
    pub const fn pinctrlupd(&self) -> &PINCTRLUPD {
        &self.pinctrlupd
    }
    #[doc = "0x0d - Pin Control Set"]
    #[inline(always)]
    pub const fn pinctrlset(&self) -> &PINCTRLSET {
        &self.pinctrlset
    }
    #[doc = "0x0e - Pin Control Clear"]
    #[inline(always)]
    pub const fn pinctrlclr(&self) -> &PINCTRLCLR {
        &self.pinctrlclr
    }
    #[doc = "0x10..0x18 - Pin %s Control"]
    #[inline(always)]
    pub const fn pinctrl(&self, n: usize) -> &PINCTRL {
        &self.pinctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x18 - Pin %s Control"]
    #[inline(always)]
    pub fn pinctrl_iter(&self) -> impl Iterator<Item = &PINCTRL> {
        self.pinctrl.iter()
    }
    #[doc = "0x10 - Pin 0 Control"]
    #[inline(always)]
    pub const fn pin0ctrl(&self) -> &PINCTRL {
        self.pinctrl(0)
    }
    #[doc = "0x11 - Pin 1 Control"]
    #[inline(always)]
    pub const fn pin1ctrl(&self) -> &PINCTRL {
        self.pinctrl(1)
    }
    #[doc = "0x12 - Pin 2 Control"]
    #[inline(always)]
    pub const fn pin2ctrl(&self) -> &PINCTRL {
        self.pinctrl(2)
    }
    #[doc = "0x13 - Pin 3 Control"]
    #[inline(always)]
    pub const fn pin3ctrl(&self) -> &PINCTRL {
        self.pinctrl(3)
    }
    #[doc = "0x14 - Pin 4 Control"]
    #[inline(always)]
    pub const fn pin4ctrl(&self) -> &PINCTRL {
        self.pinctrl(4)
    }
    #[doc = "0x15 - Pin 5 Control"]
    #[inline(always)]
    pub const fn pin5ctrl(&self) -> &PINCTRL {
        self.pinctrl(5)
    }
    #[doc = "0x16 - Pin 6 Control"]
    #[inline(always)]
    pub const fn pin6ctrl(&self) -> &PINCTRL {
        self.pinctrl(6)
    }
    #[doc = "0x17 - Pin 7 Control"]
    #[inline(always)]
    pub const fn pin7ctrl(&self) -> &PINCTRL {
        self.pinctrl(7)
    }
}
#[doc = "DIR (rw) register accessor: Data Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`]
module"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "Data Direction"]
pub mod dir;
#[doc = "DIRCLR (rw) register accessor: Data Direction Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dirclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dirclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirclr`]
module"]
pub type DIRCLR = crate::Reg<dirclr::DIRCLR_SPEC>;
#[doc = "Data Direction Clear"]
pub mod dirclr;
#[doc = "DIRSET (rw) register accessor: Data Direction Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dirset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dirset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirset`]
module"]
pub type DIRSET = crate::Reg<dirset::DIRSET_SPEC>;
#[doc = "Data Direction Set"]
pub mod dirset;
#[doc = "DIRTGL (rw) register accessor: Data Direction Toggle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dirtgl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dirtgl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirtgl`]
module"]
pub type DIRTGL = crate::Reg<dirtgl::DIRTGL_SPEC>;
#[doc = "Data Direction Toggle"]
pub mod dirtgl;
#[doc = "IN (rw) register accessor: Input Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`]
module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Input Value"]
pub mod in_;
#[doc = "INTFLAGS (rw) register accessor: Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflags::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflags::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflags`]
module"]
pub type INTFLAGS = crate::Reg<intflags::INTFLAGS_SPEC>;
#[doc = "Interrupt Flags"]
pub mod intflags;
#[doc = "OUT (rw) register accessor: Output Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`]
module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Output Value"]
pub mod out;
#[doc = "OUTCLR (rw) register accessor: Output Value Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outclr`]
module"]
pub type OUTCLR = crate::Reg<outclr::OUTCLR_SPEC>;
#[doc = "Output Value Clear"]
pub mod outclr;
#[doc = "OUTSET (rw) register accessor: Output Value Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outset`]
module"]
pub type OUTSET = crate::Reg<outset::OUTSET_SPEC>;
#[doc = "Output Value Set"]
pub mod outset;
#[doc = "OUTTGL (rw) register accessor: Output Value Toggle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outtgl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outtgl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outtgl`]
module"]
pub type OUTTGL = crate::Reg<outtgl::OUTTGL_SPEC>;
#[doc = "Output Value Toggle"]
pub mod outtgl;
#[doc = "PINCTRL (rw) register accessor: Pin %s Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinctrl`]
module"]
pub type PINCTRL = crate::Reg<pinctrl::PINCTRL_SPEC>;
#[doc = "Pin %s Control"]
pub mod pinctrl;
#[doc = "PINCONFIG (rw) register accessor: Pin Control Config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinconfig`]
module"]
pub type PINCONFIG = crate::Reg<pinconfig::PINCONFIG_SPEC>;
#[doc = "Pin Control Config"]
pub mod pinconfig;
#[doc = "PINCTRLCLR (rw) register accessor: Pin Control Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinctrlclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinctrlclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinctrlclr`]
module"]
pub type PINCTRLCLR = crate::Reg<pinctrlclr::PINCTRLCLR_SPEC>;
#[doc = "Pin Control Clear"]
pub mod pinctrlclr;
#[doc = "PINCTRLSET (rw) register accessor: Pin Control Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinctrlset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinctrlset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinctrlset`]
module"]
pub type PINCTRLSET = crate::Reg<pinctrlset::PINCTRLSET_SPEC>;
#[doc = "Pin Control Set"]
pub mod pinctrlset;
#[doc = "PINCTRLUPD (rw) register accessor: Pin Control Update\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinctrlupd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinctrlupd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinctrlupd`]
module"]
pub type PINCTRLUPD = crate::Reg<pinctrlupd::PINCTRLUPD_SPEC>;
#[doc = "Pin Control Update"]
pub mod pinctrlupd;
#[doc = "PORTCTRL (rw) register accessor: Port Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portctrl`]
module"]
pub type PORTCTRL = crate::Reg<portctrl::PORTCTRL_SPEC>;
#[doc = "Port Control"]
pub mod portctrl;
