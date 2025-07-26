#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    amiscr: AMISCR,
}
impl RegisterBlock {
    #[doc = "0x00 - Analog Miscellaneous Control Register (Shared with AD_CONVERTER IO_MODULE)"]
    #[inline(always)]
    pub const fn amiscr(&self) -> &AMISCR {
        &self.amiscr
    }
}
#[doc = "AMISCR (rw) register accessor: Analog Miscellaneous Control Register (Shared with AD_CONVERTER IO_MODULE)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amiscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amiscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amiscr`]
module"]
pub type AMISCR = crate::Reg<amiscr::AMISCR_SPEC>;
#[doc = "Analog Miscellaneous Control Register (Shared with AD_CONVERTER IO_MODULE)"]
pub mod amiscr;
