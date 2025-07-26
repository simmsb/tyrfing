#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pink: PINK,
    ddrk: DDRK,
    portk: PORTK,
}
impl RegisterBlock {
    #[doc = "0x00 - PORT K Input Pins"]
    #[inline(always)]
    pub const fn pink(&self) -> &PINK {
        &self.pink
    }
    #[doc = "0x01 - PORT K Data Direction Register"]
    #[inline(always)]
    pub const fn ddrk(&self) -> &DDRK {
        &self.ddrk
    }
    #[doc = "0x02 - PORT K Data Register"]
    #[inline(always)]
    pub const fn portk(&self) -> &PORTK {
        &self.portk
    }
}
#[doc = "DDRK (rw) register accessor: PORT K Data Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrk`]
module"]
pub type DDRK = crate::Reg<ddrk::DDRK_SPEC>;
#[doc = "PORT K Data Direction Register"]
pub mod ddrk;
#[doc = "PINK (rw) register accessor: PORT K Input Pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pink::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pink::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pink`]
module"]
pub type PINK = crate::Reg<pink::PINK_SPEC>;
#[doc = "PORT K Input Pins"]
pub mod pink;
#[doc = "PORTK (rw) register accessor: PORT K Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portk`]
module"]
pub type PORTK = crate::Reg<portk::PORTK_SPEC>;
#[doc = "PORT K Data Register"]
pub mod portk;
