#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    asyncstrobe: ASYNCSTROBE,
    syncstrobe: SYNCSTROBE,
    asyncch: [ASYNCCH; 4],
    _reserved3: [u8; 0x04],
    syncch: [SYNCCH; 2],
    _reserved4: [u8; 0x06],
    asyncuser: [ASYNCUSER; 13],
    _reserved5: [u8; 0x03],
    syncuser: [SYNCUSER; 2],
}
impl RegisterBlock {
    #[doc = "0x00 - Asynchronous Channel Strobe"]
    #[inline(always)]
    pub const fn asyncstrobe(&self) -> &ASYNCSTROBE {
        &self.asyncstrobe
    }
    #[doc = "0x01 - Synchronous Channel Strobe"]
    #[inline(always)]
    pub const fn syncstrobe(&self) -> &SYNCSTROBE {
        &self.syncstrobe
    }
    #[doc = "0x02 - Asynchronous Channel %s Generator Selection"]
    #[inline(always)]
    pub const fn asyncch(&self, n: usize) -> &ASYNCCH {
        &self.asyncch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x02 - Asynchronous Channel %s Generator Selection"]
    #[inline(always)]
    pub fn asyncch_iter(&self) -> impl Iterator<Item = &ASYNCCH> {
        self.asyncch.iter()
    }
    #[doc = "0x0a - Synchronous Channel %s Generator Selection"]
    #[inline(always)]
    pub const fn syncch(&self, n: usize) -> &SYNCCH {
        &self.syncch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0a - Synchronous Channel %s Generator Selection"]
    #[inline(always)]
    pub fn syncch_iter(&self) -> impl Iterator<Item = &SYNCCH> {
        self.syncch.iter()
    }
    #[doc = "0x12..0x1f - Asynchronous User Ch %s Input Selection - TCB%s"]
    #[inline(always)]
    pub const fn asyncuser(&self, n: usize) -> &ASYNCUSER {
        &self.asyncuser[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x12..0x1f - Asynchronous User Ch %s Input Selection - TCB%s"]
    #[inline(always)]
    pub fn asyncuser_iter(&self) -> impl Iterator<Item = &ASYNCUSER> {
        self.asyncuser.iter()
    }
    #[doc = "0x22 - Synchronous User Ch %s - TCA%s"]
    #[inline(always)]
    pub const fn syncuser(&self, n: usize) -> &SYNCUSER {
        &self.syncuser[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x22 - Synchronous User Ch %s - TCA%s"]
    #[inline(always)]
    pub fn syncuser_iter(&self) -> impl Iterator<Item = &SYNCUSER> {
        self.syncuser.iter()
    }
}
#[doc = "ASYNCCH (rw) register accessor: Asynchronous Channel %s Generator Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyncch`]
module"]
pub type ASYNCCH = crate::Reg<asyncch::ASYNCCH_SPEC>;
#[doc = "Asynchronous Channel %s Generator Selection"]
pub mod asyncch;
#[doc = "ASYNCSTROBE (w) register accessor: Asynchronous Channel Strobe\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncstrobe::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyncstrobe`]
module"]
pub type ASYNCSTROBE = crate::Reg<asyncstrobe::ASYNCSTROBE_SPEC>;
#[doc = "Asynchronous Channel Strobe"]
pub mod asyncstrobe;
#[doc = "ASYNCUSER (rw) register accessor: Asynchronous User Ch %s Input Selection - TCB%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncuser::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncuser::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyncuser`]
module"]
pub type ASYNCUSER = crate::Reg<asyncuser::ASYNCUSER_SPEC>;
#[doc = "Asynchronous User Ch %s Input Selection - TCB%s"]
pub mod asyncuser;
#[doc = "SYNCCH (rw) register accessor: Synchronous Channel %s Generator Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syncch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncch`]
module"]
pub type SYNCCH = crate::Reg<syncch::SYNCCH_SPEC>;
#[doc = "Synchronous Channel %s Generator Selection"]
pub mod syncch;
#[doc = "SYNCSTROBE (w) register accessor: Synchronous Channel Strobe\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syncstrobe::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncstrobe`]
module"]
pub type SYNCSTROBE = crate::Reg<syncstrobe::SYNCSTROBE_SPEC>;
#[doc = "Synchronous Channel Strobe"]
pub mod syncstrobe;
#[doc = "SYNCUSER (rw) register accessor: Synchronous User Ch %s - TCA%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncuser::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syncuser::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncuser`]
module"]
pub type SYNCUSER = crate::Reg<syncuser::SYNCUSER_SPEC>;
#[doc = "Synchronous User Ch %s - TCA%s"]
pub mod syncuser;
