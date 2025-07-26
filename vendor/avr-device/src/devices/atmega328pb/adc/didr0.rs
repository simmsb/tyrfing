#[doc = "Register `DIDR0` reader"]
pub type R = crate::R<DIDR0_SPEC>;
#[doc = "Register `DIDR0` writer"]
pub type W = crate::W<DIDR0_SPEC>;
#[doc = "Field `ADC0D` reader - ADC Digital Input Disable"]
pub type ADC0D_R = crate::BitReader;
#[doc = "Field `ADC0D` writer - ADC Digital Input Disable"]
pub type ADC0D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1D` reader - ADC Digital Input Disable"]
pub type ADC1D_R = crate::BitReader;
#[doc = "Field `ADC1D` writer - ADC Digital Input Disable"]
pub type ADC1D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2D` reader - ADC Digital Input Disable"]
pub type ADC2D_R = crate::BitReader;
#[doc = "Field `ADC2D` writer - ADC Digital Input Disable"]
pub type ADC2D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3D` reader - ADC Digital Input Disable"]
pub type ADC3D_R = crate::BitReader;
#[doc = "Field `ADC3D` writer - ADC Digital Input Disable"]
pub type ADC3D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC4D` reader - ADC Digital Input Disable"]
pub type ADC4D_R = crate::BitReader;
#[doc = "Field `ADC4D` writer - ADC Digital Input Disable"]
pub type ADC4D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC5D` reader - ADC Digital Input Disable"]
pub type ADC5D_R = crate::BitReader;
#[doc = "Field `ADC5D` writer - ADC Digital Input Disable"]
pub type ADC5D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC6D` reader - ADC Digital Input Disable"]
pub type ADC6D_R = crate::BitReader;
#[doc = "Field `ADC6D` writer - ADC Digital Input Disable"]
pub type ADC6D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC7D` reader - ADC Digital Input Disable"]
pub type ADC7D_R = crate::BitReader;
#[doc = "Field `ADC7D` writer - ADC Digital Input Disable"]
pub type ADC7D_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC Digital Input Disable"]
    #[inline(always)]
    pub fn adc0d(&self) -> ADC0D_R {
        ADC0D_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Digital Input Disable"]
    #[inline(always)]
    pub fn adc1d(&self) -> ADC1D_R {
        ADC1D_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Digital Input Disable"]
    #[inline(always)]
    pub fn adc2d(&self) -> ADC2D_R {
        ADC2D_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Digital Input Disable"]
    #[inline(always)]
    pub fn adc3d(&self) -> ADC3D_R {
        ADC3D_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC Digital Input Disable"]
    #[inline(always)]
    pub fn adc4d(&self) -> ADC4D_R {
        ADC4D_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC Digital Input Disable"]
    #[inline(always)]
    pub fn adc5d(&self) -> ADC5D_R {
        ADC5D_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC Digital Input Disable"]
    #[inline(always)]
    pub fn adc6d(&self) -> ADC6D_R {
        ADC6D_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC Digital Input Disable"]
    #[inline(always)]
    pub fn adc7d(&self) -> ADC7D_R {
        ADC7D_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0d(&mut self) -> ADC0D_W<DIDR0_SPEC> {
        ADC0D_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1d(&mut self) -> ADC1D_W<DIDR0_SPEC> {
        ADC1D_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc2d(&mut self) -> ADC2D_W<DIDR0_SPEC> {
        ADC2D_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc3d(&mut self) -> ADC3D_W<DIDR0_SPEC> {
        ADC3D_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc4d(&mut self) -> ADC4D_W<DIDR0_SPEC> {
        ADC4D_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc5d(&mut self) -> ADC5D_W<DIDR0_SPEC> {
        ADC5D_W::new(self, 5)
    }
    #[doc = "Bit 6 - ADC Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc6d(&mut self) -> ADC6D_W<DIDR0_SPEC> {
        ADC6D_W::new(self, 6)
    }
    #[doc = "Bit 7 - ADC Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc7d(&mut self) -> ADC7D_W<DIDR0_SPEC> {
        ADC7D_W::new(self, 7)
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
#[doc = "Digital Input Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`didr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`didr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
