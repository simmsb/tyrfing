#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    usicr: USICR,
    usisr: USISR,
    usidr: USIDR,
    usibr: USIBR,
    usipp: USIPP,
}
impl RegisterBlock {
    #[doc = "0x00 - USI Control Register"]
    #[inline(always)]
    pub const fn usicr(&self) -> &USICR {
        &self.usicr
    }
    #[doc = "0x01 - USI Status Register"]
    #[inline(always)]
    pub const fn usisr(&self) -> &USISR {
        &self.usisr
    }
    #[doc = "0x02 - USI Data Register"]
    #[inline(always)]
    pub const fn usidr(&self) -> &USIDR {
        &self.usidr
    }
    #[doc = "0x03 - USI Buffer Register"]
    #[inline(always)]
    pub const fn usibr(&self) -> &USIBR {
        &self.usibr
    }
    #[doc = "0x04 - USI Pin Position"]
    #[inline(always)]
    pub const fn usipp(&self) -> &USIPP {
        &self.usipp
    }
}
#[doc = "USIBR (rw) register accessor: USI Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usibr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usibr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usibr`]
module"]
pub type USIBR = crate::Reg<usibr::USIBR_SPEC>;
#[doc = "USI Buffer Register"]
pub mod usibr;
#[doc = "USICR (rw) register accessor: USI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usicr`]
module"]
pub type USICR = crate::Reg<usicr::USICR_SPEC>;
#[doc = "USI Control Register"]
pub mod usicr;
#[doc = "USIDR (rw) register accessor: USI Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usidr`]
module"]
pub type USIDR = crate::Reg<usidr::USIDR_SPEC>;
#[doc = "USI Data Register"]
pub mod usidr;
#[doc = "USIPP (rw) register accessor: USI Pin Position\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usipp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usipp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usipp`]
module"]
pub type USIPP = crate::Reg<usipp::USIPP_SPEC>;
#[doc = "USI Pin Position"]
pub mod usipp;
#[doc = "USISR (rw) register accessor: USI Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usisr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usisr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usisr`]
module"]
pub type USISR = crate::Reg<usisr::USISR_SPEC>;
#[doc = "USI Status Register"]
pub mod usisr;
