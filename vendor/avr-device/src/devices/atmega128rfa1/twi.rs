#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    twbr: TWBR,
    twsr: TWSR,
    twar: TWAR,
    twdr: TWDR,
    twcr: TWCR,
    twamr: TWAMR,
}
impl RegisterBlock {
    #[doc = "0x00 - TWI Bit Rate Register"]
    #[inline(always)]
    pub const fn twbr(&self) -> &TWBR {
        &self.twbr
    }
    #[doc = "0x01 - TWI Status Register"]
    #[inline(always)]
    pub const fn twsr(&self) -> &TWSR {
        &self.twsr
    }
    #[doc = "0x02 - TWI (Slave) Address Register"]
    #[inline(always)]
    pub const fn twar(&self) -> &TWAR {
        &self.twar
    }
    #[doc = "0x03 - TWI Data Register"]
    #[inline(always)]
    pub const fn twdr(&self) -> &TWDR {
        &self.twdr
    }
    #[doc = "0x04 - TWI Control Register"]
    #[inline(always)]
    pub const fn twcr(&self) -> &TWCR {
        &self.twcr
    }
    #[doc = "0x05 - TWI (Slave) Address Mask Register"]
    #[inline(always)]
    pub const fn twamr(&self) -> &TWAMR {
        &self.twamr
    }
}
#[doc = "TWAMR (rw) register accessor: TWI (Slave) Address Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twamr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twamr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twamr`]
module"]
pub type TWAMR = crate::Reg<twamr::TWAMR_SPEC>;
#[doc = "TWI (Slave) Address Mask Register"]
pub mod twamr;
#[doc = "TWAR (rw) register accessor: TWI (Slave) Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twar`]
module"]
pub type TWAR = crate::Reg<twar::TWAR_SPEC>;
#[doc = "TWI (Slave) Address Register"]
pub mod twar;
#[doc = "TWBR (rw) register accessor: TWI Bit Rate Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twbr`]
module"]
pub type TWBR = crate::Reg<twbr::TWBR_SPEC>;
#[doc = "TWI Bit Rate Register"]
pub mod twbr;
#[doc = "TWCR (rw) register accessor: TWI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twcr`]
module"]
pub type TWCR = crate::Reg<twcr::TWCR_SPEC>;
#[doc = "TWI Control Register"]
pub mod twcr;
#[doc = "TWDR (rw) register accessor: TWI Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twdr`]
module"]
pub type TWDR = crate::Reg<twdr::TWDR_SPEC>;
#[doc = "TWI Data Register"]
pub mod twdr;
#[doc = "TWSR (rw) register accessor: TWI Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twsr`]
module"]
pub type TWSR = crate::Reg<twsr::TWSR_SPEC>;
#[doc = "TWI Status Register"]
pub mod twsr;
