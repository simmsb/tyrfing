#[doc = "Register `ASYNCSTROBE` writer"]
pub type W = crate::W<ASYNCSTROBE_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<ASYNCSTROBE_SPEC> {
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
#[doc = "Asynchronous Channel Strobe\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncstrobe::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASYNCSTROBE_SPEC;
impl crate::RegisterSpec for ASYNCSTROBE_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`asyncstrobe::W`](W) writer structure"]
impl crate::Writable for ASYNCSTROBE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASYNCSTROBE to value 0"]
impl crate::Resettable for ASYNCSTROBE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
