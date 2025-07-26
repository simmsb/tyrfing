#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pinj: PINJ,
    ddrj: DDRJ,
    portj: PORTJ,
}
impl RegisterBlock {
    #[doc = "0x00 - PORT J Input Pins"]
    #[inline(always)]
    pub const fn pinj(&self) -> &PINJ {
        &self.pinj
    }
    #[doc = "0x01 - PORT J Data Direction Register"]
    #[inline(always)]
    pub const fn ddrj(&self) -> &DDRJ {
        &self.ddrj
    }
    #[doc = "0x02 - PORT J Data Register"]
    #[inline(always)]
    pub const fn portj(&self) -> &PORTJ {
        &self.portj
    }
}
#[doc = "DDRJ (rw) register accessor: PORT J Data Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrj::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrj::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrj`]
module"]
pub type DDRJ = crate::Reg<ddrj::DDRJ_SPEC>;
#[doc = "PORT J Data Direction Register"]
pub mod ddrj;
#[doc = "PINJ (rw) register accessor: PORT J Input Pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinj::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinj::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinj`]
module"]
pub type PINJ = crate::Reg<pinj::PINJ_SPEC>;
#[doc = "PORT J Input Pins"]
pub mod pinj;
#[doc = "PORTJ (rw) register accessor: PORT J Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portj::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portj::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portj`]
module"]
pub type PORTJ = crate::Reg<portj::PORTJ_SPEC>;
#[doc = "PORT J Data Register"]
pub mod portj;
