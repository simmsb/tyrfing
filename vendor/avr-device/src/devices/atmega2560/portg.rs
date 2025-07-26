#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ping: PING,
    ddrg: DDRG,
    portg: PORTG,
}
impl RegisterBlock {
    #[doc = "0x00 - Input Pins, Port G"]
    #[inline(always)]
    pub const fn ping(&self) -> &PING {
        &self.ping
    }
    #[doc = "0x01 - Data Direction Register, Port G"]
    #[inline(always)]
    pub const fn ddrg(&self) -> &DDRG {
        &self.ddrg
    }
    #[doc = "0x02 - Data Register, Port G"]
    #[inline(always)]
    pub const fn portg(&self) -> &PORTG {
        &self.portg
    }
}
#[doc = "DDRG (rw) register accessor: Data Direction Register, Port G\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrg`]
module"]
pub type DDRG = crate::Reg<ddrg::DDRG_SPEC>;
#[doc = "Data Direction Register, Port G"]
pub mod ddrg;
#[doc = "PING (rw) register accessor: Input Pins, Port G\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ping::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ping::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ping`]
module"]
pub type PING = crate::Reg<ping::PING_SPEC>;
#[doc = "Input Pins, Port G"]
pub mod ping;
#[doc = "PORTG (rw) register accessor: Data Register, Port G\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portg`]
module"]
pub type PORTG = crate::Reg<portg::PORTG_SPEC>;
#[doc = "Data Register, Port G"]
pub mod portg;
