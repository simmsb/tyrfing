#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pcmsk: PCMSK,
    _reserved1: [u8; 0x1f],
    mcucr: MCUCR,
    _reserved2: [u8; 0x04],
    gifr: GIFR,
    gimsk: GIMSK,
}
impl RegisterBlock {
    #[doc = "0x00 - Pin Change Enable Mask"]
    #[inline(always)]
    pub const fn pcmsk(&self) -> &PCMSK {
        &self.pcmsk
    }
    #[doc = "0x20 - MCU Control Register"]
    #[inline(always)]
    pub const fn mcucr(&self) -> &MCUCR {
        &self.mcucr
    }
    #[doc = "0x25 - General Interrupt Flag register"]
    #[inline(always)]
    pub const fn gifr(&self) -> &GIFR {
        &self.gifr
    }
    #[doc = "0x26 - General Interrupt Mask Register"]
    #[inline(always)]
    pub const fn gimsk(&self) -> &GIMSK {
        &self.gimsk
    }
}
#[doc = "GIFR (rw) register accessor: General Interrupt Flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gifr`]
module"]
pub type GIFR = crate::Reg<gifr::GIFR_SPEC>;
#[doc = "General Interrupt Flag register"]
pub mod gifr;
#[doc = "GIMSK (rw) register accessor: General Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gimsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gimsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gimsk`]
module"]
pub type GIMSK = crate::Reg<gimsk::GIMSK_SPEC>;
#[doc = "General Interrupt Mask Register"]
pub mod gimsk;
#[doc = "MCUCR (rw) register accessor: MCU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcucr`]
module"]
pub type MCUCR = crate::Reg<mcucr::MCUCR_SPEC>;
#[doc = "MCU Control Register"]
pub mod mcucr;
#[doc = "PCMSK (rw) register accessor: Pin Change Enable Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmsk`]
module"]
pub type PCMSK = crate::Reg<pcmsk::PCMSK_SPEC>;
#[doc = "Pin Change Enable Mask"]
pub mod pcmsk;
