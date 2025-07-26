#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pinh: PINH,
    ddrh: DDRH,
    porth: PORTH,
}
impl RegisterBlock {
    #[doc = "0x00 - PORT H Input Pins"]
    #[inline(always)]
    pub const fn pinh(&self) -> &PINH {
        &self.pinh
    }
    #[doc = "0x01 - PORT H Data Direction Register"]
    #[inline(always)]
    pub const fn ddrh(&self) -> &DDRH {
        &self.ddrh
    }
    #[doc = "0x02 - PORT H Data Register"]
    #[inline(always)]
    pub const fn porth(&self) -> &PORTH {
        &self.porth
    }
}
#[doc = "DDRH (rw) register accessor: PORT H Data Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrh`]
module"]
pub type DDRH = crate::Reg<ddrh::DDRH_SPEC>;
#[doc = "PORT H Data Direction Register"]
pub mod ddrh;
#[doc = "PINH (rw) register accessor: PORT H Input Pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinh`]
module"]
pub type PINH = crate::Reg<pinh::PINH_SPEC>;
#[doc = "PORT H Input Pins"]
pub mod pinh;
#[doc = "PORTH (rw) register accessor: PORT H Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`porth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`porth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porth`]
module"]
pub type PORTH = crate::Reg<porth::PORTH_SPEC>;
#[doc = "PORT H Data Register"]
pub mod porth;
