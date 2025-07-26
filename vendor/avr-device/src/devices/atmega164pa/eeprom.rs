#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    eecr: EECR,
    eedr: EEDR,
    eear: EEAR,
}
impl RegisterBlock {
    #[doc = "0x00 - EEPROM Control Register"]
    #[inline(always)]
    pub const fn eecr(&self) -> &EECR {
        &self.eecr
    }
    #[doc = "0x01 - EEPROM Data Register"]
    #[inline(always)]
    pub const fn eedr(&self) -> &EEDR {
        &self.eedr
    }
    #[doc = "0x02 - EEPROM Address Register Low Bytes"]
    #[inline(always)]
    pub const fn eear(&self) -> &EEAR {
        &self.eear
    }
}
#[doc = "EEAR (rw) register accessor: EEPROM Address Register Low Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eear`]
module"]
pub type EEAR = crate::Reg<eear::EEAR_SPEC>;
#[doc = "EEPROM Address Register Low Bytes"]
pub mod eear;
#[doc = "EECR (rw) register accessor: EEPROM Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eecr`]
module"]
pub type EECR = crate::Reg<eecr::EECR_SPEC>;
#[doc = "EEPROM Control Register"]
pub mod eecr;
#[doc = "EEDR (rw) register accessor: EEPROM Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eedr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eedr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eedr`]
module"]
pub type EEDR = crate::Reg<eedr::EEDR_SPEC>;
#[doc = "EEPROM Data Register"]
pub mod eedr;
