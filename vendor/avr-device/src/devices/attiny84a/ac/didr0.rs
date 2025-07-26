#[doc = "Register `DIDR0` reader"]
pub type R = crate::R<DIDR0_SPEC>;
#[doc = "Register `DIDR0` writer"]
pub type W = crate::W<DIDR0_SPEC>;
#[doc = "Field `ADC0D` reader - ADC 0 Digital input buffer disable"]
pub type ADC0D_R = crate::BitReader;
#[doc = "Field `ADC0D` writer - ADC 0 Digital input buffer disable"]
pub type ADC0D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1D` reader - ADC 1 Digital input buffer disable"]
pub type ADC1D_R = crate::BitReader;
#[doc = "Field `ADC1D` writer - ADC 1 Digital input buffer disable"]
pub type ADC1D_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC 0 Digital input buffer disable"]
    #[inline(always)]
    pub fn adc0d(&self) -> ADC0D_R {
        ADC0D_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC 1 Digital input buffer disable"]
    #[inline(always)]
    pub fn adc1d(&self) -> ADC1D_R {
        ADC1D_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC 0 Digital input buffer disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0d(&mut self) -> ADC0D_W<DIDR0_SPEC> {
        ADC0D_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC 1 Digital input buffer disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1d(&mut self) -> ADC1D_W<DIDR0_SPEC> {
        ADC1D_W::new(self, 1)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`didr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`didr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIDR0_SPEC;
impl crate::RegisterSpec for DIDR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`didr0::R`](R) reader structure"]
impl crate::Readable for DIDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`didr0::W`](W) writer structure"]
impl crate::Writable for DIDR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIDR0 to value 0"]
impl crate::Resettable for DIDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
