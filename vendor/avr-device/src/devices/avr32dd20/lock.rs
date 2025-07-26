#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    key: KEY,
}
impl RegisterBlock {
    #[doc = "0x00 - Lock Key Bits"]
    #[inline(always)]
    pub const fn key(&self) -> &KEY {
        &self.key
    }
}
#[doc = "KEY (rw) register accessor: Lock Key Bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`]
module"]
pub type KEY = crate::Reg<key::KEY_SPEC>;
#[doc = "Lock Key Bits"]
pub mod key;
