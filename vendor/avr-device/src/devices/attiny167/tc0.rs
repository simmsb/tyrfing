#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tifr0: TIFR0,
    _reserved1: [u8; 0x0d],
    gtccr: GTCCR,
    _reserved2: [u8; 0x01],
    tccr0a: TCCR0A,
    tccr0b: TCCR0B,
    tcnt0: TCNT0,
    ocr0a: OCR0A,
    _reserved6: [u8; 0x25],
    timsk0: TIMSK0,
    _reserved7: [u8; 0x47],
    assr: ASSR,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer/Counter0 Interrupt Flag Register"]
    #[inline(always)]
    pub const fn tifr0(&self) -> &TIFR0 {
        &self.tifr0
    }
    #[doc = "0x0e - General Timer Counter Control register"]
    #[inline(always)]
    pub const fn gtccr(&self) -> &GTCCR {
        &self.gtccr
    }
    #[doc = "0x10 - Timer/Counter0 Control Register A"]
    #[inline(always)]
    pub const fn tccr0a(&self) -> &TCCR0A {
        &self.tccr0a
    }
    #[doc = "0x11 - Timer/Counter0 Control Register B"]
    #[inline(always)]
    pub const fn tccr0b(&self) -> &TCCR0B {
        &self.tccr0b
    }
    #[doc = "0x12 - Timer/Counter0"]
    #[inline(always)]
    pub const fn tcnt0(&self) -> &TCNT0 {
        &self.tcnt0
    }
    #[doc = "0x13 - Timer/Counter0 Output Compare Register A"]
    #[inline(always)]
    pub const fn ocr0a(&self) -> &OCR0A {
        &self.ocr0a
    }
    #[doc = "0x39 - Timer/Counter0 Interrupt Mask register"]
    #[inline(always)]
    pub const fn timsk0(&self) -> &TIMSK0 {
        &self.timsk0
    }
    #[doc = "0x81 - Asynchronous Status Register"]
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
#[doc = "OCR0A (rw) register accessor: Timer/Counter0 Output Compare Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr0a`]
module"]
pub type OCR0A = crate::Reg<ocr0a::OCR0A_SPEC>;
#[doc = "Timer/Counter0 Output Compare Register A"]
pub mod ocr0a;
#[doc = "TCCR0A (rw) register accessor: Timer/Counter0 Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr0a`]
module"]
pub type TCCR0A = crate::Reg<tccr0a::TCCR0A_SPEC>;
#[doc = "Timer/Counter0 Control Register A"]
pub mod tccr0a;
#[doc = "TCCR0B (rw) register accessor: Timer/Counter0 Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr0b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr0b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr0b`]
module"]
pub type TCCR0B = crate::Reg<tccr0b::TCCR0B_SPEC>;
#[doc = "Timer/Counter0 Control Register B"]
pub mod tccr0b;
#[doc = "TCNT0 (rw) register accessor: Timer/Counter0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcnt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcnt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcnt0`]
module"]
pub type TCNT0 = crate::Reg<tcnt0::TCNT0_SPEC>;
#[doc = "Timer/Counter0"]
pub mod tcnt0;
#[doc = "TIFR0 (rw) register accessor: Timer/Counter0 Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tifr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tifr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tifr0`]
module"]
pub type TIFR0 = crate::Reg<tifr0::TIFR0_SPEC>;
#[doc = "Timer/Counter0 Interrupt Flag Register"]
pub mod tifr0;
#[doc = "TIMSK0 (rw) register accessor: Timer/Counter0 Interrupt Mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timsk0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timsk0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timsk0`]
module"]
pub type TIMSK0 = crate::Reg<timsk0::TIMSK0_SPEC>;
#[doc = "Timer/Counter0 Interrupt Mask register"]
pub mod timsk0;
