#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pcifr: PCIFR,
    eifr: EIFR,
    eimsk: EIMSK,
    _reserved3: [u8; 0x2a],
    pcicr: PCICR,
    eicra: EICRA,
    _reserved5: [u8; 0x01],
    pcmsk0: PCMSK0,
    pcmsk1: PCMSK1,
    pcmsk2: PCMSK2,
    _reserved8: [u8; 0x05],
    pcmsk3: PCMSK3,
}
impl RegisterBlock {
    #[doc = "0x00 - Pin Change Interrupt Flag Register"]
    #[inline(always)]
    pub const fn pcifr(&self) -> &PCIFR {
        &self.pcifr
    }
    #[doc = "0x01 - External Interrupt Flag Register"]
    #[inline(always)]
    pub const fn eifr(&self) -> &EIFR {
        &self.eifr
    }
    #[doc = "0x02 - External Interrupt Mask Register"]
    #[inline(always)]
    pub const fn eimsk(&self) -> &EIMSK {
        &self.eimsk
    }
    #[doc = "0x2d - Pin Change Interrupt Control Register"]
    #[inline(always)]
    pub const fn pcicr(&self) -> &PCICR {
        &self.pcicr
    }
    #[doc = "0x2e - External Interrupt Control Register A"]
    #[inline(always)]
    pub const fn eicra(&self) -> &EICRA {
        &self.eicra
    }
    #[doc = "0x30 - Pin Change Mask Register 0"]
    #[inline(always)]
    pub const fn pcmsk0(&self) -> &PCMSK0 {
        &self.pcmsk0
    }
    #[doc = "0x31 - Pin Change Mask Register 1"]
    #[inline(always)]
    pub const fn pcmsk1(&self) -> &PCMSK1 {
        &self.pcmsk1
    }
    #[doc = "0x32 - Pin Change Mask Register 2"]
    #[inline(always)]
    pub const fn pcmsk2(&self) -> &PCMSK2 {
        &self.pcmsk2
    }
    #[doc = "0x38 - Pin Change Mask Register 3"]
    #[inline(always)]
    pub const fn pcmsk3(&self) -> &PCMSK3 {
        &self.pcmsk3
    }
}
#[doc = "EICRA (rw) register accessor: External Interrupt Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eicra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eicra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eicra`]
module"]
pub type EICRA = crate::Reg<eicra::EICRA_SPEC>;
#[doc = "External Interrupt Control Register A"]
pub mod eicra;
#[doc = "EIFR (rw) register accessor: External Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eifr`]
module"]
pub type EIFR = crate::Reg<eifr::EIFR_SPEC>;
#[doc = "External Interrupt Flag Register"]
pub mod eifr;
#[doc = "EIMSK (rw) register accessor: External Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eimsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eimsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eimsk`]
module"]
pub type EIMSK = crate::Reg<eimsk::EIMSK_SPEC>;
#[doc = "External Interrupt Mask Register"]
pub mod eimsk;
#[doc = "PCICR (rw) register accessor: Pin Change Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcicr`]
module"]
pub type PCICR = crate::Reg<pcicr::PCICR_SPEC>;
#[doc = "Pin Change Interrupt Control Register"]
pub mod pcicr;
#[doc = "PCIFR (rw) register accessor: Pin Change Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcifr`]
module"]
pub type PCIFR = crate::Reg<pcifr::PCIFR_SPEC>;
#[doc = "Pin Change Interrupt Flag Register"]
pub mod pcifr;
#[doc = "PCMSK0 (rw) register accessor: Pin Change Mask Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmsk0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmsk0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmsk0`]
module"]
pub type PCMSK0 = crate::Reg<pcmsk0::PCMSK0_SPEC>;
#[doc = "Pin Change Mask Register 0"]
pub mod pcmsk0;
#[doc = "PCMSK1 (rw) register accessor: Pin Change Mask Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmsk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmsk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmsk1`]
module"]
pub type PCMSK1 = crate::Reg<pcmsk1::PCMSK1_SPEC>;
#[doc = "Pin Change Mask Register 1"]
pub mod pcmsk1;
#[doc = "PCMSK2 (rw) register accessor: Pin Change Mask Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmsk2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmsk2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmsk2`]
module"]
pub type PCMSK2 = crate::Reg<pcmsk2::PCMSK2_SPEC>;
#[doc = "Pin Change Mask Register 2"]
pub mod pcmsk2;
#[doc = "PCMSK3 (rw) register accessor: Pin Change Mask Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmsk3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmsk3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmsk3`]
module"]
pub type PCMSK3 = crate::Reg<pcmsk3::PCMSK3_SPEC>;
#[doc = "Pin Change Mask Register 3"]
pub mod pcmsk3;
