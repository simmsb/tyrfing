#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    udcon: UDCON,
    udint: UDINT,
    udien: UDIEN,
    udaddr: UDADDR,
    udfnum: UDFNUM,
    udmfn: UDMFN,
    _reserved6: [u8; 0x01],
    ueintx: UEINTX,
    uenum: UENUM,
    uerst: UERST,
    ueconx: UECONX,
    uecfg0x: UECFG0X,
    uecfg1x: UECFG1X,
    uesta0x: UESTA0X,
    uesta1x: UESTA1X,
    ueienx: UEIENX,
    uedatx: UEDATX,
    uebclx: UEBCLX,
    uebchx: UEBCHX,
    ueint: UEINT,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description."]
    #[inline(always)]
    pub const fn udcon(&self) -> &UDCON {
        &self.udcon
    }
    #[doc = "0x01 - No Description."]
    #[inline(always)]
    pub const fn udint(&self) -> &UDINT {
        &self.udint
    }
    #[doc = "0x02 - No Description."]
    #[inline(always)]
    pub const fn udien(&self) -> &UDIEN {
        &self.udien
    }
    #[doc = "0x03 - No Description."]
    #[inline(always)]
    pub const fn udaddr(&self) -> &UDADDR {
        &self.udaddr
    }
    #[doc = "0x04 - No Description."]
    #[inline(always)]
    pub const fn udfnum(&self) -> &UDFNUM {
        &self.udfnum
    }
    #[doc = "0x06 - No Description."]
    #[inline(always)]
    pub const fn udmfn(&self) -> &UDMFN {
        &self.udmfn
    }
    #[doc = "0x08 - No Description."]
    #[inline(always)]
    pub const fn ueintx(&self) -> &UEINTX {
        &self.ueintx
    }
    #[doc = "0x09 - No Description."]
    #[inline(always)]
    pub const fn uenum(&self) -> &UENUM {
        &self.uenum
    }
    #[doc = "0x0a - No Description."]
    #[inline(always)]
    pub const fn uerst(&self) -> &UERST {
        &self.uerst
    }
    #[doc = "0x0b - No Description."]
    #[inline(always)]
    pub const fn ueconx(&self) -> &UECONX {
        &self.ueconx
    }
    #[doc = "0x0c - No Description."]
    #[inline(always)]
    pub const fn uecfg0x(&self) -> &UECFG0X {
        &self.uecfg0x
    }
    #[doc = "0x0d - No Description."]
    #[inline(always)]
    pub const fn uecfg1x(&self) -> &UECFG1X {
        &self.uecfg1x
    }
    #[doc = "0x0e - No Description."]
    #[inline(always)]
    pub const fn uesta0x(&self) -> &UESTA0X {
        &self.uesta0x
    }
    #[doc = "0x0f - No Description."]
    #[inline(always)]
    pub const fn uesta1x(&self) -> &UESTA1X {
        &self.uesta1x
    }
    #[doc = "0x10 - No Description."]
    #[inline(always)]
    pub const fn ueienx(&self) -> &UEIENX {
        &self.ueienx
    }
    #[doc = "0x11 - No Description."]
    #[inline(always)]
    pub const fn uedatx(&self) -> &UEDATX {
        &self.uedatx
    }
    #[doc = "0x12 - No Description."]
    #[inline(always)]
    pub const fn uebclx(&self) -> &UEBCLX {
        &self.uebclx
    }
    #[doc = "0x13 - No Description."]
    #[inline(always)]
    pub const fn uebchx(&self) -> &UEBCHX {
        &self.uebchx
    }
    #[doc = "0x14 - No Description."]
    #[inline(always)]
    pub const fn ueint(&self) -> &UEINT {
        &self.ueint
    }
}
#[doc = "UDADDR (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udaddr`]
module"]
pub type UDADDR = crate::Reg<udaddr::UDADDR_SPEC>;
#[doc = "No Description."]
pub mod udaddr;
#[doc = "UDCON (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udcon`]
module"]
pub type UDCON = crate::Reg<udcon::UDCON_SPEC>;
#[doc = "No Description."]
pub mod udcon;
#[doc = "UDFNUM (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udfnum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udfnum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udfnum`]
module"]
pub type UDFNUM = crate::Reg<udfnum::UDFNUM_SPEC>;
#[doc = "No Description."]
pub mod udfnum;
#[doc = "UDIEN (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udien`]
module"]
pub type UDIEN = crate::Reg<udien::UDIEN_SPEC>;
#[doc = "No Description."]
pub mod udien;
#[doc = "UDINT (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udint`]
module"]
pub type UDINT = crate::Reg<udint::UDINT_SPEC>;
#[doc = "No Description."]
pub mod udint;
#[doc = "UDMFN (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmfn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmfn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmfn`]
module"]
pub type UDMFN = crate::Reg<udmfn::UDMFN_SPEC>;
#[doc = "No Description."]
pub mod udmfn;
#[doc = "UEBCHX (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uebchx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uebchx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uebchx`]
module"]
pub type UEBCHX = crate::Reg<uebchx::UEBCHX_SPEC>;
#[doc = "No Description."]
pub mod uebchx;
#[doc = "UEBCLX (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uebclx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uebclx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uebclx`]
module"]
pub type UEBCLX = crate::Reg<uebclx::UEBCLX_SPEC>;
#[doc = "No Description."]
pub mod uebclx;
#[doc = "UECFG0X (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uecfg0x::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uecfg0x::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uecfg0x`]
module"]
pub type UECFG0X = crate::Reg<uecfg0x::UECFG0X_SPEC>;
#[doc = "No Description."]
pub mod uecfg0x;
#[doc = "UECFG1X (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uecfg1x::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uecfg1x::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uecfg1x`]
module"]
pub type UECFG1X = crate::Reg<uecfg1x::UECFG1X_SPEC>;
#[doc = "No Description."]
pub mod uecfg1x;
#[doc = "UECONX (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ueconx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ueconx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ueconx`]
module"]
pub type UECONX = crate::Reg<ueconx::UECONX_SPEC>;
#[doc = "No Description."]
pub mod ueconx;
#[doc = "UEDATX (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uedatx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uedatx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uedatx`]
module"]
pub type UEDATX = crate::Reg<uedatx::UEDATX_SPEC>;
#[doc = "No Description."]
pub mod uedatx;
#[doc = "UEIENX (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ueienx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ueienx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ueienx`]
module"]
pub type UEIENX = crate::Reg<ueienx::UEIENX_SPEC>;
#[doc = "No Description."]
pub mod ueienx;
#[doc = "UEINT (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ueint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ueint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ueint`]
module"]
pub type UEINT = crate::Reg<ueint::UEINT_SPEC>;
#[doc = "No Description."]
pub mod ueint;
#[doc = "UEINTX (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ueintx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ueintx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ueintx`]
module"]
pub type UEINTX = crate::Reg<ueintx::UEINTX_SPEC>;
#[doc = "No Description."]
pub mod ueintx;
#[doc = "UENUM (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uenum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uenum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uenum`]
module"]
pub type UENUM = crate::Reg<uenum::UENUM_SPEC>;
#[doc = "No Description."]
pub mod uenum;
#[doc = "UERST (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uerst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uerst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uerst`]
module"]
pub type UERST = crate::Reg<uerst::UERST_SPEC>;
#[doc = "No Description."]
pub mod uerst;
#[doc = "UESTA0X (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uesta0x::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uesta0x::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uesta0x`]
module"]
pub type UESTA0X = crate::Reg<uesta0x::UESTA0X_SPEC>;
#[doc = "No Description."]
pub mod uesta0x;
#[doc = "UESTA1X (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uesta1x::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uesta1x::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uesta1x`]
module"]
pub type UESTA1X = crate::Reg<uesta1x::UESTA1X_SPEC>;
#[doc = "No Description."]
pub mod uesta1x;
