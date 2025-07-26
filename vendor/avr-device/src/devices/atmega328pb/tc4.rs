#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tifr4: TIFR4,
    _reserved1: [u8; 0x38],
    timsk4: TIMSK4,
    _reserved2: [u8; 0x2d],
    tccr4a: TCCR4A,
    tccr4b: TCCR4B,
    tccr4c: TCCR4C,
    _reserved5: [u8; 0x01],
    tcnt4: TCNT4,
    icr4: ICR4,
    ocr4a: OCR4A,
    ocr4b: OCR4B,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer/Counter4 Interrupt Flag register"]
    #[inline(always)]
    pub const fn tifr4(&self) -> &TIFR4 {
        &self.tifr4
    }
    #[doc = "0x39 - Timer/Counter4 Interrupt Mask Register"]
    #[inline(always)]
    pub const fn timsk4(&self) -> &TIMSK4 {
        &self.timsk4
    }
    #[doc = "0x67 - Timer/Counter4 Control Register A"]
    #[inline(always)]
    pub const fn tccr4a(&self) -> &TCCR4A {
        &self.tccr4a
    }
    #[doc = "0x68 - Timer/Counter4 Control Register B"]
    #[inline(always)]
    pub const fn tccr4b(&self) -> &TCCR4B {
        &self.tccr4b
    }
    #[doc = "0x69 - Timer/Counter4 Control Register C"]
    #[inline(always)]
    pub const fn tccr4c(&self) -> &TCCR4C {
        &self.tccr4c
    }
    #[doc = "0x6b - Timer/Counter4 Bytes"]
    #[inline(always)]
    pub const fn tcnt4(&self) -> &TCNT4 {
        &self.tcnt4
    }
    #[doc = "0x6d - Timer/Counter4 Input Capture Register Bytes"]
    #[inline(always)]
    pub const fn icr4(&self) -> &ICR4 {
        &self.icr4
    }
    #[doc = "0x6f - Timer/Counter4 Output Compare Register Bytes"]
    #[inline(always)]
    pub const fn ocr4a(&self) -> &OCR4A {
        &self.ocr4a
    }
    #[doc = "0x71 - Timer/Counter4 Output Compare Register Bytes"]
    #[inline(always)]
    pub const fn ocr4b(&self) -> &OCR4B {
        &self.ocr4b
    }
}
#[doc = "ICR4 (rw) register accessor: Timer/Counter4 Input Capture Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr4`]
module"]
pub type ICR4 = crate::Reg<icr4::ICR4_SPEC>;
#[doc = "Timer/Counter4 Input Capture Register Bytes"]
pub mod icr4;
#[doc = "OCR4A (rw) register accessor: Timer/Counter4 Output Compare Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr4a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr4a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr4a`]
module"]
pub type OCR4A = crate::Reg<ocr4a::OCR4A_SPEC>;
#[doc = "Timer/Counter4 Output Compare Register Bytes"]
pub mod ocr4a;
#[doc = "OCR4B (rw) register accessor: Timer/Counter4 Output Compare Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr4b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr4b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr4b`]
module"]
pub type OCR4B = crate::Reg<ocr4b::OCR4B_SPEC>;
#[doc = "Timer/Counter4 Output Compare Register Bytes"]
pub mod ocr4b;
#[doc = "TCCR4A (rw) register accessor: Timer/Counter4 Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr4a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr4a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr4a`]
module"]
pub type TCCR4A = crate::Reg<tccr4a::TCCR4A_SPEC>;
#[doc = "Timer/Counter4 Control Register A"]
pub mod tccr4a;
#[doc = "TCCR4B (rw) register accessor: Timer/Counter4 Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr4b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr4b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr4b`]
module"]
pub type TCCR4B = crate::Reg<tccr4b::TCCR4B_SPEC>;
#[doc = "Timer/Counter4 Control Register B"]
pub mod tccr4b;
#[doc = "TCCR4C (rw) register accessor: Timer/Counter4 Control Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr4c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr4c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr4c`]
module"]
pub type TCCR4C = crate::Reg<tccr4c::TCCR4C_SPEC>;
#[doc = "Timer/Counter4 Control Register C"]
pub mod tccr4c;
#[doc = "TCNT4 (rw) register accessor: Timer/Counter4 Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcnt4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcnt4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcnt4`]
module"]
pub type TCNT4 = crate::Reg<tcnt4::TCNT4_SPEC>;
#[doc = "Timer/Counter4 Bytes"]
pub mod tcnt4;
#[doc = "TIFR4 (rw) register accessor: Timer/Counter4 Interrupt Flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tifr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tifr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tifr4`]
module"]
pub type TIFR4 = crate::Reg<tifr4::TIFR4_SPEC>;
#[doc = "Timer/Counter4 Interrupt Flag register"]
pub mod tifr4;
#[doc = "TIMSK4 (rw) register accessor: Timer/Counter4 Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timsk4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timsk4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timsk4`]
module"]
pub type TIMSK4 = crate::Reg<timsk4::TIMSK4_SPEC>;
#[doc = "Timer/Counter4 Interrupt Mask Register"]
pub mod timsk4;
