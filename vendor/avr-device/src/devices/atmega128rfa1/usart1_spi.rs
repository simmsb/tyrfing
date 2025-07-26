#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ucsr1a: UCSR1A,
    ucsr1b: UCSR1B,
    ucsr1c: UCSR1C,
}
impl RegisterBlock {
    #[doc = "0x00 - USART1 MSPIM Control and Status Register A"]
    #[inline(always)]
    pub const fn ucsr1a(&self) -> &UCSR1A {
        &self.ucsr1a
    }
    #[doc = "0x01 - USART1 MSPIM Control and Status Register B"]
    #[inline(always)]
    pub const fn ucsr1b(&self) -> &UCSR1B {
        &self.ucsr1b
    }
    #[doc = "0x02 - USART1 MSPIM Control and Status Register C"]
    #[inline(always)]
    pub const fn ucsr1c(&self) -> &UCSR1C {
        &self.ucsr1c
    }
}
#[doc = "UCSR1A (rw) register accessor: USART1 MSPIM Control and Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr1a`]
module"]
pub type UCSR1A = crate::Reg<ucsr1a::UCSR1A_SPEC>;
#[doc = "USART1 MSPIM Control and Status Register A"]
pub mod ucsr1a;
#[doc = "UCSR1B (rw) register accessor: USART1 MSPIM Control and Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr1b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr1b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr1b`]
module"]
pub type UCSR1B = crate::Reg<ucsr1b::UCSR1B_SPEC>;
#[doc = "USART1 MSPIM Control and Status Register B"]
pub mod ucsr1b;
#[doc = "UCSR1C (rw) register accessor: USART1 MSPIM Control and Status Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr1c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr1c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr1c`]
module"]
pub type UCSR1C = crate::Reg<ucsr1c::UCSR1C_SPEC>;
#[doc = "USART1 MSPIM Control and Status Register C"]
pub mod ucsr1c;
