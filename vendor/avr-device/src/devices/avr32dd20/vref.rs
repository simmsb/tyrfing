#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    adc0ref: ADC0REF,
    _reserved1: [u8; 0x01],
    dac0ref: DAC0REF,
    _reserved2: [u8; 0x01],
    acref: ACREF,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC0 Reference"]
    #[inline(always)]
    pub const fn adc0ref(&self) -> &ADC0REF {
        &self.adc0ref
    }
    #[doc = "0x02 - DAC0 Reference"]
    #[inline(always)]
    pub const fn dac0ref(&self) -> &DAC0REF {
        &self.dac0ref
    }
    #[doc = "0x04 - AC Reference"]
    #[inline(always)]
    pub const fn acref(&self) -> &ACREF {
        &self.acref
    }
}
#[doc = "ACREF (rw) register accessor: AC Reference\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acref::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acref::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acref`]
module"]
pub type ACREF = crate::Reg<acref::ACREF_SPEC>;
#[doc = "AC Reference"]
pub mod acref;
#[doc = "ADC0REF (rw) register accessor: ADC0 Reference\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc0ref::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc0ref::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0ref`]
module"]
pub type ADC0REF = crate::Reg<adc0ref::ADC0REF_SPEC>;
#[doc = "ADC0 Reference"]
pub mod adc0ref;
#[doc = "DAC0REF (rw) register accessor: DAC0 Reference\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0ref::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0ref::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0ref`]
module"]
pub type DAC0REF = crate::Reg<dac0ref::DAC0REF_SPEC>;
#[doc = "DAC0 Reference"]
pub mod dac0ref;
