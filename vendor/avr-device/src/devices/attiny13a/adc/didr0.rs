#[doc = "Register `DIDR0` reader"]
pub type R = crate::R<DIDR0_SPEC>;
#[doc = "Register `DIDR0` writer"]
pub type W = crate::W<DIDR0_SPEC>;
#[doc = "Field `ADC1D` reader - ADC2 Digital input Disable"]
pub type ADC1D_R = crate::BitReader;
#[doc = "Field `ADC1D` writer - ADC2 Digital input Disable"]
pub type ADC1D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3D` reader - ADC3 Digital input Disable"]
pub type ADC3D_R = crate::BitReader;
#[doc = "Field `ADC3D` writer - ADC3 Digital input Disable"]
pub type ADC3D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2D` reader - ADC2 Digital input Disable"]
pub type ADC2D_R = crate::BitReader;
#[doc = "Field `ADC2D` writer - ADC2 Digital input Disable"]
pub type ADC2D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0D` reader - ADC0 Digital input Disable"]
pub type ADC0D_R = crate::BitReader;
#[doc = "Field `ADC0D` writer - ADC0 Digital input Disable"]
pub type ADC0D_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - ADC2 Digital input Disable"]
    #[inline(always)]
    pub fn adc1d(&self) -> ADC1D_R {
        ADC1D_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC3 Digital input Disable"]
    #[inline(always)]
    pub fn adc3d(&self) -> ADC3D_R {
        ADC3D_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC2 Digital input Disable"]
    #[inline(always)]
    pub fn adc2d(&self) -> ADC2D_R {
        ADC2D_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC0 Digital input Disable"]
    #[inline(always)]
    pub fn adc0d(&self) -> ADC0D_R {
        ADC0D_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - ADC2 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1d(&mut self) -> ADC1D_W<DIDR0_SPEC> {
        ADC1D_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC3 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc3d(&mut self) -> ADC3D_W<DIDR0_SPEC> {
        ADC3D_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC2 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc2d(&mut self) -> ADC2D_W<DIDR0_SPEC> {
        ADC2D_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC0 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0d(&mut self) -> ADC0D_W<DIDR0_SPEC> {
        ADC0D_W::new(self, 5)
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
#[doc = "Digital Input Disable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`didr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`didr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
