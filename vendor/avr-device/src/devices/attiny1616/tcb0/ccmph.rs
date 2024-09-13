#[doc = "Register `CCMPH` reader"]
pub type R = crate::R<CCMPH_SPEC>;
#[doc = "Register `CCMPH` writer"]
pub type W = crate::W<CCMPH_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CCMPH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Compare Capture high bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmph::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmph::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMPH_SPEC;
impl crate::RegisterSpec for CCMPH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ccmph::R`](R) reader structure"]
impl crate::Readable for CCMPH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccmph::W`](W) writer structure"]
impl crate::Writable for CCMPH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCMPH to value 0"]
impl crate::Resettable for CCMPH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
