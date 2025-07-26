#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tifr3: TIFR3,
    _reserved1: [u8; 0x38],
    timsk3: TIMSK3,
    _reserved2: [u8; 0x1e],
    tccr3a: TCCR3A,
    tccr3b: TCCR3B,
    tccr3c: TCCR3C,
    _reserved5: [u8; 0x01],
    tcnt3: TCNT3,
    icr3: ICR3,
    ocr3a: OCR3A,
    ocr3b: OCR3B,
    ocr3c: OCR3C,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer/Counter3 Interrupt Flag Register"]
    #[inline(always)]
    pub const fn tifr3(&self) -> &TIFR3 {
        &self.tifr3
    }
    #[doc = "0x39 - Timer/Counter3 Interrupt Mask Register"]
    #[inline(always)]
    pub const fn timsk3(&self) -> &TIMSK3 {
        &self.timsk3
    }
    #[doc = "0x58 - Timer/Counter3 Control Register A"]
    #[inline(always)]
    pub const fn tccr3a(&self) -> &TCCR3A {
        &self.tccr3a
    }
    #[doc = "0x59 - Timer/Counter3 Control Register B"]
    #[inline(always)]
    pub const fn tccr3b(&self) -> &TCCR3B {
        &self.tccr3b
    }
    #[doc = "0x5a - Timer/Counter3 Control Register C"]
    #[inline(always)]
    pub const fn tccr3c(&self) -> &TCCR3C {
        &self.tccr3c
    }
    #[doc = "0x5c - Timer/Counter3 Bytes"]
    #[inline(always)]
    pub const fn tcnt3(&self) -> &TCNT3 {
        &self.tcnt3
    }
    #[doc = "0x5e - Timer/Counter3 Input Capture Register Bytes"]
    #[inline(always)]
    pub const fn icr3(&self) -> &ICR3 {
        &self.icr3
    }
    #[doc = "0x60 - Timer/Counter3 Output Compare Register A Bytes"]
    #[inline(always)]
    pub const fn ocr3a(&self) -> &OCR3A {
        &self.ocr3a
    }
    #[doc = "0x62 - Timer/Counter3 Output Compare Register B Bytes"]
    #[inline(always)]
    pub const fn ocr3b(&self) -> &OCR3B {
        &self.ocr3b
    }
    #[doc = "0x64 - Timer/Counter3 Output Compare Register C Bytes"]
    #[inline(always)]
    pub const fn ocr3c(&self) -> &OCR3C {
        &self.ocr3c
    }
}
#[doc = "ICR3 (rw) register accessor: Timer/Counter3 Input Capture Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr3`]
module"]
pub type ICR3 = crate::Reg<icr3::ICR3_SPEC>;
#[doc = "Timer/Counter3 Input Capture Register Bytes"]
pub mod icr3;
#[doc = "OCR3A (rw) register accessor: Timer/Counter3 Output Compare Register A Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr3a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr3a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr3a`]
module"]
pub type OCR3A = crate::Reg<ocr3a::OCR3A_SPEC>;
#[doc = "Timer/Counter3 Output Compare Register A Bytes"]
pub mod ocr3a;
#[doc = "OCR3B (rw) register accessor: Timer/Counter3 Output Compare Register B Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr3b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr3b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr3b`]
module"]
pub type OCR3B = crate::Reg<ocr3b::OCR3B_SPEC>;
#[doc = "Timer/Counter3 Output Compare Register B Bytes"]
pub mod ocr3b;
#[doc = "OCR3C (rw) register accessor: Timer/Counter3 Output Compare Register C Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr3c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr3c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr3c`]
module"]
pub type OCR3C = crate::Reg<ocr3c::OCR3C_SPEC>;
#[doc = "Timer/Counter3 Output Compare Register C Bytes"]
pub mod ocr3c;
#[doc = "TCCR3A (rw) register accessor: Timer/Counter3 Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr3a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr3a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr3a`]
module"]
pub type TCCR3A = crate::Reg<tccr3a::TCCR3A_SPEC>;
#[doc = "Timer/Counter3 Control Register A"]
pub mod tccr3a;
#[doc = "TCCR3B (rw) register accessor: Timer/Counter3 Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr3b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr3b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr3b`]
module"]
pub type TCCR3B = crate::Reg<tccr3b::TCCR3B_SPEC>;
#[doc = "Timer/Counter3 Control Register B"]
pub mod tccr3b;
#[doc = "TCCR3C (rw) register accessor: Timer/Counter3 Control Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr3c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr3c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr3c`]
module"]
pub type TCCR3C = crate::Reg<tccr3c::TCCR3C_SPEC>;
#[doc = "Timer/Counter3 Control Register C"]
pub mod tccr3c;
#[doc = "TCNT3 (rw) register accessor: Timer/Counter3 Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcnt3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcnt3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcnt3`]
module"]
pub type TCNT3 = crate::Reg<tcnt3::TCNT3_SPEC>;
#[doc = "Timer/Counter3 Bytes"]
pub mod tcnt3;
#[doc = "TIFR3 (rw) register accessor: Timer/Counter3 Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tifr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tifr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tifr3`]
module"]
pub type TIFR3 = crate::Reg<tifr3::TIFR3_SPEC>;
#[doc = "Timer/Counter3 Interrupt Flag Register"]
pub mod tifr3;
#[doc = "TIMSK3 (rw) register accessor: Timer/Counter3 Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timsk3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timsk3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timsk3`]
module"]
pub type TIMSK3 = crate::Reg<timsk3::TIMSK3_SPEC>;
#[doc = "Timer/Counter3 Interrupt Mask Register"]
pub mod timsk3;
