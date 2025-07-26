#[doc = "Register `USBSTA` reader"]
pub type R = crate::R<USBSTA_SPEC>;
#[doc = "Register `USBSTA` writer"]
pub type W = crate::W<USBSTA_SPEC>;
#[doc = "Field `VBUS` reader - No Description."]
pub type VBUS_R = crate::BitReader;
#[doc = "Field `VBUS` writer - No Description."]
pub type VBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID` reader - No Description."]
pub type ID_R = crate::BitReader;
#[doc = "Field `ID` writer - No Description."]
pub type ID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPEED` reader - No Description."]
pub type SPEED_R = crate::BitReader;
#[doc = "Field `SPEED` writer - No Description."]
pub type SPEED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn vbus(&self) -> VBUS_R {
        VBUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn vbus(&mut self) -> VBUS_W<USBSTA_SPEC> {
        VBUS_W::new(self, 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<USBSTA_SPEC> {
        ID_W::new(self, 1)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<USBSTA_SPEC> {
        SPEED_W::new(self, 3)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbsta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbsta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBSTA_SPEC;
impl crate::RegisterSpec for USBSTA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usbsta::R`](R) reader structure"]
impl crate::Readable for USBSTA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbsta::W`](W) writer structure"]
impl crate::Writable for USBSTA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBSTA to value 0"]
impl crate::Resettable for USBSTA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
