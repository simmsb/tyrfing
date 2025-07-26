#[doc = "Register `DIDR0` reader"]
pub type R = crate::R<DIDR0_SPEC>;
#[doc = "Register `DIDR0` writer"]
pub type W = crate::W<DIDR0_SPEC>;
#[doc = "Field `AIN0D` reader - AIN0 Digital Input Disable"]
pub type AIN0D_R = crate::BitReader;
#[doc = "Field `AIN0D` writer - AIN0 Digital Input Disable"]
pub type AIN0D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIN1D` reader - AIN1 Digital Input Disable"]
pub type AIN1D_R = crate::BitReader;
#[doc = "Field `AIN1D` writer - AIN1 Digital Input Disable"]
pub type AIN1D_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AIN0 Digital Input Disable"]
    #[inline(always)]
    pub fn ain0d(&self) -> AIN0D_R {
        AIN0D_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AIN1 Digital Input Disable"]
    #[inline(always)]
    pub fn ain1d(&self) -> AIN1D_R {
        AIN1D_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AIN0 Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ain0d(&mut self) -> AIN0D_W<DIDR0_SPEC> {
        AIN0D_W::new(self, 0)
    }
    #[doc = "Bit 1 - AIN1 Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ain1d(&mut self) -> AIN1D_W<DIDR0_SPEC> {
        AIN1D_W::new(self, 1)
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
