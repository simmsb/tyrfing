#[doc = "Register `SFIOR` reader"]
pub type R = crate::R<SFIOR_SPEC>;
#[doc = "Register `SFIOR` writer"]
pub type W = crate::W<SFIOR_SPEC>;
#[doc = "Field `PSR10` reader - Prescaler Reset Timer/Counter1 and Timer/Counter0"]
pub type PSR10_R = crate::BitReader;
#[doc = "Field `PSR10` writer - Prescaler Reset Timer/Counter1 and Timer/Counter0"]
pub type PSR10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUD` reader - Pull-up Disable"]
pub type PUD_R = crate::BitReader;
#[doc = "Field `PUD` writer - Pull-up Disable"]
pub type PUD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADHSM` reader - ADC High Speed Mode"]
pub type ADHSM_R = crate::BitReader;
#[doc = "Field `ADHSM` writer - ADC High Speed Mode"]
pub type ADHSM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Prescaler Reset Timer/Counter1 and Timer/Counter0"]
    #[inline(always)]
    pub fn psr10(&self) -> PSR10_R {
        PSR10_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Pull-up Disable"]
    #[inline(always)]
    pub fn pud(&self) -> PUD_R {
        PUD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC High Speed Mode"]
    #[inline(always)]
    pub fn adhsm(&self) -> ADHSM_R {
        ADHSM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Prescaler Reset Timer/Counter1 and Timer/Counter0"]
    #[inline(always)]
    #[must_use]
    pub fn psr10(&mut self) -> PSR10_W<SFIOR_SPEC> {
        PSR10_W::new(self, 0)
    }
    #[doc = "Bit 2 - Pull-up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pud(&mut self) -> PUD_W<SFIOR_SPEC> {
        PUD_W::new(self, 2)
    }
    #[doc = "Bit 4 - ADC High Speed Mode"]
    #[inline(always)]
    #[must_use]
    pub fn adhsm(&mut self) -> ADHSM_W<SFIOR_SPEC> {
        ADHSM_W::new(self, 4)
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
#[doc = "Special Function IO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfior::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfior::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFIOR_SPEC;
impl crate::RegisterSpec for SFIOR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sfior::R`](R) reader structure"]
impl crate::Readable for SFIOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sfior::W`](W) writer structure"]
impl crate::Writable for SFIOR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFIOR to value 0"]
impl crate::Resettable for SFIOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
