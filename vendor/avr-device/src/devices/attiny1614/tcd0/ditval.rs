#[doc = "Register `DITVAL` reader"]
pub type R = crate::R<DITVAL_SPEC>;
#[doc = "Register `DITVAL` writer"]
pub type W = crate::W<DITVAL_SPEC>;
#[doc = "Field `DITHER` reader - Dither value"]
pub type DITHER_R = crate::FieldReader;
#[doc = "Field `DITHER` writer - Dither value"]
pub type DITHER_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Dither value"]
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dither value"]
    #[inline(always)]
    #[must_use]
    pub fn dither(&mut self) -> DITHER_W<DITVAL_SPEC> {
        DITHER_W::new(self, 0)
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
#[doc = "Dither value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ditval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ditval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DITVAL_SPEC;
impl crate::RegisterSpec for DITVAL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ditval::R`](R) reader structure"]
impl crate::Readable for DITVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ditval::W`](W) writer structure"]
impl crate::Writable for DITVAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DITVAL to value 0"]
impl crate::Resettable for DITVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
