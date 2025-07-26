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
    _reserved10: [u8; 0x06],
    pin0ctrl: PIN0CTRL,
    pin1ctrl: PIN1CTRL,
    pin2ctrl: PIN2CTRL,
    pin3ctrl: PIN3CTRL,
    pin4ctrl: PIN4CTRL,
    pin5ctrl: PIN5CTRL,
    pin6ctrl: PIN6CTRL,
    pin7ctrl: PIN7CTRL,
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
    #[doc = "0x10 - Pin 0 Control"]
    #[inline(always)]
    pub const fn pin0ctrl(&self) -> &PIN0CTRL {
        &self.pin0ctrl
    }
    #[doc = "0x11 - Pin 1 Control"]
    #[inline(always)]
    pub const fn pin1ctrl(&self) -> &PIN1CTRL {
        &self.pin1ctrl
    }
    #[doc = "0x12 - Pin 2 Control"]
    #[inline(always)]
    pub const fn pin2ctrl(&self) -> &PIN2CTRL {
        &self.pin2ctrl
    }
    #[doc = "0x13 - Pin 3 Control"]
    #[inline(always)]
    pub const fn pin3ctrl(&self) -> &PIN3CTRL {
        &self.pin3ctrl
    }
    #[doc = "0x14 - Pin 4 Control"]
    #[inline(always)]
    pub const fn pin4ctrl(&self) -> &PIN4CTRL {
        &self.pin4ctrl
    }
    #[doc = "0x15 - Pin 5 Control"]
    #[inline(always)]
    pub const fn pin5ctrl(&self) -> &PIN5CTRL {
        &self.pin5ctrl
    }
    #[doc = "0x16 - Pin 6 Control"]
    #[inline(always)]
    pub const fn pin6ctrl(&self) -> &PIN6CTRL {
        &self.pin6ctrl
    }
    #[doc = "0x17 - Pin 7 Control"]
    #[inline(always)]
    pub const fn pin7ctrl(&self) -> &PIN7CTRL {
        &self.pin7ctrl
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
#[doc = "PIN0CTRL (rw) register accessor: Pin 0 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin0ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin0ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin0ctrl`]
module"]
pub type PIN0CTRL = crate::Reg<pin0ctrl::PIN0CTRL_SPEC>;
#[doc = "Pin 0 Control"]
pub mod pin0ctrl;
#[doc = "PIN1CTRL (rw) register accessor: Pin 1 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin1ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin1ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin1ctrl`]
module"]
pub type PIN1CTRL = crate::Reg<pin1ctrl::PIN1CTRL_SPEC>;
#[doc = "Pin 1 Control"]
pub mod pin1ctrl;
#[doc = "PIN2CTRL (rw) register accessor: Pin 2 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin2ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin2ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin2ctrl`]
module"]
pub type PIN2CTRL = crate::Reg<pin2ctrl::PIN2CTRL_SPEC>;
#[doc = "Pin 2 Control"]
pub mod pin2ctrl;
#[doc = "PIN3CTRL (rw) register accessor: Pin 3 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin3ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin3ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin3ctrl`]
module"]
pub type PIN3CTRL = crate::Reg<pin3ctrl::PIN3CTRL_SPEC>;
#[doc = "Pin 3 Control"]
pub mod pin3ctrl;
#[doc = "PIN4CTRL (rw) register accessor: Pin 4 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin4ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin4ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin4ctrl`]
module"]
pub type PIN4CTRL = crate::Reg<pin4ctrl::PIN4CTRL_SPEC>;
#[doc = "Pin 4 Control"]
pub mod pin4ctrl;
#[doc = "PIN5CTRL (rw) register accessor: Pin 5 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin5ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin5ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin5ctrl`]
module"]
pub type PIN5CTRL = crate::Reg<pin5ctrl::PIN5CTRL_SPEC>;
#[doc = "Pin 5 Control"]
pub mod pin5ctrl;
#[doc = "PIN6CTRL (rw) register accessor: Pin 6 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin6ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin6ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin6ctrl`]
module"]
pub type PIN6CTRL = crate::Reg<pin6ctrl::PIN6CTRL_SPEC>;
#[doc = "Pin 6 Control"]
pub mod pin6ctrl;
#[doc = "PIN7CTRL (rw) register accessor: Pin 7 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin7ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin7ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin7ctrl`]
module"]
pub type PIN7CTRL = crate::Reg<pin7ctrl::PIN7CTRL_SPEC>;
#[doc = "Pin 7 Control"]
pub mod pin7ctrl;
