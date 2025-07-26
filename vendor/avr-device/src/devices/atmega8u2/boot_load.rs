#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    spmcsr: SPMCSR,
}
impl RegisterBlock {
    #[doc = "0x00 - Store Program Memory Control Register"]
    #[inline(always)]
    pub const fn spmcsr(&self) -> &SPMCSR {
        &self.spmcsr
    }
}
#[doc = "SPMCSR (rw) register accessor: Store Program Memory Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spmcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spmcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spmcsr`]
module"]
pub type SPMCSR = crate::Reg<spmcsr::SPMCSR_SPEC>;
#[doc = "Store Program Memory Control Register"]
pub mod spmcsr;
