#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    low: LOW,
    high: HIGH,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description."]
    #[inline(always)]
    pub const fn low(&self) -> &LOW {
        &self.low
    }
    #[doc = "0x01 - No Description."]
    #[inline(always)]
    pub const fn high(&self) -> &HIGH {
        &self.high
    }
}
#[doc = "HIGH (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@high`]
module"]
pub type HIGH = crate::Reg<high::HIGH_SPEC>;
#[doc = "No Description."]
pub mod high;
#[doc = "LOW (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@low`]
module"]
pub type LOW = crate::Reg<low::LOW_SPEC>;
#[doc = "No Description."]
pub mod low;
