#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pine: PINE,
    ddre: DDRE,
    porte: PORTE,
}
impl RegisterBlock {
    #[doc = "0x00 - Port E Input Pins Address"]
    #[inline(always)]
    pub const fn pine(&self) -> &PINE {
        &self.pine
    }
    #[doc = "0x01 - Port E Data Direction Register"]
    #[inline(always)]
    pub const fn ddre(&self) -> &DDRE {
        &self.ddre
    }
    #[doc = "0x02 - Port E Data Register"]
    #[inline(always)]
    pub const fn porte(&self) -> &PORTE {
        &self.porte
    }
}
#[doc = "DDRE (rw) register accessor: Port E Data Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddre::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddre::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddre`]
module"]
pub type DDRE = crate::Reg<ddre::DDRE_SPEC>;
#[doc = "Port E Data Direction Register"]
pub mod ddre;
#[doc = "PINE (rw) register accessor: Port E Input Pins Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pine::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pine::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pine`]
module"]
pub type PINE = crate::Reg<pine::PINE_SPEC>;
#[doc = "Port E Input Pins Address"]
pub mod pine;
#[doc = "PORTE (rw) register accessor: Port E Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`porte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`porte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porte`]
module"]
pub type PORTE = crate::Reg<porte::PORTE_SPEC>;
#[doc = "Port E Data Register"]
pub mod porte;
