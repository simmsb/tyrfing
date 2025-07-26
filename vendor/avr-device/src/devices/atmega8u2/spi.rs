#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    spcr: SPCR,
    spsr: SPSR,
    spdr: SPDR,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI Control Register"]
    #[inline(always)]
    pub const fn spcr(&self) -> &SPCR {
        &self.spcr
    }
    #[doc = "0x01 - SPI Status Register"]
    #[inline(always)]
    pub const fn spsr(&self) -> &SPSR {
        &self.spsr
    }
    #[doc = "0x02 - SPI Data Register"]
    #[inline(always)]
    pub const fn spdr(&self) -> &SPDR {
        &self.spdr
    }
}
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
