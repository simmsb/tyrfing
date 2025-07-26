#[doc = "Register `CAPTUREB` reader"]
pub type R = crate::R<CAPTUREB_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CAPTUREB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Capture B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`captureb::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPTUREB_SPEC;
impl crate::RegisterSpec for CAPTUREB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`captureb::R`](R) reader structure"]
impl crate::Readable for CAPTUREB_SPEC {}
#[doc = "`reset()` method sets CAPTUREB to value 0"]
impl crate::Resettable for CAPTUREB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
