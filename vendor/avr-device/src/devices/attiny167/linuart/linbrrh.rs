#[doc = "Register `LINBRRH` reader"]
pub type R = crate::R<LINBRRH_SPEC>;
#[doc = "Register `LINBRRH` writer"]
pub type W = crate::W<LINBRRH_SPEC>;
#[doc = "Field `LDIV` reader - No Description."]
pub type LDIV_R = crate::FieldReader;
#[doc = "Field `LDIV` writer - No Description."]
pub type LDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    pub fn ldiv(&self) -> LDIV_R {
        LDIV_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn ldiv(&mut self) -> LDIV_W<LINBRRH_SPEC> {
        LDIV_W::new(self, 0)
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
#[doc = "LIN Baud Rate High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linbrrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linbrrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINBRRH_SPEC;
impl crate::RegisterSpec for LINBRRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`linbrrh::R`](R) reader structure"]
impl crate::Readable for LINBRRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`linbrrh::W`](W) writer structure"]
impl crate::Writable for LINBRRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINBRRH to value 0"]
impl crate::Resettable for LINBRRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
