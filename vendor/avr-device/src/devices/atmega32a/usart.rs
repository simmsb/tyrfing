#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ubrrl: UBRRL,
    ucsrb: UCSRB,
    ucsra: UCSRA,
    udr: UDR,
    _reserved4: [u8; 0x13],
    _reserved_4_ubrrh: [u8; 0x01],
}
impl RegisterBlock {
    #[doc = "0x00 - USART Baud Rate Register Low Byte"]
    #[inline(always)]
    pub const fn ubrrl(&self) -> &UBRRL {
        &self.ubrrl
    }
    #[doc = "0x01 - USART Control and Status Register B"]
    #[inline(always)]
    pub const fn ucsrb(&self) -> &UCSRB {
        &self.ucsrb
    }
    #[doc = "0x02 - USART Control and Status Register A"]
    #[inline(always)]
    pub const fn ucsra(&self) -> &UCSRA {
        &self.ucsra
    }
    #[doc = "0x03 - USART I/O Data Register"]
    #[inline(always)]
    pub const fn udr(&self) -> &UDR {
        &self.udr
    }
    #[doc = "0x17 - USART Control and Status Register C"]
    #[inline(always)]
    pub const fn ucsrc(&self) -> &UCSRC {
        unsafe { &*(self as *const Self).cast::<u8>().add(23).cast() }
    }
    #[doc = "0x17 - USART Baud Rate Register Hight Byte"]
    #[inline(always)]
    pub const fn ubrrh(&self) -> &UBRRH {
        unsafe { &*(self as *const Self).cast::<u8>().add(23).cast() }
    }
}
#[doc = "UBRRH (rw) register accessor: USART Baud Rate Register Hight Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ubrrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ubrrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ubrrh`]
module"]
pub type UBRRH = crate::Reg<ubrrh::UBRRH_SPEC>;
#[doc = "USART Baud Rate Register Hight Byte"]
pub mod ubrrh;
#[doc = "UBRRL (rw) register accessor: USART Baud Rate Register Low Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ubrrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ubrrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ubrrl`]
module"]
pub type UBRRL = crate::Reg<ubrrl::UBRRL_SPEC>;
#[doc = "USART Baud Rate Register Low Byte"]
pub mod ubrrl;
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
#[doc = "UDR (rw) register accessor: USART I/O Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udr`]
module"]
pub type UDR = crate::Reg<udr::UDR_SPEC>;
#[doc = "USART I/O Data Register"]
pub mod udr;
