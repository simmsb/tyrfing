#[doc = "Register `SERNUM4` reader"]
pub type R = crate::R<SERNUM4_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SERNUM4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "LOTNUM4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SERNUM4_SPEC;
impl crate::RegisterSpec for SERNUM4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sernum4::R`](R) reader structure"]
impl crate::Readable for SERNUM4_SPEC {}
#[doc = "`reset()` method sets SERNUM4 to value 0"]
impl crate::Resettable for SERNUM4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
