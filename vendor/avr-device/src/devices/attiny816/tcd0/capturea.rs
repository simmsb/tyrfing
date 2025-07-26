#[doc = "Register `CAPTUREA` reader"]
pub type R = crate::R<CAPTUREA_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CAPTUREA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Capture A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capturea::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPTUREA_SPEC;
impl crate::RegisterSpec for CAPTUREA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`capturea::R`](R) reader structure"]
impl crate::Readable for CAPTUREA_SPEC {}
#[doc = "`reset()` method sets CAPTUREA to value 0"]
impl crate::Resettable for CAPTUREA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
