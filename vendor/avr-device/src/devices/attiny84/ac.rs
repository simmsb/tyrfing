#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    didr0: DIDR0,
    _reserved1: [u8; 0x01],
    adcsrb: ADCSRB,
    _reserved2: [u8; 0x04],
    acsr: ACSR,
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
    #[doc = "0x07 - Analog Comparator Control And Status Register"]
    #[inline(always)]
    pub const fn acsr(&self) -> &ACSR {
        &self.acsr
    }
}
#[doc = "ACSR (rw) register accessor: Analog Comparator Control And Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acsr`]
module"]
pub type ACSR = crate::Reg<acsr::ACSR_SPEC>;
#[doc = "Analog Comparator Control And Status Register"]
pub mod acsr;
#[doc = "ADCSRB (rw) register accessor: ADC Control and Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcsrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcsrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsrb`]
module"]
pub type ADCSRB = crate::Reg<adcsrb::ADCSRB_SPEC>;
#[doc = "ADC Control and Status Register B"]
pub mod adcsrb;
#[doc = "DIDR0 (rw) register accessor: Digital Input Disable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`didr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`didr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@didr0`]
module"]
pub type DIDR0 = crate::Reg<didr0::DIDR0_SPEC>;
#[doc = "Digital Input Disable Register 0"]
pub mod didr0;
