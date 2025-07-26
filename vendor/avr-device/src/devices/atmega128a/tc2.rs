#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ocr2: OCR2,
    tcnt2: TCNT2,
    tccr2: TCCR2,
    _reserved3: [u8; 0x10],
    tifr: TIFR,
    timsk: TIMSK,
}
impl RegisterBlock {
    #[doc = "0x00 - Output Compare Register"]
    #[inline(always)]
    pub const fn ocr2(&self) -> &OCR2 {
        &self.ocr2
    }
    #[doc = "0x01 - Timer/Counter Register"]
    #[inline(always)]
    pub const fn tcnt2(&self) -> &TCNT2 {
        &self.tcnt2
    }
    #[doc = "0x02 - Timer/Counter Control Register"]
    #[inline(always)]
    pub const fn tccr2(&self) -> &TCCR2 {
        &self.tccr2
    }
    #[doc = "0x13 - Timer/Counter Interrupt Flag Register"]
    #[inline(always)]
    pub const fn tifr(&self) -> &TIFR {
        &self.tifr
    }
    #[doc = "0x14 - No Description."]
    #[inline(always)]
    pub const fn timsk(&self) -> &TIMSK {
        &self.timsk
    }
}
#[doc = "OCR2 (rw) register accessor: Output Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr2`]
module"]
pub type OCR2 = crate::Reg<ocr2::OCR2_SPEC>;
#[doc = "Output Compare Register"]
pub mod ocr2;
#[doc = "TCCR2 (rw) register accessor: Timer/Counter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr2`]
module"]
pub type TCCR2 = crate::Reg<tccr2::TCCR2_SPEC>;
#[doc = "Timer/Counter Control Register"]
pub mod tccr2;
#[doc = "TCNT2 (rw) register accessor: Timer/Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcnt2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcnt2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcnt2`]
module"]
pub type TCNT2 = crate::Reg<tcnt2::TCNT2_SPEC>;
#[doc = "Timer/Counter Register"]
pub mod tcnt2;
#[doc = "TIFR (rw) register accessor: Timer/Counter Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tifr`]
module"]
pub type TIFR = crate::Reg<tifr::TIFR_SPEC>;
#[doc = "Timer/Counter Interrupt Flag Register"]
pub mod tifr;
#[doc = "TIMSK (rw) register accessor: No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timsk`]
module"]
pub type TIMSK = crate::Reg<timsk::TIMSK_SPEC>;
#[doc = "No Description."]
pub mod timsk;
