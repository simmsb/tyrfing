#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tifr5: TIFR5,
    _reserved1: [u8; 0x38],
    timsk5: TIMSK5,
    _reserved2: [u8; 0xac],
    tccr5a: TCCR5A,
    tccr5b: TCCR5B,
    tccr5c: TCCR5C,
    _reserved5: [u8; 0x01],
    tcnt5: TCNT5,
    icr5: ICR5,
    ocr5a: OCR5A,
    ocr5b: OCR5B,
    ocr5c: OCR5C,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer/Counter5 Interrupt Flag Register"]
    #[inline(always)]
    pub const fn tifr5(&self) -> &TIFR5 {
        &self.tifr5
    }
    #[doc = "0x39 - Timer/Counter5 Interrupt Mask Register"]
    #[inline(always)]
    pub const fn timsk5(&self) -> &TIMSK5 {
        &self.timsk5
    }
    #[doc = "0xe6 - Timer/Counter5 Control Register A"]
    #[inline(always)]
    pub const fn tccr5a(&self) -> &TCCR5A {
        &self.tccr5a
    }
    #[doc = "0xe7 - Timer/Counter5 Control Register B"]
    #[inline(always)]
    pub const fn tccr5b(&self) -> &TCCR5B {
        &self.tccr5b
    }
    #[doc = "0xe8 - Timer/Counter5 Control Register C"]
    #[inline(always)]
    pub const fn tccr5c(&self) -> &TCCR5C {
        &self.tccr5c
    }
    #[doc = "0xea - Timer/Counter5 Bytes"]
    #[inline(always)]
    pub const fn tcnt5(&self) -> &TCNT5 {
        &self.tcnt5
    }
    #[doc = "0xec - Timer/Counter5 Input Capture Register Bytes"]
    #[inline(always)]
    pub const fn icr5(&self) -> &ICR5 {
        &self.icr5
    }
    #[doc = "0xee - Timer/Counter5 Output Compare Register A Bytes"]
    #[inline(always)]
    pub const fn ocr5a(&self) -> &OCR5A {
        &self.ocr5a
    }
    #[doc = "0xf0 - Timer/Counter5 Output Compare Register B Bytes"]
    #[inline(always)]
    pub const fn ocr5b(&self) -> &OCR5B {
        &self.ocr5b
    }
    #[doc = "0xf2 - Timer/Counter5 Output Compare Register C Bytes"]
    #[inline(always)]
    pub const fn ocr5c(&self) -> &OCR5C {
        &self.ocr5c
    }
}
#[doc = "ICR5 (rw) register accessor: Timer/Counter5 Input Capture Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr5`]
module"]
pub type ICR5 = crate::Reg<icr5::ICR5_SPEC>;
#[doc = "Timer/Counter5 Input Capture Register Bytes"]
pub mod icr5;
#[doc = "OCR5A (rw) register accessor: Timer/Counter5 Output Compare Register A Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr5a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr5a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr5a`]
module"]
pub type OCR5A = crate::Reg<ocr5a::OCR5A_SPEC>;
#[doc = "Timer/Counter5 Output Compare Register A Bytes"]
pub mod ocr5a;
#[doc = "OCR5B (rw) register accessor: Timer/Counter5 Output Compare Register B Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr5b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr5b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr5b`]
module"]
pub type OCR5B = crate::Reg<ocr5b::OCR5B_SPEC>;
#[doc = "Timer/Counter5 Output Compare Register B Bytes"]
pub mod ocr5b;
#[doc = "OCR5C (rw) register accessor: Timer/Counter5 Output Compare Register C Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr5c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr5c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr5c`]
module"]
pub type OCR5C = crate::Reg<ocr5c::OCR5C_SPEC>;
#[doc = "Timer/Counter5 Output Compare Register C Bytes"]
pub mod ocr5c;
#[doc = "TCCR5A (rw) register accessor: Timer/Counter5 Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr5a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr5a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr5a`]
module"]
pub type TCCR5A = crate::Reg<tccr5a::TCCR5A_SPEC>;
#[doc = "Timer/Counter5 Control Register A"]
pub mod tccr5a;
#[doc = "TCCR5B (rw) register accessor: Timer/Counter5 Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr5b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr5b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr5b`]
module"]
pub type TCCR5B = crate::Reg<tccr5b::TCCR5B_SPEC>;
#[doc = "Timer/Counter5 Control Register B"]
pub mod tccr5b;
#[doc = "TCCR5C (rw) register accessor: Timer/Counter5 Control Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr5c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr5c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr5c`]
module"]
pub type TCCR5C = crate::Reg<tccr5c::TCCR5C_SPEC>;
#[doc = "Timer/Counter5 Control Register C"]
pub mod tccr5c;
#[doc = "TCNT5 (rw) register accessor: Timer/Counter5 Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcnt5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcnt5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcnt5`]
module"]
pub type TCNT5 = crate::Reg<tcnt5::TCNT5_SPEC>;
#[doc = "Timer/Counter5 Bytes"]
pub mod tcnt5;
#[doc = "TIFR5 (rw) register accessor: Timer/Counter5 Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tifr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tifr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tifr5`]
module"]
pub type TIFR5 = crate::Reg<tifr5::TIFR5_SPEC>;
#[doc = "Timer/Counter5 Interrupt Flag Register"]
pub mod tifr5;
#[doc = "TIMSK5 (rw) register accessor: Timer/Counter5 Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timsk5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timsk5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timsk5`]
module"]
pub type TIMSK5 = crate::Reg<timsk5::TIMSK5_SPEC>;
#[doc = "Timer/Counter5 Interrupt Mask Register"]
pub mod timsk5;
