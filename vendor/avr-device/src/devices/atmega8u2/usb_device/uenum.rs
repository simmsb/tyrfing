#[doc = "Register `UENUM` reader"]
pub type R = crate::R<UENUM_SPEC>;
#[doc = "Register `UENUM` writer"]
pub type W = crate::W<UENUM_SPEC>;
#[doc = "Field `EPNUM` reader - Endpoint Number bits"]
pub type EPNUM_R = crate::FieldReader;
#[doc = "Field `EPNUM` writer - Endpoint Number bits"]
pub type EPNUM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Endpoint Number bits"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Endpoint Number bits"]
    #[inline(always)]
    #[must_use]
    pub fn epnum(&mut self) -> EPNUM_W<UENUM_SPEC> {
        EPNUM_W::new(self, 0)
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
#[doc = "USB Endpoint Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uenum::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uenum::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UENUM_SPEC;
impl crate::RegisterSpec for UENUM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uenum::R`](R) reader structure"]
impl crate::Readable for UENUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uenum::W`](W) writer structure"]
impl crate::Writable for UENUM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UENUM to value 0"]
impl crate::Resettable for UENUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
