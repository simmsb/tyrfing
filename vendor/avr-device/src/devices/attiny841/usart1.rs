#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    udr1: UDR1,
    ubrr1: UBRR1,
    ucsr1d: UCSR1D,
    ucsr1c: UCSR1C,
    ucsr1b: UCSR1B,
    ucsr1a: UCSR1A,
}
impl RegisterBlock {
    #[doc = "0x00 - USART I/O Data Register"]
    #[inline(always)]
    pub const fn udr1(&self) -> &UDR1 {
        &self.udr1
    }
    #[doc = "0x01 - USART Baud Rate Register Bytes"]
    #[inline(always)]
    pub const fn ubrr1(&self) -> &UBRR1 {
        &self.ubrr1
    }
    #[doc = "0x03 - USART Control and Status Register D"]
    #[inline(always)]
    pub const fn ucsr1d(&self) -> &UCSR1D {
        &self.ucsr1d
    }
    #[doc = "0x04 - USART Control and Status Register C"]
    #[inline(always)]
    pub const fn ucsr1c(&self) -> &UCSR1C {
        &self.ucsr1c
    }
    #[doc = "0x05 - USART Control and Status Register B"]
    #[inline(always)]
    pub const fn ucsr1b(&self) -> &UCSR1B {
        &self.ucsr1b
    }
    #[doc = "0x06 - USART Control and Status Register A"]
    #[inline(always)]
    pub const fn ucsr1a(&self) -> &UCSR1A {
        &self.ucsr1a
    }
}
#[doc = "UBRR1 (rw) register accessor: USART Baud Rate Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ubrr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ubrr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ubrr1`]
module"]
pub type UBRR1 = crate::Reg<ubrr1::UBRR1_SPEC>;
#[doc = "USART Baud Rate Register Bytes"]
pub mod ubrr1;
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
#[doc = "UCSR1D (rw) register accessor: USART Control and Status Register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr1d::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr1d::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr1d`]
module"]
pub type UCSR1D = crate::Reg<ucsr1d::UCSR1D_SPEC>;
#[doc = "USART Control and Status Register D"]
pub mod ucsr1d;
#[doc = "UDR1 (rw) register accessor: USART I/O Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udr1`]
module"]
pub type UDR1 = crate::Reg<udr1::UDR1_SPEC>;
#[doc = "USART I/O Data Register"]
pub mod udr1;
