#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    remap: REMAP,
    _reserved1: [u8; 0x4a],
    spdr: SPDR,
    spsr: SPSR,
    spcr: SPCR,
}
impl RegisterBlock {
    #[doc = "0x00 - Remap Port Pins"]
    #[inline(always)]
    pub const fn remap(&self) -> &REMAP {
        &self.remap
    }
    #[doc = "0x4b - SPI Data Register"]
    #[inline(always)]
    pub const fn spdr(&self) -> &SPDR {
        &self.spdr
    }
    #[doc = "0x4c - SPI Status Register"]
    #[inline(always)]
    pub const fn spsr(&self) -> &SPSR {
        &self.spsr
    }
    #[doc = "0x4d - SPI Control Register"]
    #[inline(always)]
    pub const fn spcr(&self) -> &SPCR {
        &self.spcr
    }
}
#[doc = "REMAP (rw) register accessor: Remap Port Pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap`]
module"]
pub type REMAP = crate::Reg<remap::REMAP_SPEC>;
#[doc = "Remap Port Pins"]
pub mod remap;
#[doc = "SPCR (rw) register accessor: SPI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spcr`]
module"]
pub type SPCR = crate::Reg<spcr::SPCR_SPEC>;
#[doc = "SPI Control Register"]
pub mod spcr;
#[doc = "SPDR (rw) register accessor: SPI Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdr`]
module"]
pub type SPDR = crate::Reg<spdr::SPDR_SPEC>;
#[doc = "SPI Data Register"]
pub mod spdr;
#[doc = "SPSR (rw) register accessor: SPI Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spsr`]
module"]
pub type SPSR = crate::Reg<spsr::SPSR_SPEC>;
#[doc = "SPI Status Register"]
pub mod spsr;
