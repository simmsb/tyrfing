#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    gpr0: GPR0,
    gpr1: GPR1,
    gpr2: GPR2,
    gpr3: GPR3,
}
impl RegisterBlock {
    #[doc = "0x00 - General Purpose Register 0"]
    #[inline(always)]
    pub const fn gpr0(&self) -> &GPR0 {
        &self.gpr0
    }
    #[doc = "0x01 - General Purpose Register 1"]
    #[inline(always)]
    pub const fn gpr1(&self) -> &GPR1 {
        &self.gpr1
    }
    #[doc = "0x02 - General Purpose Register 2"]
    #[inline(always)]
    pub const fn gpr2(&self) -> &GPR2 {
        &self.gpr2
    }
    #[doc = "0x03 - General Purpose Register 3"]
    #[inline(always)]
    pub const fn gpr3(&self) -> &GPR3 {
        &self.gpr3
    }
}
#[doc = "GPR0 (rw) register accessor: General Purpose Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpr0`]
module"]
pub type GPR0 = crate::Reg<gpr0::GPR0_SPEC>;
#[doc = "General Purpose Register 0"]
pub mod gpr0;
#[doc = "GPR1 (rw) register accessor: General Purpose Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpr1`]
module"]
pub type GPR1 = crate::Reg<gpr1::GPR1_SPEC>;
#[doc = "General Purpose Register 1"]
pub mod gpr1;
#[doc = "GPR2 (rw) register accessor: General Purpose Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpr2`]
module"]
pub type GPR2 = crate::Reg<gpr2::GPR2_SPEC>;
#[doc = "General Purpose Register 2"]
pub mod gpr2;
#[doc = "GPR3 (rw) register accessor: General Purpose Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpr3`]
module"]
pub type GPR3 = crate::Reg<gpr3::GPR3_SPEC>;
#[doc = "General Purpose Register 3"]
pub mod gpr3;
