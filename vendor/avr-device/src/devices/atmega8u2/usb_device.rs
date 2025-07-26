#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    regcr: REGCR,
    _reserved1: [u8; 0x74],
    usbcon: USBCON,
    _reserved2: [u8; 0x07],
    udcon: UDCON,
    udint: UDINT,
    udien: UDIEN,
    udaddr: UDADDR,
    udfnum: UDFNUM,
    udmfn: UDMFN,
    _reserved8: [u8; 0x01],
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
    _reserved19: [u8; 0x01],
    ueint: UEINT,
    _reserved20: [u8; 0x06],
    upoe: UPOE,
}
impl RegisterBlock {
    #[doc = "0x00 - Regulator Control Register"]
    #[inline(always)]
    pub const fn regcr(&self) -> &REGCR {
        &self.regcr
    }
    #[doc = "0x75 - USB General Control Register"]
    #[inline(always)]
    pub const fn usbcon(&self) -> &USBCON {
        &self.usbcon
    }
    #[doc = "0x7d - USB Device Control Registers"]
    #[inline(always)]
    pub const fn udcon(&self) -> &UDCON {
        &self.udcon
    }
    #[doc = "0x7e - USB Device Interrupt Register"]
    #[inline(always)]
    pub const fn udint(&self) -> &UDINT {
        &self.udint
    }
    #[doc = "0x7f - USB Device Interrupt Enable Register"]
    #[inline(always)]
    pub const fn udien(&self) -> &UDIEN {
        &self.udien
    }
    #[doc = "0x80 - USB Device Address Register"]
    #[inline(always)]
    pub const fn udaddr(&self) -> &UDADDR {
        &self.udaddr
    }
    #[doc = "0x81 - USB Device Frame Number High Register"]
    #[inline(always)]
    pub const fn udfnum(&self) -> &UDFNUM {
        &self.udfnum
    }
    #[doc = "0x83 - USB Device Micro Frame Number"]
    #[inline(always)]
    pub const fn udmfn(&self) -> &UDMFN {
        &self.udmfn
    }
    #[doc = "0x85 - USB Endpoint Interrupt Register"]
    #[inline(always)]
    pub const fn ueintx(&self) -> &UEINTX {
        &self.ueintx
    }
    #[doc = "0x86 - USB Endpoint Number"]
    #[inline(always)]
    pub const fn uenum(&self) -> &UENUM {
        &self.uenum
    }
    #[doc = "0x87 - USB Endpoint Reset Register"]
    #[inline(always)]
    pub const fn uerst(&self) -> &UERST {
        &self.uerst
    }
    #[doc = "0x88 - USB Endpoint Control Register"]
    #[inline(always)]
    pub const fn ueconx(&self) -> &UECONX {
        &self.ueconx
    }
    #[doc = "0x89 - USB Endpoint Configuration 0 Register"]
    #[inline(always)]
    pub const fn uecfg0x(&self) -> &UECFG0X {
        &self.uecfg0x
    }
    #[doc = "0x8a - USB Endpoint Configuration 1 Register"]
    #[inline(always)]
    pub const fn uecfg1x(&self) -> &UECFG1X {
        &self.uecfg1x
    }
    #[doc = "0x8b - USB Endpoint Status 0 Register"]
    #[inline(always)]
    pub const fn uesta0x(&self) -> &UESTA0X {
        &self.uesta0x
    }
    #[doc = "0x8c - USB Endpoint Status 1 Register"]
    #[inline(always)]
    pub const fn uesta1x(&self) -> &UESTA1X {
        &self.uesta1x
    }
    #[doc = "0x8d - USB Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ueienx(&self) -> &UEIENX {
        &self.ueienx
    }
    #[doc = "0x8e - USB Data Endpoint"]
    #[inline(always)]
    pub const fn uedatx(&self) -> &UEDATX {
        &self.uedatx
    }
    #[doc = "0x8f - USB Endpoint Byte Count Register"]
    #[inline(always)]
    pub const fn uebclx(&self) -> &UEBCLX {
        &self.uebclx
    }
    #[doc = "0x91 - USB Endpoint Number Interrupt Register"]
    #[inline(always)]
    pub const fn ueint(&self) -> &UEINT {
        &self.ueint
    }
    #[doc = "0x98 - USB Software Output Enable register"]
    #[inline(always)]
    pub const fn upoe(&self) -> &UPOE {
        &self.upoe
    }
}
#[doc = "REGCR (rw) register accessor: Regulator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regcr`]
module"]
pub type REGCR = crate::Reg<regcr::REGCR_SPEC>;
#[doc = "Regulator Control Register"]
pub mod regcr;
#[doc = "UDADDR (rw) register accessor: USB Device Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udaddr`]
module"]
pub type UDADDR = crate::Reg<udaddr::UDADDR_SPEC>;
#[doc = "USB Device Address Register"]
pub mod udaddr;
#[doc = "UDCON (rw) register accessor: USB Device Control Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udcon`]
module"]
pub type UDCON = crate::Reg<udcon::UDCON_SPEC>;
#[doc = "USB Device Control Registers"]
pub mod udcon;
#[doc = "UDFNUM (rw) register accessor: USB Device Frame Number High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udfnum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udfnum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udfnum`]
module"]
pub type UDFNUM = crate::Reg<udfnum::UDFNUM_SPEC>;
#[doc = "USB Device Frame Number High Register"]
pub mod udfnum;
#[doc = "UDIEN (rw) register accessor: USB Device Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udien`]
module"]
pub type UDIEN = crate::Reg<udien::UDIEN_SPEC>;
#[doc = "USB Device Interrupt Enable Register"]
pub mod udien;
#[doc = "UDINT (rw) register accessor: USB Device Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udint`]
module"]
pub type UDINT = crate::Reg<udint::UDINT_SPEC>;
#[doc = "USB Device Interrupt Register"]
pub mod udint;
#[doc = "UDMFN (rw) register accessor: USB Device Micro Frame Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmfn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmfn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmfn`]
module"]
pub type UDMFN = crate::Reg<udmfn::UDMFN_SPEC>;
#[doc = "USB Device Micro Frame Number"]
pub mod udmfn;
#[doc = "UEBCLX (rw) register accessor: USB Endpoint Byte Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uebclx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uebclx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uebclx`]
module"]
pub type UEBCLX = crate::Reg<uebclx::UEBCLX_SPEC>;
#[doc = "USB Endpoint Byte Count Register"]
pub mod uebclx;
#[doc = "UECFG0X (rw) register accessor: USB Endpoint Configuration 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uecfg0x::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uecfg0x::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uecfg0x`]
module"]
pub type UECFG0X = crate::Reg<uecfg0x::UECFG0X_SPEC>;
#[doc = "USB Endpoint Configuration 0 Register"]
pub mod uecfg0x;
#[doc = "UECFG1X (rw) register accessor: USB Endpoint Configuration 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uecfg1x::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uecfg1x::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uecfg1x`]
module"]
pub type UECFG1X = crate::Reg<uecfg1x::UECFG1X_SPEC>;
#[doc = "USB Endpoint Configuration 1 Register"]
pub mod uecfg1x;
#[doc = "UECONX (rw) register accessor: USB Endpoint Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ueconx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ueconx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ueconx`]
module"]
pub type UECONX = crate::Reg<ueconx::UECONX_SPEC>;
#[doc = "USB Endpoint Control Register"]
pub mod ueconx;
#[doc = "UEDATX (rw) register accessor: USB Data Endpoint\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uedatx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uedatx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uedatx`]
module"]
pub type UEDATX = crate::Reg<uedatx::UEDATX_SPEC>;
#[doc = "USB Data Endpoint"]
pub mod uedatx;
#[doc = "UEIENX (rw) register accessor: USB Endpoint Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ueienx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ueienx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ueienx`]
module"]
pub type UEIENX = crate::Reg<ueienx::UEIENX_SPEC>;
#[doc = "USB Endpoint Interrupt Enable Register"]
pub mod ueienx;
#[doc = "UEINT (rw) register accessor: USB Endpoint Number Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ueint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ueint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ueint`]
module"]
pub type UEINT = crate::Reg<ueint::UEINT_SPEC>;
#[doc = "USB Endpoint Number Interrupt Register"]
pub mod ueint;
#[doc = "UEINTX (rw) register accessor: USB Endpoint Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ueintx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ueintx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ueintx`]
module"]
pub type UEINTX = crate::Reg<ueintx::UEINTX_SPEC>;
#[doc = "USB Endpoint Interrupt Register"]
pub mod ueintx;
#[doc = "UENUM (rw) register accessor: USB Endpoint Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uenum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uenum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uenum`]
module"]
pub type UENUM = crate::Reg<uenum::UENUM_SPEC>;
#[doc = "USB Endpoint Number"]
pub mod uenum;
#[doc = "UERST (rw) register accessor: USB Endpoint Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uerst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uerst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uerst`]
module"]
pub type UERST = crate::Reg<uerst::UERST_SPEC>;
#[doc = "USB Endpoint Reset Register"]
pub mod uerst;
#[doc = "UESTA0X (rw) register accessor: USB Endpoint Status 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uesta0x::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uesta0x::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uesta0x`]
module"]
pub type UESTA0X = crate::Reg<uesta0x::UESTA0X_SPEC>;
#[doc = "USB Endpoint Status 0 Register"]
pub mod uesta0x;
#[doc = "UESTA1X (rw) register accessor: USB Endpoint Status 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uesta1x::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uesta1x::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uesta1x`]
module"]
pub type UESTA1X = crate::Reg<uesta1x::UESTA1X_SPEC>;
#[doc = "USB Endpoint Status 1 Register"]
pub mod uesta1x;
#[doc = "UPOE (rw) register accessor: USB Software Output Enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upoe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upoe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upoe`]
module"]
pub type UPOE = crate::Reg<upoe::UPOE_SPEC>;
#[doc = "USB Software Output Enable register"]
pub mod upoe;
#[doc = "USBCON (rw) register accessor: USB General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbcon`]
module"]
pub type USBCON = crate::Reg<usbcon::USBCON_SPEC>;
#[doc = "USB General Control Register"]
pub mod usbcon;
