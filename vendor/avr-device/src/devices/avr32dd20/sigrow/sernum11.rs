#[doc = "Register `SERNUM11` reader"]
pub type R = crate::R<SERNUM11_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SERNUM11_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "YPOS1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum11::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SERNUM11_SPEC;
impl crate::RegisterSpec for SERNUM11_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sernum11::R`](R) reader structure"]
impl crate::Readable for SERNUM11_SPEC {}
#[doc = "`reset()` method sets SERNUM11 to value 0"]
impl crate::Resettable for SERNUM11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
