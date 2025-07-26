#[doc = "Register `SERNUM13` reader"]
pub type R = crate::R<SERNUM13_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SERNUM13_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RES1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum13::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SERNUM13_SPEC;
impl crate::RegisterSpec for SERNUM13_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sernum13::R`](R) reader structure"]
impl crate::Readable for SERNUM13_SPEC {}
#[doc = "`reset()` method sets SERNUM13 to value 0"]
impl crate::Resettable for SERNUM13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
