#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    rxdatal: RXDATAL,
    rxdatah: RXDATAH,
    txdatal: TXDATAL,
    txdatah: TXDATAH,
    status: STATUS,
    ctrla: CTRLA,
    ctrlb: CTRLB,
    ctrlc: CTRLC,
    baud: BAUD,
    _reserved9: [u8; 0x01],
    dbgctrl: DBGCTRL,
    evctrl: EVCTRL,
    txplctrl: TXPLCTRL,
    rxplctrl: RXPLCTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Receive Data Low Byte"]
    #[inline(always)]
    pub const fn rxdatal(&self) -> &RXDATAL {
        &self.rxdatal
    }
    #[doc = "0x01 - Receive Data High Byte"]
    #[inline(always)]
    pub const fn rxdatah(&self) -> &RXDATAH {
        &self.rxdatah
    }
    #[doc = "0x02 - Transmit Data Low Byte"]
    #[inline(always)]
    pub const fn txdatal(&self) -> &TXDATAL {
        &self.txdatal
    }
    #[doc = "0x03 - Transmit Data High Byte"]
    #[inline(always)]
    pub const fn txdatah(&self) -> &TXDATAH {
        &self.txdatah
    }
    #[doc = "0x04 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x05 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &CTRLA {
        &self.ctrla
    }
    #[doc = "0x06 - Control B"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &CTRLB {
        &self.ctrlb
    }
    #[doc = "0x07 - Control C"]
    #[inline(always)]
    pub const fn ctrlc(&self) -> &CTRLC {
        &self.ctrlc
    }
    #[doc = "0x08 - Baud Rate"]
    #[inline(always)]
    pub const fn baud(&self) -> &BAUD {
        &self.baud
    }
    #[doc = "0x0b - Debug Control"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &DBGCTRL {
        &self.dbgctrl
    }
    #[doc = "0x0c - Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &EVCTRL {
        &self.evctrl
    }
    #[doc = "0x0d - IRCOM Transmitter Pulse Length Control"]
    #[inline(always)]
    pub const fn txplctrl(&self) -> &TXPLCTRL {
        &self.txplctrl
    }
    #[doc = "0x0e - IRCOM Receiver Pulse Length Control"]
    #[inline(always)]
    pub const fn rxplctrl(&self) -> &RXPLCTRL {
        &self.rxplctrl
    }
}
#[doc = "BAUD (rw) register accessor: Baud Rate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baud::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baud::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud`]
module"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "Baud Rate"]
pub mod baud;
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`]
module"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "CTRLC (rw) register accessor: Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlc`]
module"]
pub type CTRLC = crate::Reg<ctrlc::CTRLC_SPEC>;
#[doc = "Control C"]
pub mod ctrlc;
#[doc = "DBGCTRL (rw) register accessor: Debug Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "EVCTRL (rw) register accessor: Event Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`]
module"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "RXDATAH (r) register accessor: Receive Data High Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdatah::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdatah`]
module"]
pub type RXDATAH = crate::Reg<rxdatah::RXDATAH_SPEC>;
#[doc = "Receive Data High Byte"]
pub mod rxdatah;
#[doc = "RXDATAL (r) register accessor: Receive Data Low Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdatal::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdatal`]
module"]
pub type RXDATAL = crate::Reg<rxdatal::RXDATAL_SPEC>;
#[doc = "Receive Data Low Byte"]
pub mod rxdatal;
#[doc = "RXPLCTRL (rw) register accessor: IRCOM Receiver Pulse Length Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxplctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxplctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxplctrl`]
module"]
pub type RXPLCTRL = crate::Reg<rxplctrl::RXPLCTRL_SPEC>;
#[doc = "IRCOM Receiver Pulse Length Control"]
pub mod rxplctrl;
#[doc = "STATUS (rw) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "TXDATAH (rw) register accessor: Transmit Data High Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdatah::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdatah::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdatah`]
module"]
pub type TXDATAH = crate::Reg<txdatah::TXDATAH_SPEC>;
#[doc = "Transmit Data High Byte"]
pub mod txdatah;
#[doc = "TXDATAL (rw) register accessor: Transmit Data Low Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdatal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdatal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdatal`]
module"]
pub type TXDATAL = crate::Reg<txdatal::TXDATAL_SPEC>;
#[doc = "Transmit Data Low Byte"]
pub mod txdatal;
#[doc = "TXPLCTRL (rw) register accessor: IRCOM Transmitter Pulse Length Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txplctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txplctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txplctrl`]
module"]
pub type TXPLCTRL = crate::Reg<txplctrl::TXPLCTRL_SPEC>;
#[doc = "IRCOM Transmitter Pulse Length Control"]
pub mod txplctrl;
