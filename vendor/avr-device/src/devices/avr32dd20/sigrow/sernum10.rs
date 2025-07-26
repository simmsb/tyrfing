#[doc = "Register `SERNUM10` reader"]
pub type R = crate::R<SERNUM10_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SERNUM10_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "YPOS0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum10::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SERNUM10_SPEC;
impl crate::RegisterSpec for SERNUM10_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sernum10::R`](R) reader structure"]
impl crate::Readable for SERNUM10_SPEC {}
#[doc = "`reset()` method sets SERNUM10 to value 0"]
impl crate::Resettable for SERNUM10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
