#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pcmsk1: PCMSK1,
    pcmsk2: PCMSK2,
    _reserved2: [u8; 0x1a],
    pcmsk0: PCMSK0,
    _reserved3: [u8; 0x19],
    gifr: GIFR,
    gimsk: GIMSK,
}
impl RegisterBlock {
    #[doc = "0x00 - Pin Change Interrupt Mask Register 1"]
    #[inline(always)]
    pub const fn pcmsk1(&self) -> &PCMSK1 {
        &self.pcmsk1
    }
    #[doc = "0x01 - Pin Change Interrupt Mask Register 2"]
    #[inline(always)]
    pub const fn pcmsk2(&self) -> &PCMSK2 {
        &self.pcmsk2
    }
    #[doc = "0x1c - Pin Change Interrupt Mask Register 0"]
    #[inline(always)]
    pub const fn pcmsk0(&self) -> &PCMSK0 {
        &self.pcmsk0
    }
    #[doc = "0x36 - General Interrupt Flag Register"]
    #[inline(always)]
    pub const fn gifr(&self) -> &GIFR {
        &self.gifr
    }
    #[doc = "0x37 - General Interrupt Mask Register"]
    #[inline(always)]
    pub const fn gimsk(&self) -> &GIMSK {
        &self.gimsk
    }
}
#[doc = "GIFR (rw) register accessor: General Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gifr`]
module"]
pub type GIFR = crate::Reg<gifr::GIFR_SPEC>;
#[doc = "General Interrupt Flag Register"]
pub mod gifr;
#[doc = "GIMSK (rw) register accessor: General Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gimsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gimsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gimsk`]
module"]
pub type GIMSK = crate::Reg<gimsk::GIMSK_SPEC>;
#[doc = "General Interrupt Mask Register"]
pub mod gimsk;
#[doc = "PCMSK0 (rw) register accessor: Pin Change Interrupt Mask Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmsk0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmsk0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmsk0`]
module"]
pub type PCMSK0 = crate::Reg<pcmsk0::PCMSK0_SPEC>;
#[doc = "Pin Change Interrupt Mask Register 0"]
pub mod pcmsk0;
#[doc = "PCMSK1 (rw) register accessor: Pin Change Interrupt Mask Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmsk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmsk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmsk1`]
module"]
pub type PCMSK1 = crate::Reg<pcmsk1::PCMSK1_SPEC>;
#[doc = "Pin Change Interrupt Mask Register 1"]
pub mod pcmsk1;
#[doc = "PCMSK2 (rw) register accessor: Pin Change Interrupt Mask Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmsk2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmsk2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmsk2`]
module"]
pub type PCMSK2 = crate::Reg<pcmsk2::PCMSK2_SPEC>;
#[doc = "Pin Change Interrupt Mask Register 2"]
pub mod pcmsk2;
