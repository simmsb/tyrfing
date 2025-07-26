#[doc = "Register `WDTCKD` reader"]
pub type R = crate::R<WDTCKD_SPEC>;
#[doc = "Register `WDTCKD` writer"]
pub type W = crate::W<WDTCKD_SPEC>;
#[doc = "Field `WCLKD` reader - Watchdog Timer Clock Dividers"]
pub type WCLKD_R = crate::FieldReader;
#[doc = "Field `WCLKD` writer - Watchdog Timer Clock Dividers"]
pub type WCLKD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `WDEWIE` reader - Watchdog Early Warning Interrupt Enable"]
pub type WDEWIE_R = crate::BitReader;
#[doc = "Field `WDEWIE` writer - Watchdog Early Warning Interrupt Enable"]
pub type WDEWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDEWIF` reader - Watchdog Early Warning Interrupt Flag"]
pub type WDEWIF_R = crate::BitReader;
#[doc = "Field `WDEWIF` writer - Watchdog Early Warning Interrupt Flag"]
pub type WDEWIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Watchdog Timer Clock Dividers"]
    #[inline(always)]
    pub fn wclkd(&self) -> WCLKD_R {
        WCLKD_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Watchdog Early Warning Interrupt Enable"]
    #[inline(always)]
    pub fn wdewie(&self) -> WDEWIE_R {
        WDEWIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog Early Warning Interrupt Flag"]
    #[inline(always)]
    pub fn wdewif(&self) -> WDEWIF_R {
        WDEWIF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Watchdog Timer Clock Dividers"]
    #[inline(always)]
    #[must_use]
    pub fn wclkd(&mut self) -> WCLKD_W<WDTCKD_SPEC> {
        WCLKD_W::new(self, 0)
    }
    #[doc = "Bit 2 - Watchdog Early Warning Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdewie(&mut self) -> WDEWIE_W<WDTCKD_SPEC> {
        WDEWIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Watchdog Early Warning Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdewif(&mut self) -> WDEWIF_W<WDTCKD_SPEC> {
        WDEWIF_W::new(self, 3)
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
#[doc = "Watchdog Timer Clock Divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtckd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtckd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCKD_SPEC;
impl crate::RegisterSpec for WDTCKD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wdtckd::R`](R) reader structure"]
impl crate::Readable for WDTCKD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtckd::W`](W) writer structure"]
impl crate::Writable for WDTCKD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCKD to value 0"]
impl crate::Resettable for WDTCKD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
