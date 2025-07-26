#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    sfior: SFIOR,
}
impl RegisterBlock {
    #[doc = "0x00 - Special Function IO Register"]
    #[inline(always)]
    pub const fn sfior(&self) -> &SFIOR {
        &self.sfior
    }
}
#[doc = "SFIOR (rw) register accessor: Special Function IO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfior::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfior::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfior`]
module"]
pub type SFIOR = crate::Reg<sfior::SFIOR_SPEC>;
#[doc = "Special Function IO Register"]
pub mod sfior;
