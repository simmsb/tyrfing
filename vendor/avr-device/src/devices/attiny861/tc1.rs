#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tccr1e: TCCR1E,
    _reserved1: [u8; 0x23],
    dt1: DT1,
    tc1h: TC1H,
    tccr1d: TCCR1D,
    tccr1c: TCCR1C,
    _reserved5: [u8; 0x02],
    ocr1d: OCR1D,
    ocr1c: OCR1C,
    ocr1b: OCR1B,
    ocr1a: OCR1A,
    tcnt1: TCNT1,
    tccr1b: TCCR1B,
    tccr1a: TCCR1A,
    _reserved12: [u8; 0x07],
    tifr: TIFR,
    timsk: TIMSK,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer/Counter1 Control Register E"]
    #[inline(always)]
    pub const fn tccr1e(&self) -> &TCCR1E {
        &self.tccr1e
    }
    #[doc = "0x24 - Timer/Counter 1 Dead Time Value"]
    #[inline(always)]
    pub const fn dt1(&self) -> &DT1 {
        &self.dt1
    }
    #[doc = "0x25 - Timer/Counter High Bits"]
    #[inline(always)]
    pub const fn tc1h(&self) -> &TC1H {
        &self.tc1h
    }
    #[doc = "0x26 - Timer/Counter Control Register D"]
    #[inline(always)]
    pub const fn tccr1d(&self) -> &TCCR1D {
        &self.tccr1d
    }
    #[doc = "0x27 - Timer/Counter Control Register C"]
    #[inline(always)]
    pub const fn tccr1c(&self) -> &TCCR1C {
        &self.tccr1c
    }
    #[doc = "0x2a - Output compare register"]
    #[inline(always)]
    pub const fn ocr1d(&self) -> &OCR1D {
        &self.ocr1d
    }
    #[doc = "0x2b - Output compare register"]
    #[inline(always)]
    pub const fn ocr1c(&self) -> &OCR1C {
        &self.ocr1c
    }
    #[doc = "0x2c - Output Compare Register"]
    #[inline(always)]
    pub const fn ocr1b(&self) -> &OCR1B {
        &self.ocr1b
    }
    #[doc = "0x2d - Output Compare Register"]
    #[inline(always)]
    pub const fn ocr1a(&self) -> &OCR1A {
        &self.ocr1a
    }
    #[doc = "0x2e - Timer/Counter Register"]
    #[inline(always)]
    pub const fn tcnt1(&self) -> &TCNT1 {
        &self.tcnt1
    }
    #[doc = "0x2f - Timer/Counter Control Register B"]
    #[inline(always)]
    pub const fn tccr1b(&self) -> &TCCR1B {
        &self.tccr1b
    }
    #[doc = "0x30 - Timer/Counter Control Register A"]
    #[inline(always)]
    pub const fn tccr1a(&self) -> &TCCR1A {
        &self.tccr1a
    }
    #[doc = "0x38 - Timer/Counter Interrupt Flag Register"]
    #[inline(always)]
    pub const fn tifr(&self) -> &TIFR {
        &self.tifr
    }
    #[doc = "0x39 - Timer/Counter Interrupt Mask Register"]
    #[inline(always)]
    pub const fn timsk(&self) -> &TIMSK {
        &self.timsk
    }
}
#[doc = "DT1 (rw) register accessor: Timer/Counter 1 Dead Time Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt1`]
module"]
pub type DT1 = crate::Reg<dt1::DT1_SPEC>;
#[doc = "Timer/Counter 1 Dead Time Value"]
pub mod dt1;
#[doc = "OCR1A (rw) register accessor: Output Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr1a`]
module"]
pub type OCR1A = crate::Reg<ocr1a::OCR1A_SPEC>;
#[doc = "Output Compare Register"]
pub mod ocr1a;
#[doc = "OCR1B (rw) register accessor: Output Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr1b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr1b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr1b`]
module"]
pub type OCR1B = crate::Reg<ocr1b::OCR1B_SPEC>;
#[doc = "Output Compare Register"]
pub mod ocr1b;
#[doc = "OCR1C (rw) register accessor: Output compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr1c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr1c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr1c`]
module"]
pub type OCR1C = crate::Reg<ocr1c::OCR1C_SPEC>;
#[doc = "Output compare register"]
pub mod ocr1c;
#[doc = "OCR1D (rw) register accessor: Output compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr1d::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr1d::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr1d`]
module"]
pub type OCR1D = crate::Reg<ocr1d::OCR1D_SPEC>;
#[doc = "Output compare register"]
pub mod ocr1d;
#[doc = "TC1H (rw) register accessor: Timer/Counter High Bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tc1h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tc1h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tc1h`]
module"]
pub type TC1H = crate::Reg<tc1h::TC1H_SPEC>;
#[doc = "Timer/Counter High Bits"]
pub mod tc1h;
#[doc = "TCCR1A (rw) register accessor: Timer/Counter Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr1a`]
module"]
pub type TCCR1A = crate::Reg<tccr1a::TCCR1A_SPEC>;
#[doc = "Timer/Counter Control Register A"]
pub mod tccr1a;
#[doc = "TCCR1B (rw) register accessor: Timer/Counter Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr1b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr1b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr1b`]
module"]
pub type TCCR1B = crate::Reg<tccr1b::TCCR1B_SPEC>;
#[doc = "Timer/Counter Control Register B"]
pub mod tccr1b;
#[doc = "TCCR1C (rw) register accessor: Timer/Counter Control Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr1c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr1c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr1c`]
module"]
pub type TCCR1C = crate::Reg<tccr1c::TCCR1C_SPEC>;
#[doc = "Timer/Counter Control Register C"]
pub mod tccr1c;
#[doc = "TCCR1D (rw) register accessor: Timer/Counter Control Register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr1d::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr1d::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr1d`]
module"]
pub type TCCR1D = crate::Reg<tccr1d::TCCR1D_SPEC>;
#[doc = "Timer/Counter Control Register D"]
pub mod tccr1d;
#[doc = "TCCR1E (rw) register accessor: Timer/Counter1 Control Register E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr1e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr1e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr1e`]
module"]
pub type TCCR1E = crate::Reg<tccr1e::TCCR1E_SPEC>;
#[doc = "Timer/Counter1 Control Register E"]
pub mod tccr1e;
#[doc = "TCNT1 (rw) register accessor: Timer/Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcnt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcnt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcnt1`]
module"]
pub type TCNT1 = crate::Reg<tcnt1::TCNT1_SPEC>;
#[doc = "Timer/Counter Register"]
pub mod tcnt1;
#[doc = "TIFR (rw) register accessor: Timer/Counter Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tifr`]
module"]
pub type TIFR = crate::Reg<tifr::TIFR_SPEC>;
#[doc = "Timer/Counter Interrupt Flag Register"]
pub mod tifr;
#[doc = "TIMSK (rw) register accessor: Timer/Counter Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timsk`]
module"]
pub type TIMSK = crate::Reg<timsk::TIMSK_SPEC>;
#[doc = "Timer/Counter Interrupt Mask Register"]
pub mod timsk;
