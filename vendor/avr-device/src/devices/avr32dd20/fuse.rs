#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    wdtcfg: WDTCFG,
    bodcfg: BODCFG,
    osccfg: OSCCFG,
    _reserved3: [u8; 0x02],
    syscfg0: SYSCFG0,
    syscfg1: SYSCFG1,
    codesize: CODESIZE,
    bootsize: BOOTSIZE,
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
    #[doc = "0x07 - Code Section Size"]
    #[inline(always)]
    pub const fn codesize(&self) -> &CODESIZE {
        &self.codesize
    }
    #[doc = "0x08 - Boot Section Size"]
    #[inline(always)]
    pub const fn bootsize(&self) -> &BOOTSIZE {
        &self.bootsize
    }
}
#[doc = "BODCFG (rw) register accessor: BOD Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bodcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bodcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bodcfg`]
module"]
pub type BODCFG = crate::Reg<bodcfg::BODCFG_SPEC>;
#[doc = "BOD Configuration"]
pub mod bodcfg;
#[doc = "BOOTSIZE (rw) register accessor: Boot Section Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bootsize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bootsize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bootsize`]
module"]
pub type BOOTSIZE = crate::Reg<bootsize::BOOTSIZE_SPEC>;
#[doc = "Boot Section Size"]
pub mod bootsize;
#[doc = "CODESIZE (rw) register accessor: Code Section Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`codesize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`codesize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codesize`]
module"]
pub type CODESIZE = crate::Reg<codesize::CODESIZE_SPEC>;
#[doc = "Code Section Size"]
pub mod codesize;
#[doc = "OSCCFG (rw) register accessor: Oscillator Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osccfg`]
module"]
pub type OSCCFG = crate::Reg<osccfg::OSCCFG_SPEC>;
#[doc = "Oscillator Configuration"]
pub mod osccfg;
#[doc = "SYSCFG0 (rw) register accessor: System Configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg0`]
module"]
pub type SYSCFG0 = crate::Reg<syscfg0::SYSCFG0_SPEC>;
#[doc = "System Configuration 0"]
pub mod syscfg0;
#[doc = "SYSCFG1 (rw) register accessor: System Configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg1`]
module"]
pub type SYSCFG1 = crate::Reg<syscfg1::SYSCFG1_SPEC>;
#[doc = "System Configuration 1"]
pub mod syscfg1;
#[doc = "WDTCFG (rw) register accessor: Watchdog Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtcfg`]
module"]
pub type WDTCFG = crate::Reg<wdtcfg::WDTCFG_SPEC>;
#[doc = "Watchdog Configuration"]
pub mod wdtcfg;
