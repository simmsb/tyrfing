#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    acsr0a: ACSR0A,
    acsr0b: ACSR0B,
    acsr1a: ACSR1A,
    acsr1b: ACSR1B,
}
impl RegisterBlock {
    #[doc = "0x00 - Analog Comparator 0 Control And Status Register A"]
    #[inline(always)]
    pub const fn acsr0a(&self) -> &ACSR0A {
        &self.acsr0a
    }
    #[doc = "0x01 - Analog Comparator 0 Control And Status Register B"]
    #[inline(always)]
    pub const fn acsr0b(&self) -> &ACSR0B {
        &self.acsr0b
    }
    #[doc = "0x02 - Analog Comparator 1 Control And Status Register A"]
    #[inline(always)]
    pub const fn acsr1a(&self) -> &ACSR1A {
        &self.acsr1a
    }
    #[doc = "0x03 - Analog Comparator 1 Control And Status Register B"]
    #[inline(always)]
    pub const fn acsr1b(&self) -> &ACSR1B {
        &self.acsr1b
    }
}
#[doc = "ACSR0A (rw) register accessor: Analog Comparator 0 Control And Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsr0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsr0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acsr0a`]
module"]
pub type ACSR0A = crate::Reg<acsr0a::ACSR0A_SPEC>;
#[doc = "Analog Comparator 0 Control And Status Register A"]
pub mod acsr0a;
#[doc = "ACSR0B (rw) register accessor: Analog Comparator 0 Control And Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsr0b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsr0b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acsr0b`]
module"]
pub type ACSR0B = crate::Reg<acsr0b::ACSR0B_SPEC>;
#[doc = "Analog Comparator 0 Control And Status Register B"]
pub mod acsr0b;
#[doc = "ACSR1A (rw) register accessor: Analog Comparator 1 Control And Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsr1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsr1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acsr1a`]
module"]
pub type ACSR1A = crate::Reg<acsr1a::ACSR1A_SPEC>;
#[doc = "Analog Comparator 1 Control And Status Register A"]
pub mod acsr1a;
#[doc = "ACSR1B (rw) register accessor: Analog Comparator 1 Control And Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsr1b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsr1b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acsr1b`]
module"]
pub type ACSR1B = crate::Reg<acsr1b::ACSR1B_SPEC>;
#[doc = "Analog Comparator 1 Control And Status Register B"]
pub mod acsr1b;
