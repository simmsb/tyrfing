#[doc = "Register `CMPBCLR` reader"]
pub type R = crate::R<CMPBCLR_SPEC>;
#[doc = "Register `CMPBCLR` writer"]
pub type W = crate::W<CMPBCLR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CMPBCLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
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
