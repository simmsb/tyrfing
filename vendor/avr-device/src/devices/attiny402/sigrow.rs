#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    deviceid: [DEVICEID; 3],
    sernum: [SERNUM; 10],
    _reserved2: [u8; 0x13],
    tempsense0: TEMPSENSE0,
    tempsense1: TEMPSENSE1,
    osc16err3v: OSC16ERR3V,
    osc16err5v: OSC16ERR5V,
    osc20err3v: OSC20ERR3V,
    osc20err5v: OSC20ERR5V,
}
impl RegisterBlock {
    #[doc = "0x00 - Device IO Bytes"]
    #[inline(always)]
    pub const fn deviceid(&self, n: usize) -> &DEVICEID {
        &self.deviceid[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00 - Device IO Bytes"]
    #[inline(always)]
    pub fn deviceid_iter(&self) -> impl Iterator<Item = &DEVICEID> {
        self.deviceid.iter()
    }
    #[doc = "0x03..0x0d - Serial Number Bytes"]
    #[inline(always)]
    pub const fn sernum(&self, n: usize) -> &SERNUM {
        &self.sernum[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x03..0x0d - Serial Number Bytes"]
    #[inline(always)]
    pub fn sernum_iter(&self) -> impl Iterator<Item = &SERNUM> {
        self.sernum.iter()
    }
    #[doc = "0x20 - Temperature Sensor Calibration: Gain/Slope"]
    #[inline(always)]
    pub const fn tempsense0(&self) -> &TEMPSENSE0 {
        &self.tempsense0
    }
    #[doc = "0x21 - Temperature Sensor Calibration: Offset"]
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
}
#[doc = "DEVICEID (r) register accessor: Device IO Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deviceid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceid`]
module"]
pub type DEVICEID = crate::Reg<deviceid::DEVICEID_SPEC>;
#[doc = "Device IO Bytes"]
pub mod deviceid;
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
#[doc = "SERNUM (r) register accessor: Serial Number Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sernum`]
module"]
pub type SERNUM = crate::Reg<sernum::SERNUM_SPEC>;
#[doc = "Serial Number Bytes"]
pub mod sernum;
#[doc = "TEMPSENSE0 (r) register accessor: Temperature Sensor Calibration: Gain/Slope\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tempsense0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tempsense0`]
module"]
pub type TEMPSENSE0 = crate::Reg<tempsense0::TEMPSENSE0_SPEC>;
#[doc = "Temperature Sensor Calibration: Gain/Slope"]
pub mod tempsense0;
#[doc = "TEMPSENSE1 (r) register accessor: Temperature Sensor Calibration: Offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tempsense1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tempsense1`]
module"]
pub type TEMPSENSE1 = crate::Reg<tempsense1::TEMPSENSE1_SPEC>;
#[doc = "Temperature Sensor Calibration: Offset"]
pub mod tempsense1;
