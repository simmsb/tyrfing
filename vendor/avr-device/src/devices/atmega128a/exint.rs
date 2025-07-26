#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    eifr: EIFR,
    eimsk: EIMSK,
    eicrb: EICRB,
    _reserved3: [u8; 0x0f],
    eicra: EICRA,
}
impl RegisterBlock {
    #[doc = "0x00 - External Interrupt Flag Register"]
    #[inline(always)]
    pub const fn eifr(&self) -> &EIFR {
        &self.eifr
    }
    #[doc = "0x01 - External Interrupt Mask Register"]
    #[inline(always)]
    pub const fn eimsk(&self) -> &EIMSK {
        &self.eimsk
    }
    #[doc = "0x02 - External Interrupt Control Register B"]
    #[inline(always)]
    pub const fn eicrb(&self) -> &EICRB {
        &self.eicrb
    }
    #[doc = "0x12 - External Interrupt Control Register A"]
    #[inline(always)]
    pub const fn eicra(&self) -> &EICRA {
        &self.eicra
    }
}
#[doc = "EICRA (rw) register accessor: External Interrupt Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eicra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eicra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eicra`]
module"]
pub type EICRA = crate::Reg<eicra::EICRA_SPEC>;
#[doc = "External Interrupt Control Register A"]
pub mod eicra;
#[doc = "EICRB (rw) register accessor: External Interrupt Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eicrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eicrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eicrb`]
module"]
pub type EICRB = crate::Reg<eicrb::EICRB_SPEC>;
#[doc = "External Interrupt Control Register B"]
pub mod eicrb;
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
