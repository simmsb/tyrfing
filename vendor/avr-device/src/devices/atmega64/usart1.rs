#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ubrr1h: UBRR1H,
    ubrr1l: UBRR1L,
    ucsr1b: UCSR1B,
    ucsr1a: UCSR1A,
    udr1: UDR1,
    ucsr1c: UCSR1C,
}
impl RegisterBlock {
    #[doc = "0x00 - USART Baud Rate Register Hight Byte"]
    #[inline(always)]
    pub const fn ubrr1h(&self) -> &UBRR1H {
        &self.ubrr1h
    }
    #[doc = "0x01 - USART Baud Rate Register Low Byte"]
    #[inline(always)]
    pub const fn ubrr1l(&self) -> &UBRR1L {
        &self.ubrr1l
    }
    #[doc = "0x02 - USART Control and Status Register B"]
    #[inline(always)]
    pub const fn ucsr1b(&self) -> &UCSR1B {
        &self.ucsr1b
    }
    #[doc = "0x03 - USART Control and Status Register A"]
    #[inline(always)]
    pub const fn ucsr1a(&self) -> &UCSR1A {
        &self.ucsr1a
    }
    #[doc = "0x04 - USART I/O Data Register"]
    #[inline(always)]
    pub const fn udr1(&self) -> &UDR1 {
        &self.udr1
    }
    #[doc = "0x05 - USART Control and Status Register C"]
    #[inline(always)]
    pub const fn ucsr1c(&self) -> &UCSR1C {
        &self.ucsr1c
    }
}
#[doc = "UBRR1H (rw) register accessor: USART Baud Rate Register Hight Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ubrr1h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ubrr1h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ubrr1h`]
module"]
pub type UBRR1H = crate::Reg<ubrr1h::UBRR1H_SPEC>;
#[doc = "USART Baud Rate Register Hight Byte"]
pub mod ubrr1h;
#[doc = "UBRR1L (rw) register accessor: USART Baud Rate Register Low Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ubrr1l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ubrr1l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ubrr1l`]
module"]
pub type UBRR1L = crate::Reg<ubrr1l::UBRR1L_SPEC>;
#[doc = "USART Baud Rate Register Low Byte"]
pub mod ubrr1l;
#[doc = "UCSR1A (rw) register accessor: USART Control and Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr1a`]
module"]
pub type UCSR1A = crate::Reg<ucsr1a::UCSR1A_SPEC>;
#[doc = "USART Control and Status Register A"]
pub mod ucsr1a;
#[doc = "UCSR1B (rw) register accessor: USART Control and Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr1b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr1b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr1b`]
module"]
pub type UCSR1B = crate::Reg<ucsr1b::UCSR1B_SPEC>;
#[doc = "USART Control and Status Register B"]
pub mod ucsr1b;
#[doc = "UCSR1C (rw) register accessor: USART Control and Status Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr1c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr1c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr1c`]
module"]
pub type UCSR1C = crate::Reg<ucsr1c::UCSR1C_SPEC>;
#[doc = "USART Control and Status Register C"]
pub mod ucsr1c;
#[doc = "UDR1 (rw) register accessor: USART I/O Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udr1`]
module"]
pub type UDR1 = crate::Reg<udr1::UDR1_SPEC>;
#[doc = "USART I/O Data Register"]
pub mod udr1;
