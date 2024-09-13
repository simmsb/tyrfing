#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    userrow0: USERROW0,
    userrow1: USERROW1,
    userrow2: USERROW2,
    userrow3: USERROW3,
    userrow4: USERROW4,
    userrow5: USERROW5,
    userrow6: USERROW6,
    userrow7: USERROW7,
    userrow8: USERROW8,
    userrow9: USERROW9,
    userrow10: USERROW10,
    userrow11: USERROW11,
    userrow12: USERROW12,
    userrow13: USERROW13,
    userrow14: USERROW14,
    userrow15: USERROW15,
    userrow16: USERROW16,
    userrow17: USERROW17,
    userrow18: USERROW18,
    userrow19: USERROW19,
    userrow20: USERROW20,
    userrow21: USERROW21,
    userrow22: USERROW22,
    userrow23: USERROW23,
    userrow24: USERROW24,
    userrow25: USERROW25,
    userrow26: USERROW26,
    userrow27: USERROW27,
    userrow28: USERROW28,
    userrow29: USERROW29,
    userrow30: USERROW30,
    userrow31: USERROW31,
}
impl RegisterBlock {
    #[doc = "0x00 - User Row Byte 0"]
    #[inline(always)]
    pub const fn userrow0(&self) -> &USERROW0 {
        &self.userrow0
    }
    #[doc = "0x01 - User Row Byte 1"]
    #[inline(always)]
    pub const fn userrow1(&self) -> &USERROW1 {
        &self.userrow1
    }
    #[doc = "0x02 - User Row Byte 2"]
    #[inline(always)]
    pub const fn userrow2(&self) -> &USERROW2 {
        &self.userrow2
    }
    #[doc = "0x03 - User Row Byte 3"]
    #[inline(always)]
    pub const fn userrow3(&self) -> &USERROW3 {
        &self.userrow3
    }
    #[doc = "0x04 - User Row Byte 4"]
    #[inline(always)]
    pub const fn userrow4(&self) -> &USERROW4 {
        &self.userrow4
    }
    #[doc = "0x05 - User Row Byte 5"]
    #[inline(always)]
    pub const fn userrow5(&self) -> &USERROW5 {
        &self.userrow5
    }
    #[doc = "0x06 - User Row Byte 6"]
    #[inline(always)]
    pub const fn userrow6(&self) -> &USERROW6 {
        &self.userrow6
    }
    #[doc = "0x07 - User Row Byte 7"]
    #[inline(always)]
    pub const fn userrow7(&self) -> &USERROW7 {
        &self.userrow7
    }
    #[doc = "0x08 - User Row Byte 8"]
    #[inline(always)]
    pub const fn userrow8(&self) -> &USERROW8 {
        &self.userrow8
    }
    #[doc = "0x09 - User Row Byte 9"]
    #[inline(always)]
    pub const fn userrow9(&self) -> &USERROW9 {
        &self.userrow9
    }
    #[doc = "0x0a - User Row Byte 10"]
    #[inline(always)]
    pub const fn userrow10(&self) -> &USERROW10 {
        &self.userrow10
    }
    #[doc = "0x0b - User Row Byte 11"]
    #[inline(always)]
    pub const fn userrow11(&self) -> &USERROW11 {
        &self.userrow11
    }
    #[doc = "0x0c - User Row Byte 12"]
    #[inline(always)]
    pub const fn userrow12(&self) -> &USERROW12 {
        &self.userrow12
    }
    #[doc = "0x0d - User Row Byte 13"]
    #[inline(always)]
    pub const fn userrow13(&self) -> &USERROW13 {
        &self.userrow13
    }
    #[doc = "0x0e - User Row Byte 14"]
    #[inline(always)]
    pub const fn userrow14(&self) -> &USERROW14 {
        &self.userrow14
    }
    #[doc = "0x0f - User Row Byte 15"]
    #[inline(always)]
    pub const fn userrow15(&self) -> &USERROW15 {
        &self.userrow15
    }
    #[doc = "0x10 - User Row Byte 16"]
    #[inline(always)]
    pub const fn userrow16(&self) -> &USERROW16 {
        &self.userrow16
    }
    #[doc = "0x11 - User Row Byte 17"]
    #[inline(always)]
    pub const fn userrow17(&self) -> &USERROW17 {
        &self.userrow17
    }
    #[doc = "0x12 - User Row Byte 18"]
    #[inline(always)]
    pub const fn userrow18(&self) -> &USERROW18 {
        &self.userrow18
    }
    #[doc = "0x13 - User Row Byte 19"]
    #[inline(always)]
    pub const fn userrow19(&self) -> &USERROW19 {
        &self.userrow19
    }
    #[doc = "0x14 - User Row Byte 20"]
    #[inline(always)]
    pub const fn userrow20(&self) -> &USERROW20 {
        &self.userrow20
    }
    #[doc = "0x15 - User Row Byte 21"]
    #[inline(always)]
    pub const fn userrow21(&self) -> &USERROW21 {
        &self.userrow21
    }
    #[doc = "0x16 - User Row Byte 22"]
    #[inline(always)]
    pub const fn userrow22(&self) -> &USERROW22 {
        &self.userrow22
    }
    #[doc = "0x17 - User Row Byte 23"]
    #[inline(always)]
    pub const fn userrow23(&self) -> &USERROW23 {
        &self.userrow23
    }
    #[doc = "0x18 - User Row Byte 24"]
    #[inline(always)]
    pub const fn userrow24(&self) -> &USERROW24 {
        &self.userrow24
    }
    #[doc = "0x19 - User Row Byte 25"]
    #[inline(always)]
    pub const fn userrow25(&self) -> &USERROW25 {
        &self.userrow25
    }
    #[doc = "0x1a - User Row Byte 26"]
    #[inline(always)]
    pub const fn userrow26(&self) -> &USERROW26 {
        &self.userrow26
    }
    #[doc = "0x1b - User Row Byte 27"]
    #[inline(always)]
    pub const fn userrow27(&self) -> &USERROW27 {
        &self.userrow27
    }
    #[doc = "0x1c - User Row Byte 28"]
    #[inline(always)]
    pub const fn userrow28(&self) -> &USERROW28 {
        &self.userrow28
    }
    #[doc = "0x1d - User Row Byte 29"]
    #[inline(always)]
    pub const fn userrow29(&self) -> &USERROW29 {
        &self.userrow29
    }
    #[doc = "0x1e - User Row Byte 30"]
    #[inline(always)]
    pub const fn userrow30(&self) -> &USERROW30 {
        &self.userrow30
    }
    #[doc = "0x1f - User Row Byte 31"]
    #[inline(always)]
    pub const fn userrow31(&self) -> &USERROW31 {
        &self.userrow31
    }
}
#[doc = "USERROW0 (rw) register accessor: User Row Byte 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow0`]
module"]
pub type USERROW0 = crate::Reg<userrow0::USERROW0_SPEC>;
#[doc = "User Row Byte 0"]
pub mod userrow0;
#[doc = "USERROW1 (rw) register accessor: User Row Byte 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow1`]
module"]
pub type USERROW1 = crate::Reg<userrow1::USERROW1_SPEC>;
#[doc = "User Row Byte 1"]
pub mod userrow1;
#[doc = "USERROW10 (rw) register accessor: User Row Byte 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow10`]
module"]
pub type USERROW10 = crate::Reg<userrow10::USERROW10_SPEC>;
#[doc = "User Row Byte 10"]
pub mod userrow10;
#[doc = "USERROW11 (rw) register accessor: User Row Byte 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow11`]
module"]
pub type USERROW11 = crate::Reg<userrow11::USERROW11_SPEC>;
#[doc = "User Row Byte 11"]
pub mod userrow11;
#[doc = "USERROW12 (rw) register accessor: User Row Byte 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow12`]
module"]
pub type USERROW12 = crate::Reg<userrow12::USERROW12_SPEC>;
#[doc = "User Row Byte 12"]
pub mod userrow12;
#[doc = "USERROW13 (rw) register accessor: User Row Byte 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow13`]
module"]
pub type USERROW13 = crate::Reg<userrow13::USERROW13_SPEC>;
#[doc = "User Row Byte 13"]
pub mod userrow13;
#[doc = "USERROW14 (rw) register accessor: User Row Byte 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow14`]
module"]
pub type USERROW14 = crate::Reg<userrow14::USERROW14_SPEC>;
#[doc = "User Row Byte 14"]
pub mod userrow14;
#[doc = "USERROW15 (rw) register accessor: User Row Byte 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow15`]
module"]
pub type USERROW15 = crate::Reg<userrow15::USERROW15_SPEC>;
#[doc = "User Row Byte 15"]
pub mod userrow15;
#[doc = "USERROW16 (rw) register accessor: User Row Byte 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow16`]
module"]
pub type USERROW16 = crate::Reg<userrow16::USERROW16_SPEC>;
#[doc = "User Row Byte 16"]
pub mod userrow16;
#[doc = "USERROW17 (rw) register accessor: User Row Byte 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow17`]
module"]
pub type USERROW17 = crate::Reg<userrow17::USERROW17_SPEC>;
#[doc = "User Row Byte 17"]
pub mod userrow17;
#[doc = "USERROW18 (rw) register accessor: User Row Byte 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow18`]
module"]
pub type USERROW18 = crate::Reg<userrow18::USERROW18_SPEC>;
#[doc = "User Row Byte 18"]
pub mod userrow18;
#[doc = "USERROW19 (rw) register accessor: User Row Byte 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow19`]
module"]
pub type USERROW19 = crate::Reg<userrow19::USERROW19_SPEC>;
#[doc = "User Row Byte 19"]
pub mod userrow19;
#[doc = "USERROW2 (rw) register accessor: User Row Byte 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow2`]
module"]
pub type USERROW2 = crate::Reg<userrow2::USERROW2_SPEC>;
#[doc = "User Row Byte 2"]
pub mod userrow2;
#[doc = "USERROW20 (rw) register accessor: User Row Byte 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow20`]
module"]
pub type USERROW20 = crate::Reg<userrow20::USERROW20_SPEC>;
#[doc = "User Row Byte 20"]
pub mod userrow20;
#[doc = "USERROW21 (rw) register accessor: User Row Byte 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow21`]
module"]
pub type USERROW21 = crate::Reg<userrow21::USERROW21_SPEC>;
#[doc = "User Row Byte 21"]
pub mod userrow21;
#[doc = "USERROW22 (rw) register accessor: User Row Byte 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow22`]
module"]
pub type USERROW22 = crate::Reg<userrow22::USERROW22_SPEC>;
#[doc = "User Row Byte 22"]
pub mod userrow22;
#[doc = "USERROW23 (rw) register accessor: User Row Byte 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow23`]
module"]
pub type USERROW23 = crate::Reg<userrow23::USERROW23_SPEC>;
#[doc = "User Row Byte 23"]
pub mod userrow23;
#[doc = "USERROW24 (rw) register accessor: User Row Byte 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow24`]
module"]
pub type USERROW24 = crate::Reg<userrow24::USERROW24_SPEC>;
#[doc = "User Row Byte 24"]
pub mod userrow24;
#[doc = "USERROW25 (rw) register accessor: User Row Byte 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow25`]
module"]
pub type USERROW25 = crate::Reg<userrow25::USERROW25_SPEC>;
#[doc = "User Row Byte 25"]
pub mod userrow25;
#[doc = "USERROW26 (rw) register accessor: User Row Byte 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow26`]
module"]
pub type USERROW26 = crate::Reg<userrow26::USERROW26_SPEC>;
#[doc = "User Row Byte 26"]
pub mod userrow26;
#[doc = "USERROW27 (rw) register accessor: User Row Byte 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow27`]
module"]
pub type USERROW27 = crate::Reg<userrow27::USERROW27_SPEC>;
#[doc = "User Row Byte 27"]
pub mod userrow27;
#[doc = "USERROW28 (rw) register accessor: User Row Byte 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow28`]
module"]
pub type USERROW28 = crate::Reg<userrow28::USERROW28_SPEC>;
#[doc = "User Row Byte 28"]
pub mod userrow28;
#[doc = "USERROW29 (rw) register accessor: User Row Byte 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow29`]
module"]
pub type USERROW29 = crate::Reg<userrow29::USERROW29_SPEC>;
#[doc = "User Row Byte 29"]
pub mod userrow29;
#[doc = "USERROW3 (rw) register accessor: User Row Byte 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow3`]
module"]
pub type USERROW3 = crate::Reg<userrow3::USERROW3_SPEC>;
#[doc = "User Row Byte 3"]
pub mod userrow3;
#[doc = "USERROW30 (rw) register accessor: User Row Byte 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow30`]
module"]
pub type USERROW30 = crate::Reg<userrow30::USERROW30_SPEC>;
#[doc = "User Row Byte 30"]
pub mod userrow30;
#[doc = "USERROW31 (rw) register accessor: User Row Byte 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow31`]
module"]
pub type USERROW31 = crate::Reg<userrow31::USERROW31_SPEC>;
#[doc = "User Row Byte 31"]
pub mod userrow31;
#[doc = "USERROW4 (rw) register accessor: User Row Byte 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow4`]
module"]
pub type USERROW4 = crate::Reg<userrow4::USERROW4_SPEC>;
#[doc = "User Row Byte 4"]
pub mod userrow4;
#[doc = "USERROW5 (rw) register accessor: User Row Byte 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow5`]
module"]
pub type USERROW5 = crate::Reg<userrow5::USERROW5_SPEC>;
#[doc = "User Row Byte 5"]
pub mod userrow5;
#[doc = "USERROW6 (rw) register accessor: User Row Byte 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow6`]
module"]
pub type USERROW6 = crate::Reg<userrow6::USERROW6_SPEC>;
#[doc = "User Row Byte 6"]
pub mod userrow6;
#[doc = "USERROW7 (rw) register accessor: User Row Byte 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow7`]
module"]
pub type USERROW7 = crate::Reg<userrow7::USERROW7_SPEC>;
#[doc = "User Row Byte 7"]
pub mod userrow7;
#[doc = "USERROW8 (rw) register accessor: User Row Byte 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow8`]
module"]
pub type USERROW8 = crate::Reg<userrow8::USERROW8_SPEC>;
#[doc = "User Row Byte 8"]
pub mod userrow8;
#[doc = "USERROW9 (rw) register accessor: User Row Byte 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userrow9`]
module"]
pub type USERROW9 = crate::Reg<userrow9::USERROW9_SPEC>;
#[doc = "User Row Byte 9"]
pub mod userrow9;
