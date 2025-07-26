#[doc = "Register `USBCON` reader"]
pub type R = crate::R<USBCON_SPEC>;
#[doc = "Register `USBCON` writer"]
pub type W = crate::W<USBCON_SPEC>;
#[doc = "Field `VBUSTE` reader - No Description."]
pub type VBUSTE_R = crate::BitReader;
#[doc = "Field `VBUSTE` writer - No Description."]
pub type VBUSTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGPADE` reader - No Description."]
pub type OTGPADE_R = crate::BitReader;
#[doc = "Field `OTGPADE` writer - No Description."]
pub type OTGPADE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRZCLK` reader - No Description."]
pub type FRZCLK_R = crate::BitReader;
#[doc = "Field `FRZCLK` writer - No Description."]
pub type FRZCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBE` reader - No Description."]
pub type USBE_R = crate::BitReader;
#[doc = "Field `USBE` writer - No Description."]
pub type USBE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn vbuste(&self) -> VBUSTE_R {
        VBUSTE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    pub fn otgpade(&self) -> OTGPADE_R {
        OTGPADE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    pub fn frzclk(&self) -> FRZCLK_R {
        FRZCLK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    pub fn usbe(&self) -> USBE_R {
        USBE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn vbuste(&mut self) -> VBUSTE_W<USBCON_SPEC> {
        VBUSTE_W::new(self, 0)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn otgpade(&mut self) -> OTGPADE_W<USBCON_SPEC> {
        OTGPADE_W::new(self, 4)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn frzclk(&mut self) -> FRZCLK_W<USBCON_SPEC> {
        FRZCLK_W::new(self, 5)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn usbe(&mut self) -> USBE_W<USBCON_SPEC> {
        USBE_W::new(self, 7)
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
#[doc = "USB General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBCON_SPEC;
impl crate::RegisterSpec for USBCON_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usbcon::R`](R) reader structure"]
impl crate::Readable for USBCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbcon::W`](W) writer structure"]
impl crate::Writable for USBCON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBCON to value 0"]
impl crate::Resettable for USBCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
