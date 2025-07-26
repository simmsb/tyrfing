#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    wdtcfg: WDTCFG,
    bodcfg: BODCFG,
    osccfg: OSCCFG,
    _reserved3: [u8; 0x01],
    tcd0cfg: TCD0CFG,
    syscfg0: SYSCFG0,
    syscfg1: SYSCFG1,
    append: APPEND,
    bootend: BOOTEND,
}
impl RegisterBlock {
    #[doc = "0x00 - Watchdog Configuration"]
    #[inline(always)]
    pub const fn wdtcfg(&self) -> &WDTCFG {
        &self.wdtcfg
    }
    #[doc = "0x01 - BOD Configuration"]
    #[inline(always)]
    pub const fn bodcfg(&self) -> &BODCFG {
        &self.bodcfg
    }
    #[doc = "0x02 - Oscillator Configuration"]
    #[inline(always)]
    pub const fn osccfg(&self) -> &OSCCFG {
        &self.osccfg
    }
    #[doc = "0x04 - TCD0 Configuration"]
    #[inline(always)]
    pub const fn tcd0cfg(&self) -> &TCD0CFG {
        &self.tcd0cfg
    }
    #[doc = "0x05 - System Configuration 0"]
    #[inline(always)]
    pub const fn syscfg0(&self) -> &SYSCFG0 {
        &self.syscfg0
    }
    #[doc = "0x06 - System Configuration 1"]
    #[inline(always)]
    pub const fn syscfg1(&self) -> &SYSCFG1 {
        &self.syscfg1
    }
    #[doc = "0x07 - Application Code Section End"]
    #[inline(always)]
    pub const fn append(&self) -> &APPEND {
        &self.append
    }
    #[doc = "0x08 - Boot Section End"]
    #[inline(always)]
    pub const fn bootend(&self) -> &BOOTEND {
        &self.bootend
    }
}
#[doc = "APPEND (r) register accessor: Application Code Section End\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`append::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@append`]
module"]
pub type APPEND = crate::Reg<append::APPEND_SPEC>;
#[doc = "Application Code Section End"]
pub mod append;
#[doc = "BODCFG (r) register accessor: BOD Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bodcfg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bodcfg`]
module"]
pub type BODCFG = crate::Reg<bodcfg::BODCFG_SPEC>;
#[doc = "BOD Configuration"]
pub mod bodcfg;
#[doc = "BOOTEND (r) register accessor: Boot Section End\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bootend::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bootend`]
module"]
pub type BOOTEND = crate::Reg<bootend::BOOTEND_SPEC>;
#[doc = "Boot Section End"]
pub mod bootend;
#[doc = "OSCCFG (r) register accessor: Oscillator Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccfg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osccfg`]
module"]
pub type OSCCFG = crate::Reg<osccfg::OSCCFG_SPEC>;
#[doc = "Oscillator Configuration"]
pub mod osccfg;
#[doc = "SYSCFG0 (r) register accessor: System Configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg0`]
module"]
pub type SYSCFG0 = crate::Reg<syscfg0::SYSCFG0_SPEC>;
#[doc = "System Configuration 0"]
pub mod syscfg0;
#[doc = "SYSCFG1 (r) register accessor: System Configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg1`]
module"]
pub type SYSCFG1 = crate::Reg<syscfg1::SYSCFG1_SPEC>;
#[doc = "System Configuration 1"]
pub mod syscfg1;
#[doc = "TCD0CFG (r) register accessor: TCD0 Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcd0cfg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcd0cfg`]
module"]
pub type TCD0CFG = crate::Reg<tcd0cfg::TCD0CFG_SPEC>;
#[doc = "TCD0 Configuration"]
pub mod tcd0cfg;
#[doc = "WDTCFG (r) register accessor: Watchdog Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtcfg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtcfg`]
module"]
pub type WDTCFG = crate::Reg<wdtcfg::WDTCFG_SPEC>;
#[doc = "Watchdog Configuration"]
pub mod wdtcfg;
