#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    remap: REMAP,
    _reserved1: [u8; 0x1a],
    udr0: UDR0,
    ubrr0: UBRR0,
    ucsr0d: UCSR0D,
    ucsr0c: UCSR0C,
    ucsr0b: UCSR0B,
    ucsr0a: UCSR0A,
}
impl RegisterBlock {
    #[doc = "0x00 - Remap Port Pins"]
    #[inline(always)]
    pub const fn remap(&self) -> &REMAP {
        &self.remap
    }
    #[doc = "0x1b - USART I/O Data Register"]
    #[inline(always)]
    pub const fn udr0(&self) -> &UDR0 {
        &self.udr0
    }
    #[doc = "0x1c - USART Baud Rate Register Bytes"]
    #[inline(always)]
    pub const fn ubrr0(&self) -> &UBRR0 {
        &self.ubrr0
    }
    #[doc = "0x1e - USART Control and Status Register D"]
    #[inline(always)]
    pub const fn ucsr0d(&self) -> &UCSR0D {
        &self.ucsr0d
    }
    #[doc = "0x1f - USART Control and Status Register C"]
    #[inline(always)]
    pub const fn ucsr0c(&self) -> &UCSR0C {
        &self.ucsr0c
    }
    #[doc = "0x20 - USART Control and Status Register B"]
    #[inline(always)]
    pub const fn ucsr0b(&self) -> &UCSR0B {
        &self.ucsr0b
    }
    #[doc = "0x21 - USART Control and Status Register A"]
    #[inline(always)]
    pub const fn ucsr0a(&self) -> &UCSR0A {
        &self.ucsr0a
    }
}
#[doc = "REMAP (rw) register accessor: Remap Port Pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap`]
module"]
pub type REMAP = crate::Reg<remap::REMAP_SPEC>;
#[doc = "Remap Port Pins"]
pub mod remap;
#[doc = "UBRR0 (rw) register accessor: USART Baud Rate Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ubrr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ubrr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ubrr0`]
module"]
pub type UBRR0 = crate::Reg<ubrr0::UBRR0_SPEC>;
#[doc = "USART Baud Rate Register Bytes"]
pub mod ubrr0;
#[doc = "UCSR0A (rw) register accessor: USART Control and Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr0a`]
module"]
pub type UCSR0A = crate::Reg<ucsr0a::UCSR0A_SPEC>;
#[doc = "USART Control and Status Register A"]
pub mod ucsr0a;
#[doc = "UCSR0B (rw) register accessor: USART Control and Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr0b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr0b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr0b`]
module"]
pub type UCSR0B = crate::Reg<ucsr0b::UCSR0B_SPEC>;
#[doc = "USART Control and Status Register B"]
pub mod ucsr0b;
#[doc = "UCSR0C (rw) register accessor: USART Control and Status Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr0c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr0c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr0c`]
module"]
pub type UCSR0C = crate::Reg<ucsr0c::UCSR0C_SPEC>;
#[doc = "USART Control and Status Register C"]
pub mod ucsr0c;
#[doc = "UCSR0D (rw) register accessor: USART Control and Status Register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr0d::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr0d::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsr0d`]
module"]
pub type UCSR0D = crate::Reg<ucsr0d::UCSR0D_SPEC>;
#[doc = "USART Control and Status Register D"]
pub mod ucsr0d;
#[doc = "UDR0 (rw) register accessor: USART I/O Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udr0`]
module"]
pub type UDR0 = crate::Reg<udr0::UDR0_SPEC>;
#[doc = "USART I/O Data Register"]
pub mod udr0;
