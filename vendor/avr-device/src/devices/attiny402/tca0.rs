#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_split_ctrla: [u8; 0x01],
    _reserved_1_split_ctrlb: [u8; 0x01],
    _reserved_2_split_ctrlc: [u8; 0x01],
    _reserved_3_split_ctrld: [u8; 0x01],
    _reserved_4_split_ctrleclr: [u8; 0x01],
    _reserved_5_split_ctrleset: [u8; 0x01],
    single_ctrlfclr: SINGLE_CTRLFCLR,
    single_ctrlfset: SINGLE_CTRLFSET,
    _reserved8: [u8; 0x01],
    single_evctrl: SINGLE_EVCTRL,
    _reserved_9_split_intctrl: [u8; 0x01],
    _reserved_10_split_intflags: [u8; 0x01],
    _reserved11: [u8; 0x02],
    _reserved_11_split_dbgctrl: [u8; 0x01],
    single_temp: SINGLE_TEMP,
    _reserved13: [u8; 0x10],
    _reserved_13_single_cnt: [u8; 0x02],
    _reserved14: [u8; 0x04],
    _reserved_14_single_per: [u8; 0x02],
    _reserved_15_single_cmp0: [u8; 0x02],
    _reserved_16_single_cmp1: [u8; 0x02],
    _reserved_17_single_cmp2: [u8; 0x02],
    _reserved18: [u8; 0x08],
    single_perbuf: SINGLE_PERBUF,
    single_cmp0buf: SINGLE_CMP0BUF,
    single_cmp1buf: SINGLE_CMP1BUF,
    single_cmp2buf: SINGLE_CMP2BUF,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn split_ctrla(&self) -> &SPLIT_CTRLA {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn single_ctrla(&self) -> &SINGLE_CTRLA {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x01 - Control B"]
    #[inline(always)]
    pub const fn split_ctrlb(&self) -> &SPLIT_CTRLB {
        unsafe { &*(self as *const Self).cast::<u8>().add(1).cast() }
    }
    #[doc = "0x01 - Control B"]
    #[inline(always)]
    pub const fn single_ctrlb(&self) -> &SINGLE_CTRLB {
        unsafe { &*(self as *const Self).cast::<u8>().add(1).cast() }
    }
    #[doc = "0x02 - Control C"]
    #[inline(always)]
    pub const fn split_ctrlc(&self) -> &SPLIT_CTRLC {
        unsafe { &*(self as *const Self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x02 - Control C"]
    #[inline(always)]
    pub const fn single_ctrlc(&self) -> &SINGLE_CTRLC {
        unsafe { &*(self as *const Self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x03 - Control D"]
    #[inline(always)]
    pub const fn split_ctrld(&self) -> &SPLIT_CTRLD {
        unsafe { &*(self as *const Self).cast::<u8>().add(3).cast() }
    }
    #[doc = "0x03 - Control D"]
    #[inline(always)]
    pub const fn single_ctrld(&self) -> &SINGLE_CTRLD {
        unsafe { &*(self as *const Self).cast::<u8>().add(3).cast() }
    }
    #[doc = "0x04 - Control E Clear"]
    #[inline(always)]
    pub const fn split_ctrleclr(&self) -> &SPLIT_CTRLECLR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Control E Clear"]
    #[inline(always)]
    pub const fn single_ctrleclr(&self) -> &SINGLE_CTRLECLR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x05 - Control E Set"]
    #[inline(always)]
    pub const fn split_ctrleset(&self) -> &SPLIT_CTRLESET {
        unsafe { &*(self as *const Self).cast::<u8>().add(5).cast() }
    }
    #[doc = "0x05 - Control E Set"]
    #[inline(always)]
    pub const fn single_ctrleset(&self) -> &SINGLE_CTRLESET {
        unsafe { &*(self as *const Self).cast::<u8>().add(5).cast() }
    }
    #[doc = "0x06 - Control F Clear"]
    #[inline(always)]
    pub const fn single_ctrlfclr(&self) -> &SINGLE_CTRLFCLR {
        &self.single_ctrlfclr
    }
    #[doc = "0x07 - Control F Set"]
    #[inline(always)]
    pub const fn single_ctrlfset(&self) -> &SINGLE_CTRLFSET {
        &self.single_ctrlfset
    }
    #[doc = "0x09 - Event Control"]
    #[inline(always)]
    pub const fn single_evctrl(&self) -> &SINGLE_EVCTRL {
        &self.single_evctrl
    }
    #[doc = "0x0a - Interrupt Control"]
    #[inline(always)]
    pub const fn split_intctrl(&self) -> &SPLIT_INTCTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(10).cast() }
    }
    #[doc = "0x0a - Interrupt Control"]
    #[inline(always)]
    pub const fn single_intctrl(&self) -> &SINGLE_INTCTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(10).cast() }
    }
    #[doc = "0x0b - Interrupt Flags"]
    #[inline(always)]
    pub const fn split_intflags(&self) -> &SPLIT_INTFLAGS {
        unsafe { &*(self as *const Self).cast::<u8>().add(11).cast() }
    }
    #[doc = "0x0b - Interrupt Flags"]
    #[inline(always)]
    pub const fn single_intflags(&self) -> &SINGLE_INTFLAGS {
        unsafe { &*(self as *const Self).cast::<u8>().add(11).cast() }
    }
    #[doc = "0x0e - Degbug Control"]
    #[inline(always)]
    pub const fn split_dbgctrl(&self) -> &SPLIT_DBGCTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x0e - Degbug Control"]
    #[inline(always)]
    pub const fn single_dbgctrl(&self) -> &SINGLE_DBGCTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x0f - Temporary data for 16-bit Access"]
    #[inline(always)]
    pub const fn single_temp(&self) -> &SINGLE_TEMP {
        &self.single_temp
    }
    #[doc = "0x20 - Low Count"]
    #[inline(always)]
    pub const fn split_lcnt(&self) -> &SPLIT_LCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x20 - Count"]
    #[inline(always)]
    pub const fn single_cnt(&self) -> &SINGLE_CNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x21 - High Count"]
    #[inline(always)]
    pub const fn split_hcnt(&self) -> &SPLIT_HCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(33).cast() }
    }
    #[doc = "0x26 - Low Period"]
    #[inline(always)]
    pub const fn split_lper(&self) -> &SPLIT_LPER {
        unsafe { &*(self as *const Self).cast::<u8>().add(38).cast() }
    }
    #[doc = "0x26 - Period"]
    #[inline(always)]
    pub const fn single_per(&self) -> &SINGLE_PER {
        unsafe { &*(self as *const Self).cast::<u8>().add(38).cast() }
    }
    #[doc = "0x27 - High Period"]
    #[inline(always)]
    pub const fn split_hper(&self) -> &SPLIT_HPER {
        unsafe { &*(self as *const Self).cast::<u8>().add(39).cast() }
    }
    #[doc = "0x28 - Low Compare"]
    #[inline(always)]
    pub const fn split_lcmp0(&self) -> &SPLIT_LCMP0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - Compare 0"]
    #[inline(always)]
    pub const fn single_cmp0(&self) -> &SINGLE_CMP0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x29 - High Compare"]
    #[inline(always)]
    pub const fn split_hcmp0(&self) -> &SPLIT_HCMP0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(41).cast() }
    }
    #[doc = "0x2a - Low Compare"]
    #[inline(always)]
    pub const fn split_lcmp1(&self) -> &SPLIT_LCMP1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2a - Compare 1"]
    #[inline(always)]
    pub const fn single_cmp1(&self) -> &SINGLE_CMP1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2b - High Compare"]
    #[inline(always)]
    pub const fn split_hcmp1(&self) -> &SPLIT_HCMP1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(43).cast() }
    }
    #[doc = "0x2c - Low Compare"]
    #[inline(always)]
    pub const fn split_lcmp2(&self) -> &SPLIT_LCMP2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2c - Compare 2"]
    #[inline(always)]
    pub const fn single_cmp2(&self) -> &SINGLE_CMP2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2d - High Compare"]
    #[inline(always)]
    pub const fn split_hcmp2(&self) -> &SPLIT_HCMP2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(45).cast() }
    }
    #[doc = "0x36 - Period Buffer"]
    #[inline(always)]
    pub const fn single_perbuf(&self) -> &SINGLE_PERBUF {
        &self.single_perbuf
    }
    #[doc = "0x38 - Compare 0 Buffer"]
    #[inline(always)]
    pub const fn single_cmp0buf(&self) -> &SINGLE_CMP0BUF {
        &self.single_cmp0buf
    }
    #[doc = "0x3a - Compare 1 Buffer"]
    #[inline(always)]
    pub const fn single_cmp1buf(&self) -> &SINGLE_CMP1BUF {
        &self.single_cmp1buf
    }
    #[doc = "0x3c - Compare 2 Buffer"]
    #[inline(always)]
    pub const fn single_cmp2buf(&self) -> &SINGLE_CMP2BUF {
        &self.single_cmp2buf
    }
}
#[doc = "SINGLE_CMP0 (rw) register accessor: Compare 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_cmp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_cmp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_cmp0`]
module"]
pub type SINGLE_CMP0 = crate::Reg<single_cmp0::SINGLE_CMP0_SPEC>;
#[doc = "Compare 0"]
pub mod single_cmp0;
#[doc = "SINGLE_CMP0BUF (rw) register accessor: Compare 0 Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_cmp0buf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_cmp0buf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_cmp0buf`]
module"]
pub type SINGLE_CMP0BUF = crate::Reg<single_cmp0buf::SINGLE_CMP0BUF_SPEC>;
#[doc = "Compare 0 Buffer"]
pub mod single_cmp0buf;
#[doc = "SINGLE_CMP1 (rw) register accessor: Compare 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_cmp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_cmp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_cmp1`]
module"]
pub type SINGLE_CMP1 = crate::Reg<single_cmp1::SINGLE_CMP1_SPEC>;
#[doc = "Compare 1"]
pub mod single_cmp1;
#[doc = "SINGLE_CMP1BUF (rw) register accessor: Compare 1 Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_cmp1buf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_cmp1buf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_cmp1buf`]
module"]
pub type SINGLE_CMP1BUF = crate::Reg<single_cmp1buf::SINGLE_CMP1BUF_SPEC>;
#[doc = "Compare 1 Buffer"]
pub mod single_cmp1buf;
#[doc = "SINGLE_CMP2 (rw) register accessor: Compare 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_cmp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_cmp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_cmp2`]
module"]
pub type SINGLE_CMP2 = crate::Reg<single_cmp2::SINGLE_CMP2_SPEC>;
#[doc = "Compare 2"]
pub mod single_cmp2;
#[doc = "SINGLE_CMP2BUF (rw) register accessor: Compare 2 Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_cmp2buf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_cmp2buf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_cmp2buf`]
module"]
pub type SINGLE_CMP2BUF = crate::Reg<single_cmp2buf::SINGLE_CMP2BUF_SPEC>;
#[doc = "Compare 2 Buffer"]
pub mod single_cmp2buf;
#[doc = "SINGLE_CNT (rw) register accessor: Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_cnt`]
module"]
pub type SINGLE_CNT = crate::Reg<single_cnt::SINGLE_CNT_SPEC>;
#[doc = "Count"]
pub mod single_cnt;
#[doc = "SINGLE_CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_ctrla`]
module"]
pub type SINGLE_CTRLA = crate::Reg<single_ctrla::SINGLE_CTRLA_SPEC>;
#[doc = "Control A"]
pub mod single_ctrla;
#[doc = "SINGLE_CTRLB (rw) register accessor: Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_ctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_ctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_ctrlb`]
module"]
pub type SINGLE_CTRLB = crate::Reg<single_ctrlb::SINGLE_CTRLB_SPEC>;
#[doc = "Control B"]
pub mod single_ctrlb;
#[doc = "SINGLE_CTRLC (rw) register accessor: Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_ctrlc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_ctrlc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_ctrlc`]
module"]
pub type SINGLE_CTRLC = crate::Reg<single_ctrlc::SINGLE_CTRLC_SPEC>;
#[doc = "Control C"]
pub mod single_ctrlc;
#[doc = "SINGLE_CTRLD (rw) register accessor: Control D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_ctrld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_ctrld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_ctrld`]
module"]
pub type SINGLE_CTRLD = crate::Reg<single_ctrld::SINGLE_CTRLD_SPEC>;
#[doc = "Control D"]
pub mod single_ctrld;
#[doc = "SINGLE_CTRLECLR (rw) register accessor: Control E Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_ctrleclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_ctrleclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_ctrleclr`]
module"]
pub type SINGLE_CTRLECLR = crate::Reg<single_ctrleclr::SINGLE_CTRLECLR_SPEC>;
#[doc = "Control E Clear"]
pub mod single_ctrleclr;
#[doc = "SINGLE_CTRLESET (rw) register accessor: Control E Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_ctrleset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_ctrleset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_ctrleset`]
module"]
pub type SINGLE_CTRLESET = crate::Reg<single_ctrleset::SINGLE_CTRLESET_SPEC>;
#[doc = "Control E Set"]
pub mod single_ctrleset;
#[doc = "SINGLE_CTRLFCLR (rw) register accessor: Control F Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_ctrlfclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_ctrlfclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_ctrlfclr`]
module"]
pub type SINGLE_CTRLFCLR = crate::Reg<single_ctrlfclr::SINGLE_CTRLFCLR_SPEC>;
#[doc = "Control F Clear"]
pub mod single_ctrlfclr;
#[doc = "SINGLE_CTRLFSET (rw) register accessor: Control F Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_ctrlfset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_ctrlfset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_ctrlfset`]
module"]
pub type SINGLE_CTRLFSET = crate::Reg<single_ctrlfset::SINGLE_CTRLFSET_SPEC>;
#[doc = "Control F Set"]
pub mod single_ctrlfset;
#[doc = "SINGLE_DBGCTRL (rw) register accessor: Degbug Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_dbgctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_dbgctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_dbgctrl`]
module"]
pub type SINGLE_DBGCTRL = crate::Reg<single_dbgctrl::SINGLE_DBGCTRL_SPEC>;
#[doc = "Degbug Control"]
pub mod single_dbgctrl;
#[doc = "SINGLE_EVCTRL (rw) register accessor: Event Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_evctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_evctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_evctrl`]
module"]
pub type SINGLE_EVCTRL = crate::Reg<single_evctrl::SINGLE_EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod single_evctrl;
#[doc = "SINGLE_INTCTRL (rw) register accessor: Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_intctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_intctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_intctrl`]
module"]
pub type SINGLE_INTCTRL = crate::Reg<single_intctrl::SINGLE_INTCTRL_SPEC>;
#[doc = "Interrupt Control"]
pub mod single_intctrl;
#[doc = "SINGLE_INTFLAGS (rw) register accessor: Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_intflags::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_intflags::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_intflags`]
module"]
pub type SINGLE_INTFLAGS = crate::Reg<single_intflags::SINGLE_INTFLAGS_SPEC>;
#[doc = "Interrupt Flags"]
pub mod single_intflags;
#[doc = "SINGLE_PER (rw) register accessor: Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_per::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_per::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_per`]
module"]
pub type SINGLE_PER = crate::Reg<single_per::SINGLE_PER_SPEC>;
#[doc = "Period"]
pub mod single_per;
#[doc = "SINGLE_PERBUF (rw) register accessor: Period Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_perbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_perbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_perbuf`]
module"]
pub type SINGLE_PERBUF = crate::Reg<single_perbuf::SINGLE_PERBUF_SPEC>;
#[doc = "Period Buffer"]
pub mod single_perbuf;
#[doc = "SINGLE_TEMP (rw) register accessor: Temporary data for 16-bit Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_temp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_temp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_temp`]
module"]
pub type SINGLE_TEMP = crate::Reg<single_temp::SINGLE_TEMP_SPEC>;
#[doc = "Temporary data for 16-bit Access"]
pub mod single_temp;
#[doc = "SPLIT_CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_ctrla`]
module"]
pub type SPLIT_CTRLA = crate::Reg<split_ctrla::SPLIT_CTRLA_SPEC>;
#[doc = "Control A"]
pub mod split_ctrla;
#[doc = "SPLIT_CTRLB (rw) register accessor: Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_ctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_ctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_ctrlb`]
module"]
pub type SPLIT_CTRLB = crate::Reg<split_ctrlb::SPLIT_CTRLB_SPEC>;
#[doc = "Control B"]
pub mod split_ctrlb;
#[doc = "SPLIT_CTRLC (rw) register accessor: Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_ctrlc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_ctrlc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_ctrlc`]
module"]
pub type SPLIT_CTRLC = crate::Reg<split_ctrlc::SPLIT_CTRLC_SPEC>;
#[doc = "Control C"]
pub mod split_ctrlc;
#[doc = "SPLIT_CTRLD (rw) register accessor: Control D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_ctrld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_ctrld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_ctrld`]
module"]
pub type SPLIT_CTRLD = crate::Reg<split_ctrld::SPLIT_CTRLD_SPEC>;
#[doc = "Control D"]
pub mod split_ctrld;
#[doc = "SPLIT_CTRLECLR (rw) register accessor: Control E Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_ctrleclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_ctrleclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_ctrleclr`]
module"]
pub type SPLIT_CTRLECLR = crate::Reg<split_ctrleclr::SPLIT_CTRLECLR_SPEC>;
#[doc = "Control E Clear"]
pub mod split_ctrleclr;
#[doc = "SPLIT_CTRLESET (rw) register accessor: Control E Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_ctrleset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_ctrleset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_ctrleset`]
module"]
pub type SPLIT_CTRLESET = crate::Reg<split_ctrleset::SPLIT_CTRLESET_SPEC>;
#[doc = "Control E Set"]
pub mod split_ctrleset;
#[doc = "SPLIT_DBGCTRL (rw) register accessor: Degbug Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_dbgctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_dbgctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_dbgctrl`]
module"]
pub type SPLIT_DBGCTRL = crate::Reg<split_dbgctrl::SPLIT_DBGCTRL_SPEC>;
#[doc = "Degbug Control"]
pub mod split_dbgctrl;
#[doc = "SPLIT_HCMP0 (rw) register accessor: High Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_hcmp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_hcmp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_hcmp0`]
module"]
pub type SPLIT_HCMP0 = crate::Reg<split_hcmp0::SPLIT_HCMP0_SPEC>;
#[doc = "High Compare"]
pub mod split_hcmp0;
#[doc = "SPLIT_HCMP1 (rw) register accessor: High Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_hcmp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_hcmp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_hcmp1`]
module"]
pub type SPLIT_HCMP1 = crate::Reg<split_hcmp1::SPLIT_HCMP1_SPEC>;
#[doc = "High Compare"]
pub mod split_hcmp1;
#[doc = "SPLIT_HCMP2 (rw) register accessor: High Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_hcmp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_hcmp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_hcmp2`]
module"]
pub type SPLIT_HCMP2 = crate::Reg<split_hcmp2::SPLIT_HCMP2_SPEC>;
#[doc = "High Compare"]
pub mod split_hcmp2;
#[doc = "SPLIT_HCNT (rw) register accessor: High Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_hcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_hcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_hcnt`]
module"]
pub type SPLIT_HCNT = crate::Reg<split_hcnt::SPLIT_HCNT_SPEC>;
#[doc = "High Count"]
pub mod split_hcnt;
#[doc = "SPLIT_HPER (rw) register accessor: High Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_hper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_hper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_hper`]
module"]
pub type SPLIT_HPER = crate::Reg<split_hper::SPLIT_HPER_SPEC>;
#[doc = "High Period"]
pub mod split_hper;
#[doc = "SPLIT_INTCTRL (rw) register accessor: Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_intctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_intctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_intctrl`]
module"]
pub type SPLIT_INTCTRL = crate::Reg<split_intctrl::SPLIT_INTCTRL_SPEC>;
#[doc = "Interrupt Control"]
pub mod split_intctrl;
#[doc = "SPLIT_INTFLAGS (rw) register accessor: Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_intflags::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_intflags::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_intflags`]
module"]
pub type SPLIT_INTFLAGS = crate::Reg<split_intflags::SPLIT_INTFLAGS_SPEC>;
#[doc = "Interrupt Flags"]
pub mod split_intflags;
#[doc = "SPLIT_LCMP0 (rw) register accessor: Low Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_lcmp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_lcmp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_lcmp0`]
module"]
pub type SPLIT_LCMP0 = crate::Reg<split_lcmp0::SPLIT_LCMP0_SPEC>;
#[doc = "Low Compare"]
pub mod split_lcmp0;
#[doc = "SPLIT_LCMP1 (rw) register accessor: Low Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_lcmp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_lcmp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_lcmp1`]
module"]
pub type SPLIT_LCMP1 = crate::Reg<split_lcmp1::SPLIT_LCMP1_SPEC>;
#[doc = "Low Compare"]
pub mod split_lcmp1;
#[doc = "SPLIT_LCMP2 (rw) register accessor: Low Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_lcmp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_lcmp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_lcmp2`]
module"]
pub type SPLIT_LCMP2 = crate::Reg<split_lcmp2::SPLIT_LCMP2_SPEC>;
#[doc = "Low Compare"]
pub mod split_lcmp2;
#[doc = "SPLIT_LCNT (rw) register accessor: Low Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_lcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_lcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_lcnt`]
module"]
pub type SPLIT_LCNT = crate::Reg<split_lcnt::SPLIT_LCNT_SPEC>;
#[doc = "Low Count"]
pub mod split_lcnt;
#[doc = "SPLIT_LPER (rw) register accessor: Low Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_lper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_lper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@split_lper`]
module"]
pub type SPLIT_LPER = crate::Reg<split_lper::SPLIT_LPER_SPEC>;
#[doc = "Low Period"]
pub mod split_lper;
