#[doc = "Register `MCLKSTATUS` reader"]
pub type R = crate::R<MCLKSTATUS_SPEC>;
#[doc = "Field `SOSC` reader - System Oscillator changing"]
pub type SOSC_R = crate::BitReader;
#[doc = "Field `OSC20MS` reader - 20MHz oscillator status"]
pub type OSC20MS_R = crate::BitReader;
#[doc = "Field `OSC32KS` reader - 32KHz oscillator status"]
pub type OSC32KS_R = crate::BitReader;
#[doc = "Field `XOSC32KS` reader - 32.768 kHz Crystal Oscillator status"]
pub type XOSC32KS_R = crate::BitReader;
#[doc = "Field `EXTS` reader - External Clock status"]
pub type EXTS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - System Oscillator changing"]
    #[inline(always)]
    pub fn sosc(&self) -> SOSC_R {
        SOSC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 20MHz oscillator status"]
    #[inline(always)]
    pub fn osc20ms(&self) -> OSC20MS_R {
        OSC20MS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 32KHz oscillator status"]
    #[inline(always)]
    pub fn osc32ks(&self) -> OSC32KS_R {
        OSC32KS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 32.768 kHz Crystal Oscillator status"]
    #[inline(always)]
    pub fn xosc32ks(&self) -> XOSC32KS_R {
        XOSC32KS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Clock status"]
    #[inline(always)]
    pub fn exts(&self) -> EXTS_R {
        EXTS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "MCLK Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclkstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCLKSTATUS_SPEC;
impl crate::RegisterSpec for MCLKSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mclkstatus::R`](R) reader structure"]
impl crate::Readable for MCLKSTATUS_SPEC {}
#[doc = "`reset()` method sets MCLKSTATUS to value 0"]
impl crate::Resettable for MCLKSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
