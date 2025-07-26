#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tifr2: TIFR2,
    timsk2: TIMSK2,
    _reserved2: [u8; 0x11],
    gtccr: GTCCR,
    _reserved3: [u8; 0x7c],
    icr2: ICR2,
    ocr2b: OCR2B,
    ocr2a: OCR2A,
    tcnt2: TCNT2,
    tccr2c: TCCR2C,
    tccr2b: TCCR2B,
    tccr2a: TCCR2A,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer/Counter Interrupt Flag register"]
    #[inline(always)]
    pub const fn tifr2(&self) -> &TIFR2 {
        &self.tifr2
    }
    #[doc = "0x01 - Timer/Counter2 Interrupt Mask Register"]
    #[inline(always)]
    pub const fn timsk2(&self) -> &TIMSK2 {
        &self.timsk2
    }
    #[doc = "0x13 - General Timer/Counter Control Register"]
    #[inline(always)]
    pub const fn gtccr(&self) -> &GTCCR {
        &self.gtccr
    }
    #[doc = "0x90 - Timer/Counter2 Input Capture Register Bytes"]
    #[inline(always)]
    pub const fn icr2(&self) -> &ICR2 {
        &self.icr2
    }
    #[doc = "0x92 - Timer/Counter2 Output Compare Register B Bytes"]
    #[inline(always)]
    pub const fn ocr2b(&self) -> &OCR2B {
        &self.ocr2b
    }
    #[doc = "0x94 - Timer/Counter2 Output Compare Register A Bytes"]
    #[inline(always)]
    pub const fn ocr2a(&self) -> &OCR2A {
        &self.ocr2a
    }
    #[doc = "0x96 - Timer/Counter2 Bytes"]
    #[inline(always)]
    pub const fn tcnt2(&self) -> &TCNT2 {
        &self.tcnt2
    }
    #[doc = "0x98 - Timer/Counter2 Control Register C"]
    #[inline(always)]
    pub const fn tccr2c(&self) -> &TCCR2C {
        &self.tccr2c
    }
    #[doc = "0x99 - Timer/Counter2 Control Register B"]
    #[inline(always)]
    pub const fn tccr2b(&self) -> &TCCR2B {
        &self.tccr2b
    }
    #[doc = "0x9a - Timer/Counter2 Control Register A"]
    #[inline(always)]
    pub const fn tccr2a(&self) -> &TCCR2A {
        &self.tccr2a
    }
}
#[doc = "GTCCR (rw) register accessor: General Timer/Counter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtccr`]
module"]
pub type GTCCR = crate::Reg<gtccr::GTCCR_SPEC>;
#[doc = "General Timer/Counter Control Register"]
pub mod gtccr;
#[doc = "ICR2 (rw) register accessor: Timer/Counter2 Input Capture Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr2`]
module"]
pub type ICR2 = crate::Reg<icr2::ICR2_SPEC>;
#[doc = "Timer/Counter2 Input Capture Register Bytes"]
pub mod icr2;
#[doc = "OCR2A (rw) register accessor: Timer/Counter2 Output Compare Register A Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr2a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr2a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr2a`]
module"]
pub type OCR2A = crate::Reg<ocr2a::OCR2A_SPEC>;
#[doc = "Timer/Counter2 Output Compare Register A Bytes"]
pub mod ocr2a;
#[doc = "OCR2B (rw) register accessor: Timer/Counter2 Output Compare Register B Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr2b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr2b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr2b`]
module"]
pub type OCR2B = crate::Reg<ocr2b::OCR2B_SPEC>;
#[doc = "Timer/Counter2 Output Compare Register B Bytes"]
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
#[doc = "TCCR2C (rw) register accessor: Timer/Counter2 Control Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr2c`]
module"]
pub type TCCR2C = crate::Reg<tccr2c::TCCR2C_SPEC>;
#[doc = "Timer/Counter2 Control Register C"]
pub mod tccr2c;
#[doc = "TCNT2 (rw) register accessor: Timer/Counter2 Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcnt2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcnt2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcnt2`]
module"]
pub type TCNT2 = crate::Reg<tcnt2::TCNT2_SPEC>;
#[doc = "Timer/Counter2 Bytes"]
pub mod tcnt2;
#[doc = "TIFR2 (rw) register accessor: Timer/Counter Interrupt Flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tifr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tifr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tifr2`]
module"]
pub type TIFR2 = crate::Reg<tifr2::TIFR2_SPEC>;
#[doc = "Timer/Counter Interrupt Flag register"]
pub mod tifr2;
#[doc = "TIMSK2 (rw) register accessor: Timer/Counter2 Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timsk2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timsk2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timsk2`]
module"]
pub type TIMSK2 = crate::Reg<timsk2::TIMSK2_SPEC>;
#[doc = "Timer/Counter2 Interrupt Mask Register"]
pub mod timsk2;
