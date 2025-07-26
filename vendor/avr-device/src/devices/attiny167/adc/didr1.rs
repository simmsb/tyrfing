#[doc = "Register `DIDR1` reader"]
pub type R = crate::R<DIDR1_SPEC>;
#[doc = "Register `DIDR1` writer"]
pub type W = crate::W<DIDR1_SPEC>;
#[doc = "Field `ADC8D` reader - No Description."]
pub type ADC8D_R = crate::BitReader;
#[doc = "Field `ADC8D` writer - No Description."]
pub type ADC8D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC9D` reader - No Description."]
pub type ADC9D_R = crate::BitReader;
#[doc = "Field `ADC9D` writer - No Description."]
pub type ADC9D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC10D` reader - No Description."]
pub type ADC10D_R = crate::BitReader;
#[doc = "Field `ADC10D` writer - No Description."]
pub type ADC10D_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    pub fn adc8d(&self) -> ADC8D_R {
        ADC8D_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    pub fn adc9d(&self) -> ADC9D_R {
        ADC9D_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    pub fn adc10d(&self) -> ADC10D_R {
        ADC10D_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn adc8d(&mut self) -> ADC8D_W<DIDR1_SPEC> {
        ADC8D_W::new(self, 4)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn adc9d(&mut self) -> ADC9D_W<DIDR1_SPEC> {
        ADC9D_W::new(self, 5)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn adc10d(&mut self) -> ADC10D_W<DIDR1_SPEC> {
        ADC10D_W::new(self, 6)
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
#[doc = "Digital Input Disable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`didr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`didr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIDR1_SPEC;
impl crate::RegisterSpec for DIDR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`didr1::R`](R) reader structure"]
impl crate::Readable for DIDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`didr1::W`](W) writer structure"]
impl crate::Writable for DIDR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIDR1 to value 0"]
impl crate::Resettable for DIDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
