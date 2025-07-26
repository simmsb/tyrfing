#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    sfior: SFIOR,
    _reserved1: [u8; 0x0f],
    assr: ASSR,
    ocr0: OCR0,
    tcnt0: TCNT0,
    tccr0: TCCR0,
    _reserved5: [u8; 0x02],
    tifr: TIFR,
    timsk: TIMSK,
}
impl RegisterBlock {
    #[doc = "0x00 - Special Function IO Register"]
    #[inline(always)]
    pub const fn sfior(&self) -> &SFIOR {
        &self.sfior
    }
    #[doc = "0x10 - Asynchronus Status Register"]
    #[inline(always)]
    pub const fn assr(&self) -> &ASSR {
        &self.assr
    }
    #[doc = "0x11 - Output Compare Register"]
    #[inline(always)]
    pub const fn ocr0(&self) -> &OCR0 {
        &self.ocr0
    }
    #[doc = "0x12 - Timer/Counter Register"]
    #[inline(always)]
    pub const fn tcnt0(&self) -> &TCNT0 {
        &self.tcnt0
    }
    #[doc = "0x13 - Timer/Counter Control Register"]
    #[inline(always)]
    pub const fn tccr0(&self) -> &TCCR0 {
        &self.tccr0
    }
    #[doc = "0x16 - Timer/Counter Interrupt Flag register"]
    #[inline(always)]
    pub const fn tifr(&self) -> &TIFR {
        &self.tifr
    }
    #[doc = "0x17 - Timer/Counter Interrupt Mask Register"]
    #[inline(always)]
    pub const fn timsk(&self) -> &TIMSK {
        &self.timsk
    }
}
#[doc = "ASSR (rw) register accessor: Asynchronus Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`assr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`assr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@assr`]
module"]
pub type ASSR = crate::Reg<assr::ASSR_SPEC>;
#[doc = "Asynchronus Status Register"]
pub mod assr;
#[doc = "OCR0 (rw) register accessor: Output Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr0`]
module"]
pub type OCR0 = crate::Reg<ocr0::OCR0_SPEC>;
#[doc = "Output Compare Register"]
pub mod ocr0;
#[doc = "SFIOR (rw) register accessor: Special Function IO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfior::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfior::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfior`]
module"]
pub type SFIOR = crate::Reg<sfior::SFIOR_SPEC>;
#[doc = "Special Function IO Register"]
pub mod sfior;
#[doc = "TCCR0 (rw) register accessor: Timer/Counter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr0`]
module"]
pub type TCCR0 = crate::Reg<tccr0::TCCR0_SPEC>;
#[doc = "Timer/Counter Control Register"]
pub mod tccr0;
#[doc = "TCNT0 (rw) register accessor: Timer/Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcnt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcnt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcnt0`]
module"]
pub type TCNT0 = crate::Reg<tcnt0::TCNT0_SPEC>;
#[doc = "Timer/Counter Register"]
pub mod tcnt0;
#[doc = "TIFR (rw) register accessor: Timer/Counter Interrupt Flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tifr`]
module"]
pub type TIFR = crate::Reg<tifr::TIFR_SPEC>;
#[doc = "Timer/Counter Interrupt Flag register"]
pub mod tifr;
#[doc = "TIMSK (rw) register accessor: Timer/Counter Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timsk`]
module"]
pub type TIMSK = crate::Reg<timsk::TIMSK_SPEC>;
#[doc = "Timer/Counter Interrupt Mask Register"]
pub mod timsk;
