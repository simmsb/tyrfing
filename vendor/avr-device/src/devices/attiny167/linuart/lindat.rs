#[doc = "Register `LINDAT` reader"]
pub type R = crate::R<LINDAT_SPEC>;
#[doc = "Register `LINDAT` writer"]
pub type W = crate::W<LINDAT_SPEC>;
#[doc = "Field `LDATA` reader - No Description."]
pub type LDATA_R = crate::FieldReader;
#[doc = "Field `LDATA` writer - No Description."]
pub type LDATA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - No Description."]
    #[inline(always)]
    pub fn ldata(&self) -> LDATA_R {
        LDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn ldata(&mut self) -> LDATA_W<LINDAT_SPEC> {
        LDATA_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LIN Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lindat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lindat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINDAT_SPEC;
impl crate::RegisterSpec for LINDAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lindat::R`](R) reader structure"]
impl crate::Readable for LINDAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lindat::W`](W) writer structure"]
impl crate::Writable for LINDAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINDAT to value 0"]
impl crate::Resettable for LINDAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
