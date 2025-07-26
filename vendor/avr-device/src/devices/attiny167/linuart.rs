#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    lincr: LINCR,
    linsir: LINSIR,
    linenir: LINENIR,
    linerr: LINERR,
    linbtr: LINBTR,
    linbrrl: LINBRRL,
    linbrrh: LINBRRH,
    lindlr: LINDLR,
    linidr: LINIDR,
    linsel: LINSEL,
    lindat: LINDAT,
}
impl RegisterBlock {
    #[doc = "0x00 - LIN Control Register"]
    #[inline(always)]
    pub const fn lincr(&self) -> &LINCR {
        &self.lincr
    }
    #[doc = "0x01 - LIN Status and Interrupt Register"]
    #[inline(always)]
    pub const fn linsir(&self) -> &LINSIR {
        &self.linsir
    }
    #[doc = "0x02 - LIN Enable Interrupt Register"]
    #[inline(always)]
    pub const fn linenir(&self) -> &LINENIR {
        &self.linenir
    }
    #[doc = "0x03 - LIN Error Register"]
    #[inline(always)]
    pub const fn linerr(&self) -> &LINERR {
        &self.linerr
    }
    #[doc = "0x04 - LIN Bit Timing Register"]
    #[inline(always)]
    pub const fn linbtr(&self) -> &LINBTR {
        &self.linbtr
    }
    #[doc = "0x05 - LIN Baud Rate Low Register"]
    #[inline(always)]
    pub const fn linbrrl(&self) -> &LINBRRL {
        &self.linbrrl
    }
    #[doc = "0x06 - LIN Baud Rate High Register"]
    #[inline(always)]
    pub const fn linbrrh(&self) -> &LINBRRH {
        &self.linbrrh
    }
    #[doc = "0x07 - LIN Data Length Register"]
    #[inline(always)]
    pub const fn lindlr(&self) -> &LINDLR {
        &self.lindlr
    }
    #[doc = "0x08 - LIN Identifier Register"]
    #[inline(always)]
    pub const fn linidr(&self) -> &LINIDR {
        &self.linidr
    }
    #[doc = "0x09 - LIN Data Buffer Selection Register"]
    #[inline(always)]
    pub const fn linsel(&self) -> &LINSEL {
        &self.linsel
    }
    #[doc = "0x0a - LIN Data Register"]
    #[inline(always)]
    pub const fn lindat(&self) -> &LINDAT {
        &self.lindat
    }
}
#[doc = "LINBRRH (rw) register accessor: LIN Baud Rate High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linbrrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linbrrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linbrrh`]
module"]
pub type LINBRRH = crate::Reg<linbrrh::LINBRRH_SPEC>;
#[doc = "LIN Baud Rate High Register"]
pub mod linbrrh;
#[doc = "LINBRRL (rw) register accessor: LIN Baud Rate Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linbrrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linbrrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linbrrl`]
module"]
pub type LINBRRL = crate::Reg<linbrrl::LINBRRL_SPEC>;
#[doc = "LIN Baud Rate Low Register"]
pub mod linbrrl;
#[doc = "LINBTR (rw) register accessor: LIN Bit Timing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linbtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linbtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linbtr`]
module"]
pub type LINBTR = crate::Reg<linbtr::LINBTR_SPEC>;
#[doc = "LIN Bit Timing Register"]
pub mod linbtr;
#[doc = "LINCR (rw) register accessor: LIN Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lincr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lincr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lincr`]
module"]
pub type LINCR = crate::Reg<lincr::LINCR_SPEC>;
#[doc = "LIN Control Register"]
pub mod lincr;
#[doc = "LINDAT (rw) register accessor: LIN Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lindat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lindat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lindat`]
module"]
pub type LINDAT = crate::Reg<lindat::LINDAT_SPEC>;
#[doc = "LIN Data Register"]
pub mod lindat;
#[doc = "LINDLR (rw) register accessor: LIN Data Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lindlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lindlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lindlr`]
module"]
pub type LINDLR = crate::Reg<lindlr::LINDLR_SPEC>;
#[doc = "LIN Data Length Register"]
pub mod lindlr;
#[doc = "LINENIR (rw) register accessor: LIN Enable Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linenir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linenir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linenir`]
module"]
pub type LINENIR = crate::Reg<linenir::LINENIR_SPEC>;
#[doc = "LIN Enable Interrupt Register"]
pub mod linenir;
#[doc = "LINERR (rw) register accessor: LIN Error Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linerr`]
module"]
pub type LINERR = crate::Reg<linerr::LINERR_SPEC>;
#[doc = "LIN Error Register"]
pub mod linerr;
#[doc = "LINIDR (rw) register accessor: LIN Identifier Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linidr`]
module"]
pub type LINIDR = crate::Reg<linidr::LINIDR_SPEC>;
#[doc = "LIN Identifier Register"]
pub mod linidr;
#[doc = "LINSEL (rw) register accessor: LIN Data Buffer Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linsel`]
module"]
pub type LINSEL = crate::Reg<linsel::LINSEL_SPEC>;
#[doc = "LIN Data Buffer Selection Register"]
pub mod linsel;
#[doc = "LINSIR (rw) register accessor: LIN Status and Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linsir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linsir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linsir`]
module"]
pub type LINSIR = crate::Reg<linsir::LINSIR_SPEC>;
#[doc = "LIN Status and Interrupt Register"]
pub mod linsir;
