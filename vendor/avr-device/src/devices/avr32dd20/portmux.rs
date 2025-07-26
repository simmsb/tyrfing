#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    evsysroutea: EVSYSROUTEA,
    cclroutea: CCLROUTEA,
    usartroutea: USARTROUTEA,
    _reserved3: [u8; 0x02],
    spiroutea: SPIROUTEA,
    twiroutea: TWIROUTEA,
    tcaroutea: TCAROUTEA,
    tcbroutea: TCBROUTEA,
    tcdroutea: TCDROUTEA,
}
impl RegisterBlock {
    #[doc = "0x00 - EVSYS route A"]
    #[inline(always)]
    pub const fn evsysroutea(&self) -> &EVSYSROUTEA {
        &self.evsysroutea
    }
    #[doc = "0x01 - CCL route A"]
    #[inline(always)]
    pub const fn cclroutea(&self) -> &CCLROUTEA {
        &self.cclroutea
    }
    #[doc = "0x02 - USART route A"]
    #[inline(always)]
    pub const fn usartroutea(&self) -> &USARTROUTEA {
        &self.usartroutea
    }
    #[doc = "0x05 - SPI route A"]
    #[inline(always)]
    pub const fn spiroutea(&self) -> &SPIROUTEA {
        &self.spiroutea
    }
    #[doc = "0x06 - TWI route A"]
    #[inline(always)]
    pub const fn twiroutea(&self) -> &TWIROUTEA {
        &self.twiroutea
    }
    #[doc = "0x07 - TCA route A"]
    #[inline(always)]
    pub const fn tcaroutea(&self) -> &TCAROUTEA {
        &self.tcaroutea
    }
    #[doc = "0x08 - TCB route A"]
    #[inline(always)]
    pub const fn tcbroutea(&self) -> &TCBROUTEA {
        &self.tcbroutea
    }
    #[doc = "0x09 - TCD route A"]
    #[inline(always)]
    pub const fn tcdroutea(&self) -> &TCDROUTEA {
        &self.tcdroutea
    }
}
#[doc = "CCLROUTEA (rw) register accessor: CCL route A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cclroutea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cclroutea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cclroutea`]
module"]
pub type CCLROUTEA = crate::Reg<cclroutea::CCLROUTEA_SPEC>;
#[doc = "CCL route A"]
pub mod cclroutea;
#[doc = "EVSYSROUTEA (rw) register accessor: EVSYS route A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evsysroutea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evsysroutea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evsysroutea`]
module"]
pub type EVSYSROUTEA = crate::Reg<evsysroutea::EVSYSROUTEA_SPEC>;
#[doc = "EVSYS route A"]
pub mod evsysroutea;
#[doc = "SPIROUTEA (rw) register accessor: SPI route A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spiroutea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spiroutea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spiroutea`]
module"]
pub type SPIROUTEA = crate::Reg<spiroutea::SPIROUTEA_SPEC>;
#[doc = "SPI route A"]
pub mod spiroutea;
#[doc = "TCAROUTEA (rw) register accessor: TCA route A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcaroutea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcaroutea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcaroutea`]
module"]
pub type TCAROUTEA = crate::Reg<tcaroutea::TCAROUTEA_SPEC>;
#[doc = "TCA route A"]
pub mod tcaroutea;
#[doc = "TCBROUTEA (rw) register accessor: TCB route A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcbroutea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcbroutea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcbroutea`]
module"]
pub type TCBROUTEA = crate::Reg<tcbroutea::TCBROUTEA_SPEC>;
#[doc = "TCB route A"]
pub mod tcbroutea;
#[doc = "TCDROUTEA (rw) register accessor: TCD route A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcdroutea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcdroutea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcdroutea`]
module"]
pub type TCDROUTEA = crate::Reg<tcdroutea::TCDROUTEA_SPEC>;
#[doc = "TCD route A"]
pub mod tcdroutea;
#[doc = "TWIROUTEA (rw) register accessor: TWI route A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twiroutea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twiroutea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twiroutea`]
module"]
pub type TWIROUTEA = crate::Reg<twiroutea::TWIROUTEA_SPEC>;
#[doc = "TWI route A"]
pub mod twiroutea;
#[doc = "USARTROUTEA (rw) register accessor: USART route A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usartroutea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usartroutea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usartroutea`]
module"]
pub type USARTROUTEA = crate::Reg<usartroutea::USARTROUTEA_SPEC>;
#[doc = "USART route A"]
pub mod usartroutea;
