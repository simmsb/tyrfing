#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    lockbit: LOCKBIT,
}
impl RegisterBlock {
    #[doc = "0x00 - Lock Bits"]
    #[inline(always)]
    pub const fn lockbit(&self) -> &LOCKBIT {
        &self.lockbit
    }
}
#[doc = "LOCKBIT (rw) register accessor: Lock Bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lockbit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lockbit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lockbit`]
module"]
pub type LOCKBIT = crate::Reg<lockbit::LOCKBIT_SPEC>;
#[doc = "Lock Bits"]
pub mod lockbit;
