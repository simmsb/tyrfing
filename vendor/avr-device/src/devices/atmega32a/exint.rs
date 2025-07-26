#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mcucsr: MCUCSR,
    mcucr: MCUCR,
    _reserved2: [u8; 0x04],
    gifr: GIFR,
    gicr: GICR,
}
impl RegisterBlock {
    #[doc = "0x00 - MCU Control And Status Register"]
    #[inline(always)]
    pub const fn mcucsr(&self) -> &MCUCSR {
        &self.mcucsr
    }
    #[doc = "0x01 - General Interrupt Control Register"]
    #[inline(always)]
    pub const fn mcucr(&self) -> &MCUCR {
        &self.mcucr
    }
    #[doc = "0x06 - General Interrupt Flag Register"]
    #[inline(always)]
    pub const fn gifr(&self) -> &GIFR {
        &self.gifr
    }
    #[doc = "0x07 - General Interrupt Control Register"]
    #[inline(always)]
    pub const fn gicr(&self) -> &GICR {
        &self.gicr
    }
}
#[doc = "GICR (rw) register accessor: General Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicr`]
module"]
pub type GICR = crate::Reg<gicr::GICR_SPEC>;
#[doc = "General Interrupt Control Register"]
pub mod gicr;
#[doc = "GIFR (rw) register accessor: General Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gifr`]
module"]
pub type GIFR = crate::Reg<gifr::GIFR_SPEC>;
#[doc = "General Interrupt Flag Register"]
pub mod gifr;
#[doc = "MCUCR (rw) register accessor: General Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcucr`]
module"]
pub type MCUCR = crate::Reg<mcucr::MCUCR_SPEC>;
#[doc = "General Interrupt Control Register"]
pub mod mcucr;
#[doc = "MCUCSR (rw) register accessor: MCU Control And Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcucsr`]
module"]
pub type MCUCSR = crate::Reg<mcucsr::MCUCSR_SPEC>;
#[doc = "MCU Control And Status Register"]
pub mod mcucsr;
