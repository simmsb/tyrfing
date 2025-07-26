#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pina: PINA,
    ddra: DDRA,
    porta: PORTA,
}
impl RegisterBlock {
    #[doc = "0x00 - Port A Input Pins"]
    #[inline(always)]
    pub const fn pina(&self) -> &PINA {
        &self.pina
    }
    #[doc = "0x01 - Port A Data Direction Register"]
    #[inline(always)]
    pub const fn ddra(&self) -> &DDRA {
        &self.ddra
    }
    #[doc = "0x02 - Port A Data Register"]
    #[inline(always)]
    pub const fn porta(&self) -> &PORTA {
        &self.porta
    }
}
#[doc = "DDRA (rw) register accessor: Port A Data Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddra`]
module"]
pub type DDRA = crate::Reg<ddra::DDRA_SPEC>;
#[doc = "Port A Data Direction Register"]
pub mod ddra;
#[doc = "PINA (rw) register accessor: Port A Input Pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pina::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pina::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pina`]
module"]
pub type PINA = crate::Reg<pina::PINA_SPEC>;
#[doc = "Port A Input Pins"]
pub mod pina;
#[doc = "PORTA (rw) register accessor: Port A Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`porta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`porta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porta`]
module"]
pub type PORTA = crate::Reg<porta::PORTA_SPEC>;
#[doc = "Port A Data Register"]
pub mod porta;
