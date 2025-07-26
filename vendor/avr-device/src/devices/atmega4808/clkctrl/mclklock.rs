#[doc = "Register `MCLKLOCK` reader"]
pub type R = crate::R<MCLKLOCK_SPEC>;
#[doc = "Register `MCLKLOCK` writer"]
pub type W = crate::W<MCLKLOCK_SPEC>;
#[doc = "Field `LOCKEN` reader - Lock enable"]
pub type LOCKEN_R = crate::BitReader;
#[doc = "Field `LOCKEN` writer - Lock enable"]
pub type LOCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock enable"]
    #[inline(always)]
    pub fn locken(&self) -> LOCKEN_R {
        LOCKEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock enable"]
    #[inline(always)]
    #[must_use]
    pub fn locken(&mut self) -> LOCKEN_W<MCLKLOCK_SPEC> {
        LOCKEN_W::new(self, 0)
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
#[doc = "MCLK Lock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclklock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclklock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCLKLOCK_SPEC;
impl crate::RegisterSpec for MCLKLOCK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mclklock::R`](R) reader structure"]
impl crate::Readable for MCLKLOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mclklock::W`](W) writer structure"]
impl crate::Writable for MCLKLOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCLKLOCK to value 0"]
impl crate::Resettable for MCLKLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
