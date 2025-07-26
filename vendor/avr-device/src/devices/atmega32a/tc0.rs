#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tcnt0: TCNT0,
    tccr0: TCCR0,
    _reserved2: [u8; 0x04],
    tifr: TIFR,
    timsk: TIMSK,
    _reserved4: [u8; 0x02],
    ocr0: OCR0,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer/Counter Register"]
    #[inline(always)]
    pub const fn tcnt0(&self) -> &TCNT0 {
        &self.tcnt0
    }
    #[doc = "0x01 - Timer/Counter Control Register"]
    #[inline(always)]
    pub const fn tccr0(&self) -> &TCCR0 {
        &self.tccr0
    }
    #[doc = "0x06 - Timer/Counter Interrupt Flag register"]
    #[inline(always)]
    pub const fn tifr(&self) -> &TIFR {
        &self.tifr
    }
    #[doc = "0x07 - Timer/Counter Interrupt Mask Register"]
    #[inline(always)]
    pub const fn timsk(&self) -> &TIMSK {
        &self.timsk
    }
    #[doc = "0x0a - Output Compare Register"]
    #[inline(always)]
    pub const fn ocr0(&self) -> &OCR0 {
        &self.ocr0
    }
}
#[doc = "OCR0 (rw) register accessor: Output Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr0`]
module"]
pub type OCR0 = crate::Reg<ocr0::OCR0_SPEC>;
#[doc = "Output Compare Register"]
pub mod ocr0;
#[doc = "TCCR0 (rw) register accessor: Timer/Counter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr0`]
module"]
pub type TCCR0 = crate::Reg<tccr0::TCCR0_SPEC>;
#[doc = "Timer/Counter Control Register"]
pub mod tccr0;
#[doc = "TCNT0 (rw) register accessor: Timer/Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcnt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcnt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcnt0`]
module"]
pub type TCNT0 = crate::Reg<tcnt0::TCNT0_SPEC>;
#[doc = "Timer/Counter Register"]
pub mod tcnt0;
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
