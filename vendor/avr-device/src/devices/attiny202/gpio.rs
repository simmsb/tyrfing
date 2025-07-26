#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    gpio: [GPIO; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - General Purpose IO Registers"]
    #[inline(always)]
    pub const fn gpio(&self, n: usize) -> &GPIO {
        &self.gpio[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00 - General Purpose IO Registers"]
    #[inline(always)]
    pub fn gpio_iter(&self) -> impl Iterator<Item = &GPIO> {
        self.gpio.iter()
    }
    #[doc = "0x00 - General Purpose IO Registers"]
    #[inline(always)]
    pub const fn gpior0(&self) -> &GPIO {
        self.gpio(0)
    }
    #[doc = "0x01 - General Purpose IO Registers"]
    #[inline(always)]
    pub const fn gpior1(&self) -> &GPIO {
        self.gpio(1)
    }
    #[doc = "0x02 - General Purpose IO Registers"]
    #[inline(always)]
    pub const fn gpior2(&self) -> &GPIO {
        self.gpio(2)
    }
    #[doc = "0x03 - General Purpose IO Registers"]
    #[inline(always)]
    pub const fn gpior3(&self) -> &GPIO {
        self.gpio(3)
    }
}
#[doc = "GPIO (rw) register accessor: General Purpose IO Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio`]
module"]
pub type GPIO = crate::Reg<gpio::GPIO_SPEC>;
#[doc = "General Purpose IO Registers"]
pub mod gpio;
