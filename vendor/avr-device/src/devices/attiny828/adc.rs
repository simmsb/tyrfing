#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    adc: ADC,
    adcsra: ADCSRA,
    adcsrb: ADCSRB,
    admuxa: ADMUXA,
    admuxb: ADMUXB,
    didr0: DIDR0,
    didr1: DIDR1,
    _reserved7: [u8; 0x5e],
    didr2: DIDR2,
    didr3: DIDR3,
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
    #[doc = "0x03 - ADC Control and Status Register B"]
    #[inline(always)]
    pub const fn adcsrb(&self) -> &ADCSRB {
        &self.adcsrb
    }
    #[doc = "0x04 - The ADC multiplexer Selection Register A"]
    #[inline(always)]
    pub const fn admuxa(&self) -> &ADMUXA {
        &self.admuxa
    }
    #[doc = "0x05 - The ADC multiplexer Selection Register B"]
    #[inline(always)]
    pub const fn admuxb(&self) -> &ADMUXB {
        &self.admuxb
    }
    #[doc = "0x06 - Digital Input Disable Register 0"]
    #[inline(always)]
    pub const fn didr0(&self) -> &DIDR0 {
        &self.didr0
    }
    #[doc = "0x07 - Digital Input Disable Register 1"]
    #[inline(always)]
    pub const fn didr1(&self) -> &DIDR1 {
        &self.didr1
    }
    #[doc = "0x66 - Digital Input Disable Register 2"]
    #[inline(always)]
    pub const fn didr2(&self) -> &DIDR2 {
        &self.didr2
    }
    #[doc = "0x67 - Digital Input Disable Register 2"]
    #[inline(always)]
    pub const fn didr3(&self) -> &DIDR3 {
        &self.didr3
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
#[doc = "ADCSRB (rw) register accessor: ADC Control and Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcsrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcsrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsrb`]
module"]
pub type ADCSRB = crate::Reg<adcsrb::ADCSRB_SPEC>;
#[doc = "ADC Control and Status Register B"]
pub mod adcsrb;
#[doc = "ADMUXA (rw) register accessor: The ADC multiplexer Selection Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`admuxa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`admuxa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@admuxa`]
module"]
pub type ADMUXA = crate::Reg<admuxa::ADMUXA_SPEC>;
#[doc = "The ADC multiplexer Selection Register A"]
pub mod admuxa;
#[doc = "ADMUXB (rw) register accessor: The ADC multiplexer Selection Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`admuxb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`admuxb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@admuxb`]
module"]
pub type ADMUXB = crate::Reg<admuxb::ADMUXB_SPEC>;
#[doc = "The ADC multiplexer Selection Register B"]
pub mod admuxb;
#[doc = "DIDR0 (rw) register accessor: Digital Input Disable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`didr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`didr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@didr0`]
module"]
pub type DIDR0 = crate::Reg<didr0::DIDR0_SPEC>;
#[doc = "Digital Input Disable Register 0"]
pub mod didr0;
#[doc = "DIDR1 (rw) register accessor: Digital Input Disable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`didr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`didr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@didr1`]
module"]
pub type DIDR1 = crate::Reg<didr1::DIDR1_SPEC>;
#[doc = "Digital Input Disable Register 1"]
pub mod didr1;
#[doc = "DIDR2 (rw) register accessor: Digital Input Disable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`didr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`didr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@didr2`]
module"]
pub type DIDR2 = crate::Reg<didr2::DIDR2_SPEC>;
#[doc = "Digital Input Disable Register 2"]
pub mod didr2;
#[doc = "DIDR3 (rw) register accessor: Digital Input Disable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`didr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`didr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@didr3`]
module"]
pub type DIDR3 = crate::Reg<didr3::DIDR3_SPEC>;
#[doc = "Digital Input Disable Register 2"]
pub mod didr3;
