#[doc = "Register `PORTC` reader"]
pub type R = crate::R<PORTC_SPEC>;
#[doc = "Register `PORTC` writer"]
pub type W = crate::W<PORTC_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PORTC_SPEC> {
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
#[doc = "Port C Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PORTC_SPEC;
impl crate::RegisterSpec for PORTC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`portc::R`](R) reader structure"]
impl crate::Readable for PORTC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`portc::W`](W) writer structure"]
impl crate::Writable for PORTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORTC to value 0"]
impl crate::Resettable for PORTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
