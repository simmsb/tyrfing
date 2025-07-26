#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrla: CTRLA,
    _reserved1: [u8; 0x01],
    muxctrla: MUXCTRLA,
    _reserved2: [u8; 0x01],
    dacref: DACREF,
    _reserved3: [u8; 0x01],
    intctrl: INTCTRL,
    status: STATUS,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &CTRLA {
        &self.ctrla
    }
    #[doc = "0x02 - Mux Control A"]
    #[inline(always)]
    pub const fn muxctrla(&self) -> &MUXCTRLA {
        &self.muxctrla
    }
    #[doc = "0x04 - Referance scale control"]
    #[inline(always)]
    pub const fn dacref(&self) -> &DACREF {
        &self.dacref
    }
    #[doc = "0x06 - Interrupt Control"]
    #[inline(always)]
    pub const fn intctrl(&self) -> &INTCTRL {
        &self.intctrl
    }
    #[doc = "0x07 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "DACREF (rw) register accessor: Referance scale control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacref::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacref::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacref`]
module"]
pub type DACREF = crate::Reg<dacref::DACREF_SPEC>;
#[doc = "Referance scale control"]
pub mod dacref;
#[doc = "INTCTRL (rw) register accessor: Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intctrl`]
module"]
pub type INTCTRL = crate::Reg<intctrl::INTCTRL_SPEC>;
#[doc = "Interrupt Control"]
pub mod intctrl;
#[doc = "MUXCTRLA (rw) register accessor: Mux Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxctrla`]
module"]
pub type MUXCTRLA = crate::Reg<muxctrla::MUXCTRLA_SPEC>;
#[doc = "Mux Control A"]
pub mod muxctrla;
#[doc = "STATUS (rw) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
