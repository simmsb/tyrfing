#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    twsd: TWSD,
    twsam: TWSAM,
    twsa: TWSA,
    twssra: TWSSRA,
    twscrb: TWSCRB,
    twscra: TWSCRA,
}
impl RegisterBlock {
    #[doc = "0x00 - TWI Slave Data Register"]
    #[inline(always)]
    pub const fn twsd(&self) -> &TWSD {
        &self.twsd
    }
    #[doc = "0x01 - TWI Slave Address Mask Register"]
    #[inline(always)]
    pub const fn twsam(&self) -> &TWSAM {
        &self.twsam
    }
    #[doc = "0x02 - TWI Slave Address Register"]
    #[inline(always)]
    pub const fn twsa(&self) -> &TWSA {
        &self.twsa
    }
    #[doc = "0x03 - TWI Slave Status Register A"]
    #[inline(always)]
    pub const fn twssra(&self) -> &TWSSRA {
        &self.twssra
    }
    #[doc = "0x04 - TWI Slave Control Register B"]
    #[inline(always)]
    pub const fn twscrb(&self) -> &TWSCRB {
        &self.twscrb
    }
    #[doc = "0x05 - TWI Slave Control Register A"]
    #[inline(always)]
    pub const fn twscra(&self) -> &TWSCRA {
        &self.twscra
    }
}
#[doc = "TWSA (rw) register accessor: TWI Slave Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twsa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twsa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twsa`]
module"]
pub type TWSA = crate::Reg<twsa::TWSA_SPEC>;
#[doc = "TWI Slave Address Register"]
pub mod twsa;
#[doc = "TWSAM (rw) register accessor: TWI Slave Address Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twsam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twsam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twsam`]
module"]
pub type TWSAM = crate::Reg<twsam::TWSAM_SPEC>;
#[doc = "TWI Slave Address Mask Register"]
pub mod twsam;
#[doc = "TWSCRA (rw) register accessor: TWI Slave Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twscra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twscra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twscra`]
module"]
pub type TWSCRA = crate::Reg<twscra::TWSCRA_SPEC>;
#[doc = "TWI Slave Control Register A"]
pub mod twscra;
#[doc = "TWSCRB (rw) register accessor: TWI Slave Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twscrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twscrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twscrb`]
module"]
pub type TWSCRB = crate::Reg<twscrb::TWSCRB_SPEC>;
#[doc = "TWI Slave Control Register B"]
pub mod twscrb;
#[doc = "TWSD (rw) register accessor: TWI Slave Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twsd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twsd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twsd`]
module"]
pub type TWSD = crate::Reg<twsd::TWSD_SPEC>;
#[doc = "TWI Slave Data Register"]
pub mod twsd;
#[doc = "TWSSRA (rw) register accessor: TWI Slave Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twssra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twssra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twssra`]
module"]
pub type TWSSRA = crate::Reg<twssra::TWSSRA_SPEC>;
#[doc = "TWI Slave Status Register A"]
pub mod twssra;
