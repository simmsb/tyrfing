#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pinb: PINB,
    ddrb: DDRB,
    portb: PORTB,
    _reserved3: [u8; 0x29],
    pueb: PUEB,
    _reserved4: [u8; 0x01],
    portcr: PORTCR,
}
impl RegisterBlock {
    #[doc = "0x00 - Port B Data register"]
    #[inline(always)]
    pub const fn pinb(&self) -> &PINB {
        &self.pinb
    }
    #[doc = "0x01 - Data Direction Register, Port B"]
    #[inline(always)]
    pub const fn ddrb(&self) -> &DDRB {
        &self.ddrb
    }
    #[doc = "0x02 - Input Pins, Port B"]
    #[inline(always)]
    pub const fn portb(&self) -> &PORTB {
        &self.portb
    }
    #[doc = "0x2c - Pull-up Enable Control Register"]
    #[inline(always)]
    pub const fn pueb(&self) -> &PUEB {
        &self.pueb
    }
    #[doc = "0x2e - Port Control Register"]
    #[inline(always)]
    pub const fn portcr(&self) -> &PORTCR {
        &self.portcr
    }
}
#[doc = "DDRB (rw) register accessor: Data Direction Register, Port B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrb`]
module"]
pub type DDRB = crate::Reg<ddrb::DDRB_SPEC>;
#[doc = "Data Direction Register, Port B"]
pub mod ddrb;
#[doc = "PINB (rw) register accessor: Port B Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinb`]
module"]
pub type PINB = crate::Reg<pinb::PINB_SPEC>;
#[doc = "Port B Data register"]
pub mod pinb;
#[doc = "PORTB (rw) register accessor: Input Pins, Port B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portb`]
module"]
pub type PORTB = crate::Reg<portb::PORTB_SPEC>;
#[doc = "Input Pins, Port B"]
pub mod portb;
#[doc = "PORTCR (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portcr`]
module"]
pub type PORTCR = crate::Reg<portcr::PORTCR_SPEC>;
#[doc = "Port Control Register"]
pub mod portcr;
#[doc = "PUEB (rw) register accessor: Pull-up Enable Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pueb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pueb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pueb`]
module"]
pub type PUEB = crate::Reg<pueb::PUEB_SPEC>;
#[doc = "Pull-up Enable Control Register"]
pub mod pueb;
