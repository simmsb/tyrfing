#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pinc: PINC,
    ddrc: DDRC,
    portc: PORTC,
}
impl RegisterBlock {
    #[doc = "0x00 - Port C Input Pins"]
    #[inline(always)]
    pub const fn pinc(&self) -> &PINC {
        &self.pinc
    }
    #[doc = "0x01 - Port C Data Direction Register"]
    #[inline(always)]
    pub const fn ddrc(&self) -> &DDRC {
        &self.ddrc
    }
    #[doc = "0x02 - Port C Data Register"]
    #[inline(always)]
    pub const fn portc(&self) -> &PORTC {
        &self.portc
    }
}
#[doc = "DDRC (rw) register accessor: Port C Data Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrc`]
module"]
pub type DDRC = crate::Reg<ddrc::DDRC_SPEC>;
#[doc = "Port C Data Direction Register"]
pub mod ddrc;
#[doc = "PINC (rw) register accessor: Port C Input Pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinc`]
module"]
pub type PINC = crate::Reg<pinc::PINC_SPEC>;
#[doc = "Port C Input Pins"]
pub mod pinc;
#[doc = "PORTC (rw) register accessor: Port C Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portc`]
module"]
pub type PORTC = crate::Reg<portc::PORTC_SPEC>;
#[doc = "Port C Data Register"]
pub mod portc;
