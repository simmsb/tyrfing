#[doc = "Register `DIDR3` reader"]
pub type R = crate::R<DIDR3_SPEC>;
#[doc = "Register `DIDR3` writer"]
pub type W = crate::W<DIDR3_SPEC>;
#[doc = "Field `ADC24D` reader - ADC24 Digital input Disable"]
pub type ADC24D_R = crate::BitReader;
#[doc = "Field `ADC24D` writer - ADC24 Digital input Disable"]
pub type ADC24D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC25D` reader - ADC25 Digital input Disable"]
pub type ADC25D_R = crate::BitReader;
#[doc = "Field `ADC25D` writer - ADC25 Digital input Disable"]
pub type ADC25D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC26D` reader - ADC26 Digital input Disable"]
pub type ADC26D_R = crate::BitReader;
#[doc = "Field `ADC26D` writer - ADC26 Digital input Disable"]
pub type ADC26D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC27D` reader - ADC27 Digital input Disable"]
pub type ADC27D_R = crate::BitReader;
#[doc = "Field `ADC27D` writer - ADC27 Digital input Disable"]
pub type ADC27D_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC24 Digital input Disable"]
    #[inline(always)]
    pub fn adc24d(&self) -> ADC24D_R {
        ADC24D_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC25 Digital input Disable"]
    #[inline(always)]
    pub fn adc25d(&self) -> ADC25D_R {
        ADC25D_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC26 Digital input Disable"]
    #[inline(always)]
    pub fn adc26d(&self) -> ADC26D_R {
        ADC26D_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC27 Digital input Disable"]
    #[inline(always)]
    pub fn adc27d(&self) -> ADC27D_R {
        ADC27D_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC24 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc24d(&mut self) -> ADC24D_W<DIDR3_SPEC> {
        ADC24D_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC25 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc25d(&mut self) -> ADC25D_W<DIDR3_SPEC> {
        ADC25D_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC26 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc26d(&mut self) -> ADC26D_W<DIDR3_SPEC> {
        ADC26D_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC27 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc27d(&mut self) -> ADC27D_W<DIDR3_SPEC> {
        ADC27D_W::new(self, 3)
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
#[doc = "Digital Input Disable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`didr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`didr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIDR3_SPEC;
impl crate::RegisterSpec for DIDR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`didr3::R`](R) reader structure"]
impl crate::Readable for DIDR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`didr3::W`](W) writer structure"]
impl crate::Writable for DIDR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIDR3 to value 0"]
impl crate::Resettable for DIDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
