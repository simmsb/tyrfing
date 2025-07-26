#[doc = "Register `GTCCR` reader"]
pub type R = crate::R<GTCCR_SPEC>;
#[doc = "Register `GTCCR` writer"]
pub type W = crate::W<GTCCR_SPEC>;
#[doc = "Field `PSR` reader - Prescaler Reset Timer/CounterN"]
pub type PSR_R = crate::BitReader;
#[doc = "Field `PSR` writer - Prescaler Reset Timer/CounterN"]
pub type PSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSM` reader - Timer/Counter Synchronization Mode"]
pub type TSM_R = crate::BitReader;
#[doc = "Field `TSM` writer - Timer/Counter Synchronization Mode"]
pub type TSM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Prescaler Reset Timer/CounterN"]
    #[inline(always)]
    pub fn psr(&self) -> PSR_R {
        PSR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Timer/Counter Synchronization Mode"]
    #[inline(always)]
    pub fn tsm(&self) -> TSM_R {
        TSM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Prescaler Reset Timer/CounterN"]
    #[inline(always)]
    #[must_use]
    pub fn psr(&mut self) -> PSR_W<GTCCR_SPEC> {
        PSR_W::new(self, 0)
    }
    #[doc = "Bit 7 - Timer/Counter Synchronization Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tsm(&mut self) -> TSM_W<GTCCR_SPEC> {
        TSM_W::new(self, 7)
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
#[doc = "General Timer/Counter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTCCR_SPEC;
impl crate::RegisterSpec for GTCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gtccr::R`](R) reader structure"]
impl crate::Readable for GTCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtccr::W`](W) writer structure"]
impl crate::Writable for GTCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCCR to value 0"]
impl crate::Resettable for GTCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
