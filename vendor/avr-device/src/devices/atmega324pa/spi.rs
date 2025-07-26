#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    spcr0: SPCR0,
    spsr0: SPSR0,
    spdr0: SPDR0,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI Control Register"]
    #[inline(always)]
    pub const fn spcr0(&self) -> &SPCR0 {
        &self.spcr0
    }
    #[doc = "0x01 - SPI Status Register"]
    #[inline(always)]
    pub const fn spsr0(&self) -> &SPSR0 {
        &self.spsr0
    }
    #[doc = "0x02 - SPI Data Register"]
    #[inline(always)]
    pub const fn spdr0(&self) -> &SPDR0 {
        &self.spdr0
    }
}
#[doc = "SPCR0 (rw) register accessor: SPI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spcr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spcr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spcr0`]
module"]
pub type SPCR0 = crate::Reg<spcr0::SPCR0_SPEC>;
#[doc = "SPI Control Register"]
pub mod spcr0;
#[doc = "SPDR0 (rw) register accessor: SPI Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdr0`]
module"]
pub type SPDR0 = crate::Reg<spdr0::SPDR0_SPEC>;
#[doc = "SPI Data Register"]
pub mod spdr0;
#[doc = "SPSR0 (rw) register accessor: SPI Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spsr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spsr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spsr0`]
module"]
pub type SPSR0 = crate::Reg<spsr0::SPSR0_SPEC>;
#[doc = "SPI Status Register"]
pub mod spsr0;
