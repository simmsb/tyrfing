#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrla: CTRLA,
    _reserved1: [u8; 0x01],
    dbgctrl: DBGCTRL,
    mctrla: MCTRLA,
    mctrlb: MCTRLB,
    mstatus: MSTATUS,
    mbaud: MBAUD,
    maddr: MADDR,
    mdata: MDATA,
    sctrla: SCTRLA,
    sctrlb: SCTRLB,
    sstatus: SSTATUS,
    saddr: SADDR,
    sdata: SDATA,
    saddrmask: SADDRMASK,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &CTRLA {
        &self.ctrla
    }
    #[doc = "0x02 - Debug Control Register"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &DBGCTRL {
        &self.dbgctrl
    }
    #[doc = "0x03 - Master Control A"]
    #[inline(always)]
    pub const fn mctrla(&self) -> &MCTRLA {
        &self.mctrla
    }
    #[doc = "0x04 - Master Control B"]
    #[inline(always)]
    pub const fn mctrlb(&self) -> &MCTRLB {
        &self.mctrlb
    }
    #[doc = "0x05 - Master Status"]
    #[inline(always)]
    pub const fn mstatus(&self) -> &MSTATUS {
        &self.mstatus
    }
    #[doc = "0x06 - Master Baurd Rate Control"]
    #[inline(always)]
    pub const fn mbaud(&self) -> &MBAUD {
        &self.mbaud
    }
    #[doc = "0x07 - Master Address"]
    #[inline(always)]
    pub const fn maddr(&self) -> &MADDR {
        &self.maddr
    }
    #[doc = "0x08 - Master Data"]
    #[inline(always)]
    pub const fn mdata(&self) -> &MDATA {
        &self.mdata
    }
    #[doc = "0x09 - Slave Control A"]
    #[inline(always)]
    pub const fn sctrla(&self) -> &SCTRLA {
        &self.sctrla
    }
    #[doc = "0x0a - Slave Control B"]
    #[inline(always)]
    pub const fn sctrlb(&self) -> &SCTRLB {
        &self.sctrlb
    }
    #[doc = "0x0b - Slave Status"]
    #[inline(always)]
    pub const fn sstatus(&self) -> &SSTATUS {
        &self.sstatus
    }
    #[doc = "0x0c - Slave Address"]
    #[inline(always)]
    pub const fn saddr(&self) -> &SADDR {
        &self.saddr
    }
    #[doc = "0x0d - Slave Data"]
    #[inline(always)]
    pub const fn sdata(&self) -> &SDATA {
        &self.sdata
    }
    #[doc = "0x0e - Slave Address Mask"]
    #[inline(always)]
    pub const fn saddrmask(&self) -> &SADDRMASK {
        &self.saddrmask
    }
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "DBGCTRL (rw) register accessor: Debug Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control Register"]
pub mod dbgctrl;
#[doc = "MADDR (rw) register accessor: Master Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maddr`]
module"]
pub type MADDR = crate::Reg<maddr::MADDR_SPEC>;
#[doc = "Master Address"]
pub mod maddr;
#[doc = "MBAUD (rw) register accessor: Master Baurd Rate Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mbaud::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mbaud::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbaud`]
module"]
pub type MBAUD = crate::Reg<mbaud::MBAUD_SPEC>;
#[doc = "Master Baurd Rate Control"]
pub mod mbaud;
#[doc = "MCTRLA (rw) register accessor: Master Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mctrla`]
module"]
pub type MCTRLA = crate::Reg<mctrla::MCTRLA_SPEC>;
#[doc = "Master Control A"]
pub mod mctrla;
#[doc = "MCTRLB (rw) register accessor: Master Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mctrlb`]
module"]
pub type MCTRLB = crate::Reg<mctrlb::MCTRLB_SPEC>;
#[doc = "Master Control B"]
pub mod mctrlb;
#[doc = "MDATA (rw) register accessor: Master Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdata`]
module"]
pub type MDATA = crate::Reg<mdata::MDATA_SPEC>;
#[doc = "Master Data"]
pub mod mdata;
#[doc = "MSTATUS (rw) register accessor: Master Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstatus`]
module"]
pub type MSTATUS = crate::Reg<mstatus::MSTATUS_SPEC>;
#[doc = "Master Status"]
pub mod mstatus;
#[doc = "SADDR (rw) register accessor: Slave Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddr`]
module"]
pub type SADDR = crate::Reg<saddr::SADDR_SPEC>;
#[doc = "Slave Address"]
pub mod saddr;
#[doc = "SADDRMASK (rw) register accessor: Slave Address Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddrmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddrmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddrmask`]
module"]
pub type SADDRMASK = crate::Reg<saddrmask::SADDRMASK_SPEC>;
#[doc = "Slave Address Mask"]
pub mod saddrmask;
#[doc = "SCTRLA (rw) register accessor: Slave Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctrla`]
module"]
pub type SCTRLA = crate::Reg<sctrla::SCTRLA_SPEC>;
#[doc = "Slave Control A"]
pub mod sctrla;
#[doc = "SCTRLB (rw) register accessor: Slave Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctrlb`]
module"]
pub type SCTRLB = crate::Reg<sctrlb::SCTRLB_SPEC>;
#[doc = "Slave Control B"]
pub mod sctrlb;
#[doc = "SDATA (rw) register accessor: Slave Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdata`]
module"]
pub type SDATA = crate::Reg<sdata::SDATA_SPEC>;
#[doc = "Slave Data"]
pub mod sdata;
#[doc = "SSTATUS (rw) register accessor: Slave Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstatus`]
module"]
pub type SSTATUS = crate::Reg<sstatus::SSTATUS_SPEC>;
#[doc = "Slave Status"]
pub mod sstatus;
