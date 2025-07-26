#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mcucr: MCUCR,
    _reserved1: [u8; 0xd9],
    llcr: LLCR,
    lldrl: LLDRL,
    lldrh: LLDRH,
    drtram3: DRTRAM3,
    drtram2: DRTRAM2,
    drtram1: DRTRAM1,
    drtram0: DRTRAM0,
    dpds0: DPDS0,
    dpds1: DPDS1,
    _reserved10: [u8; 0x01],
    trxpr: TRXPR,
}
impl RegisterBlock {
    #[doc = "0x00 - MCU Control Register"]
    #[inline(always)]
    pub const fn mcucr(&self) -> &MCUCR {
        &self.mcucr
    }
    #[doc = "0xda - Low Leakage Voltage Regulator Control Register"]
    #[inline(always)]
    pub const fn llcr(&self) -> &LLCR {
        &self.llcr
    }
    #[doc = "0xdb - Low Leakage Voltage Regulator Data Register (Low-Byte)"]
    #[inline(always)]
    pub const fn lldrl(&self) -> &LLDRL {
        &self.lldrl
    }
    #[doc = "0xdc - Low Leakage Voltage Regulator Data Register (High-Byte)"]
    #[inline(always)]
    pub const fn lldrh(&self) -> &LLDRH {
        &self.lldrh
    }
    #[doc = "0xdd - Data Retention Configuration Register of SRAM 3"]
    #[inline(always)]
    pub const fn drtram3(&self) -> &DRTRAM3 {
        &self.drtram3
    }
    #[doc = "0xde - Data Retention Configuration Register of SRAM 2"]
    #[inline(always)]
    pub const fn drtram2(&self) -> &DRTRAM2 {
        &self.drtram2
    }
    #[doc = "0xdf - Data Retention Configuration Register of SRAM 1"]
    #[inline(always)]
    pub const fn drtram1(&self) -> &DRTRAM1 {
        &self.drtram1
    }
    #[doc = "0xe0 - Data Retention Configuration Register of SRAM 0"]
    #[inline(always)]
    pub const fn drtram0(&self) -> &DRTRAM0 {
        &self.drtram0
    }
    #[doc = "0xe1 - Port Driver Strength Register 0"]
    #[inline(always)]
    pub const fn dpds0(&self) -> &DPDS0 {
        &self.dpds0
    }
    #[doc = "0xe2 - Port Driver Strength Register 1"]
    #[inline(always)]
    pub const fn dpds1(&self) -> &DPDS1 {
        &self.dpds1
    }
    #[doc = "0xe4 - Transceiver Pin Register"]
    #[inline(always)]
    pub const fn trxpr(&self) -> &TRXPR {
        &self.trxpr
    }
}
#[doc = "DPDS0 (rw) register accessor: Port Driver Strength Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpds0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpds0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpds0`]
module"]
pub type DPDS0 = crate::Reg<dpds0::DPDS0_SPEC>;
#[doc = "Port Driver Strength Register 0"]
pub mod dpds0;
#[doc = "DPDS1 (rw) register accessor: Port Driver Strength Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpds1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpds1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpds1`]
module"]
pub type DPDS1 = crate::Reg<dpds1::DPDS1_SPEC>;
#[doc = "Port Driver Strength Register 1"]
pub mod dpds1;
#[doc = "DRTRAM0 (rw) register accessor: Data Retention Configuration Register of SRAM 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drtram0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drtram0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drtram0`]
module"]
pub type DRTRAM0 = crate::Reg<drtram0::DRTRAM0_SPEC>;
#[doc = "Data Retention Configuration Register of SRAM 0"]
pub mod drtram0;
#[doc = "DRTRAM1 (rw) register accessor: Data Retention Configuration Register of SRAM 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drtram1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drtram1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drtram1`]
module"]
pub type DRTRAM1 = crate::Reg<drtram1::DRTRAM1_SPEC>;
#[doc = "Data Retention Configuration Register of SRAM 1"]
pub mod drtram1;
#[doc = "DRTRAM2 (rw) register accessor: Data Retention Configuration Register of SRAM 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drtram2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drtram2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drtram2`]
module"]
pub type DRTRAM2 = crate::Reg<drtram2::DRTRAM2_SPEC>;
#[doc = "Data Retention Configuration Register of SRAM 2"]
pub mod drtram2;
#[doc = "DRTRAM3 (rw) register accessor: Data Retention Configuration Register of SRAM 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drtram3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drtram3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drtram3`]
module"]
pub type DRTRAM3 = crate::Reg<drtram3::DRTRAM3_SPEC>;
#[doc = "Data Retention Configuration Register of SRAM 3"]
pub mod drtram3;
#[doc = "LLCR (rw) register accessor: Low Leakage Voltage Regulator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`llcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`llcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llcr`]
module"]
pub type LLCR = crate::Reg<llcr::LLCR_SPEC>;
#[doc = "Low Leakage Voltage Regulator Control Register"]
pub mod llcr;
#[doc = "LLDRH (rw) register accessor: Low Leakage Voltage Regulator Data Register (High-Byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lldrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lldrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lldrh`]
module"]
pub type LLDRH = crate::Reg<lldrh::LLDRH_SPEC>;
#[doc = "Low Leakage Voltage Regulator Data Register (High-Byte)"]
pub mod lldrh;
#[doc = "LLDRL (rw) register accessor: Low Leakage Voltage Regulator Data Register (Low-Byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lldrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lldrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lldrl`]
module"]
pub type LLDRL = crate::Reg<lldrl::LLDRL_SPEC>;
#[doc = "Low Leakage Voltage Regulator Data Register (Low-Byte)"]
pub mod lldrl;
#[doc = "MCUCR (rw) register accessor: MCU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcucr`]
module"]
pub type MCUCR = crate::Reg<mcucr::MCUCR_SPEC>;
#[doc = "MCU Control Register"]
pub mod mcucr;
#[doc = "TRXPR (rw) register accessor: Transceiver Pin Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trxpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trxpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trxpr`]
module"]
pub type TRXPR = crate::Reg<trxpr::TRXPR_SPEC>;
#[doc = "Transceiver Pin Register"]
pub mod trxpr;
