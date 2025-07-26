#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    userrow: [USERROW; 32],
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - User Row Bytes"]
    #[inline(always)]
    pub const fn userrow(&self, n: usize) -> &USERROW {
        &self.userrow[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - User Row Bytes"]
    #[inline(always)]
    pub fn userrow_iter(&self) -> impl Iterator<Item = &USERROW> {
        self.userrow.iter()
    }
}
#[doc = "USERROW (rw) register accessor: User Row Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow`]
module"]
pub type USERROW = crate::Reg<userrow::USERROW_SPEC>;
#[doc = "User Row Bytes"]
pub mod userrow;
