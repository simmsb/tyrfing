#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ucsr3a: UCSR3A,
    ucsr3b: UCSR3B,
    ucsr3c: UCSR3C,
    _reserved3: [u8; 0x01],
    ubrr3: UBRR3,
    udr3: UDR3,
}
impl RegisterBlock {
    #[doc = "0x00 - USART Control and Status Register A"]
    #[inline(always)]
    pub const fn ucsr3a(&self) -> &UCSR3A {
        &self.ucsr3a
    }
    #[doc = "0x01 - USART Control and Status Register B"]
    #[inline(always)]
    pub const fn ucsr3b(&self) -> &UCSR3B {
        &self.ucsr3b
    }
    #[doc = "0x02 - USART Control and Status Register C"]
    #[inline(always)]
    pub const fn ucsr3c(&self) -> &UCSR3C {
        &self.ucsr3c
    }
    #[doc = "0x04 - USART Baud Rate Register Bytes"]
    #[inline(always)]
    pub const fn ubrr3(&self) -> &UBRR3 {
        &self.ubrr3
    }
    #[doc = "0x06 - USART I/O Data Register"]
    #[inline(always)]
    pub const fn udr3(&self) -> &UDR3 {
        &self.udr3
    }
}
#[doc = "UBRR3 (rw) register accessor: USART Baud Rate Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ubrr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ubrr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ubrr3`]
module"]
pub type UBRR3 = crate::Reg<ubrr3::UBRR3_SPEC>;
#[doc = "USART Baud Rate Register Bytes"]
pub mod ubrr3;
#[doc = "UCSR3A (rw) register accessor: USART Control and Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr3a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr3a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr3a`]
module"]
pub type UCSR3A = crate::Reg<ucsr3a::UCSR3A_SPEC>;
#[doc = "USART Control and Status Register A"]
pub mod ucsr3a;
#[doc = "UCSR3B (rw) register accessor: USART Control and Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr3b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr3b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr3b`]
module"]
pub type UCSR3B = crate::Reg<ucsr3b::UCSR3B_SPEC>;
#[doc = "USART Control and Status Register B"]
pub mod ucsr3b;
#[doc = "UCSR3C (rw) register accessor: USART Control and Status Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr3c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr3c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr3c`]
module"]
pub type UCSR3C = crate::Reg<ucsr3c::UCSR3C_SPEC>;
#[doc = "USART Control and Status Register C"]
pub mod ucsr3c;
#[doc = "UDR3 (rw) register accessor: USART I/O Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udr3`]
module"]
pub type UDR3 = crate::Reg<udr3::UDR3_SPEC>;
#[doc = "USART I/O Data Register"]
pub mod udr3;
