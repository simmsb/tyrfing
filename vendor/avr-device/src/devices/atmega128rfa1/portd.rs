#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pind: PIND,
    ddrd: DDRD,
    portd: PORTD,
}
impl RegisterBlock {
    #[doc = "0x00 - Port D Input Pins Address"]
    #[inline(always)]
    pub const fn pind(&self) -> &PIND {
        &self.pind
    }
    #[doc = "0x01 - Port D Data Direction Register"]
    #[inline(always)]
    pub const fn ddrd(&self) -> &DDRD {
        &self.ddrd
    }
    #[doc = "0x02 - Port D Data Register"]
    #[inline(always)]
    pub const fn portd(&self) -> &PORTD {
        &self.portd
    }
}
#[doc = "DDRD (rw) register accessor: Port D Data Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrd`]
module"]
pub type DDRD = crate::Reg<ddrd::DDRD_SPEC>;
#[doc = "Port D Data Direction Register"]
pub mod ddrd;
#[doc = "PIND (rw) register accessor: Port D Input Pins Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pind::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pind::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pind`]
module"]
pub type PIND = crate::Reg<pind::PIND_SPEC>;
#[doc = "Port D Input Pins Address"]
pub mod pind;
#[doc = "PORTD (rw) register accessor: Port D Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portd`]
module"]
pub type PORTD = crate::Reg<portd::PORTD_SPEC>;
#[doc = "Port D Data Register"]
pub mod portd;
