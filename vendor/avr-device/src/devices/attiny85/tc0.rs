#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ocr0b: OCR0B,
    ocr0a: OCR0A,
    tccr0a: TCCR0A,
    _reserved3: [u8; 0x01],
    gtccr: GTCCR,
    _reserved4: [u8; 0x05],
    tcnt0: TCNT0,
    tccr0b: TCCR0B,
    _reserved6: [u8; 0x04],
    tifr: TIFR,
    timsk: TIMSK,
}
impl RegisterBlock {
    #[doc = "0x00 - Output Compare Register B"]
    #[inline(always)]
    pub const fn ocr0b(&self) -> &OCR0B {
        &self.ocr0b
    }
    #[doc = "0x01 - Output Compare Register A"]
    #[inline(always)]
    pub const fn ocr0a(&self) -> &OCR0A {
        &self.ocr0a
    }
    #[doc = "0x02 - Timer/Counter Control Register A"]
    #[inline(always)]
    pub const fn tccr0a(&self) -> &TCCR0A {
        &self.tccr0a
    }
    #[doc = "0x04 - General Timer/Counter Control Register"]
    #[inline(always)]
    pub const fn gtccr(&self) -> &GTCCR {
        &self.gtccr
    }
    #[doc = "0x0a - Timer/Counter0"]
    #[inline(always)]
    pub const fn tcnt0(&self) -> &TCNT0 {
        &self.tcnt0
    }
    #[doc = "0x0b - Timer/Counter Control Register B"]
    #[inline(always)]
    pub const fn tccr0b(&self) -> &TCCR0B {
        &self.tccr0b
    }
    #[doc = "0x10 - Timer/Counter0 Interrupt Flag register"]
    #[inline(always)]
    pub const fn tifr(&self) -> &TIFR {
        &self.tifr
    }
    #[doc = "0x11 - Timer/Counter Interrupt Mask Register"]
    #[inline(always)]
    pub const fn timsk(&self) -> &TIMSK {
        &self.timsk
    }
}
#[doc = "GTCCR (rw) register accessor: General Timer/Counter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtccr`]
module"]
pub type GTCCR = crate::Reg<gtccr::GTCCR_SPEC>;
#[doc = "General Timer/Counter Control Register"]
pub mod gtccr;
#[doc = "OCR0A (rw) register accessor: Output Compare Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr0a`]
module"]
pub type OCR0A = crate::Reg<ocr0a::OCR0A_SPEC>;
#[doc = "Output Compare Register A"]
pub mod ocr0a;
#[doc = "OCR0B (rw) register accessor: Output Compare Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr0b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr0b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr0b`]
module"]
pub type OCR0B = crate::Reg<ocr0b::OCR0B_SPEC>;
#[doc = "Output Compare Register B"]
pub mod ocr0b;
#[doc = "TCCR0A (rw) register accessor: Timer/Counter Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr0a`]
module"]
pub type TCCR0A = crate::Reg<tccr0a::TCCR0A_SPEC>;
#[doc = "Timer/Counter Control Register A"]
pub mod tccr0a;
#[doc = "TCCR0B (rw) register accessor: Timer/Counter Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr0b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr0b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr0b`]
module"]
pub type TCCR0B = crate::Reg<tccr0b::TCCR0B_SPEC>;
#[doc = "Timer/Counter Control Register B"]
pub mod tccr0b;
#[doc = "TCNT0 (rw) register accessor: Timer/Counter0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcnt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcnt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcnt0`]
module"]
pub type TCNT0 = crate::Reg<tcnt0::TCNT0_SPEC>;
#[doc = "Timer/Counter0"]
pub mod tcnt0;
#[doc = "TIFR (rw) register accessor: Timer/Counter0 Interrupt Flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tifr`]
module"]
pub type TIFR = crate::Reg<tifr::TIFR_SPEC>;
#[doc = "Timer/Counter0 Interrupt Flag register"]
pub mod tifr;
#[doc = "TIMSK (rw) register accessor: Timer/Counter Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timsk`]
module"]
pub type TIMSK = crate::Reg<timsk::TIMSK_SPEC>;
#[doc = "Timer/Counter Interrupt Mask Register"]
pub mod timsk;
