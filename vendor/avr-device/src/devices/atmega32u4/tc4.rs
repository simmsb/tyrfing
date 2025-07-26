#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tifr4: TIFR4,
    _reserved1: [u8; 0x38],
    timsk4: TIMSK4,
    _reserved2: [u8; 0x4b],
    tcnt4: TCNT4,
    tc4h: TC4H,
    tccr4a: TCCR4A,
    tccr4b: TCCR4B,
    tccr4c: TCCR4C,
    tccr4d: TCCR4D,
    tccr4e: TCCR4E,
    _reserved9: [u8; 0x0a],
    ocr4a: OCR4A,
    ocr4b: OCR4B,
    ocr4c: OCR4C,
    ocr4d: OCR4D,
    _reserved13: [u8; 0x01],
    dt4: DT4,
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
    #[doc = "0x85 - Timer/Counter4 Low Bytes"]
    #[inline(always)]
    pub const fn tcnt4(&self) -> &TCNT4 {
        &self.tcnt4
    }
    #[doc = "0x86 - Timer/Counter High Bits"]
    #[inline(always)]
    pub const fn tc4h(&self) -> &TC4H {
        &self.tc4h
    }
    #[doc = "0x87 - Timer/Counter4 Control Register A"]
    #[inline(always)]
    pub const fn tccr4a(&self) -> &TCCR4A {
        &self.tccr4a
    }
    #[doc = "0x88 - Timer/Counter4 Control Register B"]
    #[inline(always)]
    pub const fn tccr4b(&self) -> &TCCR4B {
        &self.tccr4b
    }
    #[doc = "0x89 - Timer/Counter 4 Control Register C"]
    #[inline(always)]
    pub const fn tccr4c(&self) -> &TCCR4C {
        &self.tccr4c
    }
    #[doc = "0x8a - Timer/Counter 4 Control Register D"]
    #[inline(always)]
    pub const fn tccr4d(&self) -> &TCCR4D {
        &self.tccr4d
    }
    #[doc = "0x8b - Timer/Counter 4 Control Register E"]
    #[inline(always)]
    pub const fn tccr4e(&self) -> &TCCR4E {
        &self.tccr4e
    }
    #[doc = "0x96 - Timer/Counter4 Output Compare Register A"]
    #[inline(always)]
    pub const fn ocr4a(&self) -> &OCR4A {
        &self.ocr4a
    }
    #[doc = "0x97 - Timer/Counter4 Output Compare Register B"]
    #[inline(always)]
    pub const fn ocr4b(&self) -> &OCR4B {
        &self.ocr4b
    }
    #[doc = "0x98 - Timer/Counter4 Output Compare Register C"]
    #[inline(always)]
    pub const fn ocr4c(&self) -> &OCR4C {
        &self.ocr4c
    }
    #[doc = "0x99 - Timer/Counter4 Output Compare Register D"]
    #[inline(always)]
    pub const fn ocr4d(&self) -> &OCR4D {
        &self.ocr4d
    }
    #[doc = "0x9b - Timer/Counter 4 Dead Time Value"]
    #[inline(always)]
    pub const fn dt4(&self) -> &DT4 {
        &self.dt4
    }
}
#[doc = "DT4 (rw) register accessor: Timer/Counter 4 Dead Time Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt4`]
module"]
pub type DT4 = crate::Reg<dt4::DT4_SPEC>;
#[doc = "Timer/Counter 4 Dead Time Value"]
pub mod dt4;
#[doc = "OCR4A (rw) register accessor: Timer/Counter4 Output Compare Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr4a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr4a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr4a`]
module"]
pub type OCR4A = crate::Reg<ocr4a::OCR4A_SPEC>;
#[doc = "Timer/Counter4 Output Compare Register A"]
pub mod ocr4a;
#[doc = "OCR4B (rw) register accessor: Timer/Counter4 Output Compare Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr4b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr4b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr4b`]
module"]
pub type OCR4B = crate::Reg<ocr4b::OCR4B_SPEC>;
#[doc = "Timer/Counter4 Output Compare Register B"]
pub mod ocr4b;
#[doc = "OCR4C (rw) register accessor: Timer/Counter4 Output Compare Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr4c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr4c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr4c`]
module"]
pub type OCR4C = crate::Reg<ocr4c::OCR4C_SPEC>;
#[doc = "Timer/Counter4 Output Compare Register C"]
pub mod ocr4c;
#[doc = "OCR4D (rw) register accessor: Timer/Counter4 Output Compare Register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr4d::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr4d::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr4d`]
module"]
pub type OCR4D = crate::Reg<ocr4d::OCR4D_SPEC>;
#[doc = "Timer/Counter4 Output Compare Register D"]
pub mod ocr4d;
#[doc = "TC4H (rw) register accessor: Timer/Counter High Bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tc4h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tc4h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tc4h`]
module"]
pub type TC4H = crate::Reg<tc4h::TC4H_SPEC>;
#[doc = "Timer/Counter High Bits"]
pub mod tc4h;
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
#[doc = "TCCR4C (rw) register accessor: Timer/Counter 4 Control Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr4c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr4c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr4c`]
module"]
pub type TCCR4C = crate::Reg<tccr4c::TCCR4C_SPEC>;
#[doc = "Timer/Counter 4 Control Register C"]
pub mod tccr4c;
#[doc = "TCCR4D (rw) register accessor: Timer/Counter 4 Control Register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr4d::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr4d::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr4d`]
module"]
pub type TCCR4D = crate::Reg<tccr4d::TCCR4D_SPEC>;
#[doc = "Timer/Counter 4 Control Register D"]
pub mod tccr4d;
#[doc = "TCCR4E (rw) register accessor: Timer/Counter 4 Control Register E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr4e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr4e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr4e`]
module"]
pub type TCCR4E = crate::Reg<tccr4e::TCCR4E_SPEC>;
#[doc = "Timer/Counter 4 Control Register E"]
pub mod tccr4e;
#[doc = "TCNT4 (rw) register accessor: Timer/Counter4 Low Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcnt4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcnt4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcnt4`]
module"]
pub type TCNT4 = crate::Reg<tcnt4::TCNT4_SPEC>;
#[doc = "Timer/Counter4 Low Bytes"]
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
