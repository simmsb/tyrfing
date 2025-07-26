#[doc = "Register `SERNUM6` reader"]
pub type R = crate::R<SERNUM6_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SERNUM6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RANDOM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SERNUM6_SPEC;
impl crate::RegisterSpec for SERNUM6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sernum6::R`](R) reader structure"]
impl crate::Readable for SERNUM6_SPEC {}
#[doc = "`reset()` method sets SERNUM6 to value 0"]
impl crate::Resettable for SERNUM6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
