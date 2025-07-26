#[doc = "Register `CMPBCLR` reader"]
pub type R = crate::R<CMPBCLR_SPEC>;
#[doc = "Register `CMPBCLR` writer"]
pub type W = crate::W<CMPBCLR_SPEC>;
#[doc = "Field `COMPBCLR` reader - Compare B Clear"]
pub type COMPBCLR_R = crate::FieldReader<u16>;
#[doc = "Field `COMPBCLR` writer - Compare B Clear"]
pub type COMPBCLR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Compare B Clear"]
    #[inline(always)]
    pub fn compbclr(&self) -> COMPBCLR_R {
        COMPBCLR_R::new(self.bits & 0x0fff)
    }
}
impl W {
    #[doc = "Bits 0:11 - Compare B Clear"]
    #[inline(always)]
    #[must_use]
    pub fn compbclr(&mut self) -> COMPBCLR_W<CMPBCLR_SPEC> {
        COMPBCLR_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Compare B Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpbclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpbclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPBCLR_SPEC;
impl crate::RegisterSpec for CMPBCLR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cmpbclr::R`](R) reader structure"]
impl crate::Readable for CMPBCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpbclr::W`](W) writer structure"]
impl crate::Writable for CMPBCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPBCLR to value 0"]
impl crate::Resettable for CMPBCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
