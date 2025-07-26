#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tocpmcoe: TOCPMCOE,
    _reserved1: [u8; 0x05],
    tocpmsa0: TOCPMSA0,
    tocpmsa1: TOCPMSA1,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer Output Compare Pin Mux Channel Output Enable"]
    #[inline(always)]
    pub const fn tocpmcoe(&self) -> &TOCPMCOE {
        &self.tocpmcoe
    }
    #[doc = "0x06 - Timer Output Compare Pin Mux Selection 0"]
    #[inline(always)]
    pub const fn tocpmsa0(&self) -> &TOCPMSA0 {
        &self.tocpmsa0
    }
    #[doc = "0x07 - Timer Output Compare Pin Mux Selection 1"]
    #[inline(always)]
    pub const fn tocpmsa1(&self) -> &TOCPMSA1 {
        &self.tocpmsa1
    }
}
#[doc = "TOCPMCOE (rw) register accessor: Timer Output Compare Pin Mux Channel Output Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocpmcoe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocpmcoe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocpmcoe`]
module"]
pub type TOCPMCOE = crate::Reg<tocpmcoe::TOCPMCOE_SPEC>;
#[doc = "Timer Output Compare Pin Mux Channel Output Enable"]
pub mod tocpmcoe;
#[doc = "TOCPMSA0 (rw) register accessor: Timer Output Compare Pin Mux Selection 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocpmsa0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocpmsa0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocpmsa0`]
module"]
pub type TOCPMSA0 = crate::Reg<tocpmsa0::TOCPMSA0_SPEC>;
#[doc = "Timer Output Compare Pin Mux Selection 0"]
pub mod tocpmsa0;
#[doc = "TOCPMSA1 (rw) register accessor: Timer Output Compare Pin Mux Selection 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocpmsa1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocpmsa1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocpmsa1`]
module"]
pub type TOCPMSA1 = crate::Reg<tocpmsa1::TOCPMSA1_SPEC>;
#[doc = "Timer Output Compare Pin Mux Selection 1"]
pub mod tocpmsa1;
