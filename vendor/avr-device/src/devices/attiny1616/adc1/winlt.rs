#[doc = "Register `WINLT` reader"]
pub type R = crate::R<WINLT_SPEC>;
#[doc = "Register `WINLT` writer"]
pub type W = crate::W<WINLT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<WINLT_SPEC> {
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
#[doc = "Window comparator low threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`winlt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`winlt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WINLT_SPEC;
impl crate::RegisterSpec for WINLT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`winlt::R`](R) reader structure"]
impl crate::Readable for WINLT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`winlt::W`](W) writer structure"]
impl crate::Writable for WINLT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WINLT to value 0"]
impl crate::Resettable for WINLT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
