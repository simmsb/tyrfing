#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    asyncstrobe: ASYNCSTROBE,
    syncstrobe: SYNCSTROBE,
    asyncch0: ASYNCCH0,
    asyncch1: ASYNCCH1,
    _reserved4: [u8; 0x06],
    syncch0: SYNCCH0,
    syncch1: SYNCCH1,
    _reserved6: [u8; 0x06],
    asyncuser0: ASYNCUSER0,
    asyncuser1: ASYNCUSER1,
    asyncuser2: ASYNCUSER2,
    asyncuser3: ASYNCUSER3,
    asyncuser4: ASYNCUSER4,
    asyncuser5: ASYNCUSER5,
    asyncuser6: ASYNCUSER6,
    asyncuser7: ASYNCUSER7,
    asyncuser8: ASYNCUSER8,
    asyncuser9: ASYNCUSER9,
    asyncuser10: ASYNCUSER10,
    _reserved17: [u8; 0x05],
    syncuser0: SYNCUSER0,
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
    #[doc = "0x02 - Asynchronous Channel 0 Generator Selection"]
    #[inline(always)]
    pub const fn asyncch0(&self) -> &ASYNCCH0 {
        &self.asyncch0
    }
    #[doc = "0x03 - Asynchronous Channel 1 Generator Selection"]
    #[inline(always)]
    pub const fn asyncch1(&self) -> &ASYNCCH1 {
        &self.asyncch1
    }
    #[doc = "0x0a - Synchronous Channel 0 Generator Selection"]
    #[inline(always)]
    pub const fn syncch0(&self) -> &SYNCCH0 {
        &self.syncch0
    }
    #[doc = "0x0b - Synchronous Channel 1 Generator Selection"]
    #[inline(always)]
    pub const fn syncch1(&self) -> &SYNCCH1 {
        &self.syncch1
    }
    #[doc = "0x12 - Asynchronous User Ch 0 Input Selection - TCB0"]
    #[inline(always)]
    pub const fn asyncuser0(&self) -> &ASYNCUSER0 {
        &self.asyncuser0
    }
    #[doc = "0x13 - Asynchronous User Ch 1 Input Selection - ADC0"]
    #[inline(always)]
    pub const fn asyncuser1(&self) -> &ASYNCUSER1 {
        &self.asyncuser1
    }
    #[doc = "0x14 - Asynchronous User Ch 2 Input Selection - CCL LUT0 Event 0"]
    #[inline(always)]
    pub const fn asyncuser2(&self) -> &ASYNCUSER2 {
        &self.asyncuser2
    }
    #[doc = "0x15 - Asynchronous User Ch 3 Input Selection - CCL LUT1 Event 0"]
    #[inline(always)]
    pub const fn asyncuser3(&self) -> &ASYNCUSER3 {
        &self.asyncuser3
    }
    #[doc = "0x16 - Asynchronous User Ch 4 Input Selection - CCL LUT0 Event 1"]
    #[inline(always)]
    pub const fn asyncuser4(&self) -> &ASYNCUSER4 {
        &self.asyncuser4
    }
    #[doc = "0x17 - Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1"]
    #[inline(always)]
    pub const fn asyncuser5(&self) -> &ASYNCUSER5 {
        &self.asyncuser5
    }
    #[doc = "0x18 - Asynchronous User Ch 6 Input Selection - TCD0 Event 0"]
    #[inline(always)]
    pub const fn asyncuser6(&self) -> &ASYNCUSER6 {
        &self.asyncuser6
    }
    #[doc = "0x19 - Asynchronous User Ch 7 Input Selection - TCD0 Event 1"]
    #[inline(always)]
    pub const fn asyncuser7(&self) -> &ASYNCUSER7 {
        &self.asyncuser7
    }
    #[doc = "0x1a - Asynchronous User Ch 8 Input Selection - Event Out 0"]
    #[inline(always)]
    pub const fn asyncuser8(&self) -> &ASYNCUSER8 {
        &self.asyncuser8
    }
    #[doc = "0x1b - Asynchronous User Ch 9 Input Selection - Event Out 1"]
    #[inline(always)]
    pub const fn asyncuser9(&self) -> &ASYNCUSER9 {
        &self.asyncuser9
    }
    #[doc = "0x1c - Asynchronous User Ch 10 Input Selection - Event Out 2"]
    #[inline(always)]
    pub const fn asyncuser10(&self) -> &ASYNCUSER10 {
        &self.asyncuser10
    }
    #[doc = "0x22 - Synchronous User Ch 0 Input Selection - TCA0"]
    #[inline(always)]
    pub const fn syncuser0(&self) -> &SYNCUSER0 {
        &self.syncuser0
    }
}
#[doc = "ASYNCCH0 (rw) register accessor: Asynchronous Channel 0 Generator Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyncch0`]
module"]
pub type ASYNCCH0 = crate::Reg<asyncch0::ASYNCCH0_SPEC>;
#[doc = "Asynchronous Channel 0 Generator Selection"]
pub mod asyncch0;
#[doc = "ASYNCCH1 (rw) register accessor: Asynchronous Channel 1 Generator Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyncch1`]
module"]
pub type ASYNCCH1 = crate::Reg<asyncch1::ASYNCCH1_SPEC>;
#[doc = "Asynchronous Channel 1 Generator Selection"]
pub mod asyncch1;
#[doc = "ASYNCSTROBE (w) register accessor: Asynchronous Channel Strobe\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncstrobe::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyncstrobe`]
module"]
pub type ASYNCSTROBE = crate::Reg<asyncstrobe::ASYNCSTROBE_SPEC>;
#[doc = "Asynchronous Channel Strobe"]
pub mod asyncstrobe;
#[doc = "ASYNCUSER0 (rw) register accessor: Asynchronous User Ch 0 Input Selection - TCB0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncuser0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncuser0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyncuser0`]
module"]
pub type ASYNCUSER0 = crate::Reg<asyncuser0::ASYNCUSER0_SPEC>;
#[doc = "Asynchronous User Ch 0 Input Selection - TCB0"]
pub mod asyncuser0;
#[doc = "ASYNCUSER1 (rw) register accessor: Asynchronous User Ch 1 Input Selection - ADC0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncuser1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncuser1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyncuser1`]
module"]
pub type ASYNCUSER1 = crate::Reg<asyncuser1::ASYNCUSER1_SPEC>;
#[doc = "Asynchronous User Ch 1 Input Selection - ADC0"]
pub mod asyncuser1;
#[doc = "ASYNCUSER10 (rw) register accessor: Asynchronous User Ch 10 Input Selection - Event Out 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncuser10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncuser10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyncuser10`]
module"]
pub type ASYNCUSER10 = crate::Reg<asyncuser10::ASYNCUSER10_SPEC>;
#[doc = "Asynchronous User Ch 10 Input Selection - Event Out 2"]
pub mod asyncuser10;
#[doc = "ASYNCUSER2 (rw) register accessor: Asynchronous User Ch 2 Input Selection - CCL LUT0 Event 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncuser2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncuser2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyncuser2`]
module"]
pub type ASYNCUSER2 = crate::Reg<asyncuser2::ASYNCUSER2_SPEC>;
#[doc = "Asynchronous User Ch 2 Input Selection - CCL LUT0 Event 0"]
pub mod asyncuser2;
#[doc = "ASYNCUSER3 (rw) register accessor: Asynchronous User Ch 3 Input Selection - CCL LUT1 Event 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncuser3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncuser3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyncuser3`]
module"]
pub type ASYNCUSER3 = crate::Reg<asyncuser3::ASYNCUSER3_SPEC>;
#[doc = "Asynchronous User Ch 3 Input Selection - CCL LUT1 Event 0"]
pub mod asyncuser3;
#[doc = "ASYNCUSER4 (rw) register accessor: Asynchronous User Ch 4 Input Selection - CCL LUT0 Event 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncuser4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncuser4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyncuser4`]
module"]
pub type ASYNCUSER4 = crate::Reg<asyncuser4::ASYNCUSER4_SPEC>;
#[doc = "Asynchronous User Ch 4 Input Selection - CCL LUT0 Event 1"]
pub mod asyncuser4;
#[doc = "ASYNCUSER5 (rw) register accessor: Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncuser5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncuser5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyncuser5`]
module"]
pub type ASYNCUSER5 = crate::Reg<asyncuser5::ASYNCUSER5_SPEC>;
#[doc = "Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1"]
pub mod asyncuser5;
#[doc = "ASYNCUSER6 (rw) register accessor: Asynchronous User Ch 6 Input Selection - TCD0 Event 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncuser6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncuser6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyncuser6`]
module"]
pub type ASYNCUSER6 = crate::Reg<asyncuser6::ASYNCUSER6_SPEC>;
#[doc = "Asynchronous User Ch 6 Input Selection - TCD0 Event 0"]
pub mod asyncuser6;
#[doc = "ASYNCUSER7 (rw) register accessor: Asynchronous User Ch 7 Input Selection - TCD0 Event 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncuser7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncuser7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyncuser7`]
module"]
pub type ASYNCUSER7 = crate::Reg<asyncuser7::ASYNCUSER7_SPEC>;
#[doc = "Asynchronous User Ch 7 Input Selection - TCD0 Event 1"]
pub mod asyncuser7;
#[doc = "ASYNCUSER8 (rw) register accessor: Asynchronous User Ch 8 Input Selection - Event Out 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncuser8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncuser8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyncuser8`]
module"]
pub type ASYNCUSER8 = crate::Reg<asyncuser8::ASYNCUSER8_SPEC>;
#[doc = "Asynchronous User Ch 8 Input Selection - Event Out 0"]
pub mod asyncuser8;
#[doc = "ASYNCUSER9 (rw) register accessor: Asynchronous User Ch 9 Input Selection - Event Out 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncuser9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncuser9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asyncuser9`]
module"]
pub type ASYNCUSER9 = crate::Reg<asyncuser9::ASYNCUSER9_SPEC>;
#[doc = "Asynchronous User Ch 9 Input Selection - Event Out 1"]
pub mod asyncuser9;
#[doc = "SYNCCH0 (rw) register accessor: Synchronous Channel 0 Generator Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syncch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncch0`]
module"]
pub type SYNCCH0 = crate::Reg<syncch0::SYNCCH0_SPEC>;
#[doc = "Synchronous Channel 0 Generator Selection"]
pub mod syncch0;
#[doc = "SYNCCH1 (rw) register accessor: Synchronous Channel 1 Generator Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syncch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncch1`]
module"]
pub type SYNCCH1 = crate::Reg<syncch1::SYNCCH1_SPEC>;
#[doc = "Synchronous Channel 1 Generator Selection"]
pub mod syncch1;
#[doc = "SYNCSTROBE (w) register accessor: Synchronous Channel Strobe\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syncstrobe::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncstrobe`]
module"]
pub type SYNCSTROBE = crate::Reg<syncstrobe::SYNCSTROBE_SPEC>;
#[doc = "Synchronous Channel Strobe"]
pub mod syncstrobe;
#[doc = "SYNCUSER0 (rw) register accessor: Synchronous User Ch 0 Input Selection - TCA0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncuser0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syncuser0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncuser0`]
module"]
pub type SYNCUSER0 = crate::Reg<syncuser0::SYNCUSER0_SPEC>;
#[doc = "Synchronous User Ch 0 Input Selection - TCA0"]
pub mod syncuser0;
