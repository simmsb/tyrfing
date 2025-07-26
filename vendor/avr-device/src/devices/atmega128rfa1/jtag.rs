#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ocdr: OCDR,
    _reserved1: [u8; 0x02],
    mcusr: MCUSR,
    mcucr: MCUCR,
}
impl RegisterBlock {
    #[doc = "0x00 - On-Chip Debug Register"]
    #[inline(always)]
    pub const fn ocdr(&self) -> &OCDR {
        &self.ocdr
    }
    #[doc = "0x03 - MCU Status Register"]
    #[inline(always)]
    pub const fn mcusr(&self) -> &MCUSR {
        &self.mcusr
    }
    #[doc = "0x04 - MCU Control Register"]
    #[inline(always)]
    pub const fn mcucr(&self) -> &MCUCR {
        &self.mcucr
    }
}
#[doc = "MCUCR (rw) register accessor: MCU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcucr`]
module"]
pub type MCUCR = crate::Reg<mcucr::MCUCR_SPEC>;
#[doc = "MCU Control Register"]
pub mod mcucr;
#[doc = "MCUSR (rw) register accessor: MCU Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcusr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcusr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcusr`]
module"]
pub type MCUSR = crate::Reg<mcusr::MCUSR_SPEC>;
#[doc = "MCU Status Register"]
pub mod mcusr;
#[doc = "OCDR (rw) register accessor: On-Chip Debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocdr`]
module"]
pub type OCDR = crate::Reg<ocdr::OCDR_SPEC>;
#[doc = "On-Chip Debug Register"]
pub mod ocdr;
