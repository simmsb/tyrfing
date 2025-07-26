#[doc = "Register `PERBUF` reader"]
pub type R = crate::R<SINGLE_PERBUF_SPEC>;
#[doc = "Register `PERBUF` writer"]
pub type W = crate::W<SINGLE_PERBUF_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SINGLE_PERBUF_SPEC> {
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
#[doc = "Period Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_perbuf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_perbuf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINGLE_PERBUF_SPEC;
impl crate::RegisterSpec for SINGLE_PERBUF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`single_perbuf::R`](R) reader structure"]
impl crate::Readable for SINGLE_PERBUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`single_perbuf::W`](W) writer structure"]
impl crate::Writable for SINGLE_PERBUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERBUF to value 0"]
impl crate::Resettable for SINGLE_PERBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
