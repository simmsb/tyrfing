#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ucsra: UCSRA,
    ucsrb: UCSRB,
    ucsrc: UCSRC,
    ucsrd: UCSRD,
    ubrr: UBRR,
    udr: UDR,
}
impl RegisterBlock {
    #[doc = "0x00 - USART Control and Status Register A"]
    #[inline(always)]
    pub const fn ucsra(&self) -> &UCSRA {
        &self.ucsra
    }
    #[doc = "0x01 - USART Control and Status Register B"]
    #[inline(always)]
    pub const fn ucsrb(&self) -> &UCSRB {
        &self.ucsrb
    }
    #[doc = "0x02 - USART Control and Status Register C"]
    #[inline(always)]
    pub const fn ucsrc(&self) -> &UCSRC {
        &self.ucsrc
    }
    #[doc = "0x03 - USART Control and Status Register D"]
    #[inline(always)]
    pub const fn ucsrd(&self) -> &UCSRD {
        &self.ucsrd
    }
    #[doc = "0x04 - USART Baud Rate Register Bytes"]
    #[inline(always)]
    pub const fn ubrr(&self) -> &UBRR {
        &self.ubrr
    }
    #[doc = "0x06 - USART I/O Data Register"]
    #[inline(always)]
    pub const fn udr(&self) -> &UDR {
        &self.udr
    }
}
#[doc = "UBRR (rw) register accessor: USART Baud Rate Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ubrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ubrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ubrr`]
module"]
pub type UBRR = crate::Reg<ubrr::UBRR_SPEC>;
#[doc = "USART Baud Rate Register Bytes"]
pub mod ubrr;
#[doc = "UCSRA (rw) register accessor: USART Control and Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsra`]
module"]
pub type UCSRA = crate::Reg<ucsra::UCSRA_SPEC>;
#[doc = "USART Control and Status Register A"]
pub mod ucsra;
#[doc = "UCSRB (rw) register accessor: USART Control and Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsrb`]
module"]
pub type UCSRB = crate::Reg<ucsrb::UCSRB_SPEC>;
#[doc = "USART Control and Status Register B"]
pub mod ucsrb;
#[doc = "UCSRC (rw) register accessor: USART Control and Status Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsrc`]
module"]
pub type UCSRC = crate::Reg<ucsrc::UCSRC_SPEC>;
#[doc = "USART Control and Status Register C"]
pub mod ucsrc;
#[doc = "UCSRD (rw) register accessor: USART Control and Status Register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsrd`]
module"]
pub type UCSRD = crate::Reg<ucsrd::UCSRD_SPEC>;
#[doc = "USART Control and Status Register D"]
pub mod ucsrd;
#[doc = "UDR (rw) register accessor: USART I/O Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udr`]
module"]
pub type UDR = crate::Reg<udr::UDR_SPEC>;
#[doc = "USART I/O Data Register"]
pub mod udr;
