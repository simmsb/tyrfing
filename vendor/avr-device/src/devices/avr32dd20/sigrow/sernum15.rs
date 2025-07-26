#[doc = "Register `SERNUM15` reader"]
pub type R = crate::R<SERNUM15_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SERNUM15_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RES3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum15::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SERNUM15_SPEC;
impl crate::RegisterSpec for SERNUM15_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sernum15::R`](R) reader structure"]
impl crate::Readable for SERNUM15_SPEC {}
#[doc = "`reset()` method sets SERNUM15 to value 0"]
impl crate::Resettable for SERNUM15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
