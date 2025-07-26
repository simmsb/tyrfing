#[doc = "Register `SERNUM%s` reader"]
pub type R = crate::R<SERNUM_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SERNUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Serial Number Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SERNUM_SPEC;
impl crate::RegisterSpec for SERNUM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sernum::R`](R) reader structure"]
impl crate::Readable for SERNUM_SPEC {}
#[doc = "`reset()` method sets SERNUM%s to value 0"]
impl crate::Resettable for SERNUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
