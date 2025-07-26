#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    xfdcsr: XFDCSR,
}
impl RegisterBlock {
    #[doc = "0x00 - XOSC Failure Detection Control and Status Register"]
    #[inline(always)]
    pub const fn xfdcsr(&self) -> &XFDCSR {
        &self.xfdcsr
    }
}
#[doc = "XFDCSR (rw) register accessor: XOSC Failure Detection Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xfdcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xfdcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xfdcsr`]
module"]
pub type XFDCSR = crate::Reg<xfdcsr::XFDCSR_SPEC>;
#[doc = "XOSC Failure Detection Control and Status Register"]
pub mod xfdcsr;
