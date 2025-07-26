#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    sfior: SFIOR,
    _reserved1: [u8; 0x3b],
    etifr: ETIFR,
    etimsk: ETIMSK,
    _reserved3: [u8; 0x02],
    icr3: ICR3,
    ocr3c: OCR3C,
    ocr3b: OCR3B,
    ocr3a: OCR3A,
    tcnt3: TCNT3,
    tccr3b: TCCR3B,
    tccr3a: TCCR3A,
    tccr3c: TCCR3C,
}
impl RegisterBlock {
    #[doc = "0x00 - Special Function IO Register"]
    #[inline(always)]
    pub const fn sfior(&self) -> &SFIOR {
        &self.sfior
    }
    #[doc = "0x3c - Extended Timer/Counter Interrupt Flag register"]
    #[inline(always)]
    pub const fn etifr(&self) -> &ETIFR {
        &self.etifr
    }
    #[doc = "0x3d - Extended Timer/Counter Interrupt Mask Register"]
    #[inline(always)]
    pub const fn etimsk(&self) -> &ETIMSK {
        &self.etimsk
    }
    #[doc = "0x40 - Timer/Counter3 Input Capture Register Bytes"]
    #[inline(always)]
    pub const fn icr3(&self) -> &ICR3 {
        &self.icr3
    }
    #[doc = "0x42 - Timer/Counter3 Output compare Register C Bytes"]
    #[inline(always)]
    pub const fn ocr3c(&self) -> &OCR3C {
        &self.ocr3c
    }
    #[doc = "0x44 - Timer/Counter3 Output Compare Register B Bytes"]
    #[inline(always)]
    pub const fn ocr3b(&self) -> &OCR3B {
        &self.ocr3b
    }
    #[doc = "0x46 - Timer/Counter3 Output Compare Register A Bytes"]
    #[inline(always)]
    pub const fn ocr3a(&self) -> &OCR3A {
        &self.ocr3a
    }
    #[doc = "0x48 - Timer/Counter3 Bytes"]
    #[inline(always)]
    pub const fn tcnt3(&self) -> &TCNT3 {
        &self.tcnt3
    }
    #[doc = "0x4a - Timer/Counter3 Control Register B"]
    #[inline(always)]
    pub const fn tccr3b(&self) -> &TCCR3B {
        &self.tccr3b
    }
    #[doc = "0x4b - Timer/Counter3 Control Register A"]
    #[inline(always)]
    pub const fn tccr3a(&self) -> &TCCR3A {
        &self.tccr3a
    }
    #[doc = "0x4c - Timer/Counter3 Control Register C"]
    #[inline(always)]
    pub const fn tccr3c(&self) -> &TCCR3C {
        &self.tccr3c
    }
}
#[doc = "ETIFR (rw) register accessor: Extended Timer/Counter Interrupt Flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etifr`]
module"]
pub type ETIFR = crate::Reg<etifr::ETIFR_SPEC>;
#[doc = "Extended Timer/Counter Interrupt Flag register"]
pub mod etifr;
#[doc = "ETIMSK (rw) register accessor: Extended Timer/Counter Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etimsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etimsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etimsk`]
module"]
pub type ETIMSK = crate::Reg<etimsk::ETIMSK_SPEC>;
#[doc = "Extended Timer/Counter Interrupt Mask Register"]
pub mod etimsk;
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
#[doc = "OCR3C (rw) register accessor: Timer/Counter3 Output compare Register C Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr3c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr3c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr3c`]
module"]
pub type OCR3C = crate::Reg<ocr3c::OCR3C_SPEC>;
#[doc = "Timer/Counter3 Output compare Register C Bytes"]
pub mod ocr3c;
#[doc = "SFIOR (rw) register accessor: Special Function IO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfior::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfior::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfior`]
module"]
pub type SFIOR = crate::Reg<sfior::SFIOR_SPEC>;
#[doc = "Special Function IO Register"]
pub mod sfior;
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
