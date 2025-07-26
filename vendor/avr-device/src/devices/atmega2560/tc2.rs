#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tifr2: TIFR2,
    _reserved1: [u8; 0x0b],
    gtccr: GTCCR,
    _reserved2: [u8; 0x2c],
    timsk2: TIMSK2,
    _reserved3: [u8; 0x3f],
    tccr2a: TCCR2A,
    tccr2b: TCCR2B,
    tcnt2: TCNT2,
    ocr2a: OCR2A,
    ocr2b: OCR2B,
    _reserved8: [u8; 0x01],
    assr: ASSR,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer/Counter Interrupt Flag Register"]
    #[inline(always)]
    pub const fn tifr2(&self) -> &TIFR2 {
        &self.tifr2
    }
    #[doc = "0x0c - General Timer Counter Control register"]
    #[inline(always)]
    pub const fn gtccr(&self) -> &GTCCR {
        &self.gtccr
    }
    #[doc = "0x39 - Timer/Counter Interrupt Mask register"]
    #[inline(always)]
    pub const fn timsk2(&self) -> &TIMSK2 {
        &self.timsk2
    }
    #[doc = "0x79 - Timer/Counter2 Control Register A"]
    #[inline(always)]
    pub const fn tccr2a(&self) -> &TCCR2A {
        &self.tccr2a
    }
    #[doc = "0x7a - Timer/Counter2 Control Register B"]
    #[inline(always)]
    pub const fn tccr2b(&self) -> &TCCR2B {
        &self.tccr2b
    }
    #[doc = "0x7b - Timer/Counter2"]
    #[inline(always)]
    pub const fn tcnt2(&self) -> &TCNT2 {
        &self.tcnt2
    }
    #[doc = "0x7c - Timer/Counter2 Output Compare Register A"]
    #[inline(always)]
    pub const fn ocr2a(&self) -> &OCR2A {
        &self.ocr2a
    }
    #[doc = "0x7d - Timer/Counter2 Output Compare Register B"]
    #[inline(always)]
    pub const fn ocr2b(&self) -> &OCR2B {
        &self.ocr2b
    }
    #[doc = "0x7f - Asynchronous Status Register"]
    #[inline(always)]
    pub const fn assr(&self) -> &ASSR {
        &self.assr
    }
}
#[doc = "ASSR (rw) register accessor: Asynchronous Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`assr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`assr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@assr`]
module"]
pub type ASSR = crate::Reg<assr::ASSR_SPEC>;
#[doc = "Asynchronous Status Register"]
pub mod assr;
#[doc = "GTCCR (rw) register accessor: General Timer Counter Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtccr`]
module"]
pub type GTCCR = crate::Reg<gtccr::GTCCR_SPEC>;
#[doc = "General Timer Counter Control register"]
pub mod gtccr;
#[doc = "OCR2A (rw) register accessor: Timer/Counter2 Output Compare Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr2a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr2a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr2a`]
module"]
pub type OCR2A = crate::Reg<ocr2a::OCR2A_SPEC>;
#[doc = "Timer/Counter2 Output Compare Register A"]
pub mod ocr2a;
#[doc = "OCR2B (rw) register accessor: Timer/Counter2 Output Compare Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr2b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr2b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr2b`]
module"]
pub type OCR2B = crate::Reg<ocr2b::OCR2B_SPEC>;
#[doc = "Timer/Counter2 Output Compare Register B"]
pub mod ocr2b;
#[doc = "TCCR2A (rw) register accessor: Timer/Counter2 Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr2a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr2a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr2a`]
module"]
pub type TCCR2A = crate::Reg<tccr2a::TCCR2A_SPEC>;
#[doc = "Timer/Counter2 Control Register A"]
pub mod tccr2a;
#[doc = "TCCR2B (rw) register accessor: Timer/Counter2 Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr2b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr2b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr2b`]
module"]
pub type TCCR2B = crate::Reg<tccr2b::TCCR2B_SPEC>;
#[doc = "Timer/Counter2 Control Register B"]
pub mod tccr2b;
#[doc = "TCNT2 (rw) register accessor: Timer/Counter2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcnt2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcnt2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcnt2`]
module"]
pub type TCNT2 = crate::Reg<tcnt2::TCNT2_SPEC>;
#[doc = "Timer/Counter2"]
pub mod tcnt2;
#[doc = "TIFR2 (rw) register accessor: Timer/Counter Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tifr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tifr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tifr2`]
module"]
pub type TIFR2 = crate::Reg<tifr2::TIFR2_SPEC>;
#[doc = "Timer/Counter Interrupt Flag Register"]
pub mod tifr2;
#[doc = "TIMSK2 (rw) register accessor: Timer/Counter Interrupt Mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timsk2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timsk2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timsk2`]
module"]
pub type TIMSK2 = crate::Reg<timsk2::TIMSK2_SPEC>;
#[doc = "Timer/Counter Interrupt Mask register"]
pub mod timsk2;
