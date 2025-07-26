#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    evsysroutea: EVSYSROUTEA,
    cclroutea: CCLROUTEA,
    usartroutea: USARTROUTEA,
    twispiroutea: TWISPIROUTEA,
    tcaroutea: TCAROUTEA,
    tcbroutea: TCBROUTEA,
}
impl RegisterBlock {
    #[doc = "0x00 - Port Multiplexer EVSYS"]
    #[inline(always)]
    pub const fn evsysroutea(&self) -> &EVSYSROUTEA {
        &self.evsysroutea
    }
    #[doc = "0x01 - Port Multiplexer CCL"]
    #[inline(always)]
    pub const fn cclroutea(&self) -> &CCLROUTEA {
        &self.cclroutea
    }
    #[doc = "0x02 - Port Multiplexer USART register A"]
    #[inline(always)]
    pub const fn usartroutea(&self) -> &USARTROUTEA {
        &self.usartroutea
    }
    #[doc = "0x03 - Port Multiplexer TWI and SPI"]
    #[inline(always)]
    pub const fn twispiroutea(&self) -> &TWISPIROUTEA {
        &self.twispiroutea
    }
    #[doc = "0x04 - Port Multiplexer TCA"]
    #[inline(always)]
    pub const fn tcaroutea(&self) -> &TCAROUTEA {
        &self.tcaroutea
    }
    #[doc = "0x05 - Port Multiplexer TCB"]
    #[inline(always)]
    pub const fn tcbroutea(&self) -> &TCBROUTEA {
        &self.tcbroutea
    }
}
#[doc = "CCLROUTEA (rw) register accessor: Port Multiplexer CCL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cclroutea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cclroutea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cclroutea`]
module"]
pub type CCLROUTEA = crate::Reg<cclroutea::CCLROUTEA_SPEC>;
#[doc = "Port Multiplexer CCL"]
pub mod cclroutea;
#[doc = "EVSYSROUTEA (rw) register accessor: Port Multiplexer EVSYS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evsysroutea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evsysroutea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evsysroutea`]
module"]
pub type EVSYSROUTEA = crate::Reg<evsysroutea::EVSYSROUTEA_SPEC>;
#[doc = "Port Multiplexer EVSYS"]
pub mod evsysroutea;
#[doc = "TCAROUTEA (rw) register accessor: Port Multiplexer TCA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcaroutea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcaroutea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcaroutea`]
module"]
pub type TCAROUTEA = crate::Reg<tcaroutea::TCAROUTEA_SPEC>;
#[doc = "Port Multiplexer TCA"]
pub mod tcaroutea;
#[doc = "TCBROUTEA (rw) register accessor: Port Multiplexer TCB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcbroutea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcbroutea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcbroutea`]
module"]
pub type TCBROUTEA = crate::Reg<tcbroutea::TCBROUTEA_SPEC>;
#[doc = "Port Multiplexer TCB"]
pub mod tcbroutea;
#[doc = "TWISPIROUTEA (rw) register accessor: Port Multiplexer TWI and SPI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twispiroutea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twispiroutea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twispiroutea`]
module"]
pub type TWISPIROUTEA = crate::Reg<twispiroutea::TWISPIROUTEA_SPEC>;
#[doc = "Port Multiplexer TWI and SPI"]
pub mod twispiroutea;
#[doc = "USARTROUTEA (rw) register accessor: Port Multiplexer USART register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usartroutea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usartroutea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usartroutea`]
module"]
pub type USARTROUTEA = crate::Reg<usartroutea::USARTROUTEA_SPEC>;
#[doc = "Port Multiplexer USART register A"]
pub mod usartroutea;
