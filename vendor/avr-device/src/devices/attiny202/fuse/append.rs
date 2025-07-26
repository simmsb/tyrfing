#[doc = "Register `APPEND` reader"]
pub type R = crate::R<APPEND_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<APPEND_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Application Code Section End\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`append::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APPEND_SPEC;
impl crate::RegisterSpec for APPEND_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`append::R`](R) reader structure"]
impl crate::Readable for APPEND_SPEC {}
#[doc = "`reset()` method sets APPEND to value 0"]
impl crate::Resettable for APPEND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
