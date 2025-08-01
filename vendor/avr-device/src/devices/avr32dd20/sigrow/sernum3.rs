#[doc = "Register `SERNUM3` reader"]
pub type R = crate::R<SERNUM3_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SERNUM3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "LOTNUM3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SERNUM3_SPEC;
impl crate::RegisterSpec for SERNUM3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sernum3::R`](R) reader structure"]
impl crate::Readable for SERNUM3_SPEC {}
#[doc = "`reset()` method sets SERNUM3 to value 0"]
impl crate::Resettable for SERNUM3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
