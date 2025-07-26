#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    deviceid0: DEVICEID0,
    deviceid1: DEVICEID1,
    deviceid2: DEVICEID2,
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
    _reserved13: [u8; 0x07],
    osccal32k: OSCCAL32K,
    _reserved14: [u8; 0x03],
    osccal16m0: OSCCAL16M0,
    osccal16m1: OSCCAL16M1,
    osccal20m0: OSCCAL20M0,
    osccal20m1: OSCCAL20M1,
    _reserved18: [u8; 0x04],
    tempsense0: TEMPSENSE0,
    tempsense1: TEMPSENSE1,
    osc16err3v: OSC16ERR3V,
    osc16err5v: OSC16ERR5V,
    osc20err3v: OSC20ERR3V,
    osc20err5v: OSC20ERR5V,
    _reserved24: [u8; 0x09],
    checksum1: CHECKSUM1,
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
    #[doc = "0x03 - Serial Number Byte 0"]
    #[inline(always)]
    pub const fn sernum0(&self) -> &SERNUM0 {
        &self.sernum0
    }
    #[doc = "0x04 - Serial Number Byte 1"]
    #[inline(always)]
    pub const fn sernum1(&self) -> &SERNUM1 {
        &self.sernum1
    }
    #[doc = "0x05 - Serial Number Byte 2"]
    #[inline(always)]
    pub const fn sernum2(&self) -> &SERNUM2 {
        &self.sernum2
    }
    #[doc = "0x06 - Serial Number Byte 3"]
    #[inline(always)]
    pub const fn sernum3(&self) -> &SERNUM3 {
        &self.sernum3
    }
    #[doc = "0x07 - Serial Number Byte 4"]
    #[inline(always)]
    pub const fn sernum4(&self) -> &SERNUM4 {
        &self.sernum4
    }
    #[doc = "0x08 - Serial Number Byte 5"]
    #[inline(always)]
    pub const fn sernum5(&self) -> &SERNUM5 {
        &self.sernum5
    }
    #[doc = "0x09 - Serial Number Byte 6"]
    #[inline(always)]
    pub const fn sernum6(&self) -> &SERNUM6 {
        &self.sernum6
    }
    #[doc = "0x0a - Serial Number Byte 7"]
    #[inline(always)]
    pub const fn sernum7(&self) -> &SERNUM7 {
        &self.sernum7
    }
    #[doc = "0x0b - Serial Number Byte 8"]
    #[inline(always)]
    pub const fn sernum8(&self) -> &SERNUM8 {
        &self.sernum8
    }
    #[doc = "0x0c - Serial Number Byte 9"]
    #[inline(always)]
    pub const fn sernum9(&self) -> &SERNUM9 {
        &self.sernum9
    }
    #[doc = "0x14 - Oscillator Calibration for 32kHz ULP"]
    #[inline(always)]
    pub const fn osccal32k(&self) -> &OSCCAL32K {
        &self.osccal32k
    }
    #[doc = "0x18 - Oscillator Calibration 16 MHz Byte 0"]
    #[inline(always)]
    pub const fn osccal16m0(&self) -> &OSCCAL16M0 {
        &self.osccal16m0
    }
    #[doc = "0x19 - Oscillator Calibration 16 MHz Byte 1"]
    #[inline(always)]
    pub const fn osccal16m1(&self) -> &OSCCAL16M1 {
        &self.osccal16m1
    }
    #[doc = "0x1a - Oscillator Calibration 20 MHz Byte 0"]
    #[inline(always)]
    pub const fn osccal20m0(&self) -> &OSCCAL20M0 {
        &self.osccal20m0
    }
    #[doc = "0x1b - Oscillator Calibration 20 MHz Byte 1"]
    #[inline(always)]
    pub const fn osccal20m1(&self) -> &OSCCAL20M1 {
        &self.osccal20m1
    }
    #[doc = "0x20 - Temperature Sensor Calibration Byte 0"]
    #[inline(always)]
    pub const fn tempsense0(&self) -> &TEMPSENSE0 {
        &self.tempsense0
    }
    #[doc = "0x21 - Temperature Sensor Calibration Byte 1"]
    #[inline(always)]
    pub const fn tempsense1(&self) -> &TEMPSENSE1 {
        &self.tempsense1
    }
    #[doc = "0x22 - OSC16 error at 3V"]
    #[inline(always)]
    pub const fn osc16err3v(&self) -> &OSC16ERR3V {
        &self.osc16err3v
    }
    #[doc = "0x23 - OSC16 error at 5V"]
    #[inline(always)]
    pub const fn osc16err5v(&self) -> &OSC16ERR5V {
        &self.osc16err5v
    }
    #[doc = "0x24 - OSC20 error at 3V"]
    #[inline(always)]
    pub const fn osc20err3v(&self) -> &OSC20ERR3V {
        &self.osc20err3v
    }
    #[doc = "0x25 - OSC20 error at 5V"]
    #[inline(always)]
    pub const fn osc20err5v(&self) -> &OSC20ERR5V {
        &self.osc20err5v
    }
    #[doc = "0x2f - CRC Checksum Byte 1"]
    #[inline(always)]
    pub const fn checksum1(&self) -> &CHECKSUM1 {
        &self.checksum1
    }
}
#[doc = "CHECKSUM1 (r) register accessor: CRC Checksum Byte 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`checksum1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@checksum1`]
module"]
pub type CHECKSUM1 = crate::Reg<checksum1::CHECKSUM1_SPEC>;
#[doc = "CRC Checksum Byte 1"]
pub mod checksum1;
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
#[doc = "OSC16ERR3V (r) register accessor: OSC16 error at 3V\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc16err3v::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc16err3v`]
module"]
pub type OSC16ERR3V = crate::Reg<osc16err3v::OSC16ERR3V_SPEC>;
#[doc = "OSC16 error at 3V"]
pub mod osc16err3v;
#[doc = "OSC16ERR5V (r) register accessor: OSC16 error at 5V\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc16err5v::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc16err5v`]
module"]
pub type OSC16ERR5V = crate::Reg<osc16err5v::OSC16ERR5V_SPEC>;
#[doc = "OSC16 error at 5V"]
pub mod osc16err5v;
#[doc = "OSC20ERR3V (r) register accessor: OSC20 error at 3V\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc20err3v::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc20err3v`]
module"]
pub type OSC20ERR3V = crate::Reg<osc20err3v::OSC20ERR3V_SPEC>;
#[doc = "OSC20 error at 3V"]
pub mod osc20err3v;
#[doc = "OSC20ERR5V (r) register accessor: OSC20 error at 5V\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc20err5v::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc20err5v`]
module"]
pub type OSC20ERR5V = crate::Reg<osc20err5v::OSC20ERR5V_SPEC>;
#[doc = "OSC20 error at 5V"]
pub mod osc20err5v;
#[doc = "OSCCAL16M0 (r) register accessor: Oscillator Calibration 16 MHz Byte 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccal16m0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osccal16m0`]
module"]
pub type OSCCAL16M0 = crate::Reg<osccal16m0::OSCCAL16M0_SPEC>;
#[doc = "Oscillator Calibration 16 MHz Byte 0"]
pub mod osccal16m0;
#[doc = "OSCCAL16M1 (r) register accessor: Oscillator Calibration 16 MHz Byte 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccal16m1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osccal16m1`]
module"]
pub type OSCCAL16M1 = crate::Reg<osccal16m1::OSCCAL16M1_SPEC>;
#[doc = "Oscillator Calibration 16 MHz Byte 1"]
pub mod osccal16m1;
#[doc = "OSCCAL20M0 (r) register accessor: Oscillator Calibration 20 MHz Byte 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccal20m0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osccal20m0`]
module"]
pub type OSCCAL20M0 = crate::Reg<osccal20m0::OSCCAL20M0_SPEC>;
#[doc = "Oscillator Calibration 20 MHz Byte 0"]
pub mod osccal20m0;
#[doc = "OSCCAL20M1 (r) register accessor: Oscillator Calibration 20 MHz Byte 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccal20m1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osccal20m1`]
module"]
pub type OSCCAL20M1 = crate::Reg<osccal20m1::OSCCAL20M1_SPEC>;
#[doc = "Oscillator Calibration 20 MHz Byte 1"]
pub mod osccal20m1;
#[doc = "OSCCAL32K (r) register accessor: Oscillator Calibration for 32kHz ULP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccal32k::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osccal32k`]
module"]
pub type OSCCAL32K = crate::Reg<osccal32k::OSCCAL32K_SPEC>;
#[doc = "Oscillator Calibration for 32kHz ULP"]
pub mod osccal32k;
#[doc = "SERNUM0 (r) register accessor: Serial Number Byte 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum0`]
module"]
pub type SERNUM0 = crate::Reg<sernum0::SERNUM0_SPEC>;
#[doc = "Serial Number Byte 0"]
pub mod sernum0;
#[doc = "SERNUM1 (r) register accessor: Serial Number Byte 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum1`]
module"]
pub type SERNUM1 = crate::Reg<sernum1::SERNUM1_SPEC>;
#[doc = "Serial Number Byte 1"]
pub mod sernum1;
#[doc = "SERNUM2 (r) register accessor: Serial Number Byte 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum2`]
module"]
pub type SERNUM2 = crate::Reg<sernum2::SERNUM2_SPEC>;
#[doc = "Serial Number Byte 2"]
pub mod sernum2;
#[doc = "SERNUM3 (r) register accessor: Serial Number Byte 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum3`]
module"]
pub type SERNUM3 = crate::Reg<sernum3::SERNUM3_SPEC>;
#[doc = "Serial Number Byte 3"]
pub mod sernum3;
#[doc = "SERNUM4 (r) register accessor: Serial Number Byte 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum4`]
module"]
pub type SERNUM4 = crate::Reg<sernum4::SERNUM4_SPEC>;
#[doc = "Serial Number Byte 4"]
pub mod sernum4;
#[doc = "SERNUM5 (r) register accessor: Serial Number Byte 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum5`]
module"]
pub type SERNUM5 = crate::Reg<sernum5::SERNUM5_SPEC>;
#[doc = "Serial Number Byte 5"]
pub mod sernum5;
#[doc = "SERNUM6 (r) register accessor: Serial Number Byte 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum6`]
module"]
pub type SERNUM6 = crate::Reg<sernum6::SERNUM6_SPEC>;
#[doc = "Serial Number Byte 6"]
pub mod sernum6;
#[doc = "SERNUM7 (r) register accessor: Serial Number Byte 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum7`]
module"]
pub type SERNUM7 = crate::Reg<sernum7::SERNUM7_SPEC>;
#[doc = "Serial Number Byte 7"]
pub mod sernum7;
#[doc = "SERNUM8 (r) register accessor: Serial Number Byte 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum8`]
module"]
pub type SERNUM8 = crate::Reg<sernum8::SERNUM8_SPEC>;
#[doc = "Serial Number Byte 8"]
pub mod sernum8;
#[doc = "SERNUM9 (r) register accessor: Serial Number Byte 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum9`]
module"]
pub type SERNUM9 = crate::Reg<sernum9::SERNUM9_SPEC>;
#[doc = "Serial Number Byte 9"]
pub mod sernum9;
#[doc = "TEMPSENSE0 (r) register accessor: Temperature Sensor Calibration Byte 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tempsense0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tempsense0`]
module"]
pub type TEMPSENSE0 = crate::Reg<tempsense0::TEMPSENSE0_SPEC>;
#[doc = "Temperature Sensor Calibration Byte 0"]
pub mod tempsense0;
#[doc = "TEMPSENSE1 (r) register accessor: Temperature Sensor Calibration Byte 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tempsense1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tempsense1`]
module"]
pub type TEMPSENSE1 = crate::Reg<tempsense1::TEMPSENSE1_SPEC>;
#[doc = "Temperature Sensor Calibration Byte 1"]
pub mod tempsense1;
