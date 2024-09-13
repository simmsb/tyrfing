#[doc = "Register `CCMPL` reader"]
pub type R = crate::R<CCMPL_SPEC>;
#[doc = "Register `CCMPL` writer"]
pub type W = crate::W<CCMPL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CCMPL_SPEC> {
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
#[doc = "Compare Capture low bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmpl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmpl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMPL_SPEC;
impl crate::RegisterSpec for CCMPL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ccmpl::R`](R) reader structure"]
impl crate::Readable for CCMPL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccmpl::W`](W) writer structure"]
impl crate::Writable for CCMPL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCMPL to value 0"]
impl crate::Resettable for CCMPL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
