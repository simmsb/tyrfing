#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    adc: ADC,
    adcsra: ADCSRA,
    admux: ADMUX,
    _reserved3: [u8; 0x28],
    sfior: SFIOR,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC Data Register Bytes"]
    #[inline(always)]
    pub const fn adc(&self) -> &ADC {
        &self.adc
    }
    #[doc = "0x02 - The ADC Control and Status register"]
    #[inline(always)]
    pub const fn adcsra(&self) -> &ADCSRA {
        &self.adcsra
    }
    #[doc = "0x03 - The ADC multiplexer Selection Register"]
    #[inline(always)]
    pub const fn admux(&self) -> &ADMUX {
        &self.admux
    }
    #[doc = "0x2c - Special Function IO Register"]
    #[inline(always)]
    pub const fn sfior(&self) -> &SFIOR {
        &self.sfior
    }
}
#[doc = "ADC (rw) register accessor: ADC Data Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc`]
module"]
pub type ADC = crate::Reg<adc::ADC_SPEC>;
#[doc = "ADC Data Register Bytes"]
pub mod adc;
#[doc = "ADCSRA (rw) register accessor: The ADC Control and Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcsra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcsra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsra`]
module"]
pub type ADCSRA = crate::Reg<adcsra::ADCSRA_SPEC>;
#[doc = "The ADC Control and Status register"]
pub mod adcsra;
#[doc = "ADMUX (rw) register accessor: The ADC multiplexer Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`admux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`admux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@admux`]
module"]
pub type ADMUX = crate::Reg<admux::ADMUX_SPEC>;
#[doc = "The ADC multiplexer Selection Register"]
pub mod admux;
#[doc = "SFIOR (rw) register accessor: Special Function IO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfior::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfior::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfior`]
module"]
pub type SFIOR = crate::Reg<sfior::SFIOR_SPEC>;
#[doc = "Special Function IO Register"]
pub mod sfior;
