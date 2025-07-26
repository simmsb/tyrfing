#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tccr1c: TCCR1C,
    _reserved1: [u8; 0x01],
    icr1: ICR1,
    _reserved2: [u8; 0x02],
    ocr1b: OCR1B,
    ocr1a: OCR1A,
    tcnt1: TCNT1,
    tccr1b: TCCR1B,
    tccr1a: TCCR1A,
    _reserved7: [u8; 0x08],
    tifr: TIFR,
    timsk: TIMSK,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer/Counter1 Control Register C"]
    #[inline(always)]
    pub const fn tccr1c(&self) -> &TCCR1C {
        &self.tccr1c
    }
    #[doc = "0x02 - Timer/Counter1 Input Capture Register Bytes"]
    #[inline(always)]
    pub const fn icr1(&self) -> &ICR1 {
        &self.icr1
    }
    #[doc = "0x06 - Timer/Counter1 Output Compare Register Bytes"]
    #[inline(always)]
    pub const fn ocr1b(&self) -> &OCR1B {
        &self.ocr1b
    }
    #[doc = "0x08 - Timer/Counter1 Output Compare Register Bytes"]
    #[inline(always)]
    pub const fn ocr1a(&self) -> &OCR1A {
        &self.ocr1a
    }
    #[doc = "0x0a - Timer/Counter1 Bytes"]
    #[inline(always)]
    pub const fn tcnt1(&self) -> &TCNT1 {
        &self.tcnt1
    }
    #[doc = "0x0c - Timer/Counter1 Control Register B"]
    #[inline(always)]
    pub const fn tccr1b(&self) -> &TCCR1B {
        &self.tccr1b
    }
    #[doc = "0x0d - Timer/Counter1 Control Register A"]
    #[inline(always)]
    pub const fn tccr1a(&self) -> &TCCR1A {
        &self.tccr1a
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
#[doc = "ICR1 (rw) register accessor: Timer/Counter1 Input Capture Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr1`]
module"]
pub type ICR1 = crate::Reg<icr1::ICR1_SPEC>;
#[doc = "Timer/Counter1 Input Capture Register Bytes"]
pub mod icr1;
#[doc = "OCR1A (rw) register accessor: Timer/Counter1 Output Compare Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr1a`]
module"]
pub type OCR1A = crate::Reg<ocr1a::OCR1A_SPEC>;
#[doc = "Timer/Counter1 Output Compare Register Bytes"]
pub mod ocr1a;
#[doc = "OCR1B (rw) register accessor: Timer/Counter1 Output Compare Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr1b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr1b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr1b`]
module"]
pub type OCR1B = crate::Reg<ocr1b::OCR1B_SPEC>;
#[doc = "Timer/Counter1 Output Compare Register Bytes"]
pub mod ocr1b;
#[doc = "TCCR1A (rw) register accessor: Timer/Counter1 Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr1a`]
module"]
pub type TCCR1A = crate::Reg<tccr1a::TCCR1A_SPEC>;
#[doc = "Timer/Counter1 Control Register A"]
pub mod tccr1a;
#[doc = "TCCR1B (rw) register accessor: Timer/Counter1 Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr1b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr1b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr1b`]
module"]
pub type TCCR1B = crate::Reg<tccr1b::TCCR1B_SPEC>;
#[doc = "Timer/Counter1 Control Register B"]
pub mod tccr1b;
#[doc = "TCCR1C (rw) register accessor: Timer/Counter1 Control Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr1c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr1c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr1c`]
module"]
pub type TCCR1C = crate::Reg<tccr1c::TCCR1C_SPEC>;
#[doc = "Timer/Counter1 Control Register C"]
pub mod tccr1c;
#[doc = "TCNT1 (rw) register accessor: Timer/Counter1 Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcnt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcnt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcnt1`]
module"]
pub type TCNT1 = crate::Reg<tcnt1::TCNT1_SPEC>;
#[doc = "Timer/Counter1 Bytes"]
pub mod tcnt1;
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
