#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pine: PINE,
    ddre: DDRE,
    porte: PORTE,
}
impl RegisterBlock {
    #[doc = "0x00 - Input Pins, Port E"]
    #[inline(always)]
    pub const fn pine(&self) -> &PINE {
        &self.pine
    }
    #[doc = "0x01 - Data Direction Register, Port E"]
    #[inline(always)]
    pub const fn ddre(&self) -> &DDRE {
        &self.ddre
    }
    #[doc = "0x02 - Data Register, Port E"]
    #[inline(always)]
    pub const fn porte(&self) -> &PORTE {
        &self.porte
    }
}
#[doc = "DDRE (rw) register accessor: Data Direction Register, Port E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddre::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddre::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddre`]
module"]
pub type DDRE = crate::Reg<ddre::DDRE_SPEC>;
#[doc = "Data Direction Register, Port E"]
pub mod ddre;
#[doc = "PINE (rw) register accessor: Input Pins, Port E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pine::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pine::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pine`]
module"]
pub type PINE = crate::Reg<pine::PINE_SPEC>;
#[doc = "Input Pins, Port E"]
pub mod pine;
#[doc = "PORTE (rw) register accessor: Data Register, Port E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`porte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`porte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porte`]
module"]
pub type PORTE = crate::Reg<porte::PORTE_SPEC>;
#[doc = "Data Register, Port E"]
pub mod porte;
