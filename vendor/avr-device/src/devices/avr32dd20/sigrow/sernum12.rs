#[doc = "Register `SERNUM12` reader"]
pub type R = crate::R<SERNUM12_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SERNUM12_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RES0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum12::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SERNUM12_SPEC;
impl crate::RegisterSpec for SERNUM12_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sernum12::R`](R) reader structure"]
impl crate::Readable for SERNUM12_SPEC {}
#[doc = "`reset()` method sets SERNUM12 to value 0"]
impl crate::Resettable for SERNUM12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
