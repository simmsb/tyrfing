#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    acsr: ACSR,
    _reserved1: [u8; 0x2a],
    adcsrb: ADCSRB,
}
impl RegisterBlock {
    #[doc = "0x00 - Analog Comparator Control And Status Register"]
    #[inline(always)]
    pub const fn acsr(&self) -> &ACSR {
        &self.acsr
    }
    #[doc = "0x2b - Analog Comparator &amp; ADC Control and Status Register B (Shared with AD_CONVERTER IO_MODULE)"]
    #[inline(always)]
    pub const fn adcsrb(&self) -> &ADCSRB {
        &self.adcsrb
    }
}
#[doc = "ACSR (rw) register accessor: Analog Comparator Control And Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acsr`]
module"]
pub type ACSR = crate::Reg<acsr::ACSR_SPEC>;
#[doc = "Analog Comparator Control And Status Register"]
pub mod acsr;
#[doc = "ADCSRB (rw) register accessor: Analog Comparator &amp; ADC Control and Status Register B (Shared with AD_CONVERTER IO_MODULE)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcsrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcsrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsrb`]
module"]
pub type ADCSRB = crate::Reg<adcsrb::ADCSRB_SPEC>;
#[doc = "Analog Comparator &amp; ADC Control and Status Register B (Shared with AD_CONVERTER IO_MODULE)"]
pub mod adcsrb;
