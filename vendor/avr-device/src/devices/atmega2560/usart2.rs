#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ucsr2a: UCSR2A,
    ucsr2b: UCSR2B,
    ucsr2c: UCSR2C,
    _reserved3: [u8; 0x01],
    ubrr2: UBRR2,
    udr2: UDR2,
}
impl RegisterBlock {
    #[doc = "0x00 - USART Control and Status Register A"]
    #[inline(always)]
    pub const fn ucsr2a(&self) -> &UCSR2A {
        &self.ucsr2a
    }
    #[doc = "0x01 - USART Control and Status Register B"]
    #[inline(always)]
    pub const fn ucsr2b(&self) -> &UCSR2B {
        &self.ucsr2b
    }
    #[doc = "0x02 - USART Control and Status Register C"]
    #[inline(always)]
    pub const fn ucsr2c(&self) -> &UCSR2C {
        &self.ucsr2c
    }
    #[doc = "0x04 - USART Baud Rate Register Bytes"]
    #[inline(always)]
    pub const fn ubrr2(&self) -> &UBRR2 {
        &self.ubrr2
    }
    #[doc = "0x06 - USART I/O Data Register"]
    #[inline(always)]
    pub const fn udr2(&self) -> &UDR2 {
        &self.udr2
    }
}
#[doc = "UBRR2 (rw) register accessor: USART Baud Rate Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ubrr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ubrr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ubrr2`]
module"]
pub type UBRR2 = crate::Reg<ubrr2::UBRR2_SPEC>;
#[doc = "USART Baud Rate Register Bytes"]
pub mod ubrr2;
#[doc = "UCSR2A (rw) register accessor: USART Control and Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr2a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr2a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr2a`]
module"]
pub type UCSR2A = crate::Reg<ucsr2a::UCSR2A_SPEC>;
#[doc = "USART Control and Status Register A"]
pub mod ucsr2a;
#[doc = "UCSR2B (rw) register accessor: USART Control and Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr2b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr2b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr2b`]
module"]
pub type UCSR2B = crate::Reg<ucsr2b::UCSR2B_SPEC>;
#[doc = "USART Control and Status Register B"]
pub mod ucsr2b;
#[doc = "UCSR2C (rw) register accessor: USART Control and Status Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr2c`]
module"]
pub type UCSR2C = crate::Reg<ucsr2c::UCSR2C_SPEC>;
#[doc = "USART Control and Status Register C"]
pub mod ucsr2c;
#[doc = "UDR2 (rw) register accessor: USART I/O Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udr2`]
module"]
pub type UDR2 = crate::Reg<udr2::UDR2_SPEC>;
#[doc = "USART I/O Data Register"]
pub mod udr2;
