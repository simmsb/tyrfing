#[doc = "Register `ADC` reader"]
pub type R = crate::R<ADC_SPEC>;
#[doc = "Register `ADC` writer"]
pub type W = crate::W<ADC_SPEC>;
#[doc = "Field `ADC` reader - ADC Data bits"]
pub type ADC_R = crate::FieldReader<u16>;
#[doc = "Field `ADC` writer - ADC Data bits"]
pub type ADC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - ADC Data bits"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(self.bits & 0x03ff)
    }
}
impl W {
    #[doc = "Bits 0:9 - ADC Data bits"]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<ADC_SPEC> {
        ADC_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC Data Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_SPEC;
impl crate::RegisterSpec for ADC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc::R`](R) reader structure"]
impl crate::Readable for ADC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc::W`](W) writer structure"]
impl crate::Writable for ADC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC to value 0"]
impl crate::Resettable for ADC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
