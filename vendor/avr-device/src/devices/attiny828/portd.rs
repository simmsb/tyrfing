#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pind: PIND,
    ddrd: DDRD,
    portd: PORTD,
    pued: PUED,
}
impl RegisterBlock {
    #[doc = "0x00 - Port D Input Pins"]
    #[inline(always)]
    pub const fn pind(&self) -> &PIND {
        &self.pind
    }
    #[doc = "0x01 - Data Direction Register, Port D"]
    #[inline(always)]
    pub const fn ddrd(&self) -> &DDRD {
        &self.ddrd
    }
    #[doc = "0x02 - Port D Data Register"]
    #[inline(always)]
    pub const fn portd(&self) -> &PORTD {
        &self.portd
    }
    #[doc = "0x03 - Pull-up Enable Control Register"]
    #[inline(always)]
    pub const fn pued(&self) -> &PUED {
        &self.pued
    }
}
#[doc = "DDRD (rw) register accessor: Data Direction Register, Port D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrd`]
module"]
pub type DDRD = crate::Reg<ddrd::DDRD_SPEC>;
#[doc = "Data Direction Register, Port D"]
pub mod ddrd;
#[doc = "PIND (rw) register accessor: Port D Input Pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pind::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pind::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pind`]
module"]
pub type PIND = crate::Reg<pind::PIND_SPEC>;
#[doc = "Port D Input Pins"]
pub mod pind;
#[doc = "PORTD (rw) register accessor: Port D Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portd`]
module"]
pub type PORTD = crate::Reg<portd::PORTD_SPEC>;
#[doc = "Port D Data Register"]
pub mod portd;
#[doc = "PUED (rw) register accessor: Pull-up Enable Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pued::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pued::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pued`]
module"]
pub type PUED = crate::Reg<pued::PUED_SPEC>;
#[doc = "Pull-up Enable Control Register"]
pub mod pued;
