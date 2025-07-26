#[doc = "Register `SERNUM7` reader"]
pub type R = crate::R<SERNUM7_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SERNUM7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Serial Number Byte 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SERNUM7_SPEC;
impl crate::RegisterSpec for SERNUM7_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sernum7::R`](R) reader structure"]
impl crate::Readable for SERNUM7_SPEC {}
#[doc = "`reset()` method sets SERNUM7 to value 0"]
impl crate::Resettable for SERNUM7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
