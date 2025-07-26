#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pinb: PINB,
    ddrb: DDRB,
    portb: PORTB,
}
impl RegisterBlock {
    #[doc = "0x00 - Input Pins, Port B"]
    #[inline(always)]
    pub const fn pinb(&self) -> &PINB {
        &self.pinb
    }
    #[doc = "0x01 - Data Direction Register, Port B"]
    #[inline(always)]
    pub const fn ddrb(&self) -> &DDRB {
        &self.ddrb
    }
    #[doc = "0x02 - Data Register, Port B"]
    #[inline(always)]
    pub const fn portb(&self) -> &PORTB {
        &self.portb
    }
}
#[doc = "DDRB (rw) register accessor: Data Direction Register, Port B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrb`]
module"]
pub type DDRB = crate::Reg<ddrb::DDRB_SPEC>;
#[doc = "Data Direction Register, Port B"]
pub mod ddrb;
#[doc = "PINB (rw) register accessor: Input Pins, Port B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinb`]
module"]
pub type PINB = crate::Reg<pinb::PINB_SPEC>;
#[doc = "Input Pins, Port B"]
pub mod pinb;
#[doc = "PORTB (rw) register accessor: Data Register, Port B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portb`]
module"]
pub type PORTB = crate::Reg<portb::PORTB_SPEC>;
#[doc = "Data Register, Port B"]
pub mod portb;
