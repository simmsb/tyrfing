#[doc = "Register `CCMP` reader"]
pub type R = crate::R<CCMP_SPEC>;
#[doc = "Register `CCMP` writer"]
pub type W = crate::W<CCMP_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CCMP_SPEC> {
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
#[doc = "Compare or Capture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMP_SPEC;
impl crate::RegisterSpec for CCMP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ccmp::R`](R) reader structure"]
impl crate::Readable for CCMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccmp::W`](W) writer structure"]
impl crate::Writable for CCMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCMP to value 0"]
impl crate::Resettable for CCMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
