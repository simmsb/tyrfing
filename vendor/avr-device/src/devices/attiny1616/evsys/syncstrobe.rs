#[doc = "Register `SYNCSTROBE` writer"]
pub type W = crate::W<SYNCSTROBE_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<SYNCSTROBE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
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
#[doc = "Synchronous Channel Strobe\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syncstrobe::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNCSTROBE_SPEC;
impl crate::RegisterSpec for SYNCSTROBE_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`syncstrobe::W`](W) writer structure"]
impl crate::Writable for SYNCSTROBE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCSTROBE to value 0"]
impl crate::Resettable for SYNCSTROBE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
