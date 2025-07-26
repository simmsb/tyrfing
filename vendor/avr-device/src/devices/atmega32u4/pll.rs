#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pllcsr: PLLCSR,
    _reserved1: [u8; 0x08],
    pllfrq: PLLFRQ,
}
impl RegisterBlock {
    #[doc = "0x00 - PLL Status and Control register"]
    #[inline(always)]
    pub const fn pllcsr(&self) -> &PLLCSR {
        &self.pllcsr
    }
    #[doc = "0x09 - PLL Frequency Control Register"]
    #[inline(always)]
    pub const fn pllfrq(&self) -> &PLLFRQ {
        &self.pllfrq
    }
}
#[doc = "PLLCSR (rw) register accessor: PLL Status and Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcsr`]
module"]
pub type PLLCSR = crate::Reg<pllcsr::PLLCSR_SPEC>;
#[doc = "PLL Status and Control register"]
pub mod pllcsr;
#[doc = "PLLFRQ (rw) register accessor: PLL Frequency Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllfrq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllfrq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllfrq`]
module"]
pub type PLLFRQ = crate::Reg<pllfrq::PLLFRQ_SPEC>;
#[doc = "PLL Frequency Control Register"]
pub mod pllfrq;
