#[doc = "Register `MCLKSTATUS` reader"]
pub type R = crate::R<MCLKSTATUS_SPEC>;
#[doc = "Register `MCLKSTATUS` writer"]
pub type W = crate::W<MCLKSTATUS_SPEC>;
#[doc = "Field `SOSC` reader - System Oscillator changing"]
pub type SOSC_R = crate::BitReader;
#[doc = "Field `SOSC` writer - System Oscillator changing"]
pub type SOSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCHFS` reader - High frequency oscillator status"]
pub type OSCHFS_R = crate::BitReader;
#[doc = "Field `OSCHFS` writer - High frequency oscillator status"]
pub type OSCHFS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSC32KS` reader - 32KHz oscillator status"]
pub type OSC32KS_R = crate::BitReader;
#[doc = "Field `OSC32KS` writer - 32KHz oscillator status"]
pub type OSC32KS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSC32KS` reader - 32.768 kHz Crystal Oscillator status"]
pub type XOSC32KS_R = crate::BitReader;
#[doc = "Field `XOSC32KS` writer - 32.768 kHz Crystal Oscillator status"]
pub type XOSC32KS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTS` reader - External Clock status"]
pub type EXTS_R = crate::BitReader;
#[doc = "Field `EXTS` writer - External Clock status"]
pub type EXTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLS` reader - PLL oscillator status"]
pub type PLLS_R = crate::BitReader;
#[doc = "Field `PLLS` writer - PLL oscillator status"]
pub type PLLS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - System Oscillator changing"]
    #[inline(always)]
    pub fn sosc(&self) -> SOSC_R {
        SOSC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High frequency oscillator status"]
    #[inline(always)]
    pub fn oschfs(&self) -> OSCHFS_R {
        OSCHFS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 32KHz oscillator status"]
    #[inline(always)]
    pub fn osc32ks(&self) -> OSC32KS_R {
        OSC32KS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 32.768 kHz Crystal Oscillator status"]
    #[inline(always)]
    pub fn xosc32ks(&self) -> XOSC32KS_R {
        XOSC32KS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Clock status"]
    #[inline(always)]
    pub fn exts(&self) -> EXTS_R {
        EXTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL oscillator status"]
    #[inline(always)]
    pub fn plls(&self) -> PLLS_R {
        PLLS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System Oscillator changing"]
    #[inline(always)]
    #[must_use]
    pub fn sosc(&mut self) -> SOSC_W<MCLKSTATUS_SPEC> {
        SOSC_W::new(self, 0)
    }
    #[doc = "Bit 1 - High frequency oscillator status"]
    #[inline(always)]
    #[must_use]
    pub fn oschfs(&mut self) -> OSCHFS_W<MCLKSTATUS_SPEC> {
        OSCHFS_W::new(self, 1)
    }
    #[doc = "Bit 2 - 32KHz oscillator status"]
    #[inline(always)]
    #[must_use]
    pub fn osc32ks(&mut self) -> OSC32KS_W<MCLKSTATUS_SPEC> {
        OSC32KS_W::new(self, 2)
    }
    #[doc = "Bit 3 - 32.768 kHz Crystal Oscillator status"]
    #[inline(always)]
    #[must_use]
    pub fn xosc32ks(&mut self) -> XOSC32KS_W<MCLKSTATUS_SPEC> {
        XOSC32KS_W::new(self, 3)
    }
    #[doc = "Bit 4 - External Clock status"]
    #[inline(always)]
    #[must_use]
    pub fn exts(&mut self) -> EXTS_W<MCLKSTATUS_SPEC> {
        EXTS_W::new(self, 4)
    }
    #[doc = "Bit 5 - PLL oscillator status"]
    #[inline(always)]
    #[must_use]
    pub fn plls(&mut self) -> PLLS_W<MCLKSTATUS_SPEC> {
        PLLS_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MCLK Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclkstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclkstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCLKSTATUS_SPEC;
impl crate::RegisterSpec for MCLKSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mclkstatus::R`](R) reader structure"]
impl crate::Readable for MCLKSTATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mclkstatus::W`](W) writer structure"]
impl crate::Writable for MCLKSTATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCLKSTATUS to value 0"]
impl crate::Resettable for MCLKSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
