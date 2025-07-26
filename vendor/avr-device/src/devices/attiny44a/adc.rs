#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    didr0: DIDR0,
    _reserved1: [u8; 0x01],
    adcsrb: ADCSRB,
    adc: ADC,
    adcsra: ADCSRA,
    admux: ADMUX,
}
impl RegisterBlock {
    #[doc = "0x00 - Digital Input Disable Register 0"]
    #[inline(always)]
    pub const fn didr0(&self) -> &DIDR0 {
        &self.didr0
    }
    #[doc = "0x02 - ADC Control and Status Register B"]
    #[inline(always)]
    pub const fn adcsrb(&self) -> &ADCSRB {
        &self.adcsrb
    }
    #[doc = "0x03 - ADC Data Register Bytes"]
    #[inline(always)]
    pub const fn adc(&self) -> &ADC {
        &self.adc
    }
    #[doc = "0x05 - ADC Control and Status Register A"]
    #[inline(always)]
    pub const fn adcsra(&self) -> &ADCSRA {
        &self.adcsra
    }
    #[doc = "0x06 - ADC Multiplexer Selection Register"]
    #[inline(always)]
    pub const fn admux(&self) -> &ADMUX {
        &self.admux
    }
}
#[doc = "ADC (rw) register accessor: ADC Data Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc`]
module"]
pub type ADC = crate::Reg<adc::ADC_SPEC>;
#[doc = "ADC Data Register Bytes"]
pub mod adc;
#[doc = "ADCSRA (rw) register accessor: ADC Control and Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcsra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcsra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsra`]
module"]
pub type ADCSRA = crate::Reg<adcsra::ADCSRA_SPEC>;
#[doc = "ADC Control and Status Register A"]
pub mod adcsra;
#[doc = "ADCSRB (rw) register accessor: ADC Control and Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcsrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcsrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsrb`]
module"]
pub type ADCSRB = crate::Reg<adcsrb::ADCSRB_SPEC>;
#[doc = "ADC Control and Status Register B"]
pub mod adcsrb;
#[doc = "ADMUX (rw) register accessor: ADC Multiplexer Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`admux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`admux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@admux`]
module"]
pub type ADMUX = crate::Reg<admux::ADMUX_SPEC>;
#[doc = "ADC Multiplexer Selection Register"]
pub mod admux;
#[doc = "DIDR0 (rw) register accessor: Digital Input Disable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`didr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`didr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@didr0`]
module"]
pub type DIDR0 = crate::Reg<didr0::DIDR0_SPEC>;
#[doc = "Digital Input Disable Register 0"]
pub mod didr0;
