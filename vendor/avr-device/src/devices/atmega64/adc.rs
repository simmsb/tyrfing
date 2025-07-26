#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    adc: ADC,
    adcsra: ADCSRA,
    admux: ADMUX,
    _reserved3: [u8; 0x66],
    adcsrb: ADCSRB,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC Data Register Bytes"]
    #[inline(always)]
    pub const fn adc(&self) -> &ADC {
        &self.adc
    }
    #[doc = "0x02 - The ADC Control and Status register A"]
    #[inline(always)]
    pub const fn adcsra(&self) -> &ADCSRA {
        &self.adcsra
    }
    #[doc = "0x03 - The ADC multiplexer Selection Register"]
    #[inline(always)]
    pub const fn admux(&self) -> &ADMUX {
        &self.admux
    }
    #[doc = "0x6a - The ADC Control and Status register B"]
    #[inline(always)]
    pub const fn adcsrb(&self) -> &ADCSRB {
        &self.adcsrb
    }
}
#[doc = "ADC (rw) register accessor: ADC Data Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc`]
module"]
pub type ADC = crate::Reg<adc::ADC_SPEC>;
#[doc = "ADC Data Register Bytes"]
pub mod adc;
#[doc = "ADCSRA (rw) register accessor: The ADC Control and Status register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcsra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcsra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsra`]
module"]
pub type ADCSRA = crate::Reg<adcsra::ADCSRA_SPEC>;
#[doc = "The ADC Control and Status register A"]
pub mod adcsra;
#[doc = "ADCSRB (rw) register accessor: The ADC Control and Status register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcsrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcsrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsrb`]
module"]
pub type ADCSRB = crate::Reg<adcsrb::ADCSRB_SPEC>;
#[doc = "The ADC Control and Status register B"]
pub mod adcsrb;
#[doc = "ADMUX (rw) register accessor: The ADC multiplexer Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`admux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`admux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@admux`]
module"]
pub type ADMUX = crate::Reg<admux::ADMUX_SPEC>;
#[doc = "The ADC multiplexer Selection Register"]
pub mod admux;
