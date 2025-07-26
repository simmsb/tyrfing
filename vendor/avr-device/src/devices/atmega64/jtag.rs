#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ocdr: OCDR,
    _reserved1: [u8; 0x11],
    mcucsr: MCUCSR,
}
impl RegisterBlock {
    #[doc = "0x00 - On-Chip Debug Related Register in I/O Memory"]
    #[inline(always)]
    pub const fn ocdr(&self) -> &OCDR {
        &self.ocdr
    }
    #[doc = "0x12 - MCU Control And Status Register"]
    #[inline(always)]
    pub const fn mcucsr(&self) -> &MCUCSR {
        &self.mcucsr
    }
}
#[doc = "MCUCSR (rw) register accessor: MCU Control And Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcucsr`]
module"]
pub type MCUCSR = crate::Reg<mcucsr::MCUCSR_SPEC>;
#[doc = "MCU Control And Status Register"]
pub mod mcucsr;
#[doc = "OCDR (rw) register accessor: On-Chip Debug Related Register in I/O Memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocdr`]
module"]
pub type OCDR = crate::Reg<ocdr::OCDR_SPEC>;
#[doc = "On-Chip Debug Related Register in I/O Memory"]
pub mod ocdr;
