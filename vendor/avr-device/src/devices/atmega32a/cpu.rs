#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    sfior: SFIOR,
    osccal: OSCCAL,
    _reserved2: [u8; 0x02],
    mcucsr: MCUCSR,
    mcucr: MCUCR,
}
impl RegisterBlock {
    #[doc = "0x00 - Special Function IO Register"]
    #[inline(always)]
    pub const fn sfior(&self) -> &SFIOR {
        &self.sfior
    }
    #[doc = "0x01 - Oscillator Calibration Value"]
    #[inline(always)]
    pub const fn osccal(&self) -> &OSCCAL {
        &self.osccal
    }
    #[doc = "0x04 - MCU Control And Status Register"]
    #[inline(always)]
    pub const fn mcucsr(&self) -> &MCUCSR {
        &self.mcucsr
    }
    #[doc = "0x05 - MCU Control Register"]
    #[inline(always)]
    pub const fn mcucr(&self) -> &MCUCR {
        &self.mcucr
    }
}
#[doc = "MCUCR (rw) register accessor: MCU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcucr`]
module"]
pub type MCUCR = crate::Reg<mcucr::MCUCR_SPEC>;
#[doc = "MCU Control Register"]
pub mod mcucr;
#[doc = "MCUCSR (rw) register accessor: MCU Control And Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcucsr`]
module"]
pub type MCUCSR = crate::Reg<mcucsr::MCUCSR_SPEC>;
#[doc = "MCU Control And Status Register"]
pub mod mcucsr;
#[doc = "OSCCAL (rw) register accessor: Oscillator Calibration Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osccal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osccal`]
module"]
pub type OSCCAL = crate::Reg<osccal::OSCCAL_SPEC>;
#[doc = "Oscillator Calibration Value"]
pub mod osccal;
#[doc = "SFIOR (rw) register accessor: Special Function IO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfior::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfior::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfior`]
module"]
pub type SFIOR = crate::Reg<sfior::SFIOR_SPEC>;
#[doc = "Special Function IO Register"]
pub mod sfior;
