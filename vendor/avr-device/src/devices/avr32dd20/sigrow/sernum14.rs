#[doc = "Register `SERNUM14` reader"]
pub type R = crate::R<SERNUM14_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SERNUM14_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RES2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum14::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SERNUM14_SPEC;
impl crate::RegisterSpec for SERNUM14_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sernum14::R`](R) reader structure"]
impl crate::Readable for SERNUM14_SPEC {}
#[doc = "`reset()` method sets SERNUM14 to value 0"]
impl crate::Resettable for SERNUM14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
