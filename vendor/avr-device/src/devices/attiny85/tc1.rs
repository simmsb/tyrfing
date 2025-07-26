#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    dtps: DTPS,
    dt1b: DT1B,
    dt1a: DT1A,
    _reserved3: [u8; 0x05],
    ocr1b: OCR1B,
    gtccr: GTCCR,
    ocr1c: OCR1C,
    ocr1a: OCR1A,
    tcnt1: TCNT1,
    tccr1: TCCR1,
    _reserved9: [u8; 0x07],
    tifr: TIFR,
    timsk: TIMSK,
}
impl RegisterBlock {
    #[doc = "0x00 - Dead time prescaler register"]
    #[inline(always)]
    pub const fn dtps(&self) -> &DTPS {
        &self.dtps
    }
    #[doc = "0x01 - Dead Time Value Register B"]
    #[inline(always)]
    pub const fn dt1b(&self) -> &DT1B {
        &self.dt1b
    }
    #[doc = "0x02 - Dead Time Value Register A"]
    #[inline(always)]
    pub const fn dt1a(&self) -> &DT1A {
        &self.dt1a
    }
    #[doc = "0x08 - Output Compare Register B"]
    #[inline(always)]
    pub const fn ocr1b(&self) -> &OCR1B {
        &self.ocr1b
    }
    #[doc = "0x09 - Timer counter control register"]
    #[inline(always)]
    pub const fn gtccr(&self) -> &GTCCR {
        &self.gtccr
    }
    #[doc = "0x0a - Output Compare Register C"]
    #[inline(always)]
    pub const fn ocr1c(&self) -> &OCR1C {
        &self.ocr1c
    }
    #[doc = "0x0b - Output Compare Register A"]
    #[inline(always)]
    pub const fn ocr1a(&self) -> &OCR1A {
        &self.ocr1a
    }
    #[doc = "0x0c - Timer/Counter Register"]
    #[inline(always)]
    pub const fn tcnt1(&self) -> &TCNT1 {
        &self.tcnt1
    }
    #[doc = "0x0d - Timer/Counter Control Register"]
    #[inline(always)]
    pub const fn tccr1(&self) -> &TCCR1 {
        &self.tccr1
    }
    #[doc = "0x15 - Timer/Counter Interrupt Flag Register"]
    #[inline(always)]
    pub const fn tifr(&self) -> &TIFR {
        &self.tifr
    }
    #[doc = "0x16 - Timer/Counter Interrupt Mask Register"]
    #[inline(always)]
    pub const fn timsk(&self) -> &TIMSK {
        &self.timsk
    }
}
#[doc = "DT1A (rw) register accessor: Dead Time Value Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt1a`]
module"]
pub type DT1A = crate::Reg<dt1a::DT1A_SPEC>;
#[doc = "Dead Time Value Register A"]
pub mod dt1a;
#[doc = "DT1B (rw) register accessor: Dead Time Value Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt1b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt1b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt1b`]
module"]
pub type DT1B = crate::Reg<dt1b::DT1B_SPEC>;
#[doc = "Dead Time Value Register B"]
pub mod dt1b;
#[doc = "DTPS (rw) register accessor: Dead time prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtps::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtps::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtps`]
module"]
pub type DTPS = crate::Reg<dtps::DTPS_SPEC>;
#[doc = "Dead time prescaler register"]
pub mod dtps;
#[doc = "GTCCR (rw) register accessor: Timer counter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtccr`]
module"]
pub type GTCCR = crate::Reg<gtccr::GTCCR_SPEC>;
#[doc = "Timer counter control register"]
pub mod gtccr;
#[doc = "OCR1A (rw) register accessor: Output Compare Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr1a`]
module"]
pub type OCR1A = crate::Reg<ocr1a::OCR1A_SPEC>;
#[doc = "Output Compare Register A"]
pub mod ocr1a;
#[doc = "OCR1B (rw) register accessor: Output Compare Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr1b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr1b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr1b`]
module"]
pub type OCR1B = crate::Reg<ocr1b::OCR1B_SPEC>;
#[doc = "Output Compare Register B"]
pub mod ocr1b;
#[doc = "OCR1C (rw) register accessor: Output Compare Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr1c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr1c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr1c`]
module"]
pub type OCR1C = crate::Reg<ocr1c::OCR1C_SPEC>;
#[doc = "Output Compare Register C"]
pub mod ocr1c;
#[doc = "TCCR1 (rw) register accessor: Timer/Counter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr1`]
module"]
pub type TCCR1 = crate::Reg<tccr1::TCCR1_SPEC>;
#[doc = "Timer/Counter Control Register"]
pub mod tccr1;
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
