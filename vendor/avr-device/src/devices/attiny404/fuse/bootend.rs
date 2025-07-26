#[doc = "Register `BOOTEND` reader"]
pub type R = crate::R<BOOTEND_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<BOOTEND_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Boot Section End\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bootend::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTEND_SPEC;
impl crate::RegisterSpec for BOOTEND_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bootend::R`](R) reader structure"]
impl crate::Readable for BOOTEND_SPEC {}
#[doc = "`reset()` method sets BOOTEND to value 0"]
impl crate::Resettable for BOOTEND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
