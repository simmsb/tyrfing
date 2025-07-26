#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrla: CTRLA,
    status: STATUS,
    lvl0pri: LVL0PRI,
    lvl1vec: LVL1VEC,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &CTRLA {
        &self.ctrla
    }
    #[doc = "0x01 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x02 - Interrupt Level 0 Priority"]
    #[inline(always)]
    pub const fn lvl0pri(&self) -> &LVL0PRI {
        &self.lvl0pri
    }
    #[doc = "0x03 - Interrupt Level 1 Priority Vector"]
    #[inline(always)]
    pub const fn lvl1vec(&self) -> &LVL1VEC {
        &self.lvl1vec
    }
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "LVL0PRI (rw) register accessor: Interrupt Level 0 Priority\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvl0pri::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvl0pri::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvl0pri`]
module"]
pub type LVL0PRI = crate::Reg<lvl0pri::LVL0PRI_SPEC>;
#[doc = "Interrupt Level 0 Priority"]
pub mod lvl0pri;
#[doc = "LVL1VEC (rw) register accessor: Interrupt Level 1 Priority Vector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvl1vec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvl1vec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvl1vec`]
module"]
pub type LVL1VEC = crate::Reg<lvl1vec::LVL1VEC_SPEC>;
#[doc = "Interrupt Level 1 Priority Vector"]
pub mod lvl1vec;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
