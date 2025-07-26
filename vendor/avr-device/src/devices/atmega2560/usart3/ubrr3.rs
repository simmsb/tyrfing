#[doc = "Register `UBRR3` reader"]
pub type R = crate::R<UBRR3_SPEC>;
#[doc = "Register `UBRR3` writer"]
pub type W = crate::W<UBRR3_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<UBRR3_SPEC> {
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
#[doc = "USART Baud Rate Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ubrr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ubrr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UBRR3_SPEC;
impl crate::RegisterSpec for UBRR3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ubrr3::R`](R) reader structure"]
impl crate::Readable for UBRR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ubrr3::W`](W) writer structure"]
impl crate::Writable for UBRR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UBRR3 to value 0"]
impl crate::Resettable for UBRR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
