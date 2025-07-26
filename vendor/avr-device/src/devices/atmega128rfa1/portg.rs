#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ping: PING,
    ddrg: DDRG,
    portg: PORTG,
}
impl RegisterBlock {
    #[doc = "0x00 - Port G Input Pins Address"]
    #[inline(always)]
    pub const fn ping(&self) -> &PING {
        &self.ping
    }
    #[doc = "0x01 - Port G Data Direction Register"]
    #[inline(always)]
    pub const fn ddrg(&self) -> &DDRG {
        &self.ddrg
    }
    #[doc = "0x02 - Port G Data Register"]
    #[inline(always)]
    pub const fn portg(&self) -> &PORTG {
        &self.portg
    }
}
#[doc = "DDRG (rw) register accessor: Port G Data Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrg`]
module"]
pub type DDRG = crate::Reg<ddrg::DDRG_SPEC>;
#[doc = "Port G Data Direction Register"]
pub mod ddrg;
#[doc = "PING (rw) register accessor: Port G Input Pins Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ping::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ping::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ping`]
module"]
pub type PING = crate::Reg<ping::PING_SPEC>;
#[doc = "Port G Input Pins Address"]
pub mod ping;
#[doc = "PORTG (rw) register accessor: Port G Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portg`]
module"]
pub type PORTG = crate::Reg<portg::PORTG_SPEC>;
#[doc = "Port G Data Register"]
pub mod portg;
