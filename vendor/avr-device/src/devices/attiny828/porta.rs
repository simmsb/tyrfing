#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pina: PINA,
    ddra: DDRA,
    porta: PORTA,
    puea: PUEA,
}
impl RegisterBlock {
    #[doc = "0x00 - Port A Input Pins"]
    #[inline(always)]
    pub const fn pina(&self) -> &PINA {
        &self.pina
    }
    #[doc = "0x01 - Data Direction Register, Port A"]
    #[inline(always)]
    pub const fn ddra(&self) -> &DDRA {
        &self.ddra
    }
    #[doc = "0x02 - Port A Data Register"]
    #[inline(always)]
    pub const fn porta(&self) -> &PORTA {
        &self.porta
    }
    #[doc = "0x03 - Pull-up Enable Control Register"]
    #[inline(always)]
    pub const fn puea(&self) -> &PUEA {
        &self.puea
    }
}
#[doc = "DDRA (rw) register accessor: Data Direction Register, Port A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddra`]
module"]
pub type DDRA = crate::Reg<ddra::DDRA_SPEC>;
#[doc = "Data Direction Register, Port A"]
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
#[doc = "PUEA (rw) register accessor: Pull-up Enable Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`puea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`puea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@puea`]
module"]
pub type PUEA = crate::Reg<puea::PUEA_SPEC>;
#[doc = "Pull-up Enable Control Register"]
pub mod puea;
