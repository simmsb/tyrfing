#[doc = "Register `DIDR1` reader"]
pub type R = crate::R<DIDR1_SPEC>;
#[doc = "Register `DIDR1` writer"]
pub type W = crate::W<DIDR1_SPEC>;
#[doc = "Field `AIN0D` reader - AIN0 Digital Input Disable"]
pub type AIN0D_R = crate::BitReader;
#[doc = "Field `AIN0D` writer - AIN0 Digital Input Disable"]
pub type AIN0D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIN1D` reader - AIN1 Digital Input Disable"]
pub type AIN1D_R = crate::BitReader;
#[doc = "Field `AIN1D` writer - AIN1 Digital Input Disable"]
pub type AIN1D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIN2D` reader - AIN2 Digital Input Disable"]
pub type AIN2D_R = crate::BitReader;
#[doc = "Field `AIN2D` writer - AIN2 Digital Input Disable"]
pub type AIN2D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIN3D` reader - AIN3 Digital Input Disable"]
pub type AIN3D_R = crate::BitReader;
#[doc = "Field `AIN3D` writer - AIN3 Digital Input Disable"]
pub type AIN3D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIN4D` reader - AIN4 Digital Input Disable"]
pub type AIN4D_R = crate::BitReader;
#[doc = "Field `AIN4D` writer - AIN4 Digital Input Disable"]
pub type AIN4D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIN5D` reader - AIN5 Digital Input Disable"]
pub type AIN5D_R = crate::BitReader;
#[doc = "Field `AIN5D` writer - AIN5 Digital Input Disable"]
pub type AIN5D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIN6D` reader - AIN6 Digital Input Disable"]
pub type AIN6D_R = crate::BitReader;
#[doc = "Field `AIN6D` writer - AIN6 Digital Input Disable"]
pub type AIN6D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIN7D` reader - AIN7 Digital Input Disable"]
pub type AIN7D_R = crate::BitReader;
#[doc = "Field `AIN7D` writer - AIN7 Digital Input Disable"]
pub type AIN7D_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - AIN2 Digital Input Disable"]
    #[inline(always)]
    pub fn ain2d(&self) -> AIN2D_R {
        AIN2D_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AIN3 Digital Input Disable"]
    #[inline(always)]
    pub fn ain3d(&self) -> AIN3D_R {
        AIN3D_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AIN4 Digital Input Disable"]
    #[inline(always)]
    pub fn ain4d(&self) -> AIN4D_R {
        AIN4D_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AIN5 Digital Input Disable"]
    #[inline(always)]
    pub fn ain5d(&self) -> AIN5D_R {
        AIN5D_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AIN6 Digital Input Disable"]
    #[inline(always)]
    pub fn ain6d(&self) -> AIN6D_R {
        AIN6D_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AIN7 Digital Input Disable"]
    #[inline(always)]
    pub fn ain7d(&self) -> AIN7D_R {
        AIN7D_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AIN0 Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ain0d(&mut self) -> AIN0D_W<DIDR1_SPEC> {
        AIN0D_W::new(self, 0)
    }
    #[doc = "Bit 1 - AIN1 Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ain1d(&mut self) -> AIN1D_W<DIDR1_SPEC> {
        AIN1D_W::new(self, 1)
    }
    #[doc = "Bit 2 - AIN2 Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ain2d(&mut self) -> AIN2D_W<DIDR1_SPEC> {
        AIN2D_W::new(self, 2)
    }
    #[doc = "Bit 3 - AIN3 Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ain3d(&mut self) -> AIN3D_W<DIDR1_SPEC> {
        AIN3D_W::new(self, 3)
    }
    #[doc = "Bit 4 - AIN4 Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ain4d(&mut self) -> AIN4D_W<DIDR1_SPEC> {
        AIN4D_W::new(self, 4)
    }
    #[doc = "Bit 5 - AIN5 Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ain5d(&mut self) -> AIN5D_W<DIDR1_SPEC> {
        AIN5D_W::new(self, 5)
    }
    #[doc = "Bit 6 - AIN6 Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ain6d(&mut self) -> AIN6D_W<DIDR1_SPEC> {
        AIN6D_W::new(self, 6)
    }
    #[doc = "Bit 7 - AIN7 Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ain7d(&mut self) -> AIN7D_W<DIDR1_SPEC> {
        AIN7D_W::new(self, 7)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`didr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`didr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
