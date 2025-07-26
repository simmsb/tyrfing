#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ucsr0a: UCSR0A,
    ucsr0b: UCSR0B,
    ucsr0c: UCSR0C,
}
impl RegisterBlock {
    #[doc = "0x00 - USART0 MSPIM Control and Status Register A"]
    #[inline(always)]
    pub const fn ucsr0a(&self) -> &UCSR0A {
        &self.ucsr0a
    }
    #[doc = "0x01 - USART0 MSPIM Control and Status Register B"]
    #[inline(always)]
    pub const fn ucsr0b(&self) -> &UCSR0B {
        &self.ucsr0b
    }
    #[doc = "0x02 - USART0 MSPIM Control and Status Register C"]
    #[inline(always)]
    pub const fn ucsr0c(&self) -> &UCSR0C {
        &self.ucsr0c
    }
}
#[doc = "UCSR0A (rw) register accessor: USART0 MSPIM Control and Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr0a`]
module"]
pub type UCSR0A = crate::Reg<ucsr0a::UCSR0A_SPEC>;
#[doc = "USART0 MSPIM Control and Status Register A"]
pub mod ucsr0a;
#[doc = "UCSR0B (rw) register accessor: USART0 MSPIM Control and Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr0b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr0b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr0b`]
module"]
pub type UCSR0B = crate::Reg<ucsr0b::UCSR0B_SPEC>;
#[doc = "USART0 MSPIM Control and Status Register B"]
pub mod ucsr0b;
#[doc = "UCSR0C (rw) register accessor: USART0 MSPIM Control and Status Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr0c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr0c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr0c`]
module"]
pub type UCSR0C = crate::Reg<ucsr0c::UCSR0C_SPEC>;
#[doc = "USART0 MSPIM Control and Status Register C"]
pub mod ucsr0c;
