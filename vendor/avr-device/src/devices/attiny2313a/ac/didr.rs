#[doc = "Register `DIDR` reader"]
pub type R = crate::R<DIDR_SPEC>;
#[doc = "Register `DIDR` writer"]
pub type W = crate::W<DIDR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DIDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Digital Input Disable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`didr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`didr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIDR_SPEC;
impl crate::RegisterSpec for DIDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`didr::R`](R) reader structure"]
impl crate::Readable for DIDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`didr::W`](W) writer structure"]
impl crate::Writable for DIDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIDR to value 0"]
impl crate::Resettable for DIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
