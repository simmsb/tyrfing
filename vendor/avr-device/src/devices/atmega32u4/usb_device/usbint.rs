#[doc = "Register `USBINT` reader"]
pub type R = crate::R<USBINT_SPEC>;
#[doc = "Register `USBINT` writer"]
pub type W = crate::W<USBINT_SPEC>;
#[doc = "Field `VBUSTI` reader - No Description."]
pub type VBUSTI_R = crate::BitReader;
#[doc = "Field `VBUSTI` writer - No Description."]
pub type VBUSTI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn vbusti(&self) -> VBUSTI_R {
        VBUSTI_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn vbusti(&mut self) -> VBUSTI_W<USBINT_SPEC> {
        VBUSTI_W::new(self, 0)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBINT_SPEC;
impl crate::RegisterSpec for USBINT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usbint::R`](R) reader structure"]
impl crate::Readable for USBINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbint::W`](W) writer structure"]
impl crate::Writable for USBINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBINT to value 0"]
impl crate::Resettable for USBINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
