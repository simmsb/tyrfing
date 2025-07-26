#[doc = "Register `SERNUM0` reader"]
pub type R = crate::R<SERNUM0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SERNUM0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Serial Number Byte 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SERNUM0_SPEC;
impl crate::RegisterSpec for SERNUM0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sernum0::R`](R) reader structure"]
impl crate::Readable for SERNUM0_SPEC {}
#[doc = "`reset()` method sets SERNUM0 to value 0"]
impl crate::Resettable for SERNUM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
