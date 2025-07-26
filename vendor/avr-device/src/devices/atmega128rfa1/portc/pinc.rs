#[doc = "Register `PINC` reader"]
pub type R = crate::R<PINC_SPEC>;
#[doc = "Register `PINC` writer"]
pub type W = crate::W<PINC_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PINC_SPEC> {
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
#[doc = "Port C Input Pins Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINC_SPEC;
impl crate::RegisterSpec for PINC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pinc::R`](R) reader structure"]
impl crate::Readable for PINC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinc::W`](W) writer structure"]
impl crate::Writable for PINC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINC to value 0"]
impl crate::Resettable for PINC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
