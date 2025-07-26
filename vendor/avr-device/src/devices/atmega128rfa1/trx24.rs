#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    aes_ctrl: AES_CTRL,
    aes_status: AES_STATUS,
    aes_state: AES_STATE,
    aes_key: AES_KEY,
    _reserved4: [u8; 0x01],
    trx_status: TRX_STATUS,
    trx_state: TRX_STATE,
    trx_ctrl_0: TRX_CTRL_0,
    trx_ctrl_1: TRX_CTRL_1,
    phy_tx_pwr: PHY_TX_PWR,
    phy_rssi: PHY_RSSI,
    phy_ed_level: PHY_ED_LEVEL,
    phy_cc_cca: PHY_CC_CCA,
    cca_thres: CCA_THRES,
    rx_ctrl: RX_CTRL,
    sfd_value: SFD_VALUE,
    trx_ctrl_2: TRX_CTRL_2,
    ant_div: ANT_DIV,
    irq_mask: IRQ_MASK,
    irq_status: IRQ_STATUS,
    vreg_ctrl: VREG_CTRL,
    batmon: BATMON,
    xosc_ctrl: XOSC_CTRL,
    _reserved22: [u8; 0x02],
    rx_syn: RX_SYN,
    _reserved23: [u8; 0x01],
    xah_ctrl_1: XAH_CTRL_1,
    ftn_ctrl: FTN_CTRL,
    _reserved25: [u8; 0x01],
    pll_cf: PLL_CF,
    pll_dcu: PLL_DCU,
    part_num: PART_NUM,
    version_num: VERSION_NUM,
    man_id_0: MAN_ID_0,
    man_id_1: MAN_ID_1,
    short_addr_0: SHORT_ADDR_0,
    short_addr_1: SHORT_ADDR_1,
    pan_id_0: PAN_ID_0,
    pan_id_1: PAN_ID_1,
    ieee_addr_0: IEEE_ADDR_0,
    ieee_addr_1: IEEE_ADDR_1,
    ieee_addr_2: IEEE_ADDR_2,
    ieee_addr_3: IEEE_ADDR_3,
    ieee_addr_4: IEEE_ADDR_4,
    ieee_addr_5: IEEE_ADDR_5,
    ieee_addr_6: IEEE_ADDR_6,
    ieee_addr_7: IEEE_ADDR_7,
    xah_ctrl_0: XAH_CTRL_0,
    csma_seed_0: CSMA_SEED_0,
    csma_seed_1: CSMA_SEED_1,
    csma_be: CSMA_BE,
    _reserved47: [u8; 0x06],
    tst_ctrl_digi: TST_CTRL_DIGI,
    _reserved48: [u8; 0x04],
    tst_rx_length: TST_RX_LENGTH,
    _reserved49: [u8; 0x04],
    trxfbst: TRXFBST,
    _reserved50: [u8; 0x7e],
    trxfbend: TRXFBEND,
}
impl RegisterBlock {
    #[doc = "0x00 - AES Control Register"]
    #[inline(always)]
    pub const fn aes_ctrl(&self) -> &AES_CTRL {
        &self.aes_ctrl
    }
    #[doc = "0x01 - AES Status Register"]
    #[inline(always)]
    pub const fn aes_status(&self) -> &AES_STATUS {
        &self.aes_status
    }
    #[doc = "0x02 - AES Plain and Cipher Text Buffer Register"]
    #[inline(always)]
    pub const fn aes_state(&self) -> &AES_STATE {
        &self.aes_state
    }
    #[doc = "0x03 - AES Encryption and Decryption Key Buffer Register"]
    #[inline(always)]
    pub const fn aes_key(&self) -> &AES_KEY {
        &self.aes_key
    }
    #[doc = "0x05 - Transceiver Status Register"]
    #[inline(always)]
    pub const fn trx_status(&self) -> &TRX_STATUS {
        &self.trx_status
    }
    #[doc = "0x06 - Transceiver State Control Register"]
    #[inline(always)]
    pub const fn trx_state(&self) -> &TRX_STATE {
        &self.trx_state
    }
    #[doc = "0x07 - Reserved"]
    #[inline(always)]
    pub const fn trx_ctrl_0(&self) -> &TRX_CTRL_0 {
        &self.trx_ctrl_0
    }
    #[doc = "0x08 - Transceiver Control Register 1"]
    #[inline(always)]
    pub const fn trx_ctrl_1(&self) -> &TRX_CTRL_1 {
        &self.trx_ctrl_1
    }
    #[doc = "0x09 - Transceiver Transmit Power Control Register"]
    #[inline(always)]
    pub const fn phy_tx_pwr(&self) -> &PHY_TX_PWR {
        &self.phy_tx_pwr
    }
    #[doc = "0x0a - Receiver Signal Strength Indicator Register"]
    #[inline(always)]
    pub const fn phy_rssi(&self) -> &PHY_RSSI {
        &self.phy_rssi
    }
    #[doc = "0x0b - Transceiver Energy Detection Level Register"]
    #[inline(always)]
    pub const fn phy_ed_level(&self) -> &PHY_ED_LEVEL {
        &self.phy_ed_level
    }
    #[doc = "0x0c - Transceiver Clear Channel Assessment (CCA) Control Register"]
    #[inline(always)]
    pub const fn phy_cc_cca(&self) -> &PHY_CC_CCA {
        &self.phy_cc_cca
    }
    #[doc = "0x0d - Transceiver CCA Threshold Setting Register"]
    #[inline(always)]
    pub const fn cca_thres(&self) -> &CCA_THRES {
        &self.cca_thres
    }
    #[doc = "0x0e - Transceiver Receive Control Register"]
    #[inline(always)]
    pub const fn rx_ctrl(&self) -> &RX_CTRL {
        &self.rx_ctrl
    }
    #[doc = "0x0f - Start of Frame Delimiter Value Register"]
    #[inline(always)]
    pub const fn sfd_value(&self) -> &SFD_VALUE {
        &self.sfd_value
    }
    #[doc = "0x10 - Transceiver Control Register 2"]
    #[inline(always)]
    pub const fn trx_ctrl_2(&self) -> &TRX_CTRL_2 {
        &self.trx_ctrl_2
    }
    #[doc = "0x11 - Antenna Diversity Control Register"]
    #[inline(always)]
    pub const fn ant_div(&self) -> &ANT_DIV {
        &self.ant_div
    }
    #[doc = "0x12 - Transceiver Interrupt Enable Register"]
    #[inline(always)]
    pub const fn irq_mask(&self) -> &IRQ_MASK {
        &self.irq_mask
    }
    #[doc = "0x13 - Transceiver Interrupt Status Register"]
    #[inline(always)]
    pub const fn irq_status(&self) -> &IRQ_STATUS {
        &self.irq_status
    }
    #[doc = "0x14 - Voltage Regulator Control and Status Register"]
    #[inline(always)]
    pub const fn vreg_ctrl(&self) -> &VREG_CTRL {
        &self.vreg_ctrl
    }
    #[doc = "0x15 - Battery Monitor Control and Status Register"]
    #[inline(always)]
    pub const fn batmon(&self) -> &BATMON {
        &self.batmon
    }
    #[doc = "0x16 - Crystal Oscillator Control Register"]
    #[inline(always)]
    pub const fn xosc_ctrl(&self) -> &XOSC_CTRL {
        &self.xosc_ctrl
    }
    #[doc = "0x19 - Transceiver Receiver Sensitivity Control Register"]
    #[inline(always)]
    pub const fn rx_syn(&self) -> &RX_SYN {
        &self.rx_syn
    }
    #[doc = "0x1b - Transceiver Acknowledgment Frame Control Register 1"]
    #[inline(always)]
    pub const fn xah_ctrl_1(&self) -> &XAH_CTRL_1 {
        &self.xah_ctrl_1
    }
    #[doc = "0x1c - Transceiver Filter Tuning Control Register"]
    #[inline(always)]
    pub const fn ftn_ctrl(&self) -> &FTN_CTRL {
        &self.ftn_ctrl
    }
    #[doc = "0x1e - Transceiver Center Frequency Calibration Control Register"]
    #[inline(always)]
    pub const fn pll_cf(&self) -> &PLL_CF {
        &self.pll_cf
    }
    #[doc = "0x1f - Transceiver Delay Cell Calibration Control Register"]
    #[inline(always)]
    pub const fn pll_dcu(&self) -> &PLL_DCU {
        &self.pll_dcu
    }
    #[doc = "0x20 - Device Identification Register (Part Number)"]
    #[inline(always)]
    pub const fn part_num(&self) -> &PART_NUM {
        &self.part_num
    }
    #[doc = "0x21 - Device Identification Register (Version Number)"]
    #[inline(always)]
    pub const fn version_num(&self) -> &VERSION_NUM {
        &self.version_num
    }
    #[doc = "0x22 - Device Identification Register (Manufacture ID Low Byte)"]
    #[inline(always)]
    pub const fn man_id_0(&self) -> &MAN_ID_0 {
        &self.man_id_0
    }
    #[doc = "0x23 - Device Identification Register (Manufacture ID High Byte)"]
    #[inline(always)]
    pub const fn man_id_1(&self) -> &MAN_ID_1 {
        &self.man_id_1
    }
    #[doc = "0x24 - Transceiver MAC Short Address Register (Low Byte)"]
    #[inline(always)]
    pub const fn short_addr_0(&self) -> &SHORT_ADDR_0 {
        &self.short_addr_0
    }
    #[doc = "0x25 - Transceiver MAC Short Address Register (High Byte)"]
    #[inline(always)]
    pub const fn short_addr_1(&self) -> &SHORT_ADDR_1 {
        &self.short_addr_1
    }
    #[doc = "0x26 - Transceiver Personal Area Network ID Register (Low Byte)"]
    #[inline(always)]
    pub const fn pan_id_0(&self) -> &PAN_ID_0 {
        &self.pan_id_0
    }
    #[doc = "0x27 - Transceiver Personal Area Network ID Register (High Byte)"]
    #[inline(always)]
    pub const fn pan_id_1(&self) -> &PAN_ID_1 {
        &self.pan_id_1
    }
    #[doc = "0x28 - Transceiver MAC IEEE Address Register 0"]
    #[inline(always)]
    pub const fn ieee_addr_0(&self) -> &IEEE_ADDR_0 {
        &self.ieee_addr_0
    }
    #[doc = "0x29 - Transceiver MAC IEEE Address Register 1"]
    #[inline(always)]
    pub const fn ieee_addr_1(&self) -> &IEEE_ADDR_1 {
        &self.ieee_addr_1
    }
    #[doc = "0x2a - Transceiver MAC IEEE Address Register 2"]
    #[inline(always)]
    pub const fn ieee_addr_2(&self) -> &IEEE_ADDR_2 {
        &self.ieee_addr_2
    }
    #[doc = "0x2b - Transceiver MAC IEEE Address Register 3"]
    #[inline(always)]
    pub const fn ieee_addr_3(&self) -> &IEEE_ADDR_3 {
        &self.ieee_addr_3
    }
    #[doc = "0x2c - Transceiver MAC IEEE Address Register 4"]
    #[inline(always)]
    pub const fn ieee_addr_4(&self) -> &IEEE_ADDR_4 {
        &self.ieee_addr_4
    }
    #[doc = "0x2d - Transceiver MAC IEEE Address Register 5"]
    #[inline(always)]
    pub const fn ieee_addr_5(&self) -> &IEEE_ADDR_5 {
        &self.ieee_addr_5
    }
    #[doc = "0x2e - Transceiver MAC IEEE Address Register 6"]
    #[inline(always)]
    pub const fn ieee_addr_6(&self) -> &IEEE_ADDR_6 {
        &self.ieee_addr_6
    }
    #[doc = "0x2f - Transceiver MAC IEEE Address Register 7"]
    #[inline(always)]
    pub const fn ieee_addr_7(&self) -> &IEEE_ADDR_7 {
        &self.ieee_addr_7
    }
    #[doc = "0x30 - Transceiver Extended Operating Mode Control Register"]
    #[inline(always)]
    pub const fn xah_ctrl_0(&self) -> &XAH_CTRL_0 {
        &self.xah_ctrl_0
    }
    #[doc = "0x31 - Transceiver CSMA-CA Random Number Generator Seed Register"]
    #[inline(always)]
    pub const fn csma_seed_0(&self) -> &CSMA_SEED_0 {
        &self.csma_seed_0
    }
    #[doc = "0x32 - Transceiver Acknowledgment Frame Control Register 2"]
    #[inline(always)]
    pub const fn csma_seed_1(&self) -> &CSMA_SEED_1 {
        &self.csma_seed_1
    }
    #[doc = "0x33 - Transceiver CSMA-CA Back-off Exponent Control Register"]
    #[inline(always)]
    pub const fn csma_be(&self) -> &CSMA_BE {
        &self.csma_be
    }
    #[doc = "0x3a - Transceiver Digital Test Control Register"]
    #[inline(always)]
    pub const fn tst_ctrl_digi(&self) -> &TST_CTRL_DIGI {
        &self.tst_ctrl_digi
    }
    #[doc = "0x3f - Transceiver Received Frame Length Register"]
    #[inline(always)]
    pub const fn tst_rx_length(&self) -> &TST_RX_LENGTH {
        &self.tst_rx_length
    }
    #[doc = "0x44 - Start of frame buffer"]
    #[inline(always)]
    pub const fn trxfbst(&self) -> &TRXFBST {
        &self.trxfbst
    }
    #[doc = "0xc3 - End of frame buffer"]
    #[inline(always)]
    pub const fn trxfbend(&self) -> &TRXFBEND {
        &self.trxfbend
    }
}
#[doc = "AES_CTRL (rw) register accessor: AES Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_ctrl`]
module"]
pub type AES_CTRL = crate::Reg<aes_ctrl::AES_CTRL_SPEC>;
#[doc = "AES Control Register"]
pub mod aes_ctrl;
#[doc = "AES_KEY (rw) register accessor: AES Encryption and Decryption Key Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_key::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key`]
module"]
pub type AES_KEY = crate::Reg<aes_key::AES_KEY_SPEC>;
#[doc = "AES Encryption and Decryption Key Buffer Register"]
pub mod aes_key;
#[doc = "AES_STATE (rw) register accessor: AES Plain and Cipher Text Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_state`]
module"]
pub type AES_STATE = crate::Reg<aes_state::AES_STATE_SPEC>;
#[doc = "AES Plain and Cipher Text Buffer Register"]
pub mod aes_state;
#[doc = "AES_STATUS (rw) register accessor: AES Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_status`]
module"]
pub type AES_STATUS = crate::Reg<aes_status::AES_STATUS_SPEC>;
#[doc = "AES Status Register"]
pub mod aes_status;
#[doc = "ANT_DIV (rw) register accessor: Antenna Diversity Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ant_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ant_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ant_div`]
module"]
pub type ANT_DIV = crate::Reg<ant_div::ANT_DIV_SPEC>;
#[doc = "Antenna Diversity Control Register"]
pub mod ant_div;
#[doc = "BATMON (rw) register accessor: Battery Monitor Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batmon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batmon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@batmon`]
module"]
pub type BATMON = crate::Reg<batmon::BATMON_SPEC>;
#[doc = "Battery Monitor Control and Status Register"]
pub mod batmon;
#[doc = "CCA_THRES (rw) register accessor: Transceiver CCA Threshold Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cca_thres::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cca_thres::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cca_thres`]
module"]
pub type CCA_THRES = crate::Reg<cca_thres::CCA_THRES_SPEC>;
#[doc = "Transceiver CCA Threshold Setting Register"]
pub mod cca_thres;
#[doc = "CSMA_BE (rw) register accessor: Transceiver CSMA-CA Back-off Exponent Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csma_be::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csma_be::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csma_be`]
module"]
pub type CSMA_BE = crate::Reg<csma_be::CSMA_BE_SPEC>;
#[doc = "Transceiver CSMA-CA Back-off Exponent Control Register"]
pub mod csma_be;
#[doc = "CSMA_SEED_0 (rw) register accessor: Transceiver CSMA-CA Random Number Generator Seed Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csma_seed_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csma_seed_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csma_seed_0`]
module"]
pub type CSMA_SEED_0 = crate::Reg<csma_seed_0::CSMA_SEED_0_SPEC>;
#[doc = "Transceiver CSMA-CA Random Number Generator Seed Register"]
pub mod csma_seed_0;
#[doc = "CSMA_SEED_1 (rw) register accessor: Transceiver Acknowledgment Frame Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csma_seed_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csma_seed_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csma_seed_1`]
module"]
pub type CSMA_SEED_1 = crate::Reg<csma_seed_1::CSMA_SEED_1_SPEC>;
#[doc = "Transceiver Acknowledgment Frame Control Register 2"]
pub mod csma_seed_1;
#[doc = "FTN_CTRL (rw) register accessor: Transceiver Filter Tuning Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftn_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftn_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftn_ctrl`]
module"]
pub type FTN_CTRL = crate::Reg<ftn_ctrl::FTN_CTRL_SPEC>;
#[doc = "Transceiver Filter Tuning Control Register"]
pub mod ftn_ctrl;
#[doc = "IEEE_ADDR_0 (rw) register accessor: Transceiver MAC IEEE Address Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ieee_addr_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ieee_addr_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ieee_addr_0`]
module"]
pub type IEEE_ADDR_0 = crate::Reg<ieee_addr_0::IEEE_ADDR_0_SPEC>;
#[doc = "Transceiver MAC IEEE Address Register 0"]
pub mod ieee_addr_0;
#[doc = "IEEE_ADDR_1 (rw) register accessor: Transceiver MAC IEEE Address Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ieee_addr_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ieee_addr_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ieee_addr_1`]
module"]
pub type IEEE_ADDR_1 = crate::Reg<ieee_addr_1::IEEE_ADDR_1_SPEC>;
#[doc = "Transceiver MAC IEEE Address Register 1"]
pub mod ieee_addr_1;
#[doc = "IEEE_ADDR_2 (rw) register accessor: Transceiver MAC IEEE Address Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ieee_addr_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ieee_addr_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ieee_addr_2`]
module"]
pub type IEEE_ADDR_2 = crate::Reg<ieee_addr_2::IEEE_ADDR_2_SPEC>;
#[doc = "Transceiver MAC IEEE Address Register 2"]
pub mod ieee_addr_2;
#[doc = "IEEE_ADDR_3 (rw) register accessor: Transceiver MAC IEEE Address Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ieee_addr_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ieee_addr_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ieee_addr_3`]
module"]
pub type IEEE_ADDR_3 = crate::Reg<ieee_addr_3::IEEE_ADDR_3_SPEC>;
#[doc = "Transceiver MAC IEEE Address Register 3"]
pub mod ieee_addr_3;
#[doc = "IEEE_ADDR_4 (rw) register accessor: Transceiver MAC IEEE Address Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ieee_addr_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ieee_addr_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ieee_addr_4`]
module"]
pub type IEEE_ADDR_4 = crate::Reg<ieee_addr_4::IEEE_ADDR_4_SPEC>;
#[doc = "Transceiver MAC IEEE Address Register 4"]
pub mod ieee_addr_4;
#[doc = "IEEE_ADDR_5 (rw) register accessor: Transceiver MAC IEEE Address Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ieee_addr_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ieee_addr_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ieee_addr_5`]
module"]
pub type IEEE_ADDR_5 = crate::Reg<ieee_addr_5::IEEE_ADDR_5_SPEC>;
#[doc = "Transceiver MAC IEEE Address Register 5"]
pub mod ieee_addr_5;
#[doc = "IEEE_ADDR_6 (rw) register accessor: Transceiver MAC IEEE Address Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ieee_addr_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ieee_addr_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ieee_addr_6`]
module"]
pub type IEEE_ADDR_6 = crate::Reg<ieee_addr_6::IEEE_ADDR_6_SPEC>;
#[doc = "Transceiver MAC IEEE Address Register 6"]
pub mod ieee_addr_6;
#[doc = "IEEE_ADDR_7 (rw) register accessor: Transceiver MAC IEEE Address Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ieee_addr_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ieee_addr_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ieee_addr_7`]
module"]
pub type IEEE_ADDR_7 = crate::Reg<ieee_addr_7::IEEE_ADDR_7_SPEC>;
#[doc = "Transceiver MAC IEEE Address Register 7"]
pub mod ieee_addr_7;
#[doc = "IRQ_MASK (rw) register accessor: Transceiver Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_mask`]
module"]
pub type IRQ_MASK = crate::Reg<irq_mask::IRQ_MASK_SPEC>;
#[doc = "Transceiver Interrupt Enable Register"]
pub mod irq_mask;
#[doc = "IRQ_STATUS (rw) register accessor: Transceiver Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_status`]
module"]
pub type IRQ_STATUS = crate::Reg<irq_status::IRQ_STATUS_SPEC>;
#[doc = "Transceiver Interrupt Status Register"]
pub mod irq_status;
#[doc = "MAN_ID_0 (rw) register accessor: Device Identification Register (Manufacture ID Low Byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`man_id_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`man_id_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@man_id_0`]
module"]
pub type MAN_ID_0 = crate::Reg<man_id_0::MAN_ID_0_SPEC>;
#[doc = "Device Identification Register (Manufacture ID Low Byte)"]
pub mod man_id_0;
#[doc = "MAN_ID_1 (rw) register accessor: Device Identification Register (Manufacture ID High Byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`man_id_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`man_id_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@man_id_1`]
module"]
pub type MAN_ID_1 = crate::Reg<man_id_1::MAN_ID_1_SPEC>;
#[doc = "Device Identification Register (Manufacture ID High Byte)"]
pub mod man_id_1;
#[doc = "PAN_ID_0 (rw) register accessor: Transceiver Personal Area Network ID Register (Low Byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pan_id_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pan_id_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pan_id_0`]
module"]
pub type PAN_ID_0 = crate::Reg<pan_id_0::PAN_ID_0_SPEC>;
#[doc = "Transceiver Personal Area Network ID Register (Low Byte)"]
pub mod pan_id_0;
#[doc = "PAN_ID_1 (rw) register accessor: Transceiver Personal Area Network ID Register (High Byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pan_id_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pan_id_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pan_id_1`]
module"]
pub type PAN_ID_1 = crate::Reg<pan_id_1::PAN_ID_1_SPEC>;
#[doc = "Transceiver Personal Area Network ID Register (High Byte)"]
pub mod pan_id_1;
#[doc = "PART_NUM (rw) register accessor: Device Identification Register (Part Number)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`part_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`part_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@part_num`]
module"]
pub type PART_NUM = crate::Reg<part_num::PART_NUM_SPEC>;
#[doc = "Device Identification Register (Part Number)"]
pub mod part_num;
#[doc = "PHY_CC_CCA (rw) register accessor: Transceiver Clear Channel Assessment (CCA) Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_cc_cca::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_cc_cca::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cc_cca`]
module"]
pub type PHY_CC_CCA = crate::Reg<phy_cc_cca::PHY_CC_CCA_SPEC>;
#[doc = "Transceiver Clear Channel Assessment (CCA) Control Register"]
pub mod phy_cc_cca;
#[doc = "PHY_ED_LEVEL (rw) register accessor: Transceiver Energy Detection Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_ed_level::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_ed_level::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_ed_level`]
module"]
pub type PHY_ED_LEVEL = crate::Reg<phy_ed_level::PHY_ED_LEVEL_SPEC>;
#[doc = "Transceiver Energy Detection Level Register"]
pub mod phy_ed_level;
#[doc = "PHY_RSSI (rw) register accessor: Receiver Signal Strength Indicator Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_rssi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_rssi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_rssi`]
module"]
pub type PHY_RSSI = crate::Reg<phy_rssi::PHY_RSSI_SPEC>;
#[doc = "Receiver Signal Strength Indicator Register"]
pub mod phy_rssi;
#[doc = "PHY_TX_PWR (rw) register accessor: Transceiver Transmit Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tx_pwr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tx_pwr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_tx_pwr`]
module"]
pub type PHY_TX_PWR = crate::Reg<phy_tx_pwr::PHY_TX_PWR_SPEC>;
#[doc = "Transceiver Transmit Power Control Register"]
pub mod phy_tx_pwr;
#[doc = "PLL_CF (rw) register accessor: Transceiver Center Frequency Calibration Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_cf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_cf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_cf`]
module"]
pub type PLL_CF = crate::Reg<pll_cf::PLL_CF_SPEC>;
#[doc = "Transceiver Center Frequency Calibration Control Register"]
pub mod pll_cf;
#[doc = "PLL_DCU (rw) register accessor: Transceiver Delay Cell Calibration Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_dcu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_dcu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_dcu`]
module"]
pub type PLL_DCU = crate::Reg<pll_dcu::PLL_DCU_SPEC>;
#[doc = "Transceiver Delay Cell Calibration Control Register"]
pub mod pll_dcu;
#[doc = "RX_CTRL (rw) register accessor: Transceiver Receive Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ctrl`]
module"]
pub type RX_CTRL = crate::Reg<rx_ctrl::RX_CTRL_SPEC>;
#[doc = "Transceiver Receive Control Register"]
pub mod rx_ctrl;
#[doc = "RX_SYN (rw) register accessor: Transceiver Receiver Sensitivity Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_syn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_syn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_syn`]
module"]
pub type RX_SYN = crate::Reg<rx_syn::RX_SYN_SPEC>;
#[doc = "Transceiver Receiver Sensitivity Control Register"]
pub mod rx_syn;
#[doc = "SFD_VALUE (rw) register accessor: Start of Frame Delimiter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfd_value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfd_value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfd_value`]
module"]
pub type SFD_VALUE = crate::Reg<sfd_value::SFD_VALUE_SPEC>;
#[doc = "Start of Frame Delimiter Value Register"]
pub mod sfd_value;
#[doc = "SHORT_ADDR_0 (rw) register accessor: Transceiver MAC Short Address Register (Low Byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`short_addr_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`short_addr_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@short_addr_0`]
module"]
pub type SHORT_ADDR_0 = crate::Reg<short_addr_0::SHORT_ADDR_0_SPEC>;
#[doc = "Transceiver MAC Short Address Register (Low Byte)"]
pub mod short_addr_0;
#[doc = "SHORT_ADDR_1 (rw) register accessor: Transceiver MAC Short Address Register (High Byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`short_addr_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`short_addr_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@short_addr_1`]
module"]
pub type SHORT_ADDR_1 = crate::Reg<short_addr_1::SHORT_ADDR_1_SPEC>;
#[doc = "Transceiver MAC Short Address Register (High Byte)"]
pub mod short_addr_1;
#[doc = "TRXFBEND (rw) register accessor: End of frame buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trxfbend::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trxfbend::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trxfbend`]
module"]
pub type TRXFBEND = crate::Reg<trxfbend::TRXFBEND_SPEC>;
#[doc = "End of frame buffer"]
pub mod trxfbend;
#[doc = "TRXFBST (rw) register accessor: Start of frame buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trxfbst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trxfbst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trxfbst`]
module"]
pub type TRXFBST = crate::Reg<trxfbst::TRXFBST_SPEC>;
#[doc = "Start of frame buffer"]
pub mod trxfbst;
#[doc = "TRX_CTRL_0 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trx_ctrl_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trx_ctrl_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trx_ctrl_0`]
module"]
pub type TRX_CTRL_0 = crate::Reg<trx_ctrl_0::TRX_CTRL_0_SPEC>;
#[doc = "Reserved"]
pub mod trx_ctrl_0;
#[doc = "TRX_CTRL_1 (rw) register accessor: Transceiver Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trx_ctrl_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trx_ctrl_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trx_ctrl_1`]
module"]
pub type TRX_CTRL_1 = crate::Reg<trx_ctrl_1::TRX_CTRL_1_SPEC>;
#[doc = "Transceiver Control Register 1"]
pub mod trx_ctrl_1;
#[doc = "TRX_CTRL_2 (rw) register accessor: Transceiver Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trx_ctrl_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trx_ctrl_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trx_ctrl_2`]
module"]
pub type TRX_CTRL_2 = crate::Reg<trx_ctrl_2::TRX_CTRL_2_SPEC>;
#[doc = "Transceiver Control Register 2"]
pub mod trx_ctrl_2;
#[doc = "TRX_STATE (rw) register accessor: Transceiver State Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trx_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trx_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trx_state`]
module"]
pub type TRX_STATE = crate::Reg<trx_state::TRX_STATE_SPEC>;
#[doc = "Transceiver State Control Register"]
pub mod trx_state;
#[doc = "TRX_STATUS (rw) register accessor: Transceiver Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trx_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trx_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trx_status`]
module"]
pub type TRX_STATUS = crate::Reg<trx_status::TRX_STATUS_SPEC>;
#[doc = "Transceiver Status Register"]
pub mod trx_status;
#[doc = "TST_CTRL_DIGI (rw) register accessor: Transceiver Digital Test Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tst_ctrl_digi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tst_ctrl_digi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tst_ctrl_digi`]
module"]
pub type TST_CTRL_DIGI = crate::Reg<tst_ctrl_digi::TST_CTRL_DIGI_SPEC>;
#[doc = "Transceiver Digital Test Control Register"]
pub mod tst_ctrl_digi;
#[doc = "TST_RX_LENGTH (rw) register accessor: Transceiver Received Frame Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tst_rx_length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tst_rx_length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tst_rx_length`]
module"]
pub type TST_RX_LENGTH = crate::Reg<tst_rx_length::TST_RX_LENGTH_SPEC>;
#[doc = "Transceiver Received Frame Length Register"]
pub mod tst_rx_length;
#[doc = "VERSION_NUM (rw) register accessor: Device Identification Register (Version Number)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version_num`]
module"]
pub type VERSION_NUM = crate::Reg<version_num::VERSION_NUM_SPEC>;
#[doc = "Device Identification Register (Version Number)"]
pub mod version_num;
#[doc = "VREG_CTRL (rw) register accessor: Voltage Regulator Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vreg_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vreg_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vreg_ctrl`]
module"]
pub type VREG_CTRL = crate::Reg<vreg_ctrl::VREG_CTRL_SPEC>;
#[doc = "Voltage Regulator Control and Status Register"]
pub mod vreg_ctrl;
#[doc = "XAH_CTRL_0 (rw) register accessor: Transceiver Extended Operating Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xah_ctrl_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xah_ctrl_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xah_ctrl_0`]
module"]
pub type XAH_CTRL_0 = crate::Reg<xah_ctrl_0::XAH_CTRL_0_SPEC>;
#[doc = "Transceiver Extended Operating Mode Control Register"]
pub mod xah_ctrl_0;
#[doc = "XAH_CTRL_1 (rw) register accessor: Transceiver Acknowledgment Frame Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xah_ctrl_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xah_ctrl_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xah_ctrl_1`]
module"]
pub type XAH_CTRL_1 = crate::Reg<xah_ctrl_1::XAH_CTRL_1_SPEC>;
#[doc = "Transceiver Acknowledgment Frame Control Register 1"]
pub mod xah_ctrl_1;
#[doc = "XOSC_CTRL (rw) register accessor: Crystal Oscillator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xosc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xosc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xosc_ctrl`]
module"]
pub type XOSC_CTRL = crate::Reg<xosc_ctrl::XOSC_CTRL_SPEC>;
#[doc = "Crystal Oscillator Control Register"]
pub mod xosc_ctrl;
