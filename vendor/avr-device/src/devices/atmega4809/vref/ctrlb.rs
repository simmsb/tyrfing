#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `AC0REFEN` reader - AC0 DACREF reference enable"]
pub type AC0REFEN_R = crate::BitReader;
#[doc = "Field `AC0REFEN` writer - AC0 DACREF reference enable"]
pub type AC0REFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0REFEN` reader - ADC0 reference enable"]
pub type ADC0REFEN_R = crate::BitReader;
#[doc = "Field `ADC0REFEN` writer - ADC0 reference enable"]
pub type ADC0REFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AC0 DACREF reference enable"]
    #[inline(always)]
    pub fn ac0refen(&self) -> AC0REFEN_R {
        AC0REFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC0 reference enable"]
    #[inline(always)]
    pub fn adc0refen(&self) -> ADC0REFEN_R {
        ADC0REFEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AC0 DACREF reference enable"]
    #[inline(always)]
    #[must_use]
    pub fn ac0refen(&mut self) -> AC0REFEN_W<CTRLB_SPEC> {
        AC0REFEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC0 reference enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0refen(&mut self) -> ADC0REFEN_W<CTRLB_SPEC> {
        ADC0REFEN_W::new(self, 1)
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
#[doc = "Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
