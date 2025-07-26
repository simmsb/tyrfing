#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    deviceid0: DEVICEID0,
    deviceid1: DEVICEID1,
    deviceid2: DEVICEID2,
    _reserved3: [u8; 0x01],
    tempsense0: TEMPSENSE0,
    tempsense1: TEMPSENSE1,
    _reserved5: [u8; 0x08],
    sernum0: SERNUM0,
    sernum1: SERNUM1,
    sernum2: SERNUM2,
    sernum3: SERNUM3,
    sernum4: SERNUM4,
    sernum5: SERNUM5,
    sernum6: SERNUM6,
    sernum7: SERNUM7,
    sernum8: SERNUM8,
    sernum9: SERNUM9,
    sernum10: SERNUM10,
    sernum11: SERNUM11,
    sernum12: SERNUM12,
    sernum13: SERNUM13,
    sernum14: SERNUM14,
    sernum15: SERNUM15,
}
impl RegisterBlock {
    #[doc = "0x00 - Device ID Byte 0"]
    #[inline(always)]
    pub const fn deviceid0(&self) -> &DEVICEID0 {
        &self.deviceid0
    }
    #[doc = "0x01 - Device ID Byte 1"]
    #[inline(always)]
    pub const fn deviceid1(&self) -> &DEVICEID1 {
        &self.deviceid1
    }
    #[doc = "0x02 - Device ID Byte 2"]
    #[inline(always)]
    pub const fn deviceid2(&self) -> &DEVICEID2 {
        &self.deviceid2
    }
    #[doc = "0x04 - Temperature Calibration 0"]
    #[inline(always)]
    pub const fn tempsense0(&self) -> &TEMPSENSE0 {
        &self.tempsense0
    }
    #[doc = "0x06 - Temperature Calibration 1"]
    #[inline(always)]
    pub const fn tempsense1(&self) -> &TEMPSENSE1 {
        &self.tempsense1
    }
    #[doc = "0x10 - LOTNUM0"]
    #[inline(always)]
    pub const fn sernum0(&self) -> &SERNUM0 {
        &self.sernum0
    }
    #[doc = "0x11 - LOTNUM1"]
    #[inline(always)]
    pub const fn sernum1(&self) -> &SERNUM1 {
        &self.sernum1
    }
    #[doc = "0x12 - LOTNUM2"]
    #[inline(always)]
    pub const fn sernum2(&self) -> &SERNUM2 {
        &self.sernum2
    }
    #[doc = "0x13 - LOTNUM3"]
    #[inline(always)]
    pub const fn sernum3(&self) -> &SERNUM3 {
        &self.sernum3
    }
    #[doc = "0x14 - LOTNUM4"]
    #[inline(always)]
    pub const fn sernum4(&self) -> &SERNUM4 {
        &self.sernum4
    }
    #[doc = "0x15 - LOTNUM5"]
    #[inline(always)]
    pub const fn sernum5(&self) -> &SERNUM5 {
        &self.sernum5
    }
    #[doc = "0x16 - RANDOM"]
    #[inline(always)]
    pub const fn sernum6(&self) -> &SERNUM6 {
        &self.sernum6
    }
    #[doc = "0x17 - SCRIBE"]
    #[inline(always)]
    pub const fn sernum7(&self) -> &SERNUM7 {
        &self.sernum7
    }
    #[doc = "0x18 - XPOS0"]
    #[inline(always)]
    pub const fn sernum8(&self) -> &SERNUM8 {
        &self.sernum8
    }
    #[doc = "0x19 - XPOS1"]
    #[inline(always)]
    pub const fn sernum9(&self) -> &SERNUM9 {
        &self.sernum9
    }
    #[doc = "0x1a - YPOS0"]
    #[inline(always)]
    pub const fn sernum10(&self) -> &SERNUM10 {
        &self.sernum10
    }
    #[doc = "0x1b - YPOS1"]
    #[inline(always)]
    pub const fn sernum11(&self) -> &SERNUM11 {
        &self.sernum11
    }
    #[doc = "0x1c - RES0"]
    #[inline(always)]
    pub const fn sernum12(&self) -> &SERNUM12 {
        &self.sernum12
    }
    #[doc = "0x1d - RES1"]
    #[inline(always)]
    pub const fn sernum13(&self) -> &SERNUM13 {
        &self.sernum13
    }
    #[doc = "0x1e - RES2"]
    #[inline(always)]
    pub const fn sernum14(&self) -> &SERNUM14 {
        &self.sernum14
    }
    #[doc = "0x1f - RES3"]
    #[inline(always)]
    pub const fn sernum15(&self) -> &SERNUM15 {
        &self.sernum15
    }
}
#[doc = "DEVICEID0 (r) register accessor: Device ID Byte 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deviceid0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceid0`]
module"]
pub type DEVICEID0 = crate::Reg<deviceid0::DEVICEID0_SPEC>;
#[doc = "Device ID Byte 0"]
pub mod deviceid0;
#[doc = "DEVICEID1 (r) register accessor: Device ID Byte 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deviceid1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceid1`]
module"]
pub type DEVICEID1 = crate::Reg<deviceid1::DEVICEID1_SPEC>;
#[doc = "Device ID Byte 1"]
pub mod deviceid1;
#[doc = "DEVICEID2 (r) register accessor: Device ID Byte 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deviceid2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceid2`]
module"]
pub type DEVICEID2 = crate::Reg<deviceid2::DEVICEID2_SPEC>;
#[doc = "Device ID Byte 2"]
pub mod deviceid2;
#[doc = "SERNUM0 (r) register accessor: LOTNUM0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum0`]
module"]
pub type SERNUM0 = crate::Reg<sernum0::SERNUM0_SPEC>;
#[doc = "LOTNUM0"]
pub mod sernum0;
#[doc = "SERNUM1 (r) register accessor: LOTNUM1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum1`]
module"]
pub type SERNUM1 = crate::Reg<sernum1::SERNUM1_SPEC>;
#[doc = "LOTNUM1"]
pub mod sernum1;
#[doc = "SERNUM10 (r) register accessor: YPOS0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum10`]
module"]
pub type SERNUM10 = crate::Reg<sernum10::SERNUM10_SPEC>;
#[doc = "YPOS0"]
pub mod sernum10;
#[doc = "SERNUM11 (r) register accessor: YPOS1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum11`]
module"]
pub type SERNUM11 = crate::Reg<sernum11::SERNUM11_SPEC>;
#[doc = "YPOS1"]
pub mod sernum11;
#[doc = "SERNUM12 (r) register accessor: RES0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum12`]
module"]
pub type SERNUM12 = crate::Reg<sernum12::SERNUM12_SPEC>;
#[doc = "RES0"]
pub mod sernum12;
#[doc = "SERNUM13 (r) register accessor: RES1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum13`]
module"]
pub type SERNUM13 = crate::Reg<sernum13::SERNUM13_SPEC>;
#[doc = "RES1"]
pub mod sernum13;
#[doc = "SERNUM14 (r) register accessor: RES2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum14::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum14`]
module"]
pub type SERNUM14 = crate::Reg<sernum14::SERNUM14_SPEC>;
#[doc = "RES2"]
pub mod sernum14;
#[doc = "SERNUM15 (r) register accessor: RES3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum15::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum15`]
module"]
pub type SERNUM15 = crate::Reg<sernum15::SERNUM15_SPEC>;
#[doc = "RES3"]
pub mod sernum15;
#[doc = "SERNUM2 (r) register accessor: LOTNUM2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum2`]
module"]
pub type SERNUM2 = crate::Reg<sernum2::SERNUM2_SPEC>;
#[doc = "LOTNUM2"]
pub mod sernum2;
#[doc = "SERNUM3 (r) register accessor: LOTNUM3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum3`]
module"]
pub type SERNUM3 = crate::Reg<sernum3::SERNUM3_SPEC>;
#[doc = "LOTNUM3"]
pub mod sernum3;
#[doc = "SERNUM4 (r) register accessor: LOTNUM4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum4`]
module"]
pub type SERNUM4 = crate::Reg<sernum4::SERNUM4_SPEC>;
#[doc = "LOTNUM4"]
pub mod sernum4;
#[doc = "SERNUM5 (r) register accessor: LOTNUM5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum5`]
module"]
pub type SERNUM5 = crate::Reg<sernum5::SERNUM5_SPEC>;
#[doc = "LOTNUM5"]
pub mod sernum5;
#[doc = "SERNUM6 (r) register accessor: RANDOM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum6`]
module"]
pub type SERNUM6 = crate::Reg<sernum6::SERNUM6_SPEC>;
#[doc = "RANDOM"]
pub mod sernum6;
#[doc = "SERNUM7 (r) register accessor: SCRIBE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum7`]
module"]
pub type SERNUM7 = crate::Reg<sernum7::SERNUM7_SPEC>;
#[doc = "SCRIBE"]
pub mod sernum7;
#[doc = "SERNUM8 (r) register accessor: XPOS0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum8`]
module"]
pub type SERNUM8 = crate::Reg<sernum8::SERNUM8_SPEC>;
#[doc = "XPOS0"]
pub mod sernum8;
#[doc = "SERNUM9 (r) register accessor: XPOS1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum9`]
module"]
pub type SERNUM9 = crate::Reg<sernum9::SERNUM9_SPEC>;
#[doc = "XPOS1"]
pub mod sernum9;
#[doc = "TEMPSENSE0 (r) register accessor: Temperature Calibration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tempsense0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tempsense0`]
module"]
pub type TEMPSENSE0 = crate::Reg<tempsense0::TEMPSENSE0_SPEC>;
#[doc = "Temperature Calibration 0"]
pub mod tempsense0;
#[doc = "TEMPSENSE1 (r) register accessor: Temperature Calibration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tempsense1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tempsense1`]
module"]
pub type TEMPSENSE1 = crate::Reg<tempsense1::TEMPSENSE1_SPEC>;
#[doc = "Temperature Calibration 1"]
pub mod tempsense1;
