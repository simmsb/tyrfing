#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    uhwcon: UHWCON,
    usbcon: USBCON,
    usbsta: USBSTA,
    usbint: USBINT,
}
impl RegisterBlock {
    #[doc = "0x00 - USB Hardware Configuration Register"]
    #[inline(always)]
    pub const fn uhwcon(&self) -> &UHWCON {
        &self.uhwcon
    }
    #[doc = "0x01 - USB General Control Register"]
    #[inline(always)]
    pub const fn usbcon(&self) -> &USBCON {
        &self.usbcon
    }
    #[doc = "0x02 - No Description."]
    #[inline(always)]
    pub const fn usbsta(&self) -> &USBSTA {
        &self.usbsta
    }
    #[doc = "0x03 - No Description."]
    #[inline(always)]
    pub const fn usbint(&self) -> &USBINT {
        &self.usbint
    }
}
#[doc = "UHWCON (rw) register accessor: USB Hardware Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhwcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhwcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhwcon`]
module"]
pub type UHWCON = crate::Reg<uhwcon::UHWCON_SPEC>;
#[doc = "USB Hardware Configuration Register"]
pub mod uhwcon;
#[doc = "USBCON (rw) register accessor: USB General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbcon`]
module"]
pub type USBCON = crate::Reg<usbcon::USBCON_SPEC>;
#[doc = "USB General Control Register"]
pub mod usbcon;
#[doc = "USBINT (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbint`]
module"]
pub type USBINT = crate::Reg<usbint::USBINT_SPEC>;
#[doc = "No Description."]
pub mod usbint;
#[doc = "USBSTA (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbsta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbsta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbsta`]
module"]
pub type USBSTA = crate::Reg<usbsta::USBSTA_SPEC>;
#[doc = "No Description."]
pub mod usbsta;
