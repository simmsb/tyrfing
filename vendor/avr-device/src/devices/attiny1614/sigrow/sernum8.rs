#[doc = "Register `SERNUM8` reader"]
pub type R = crate::R<SERNUM8_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SERNUM8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Serial Number Byte 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SERNUM8_SPEC;
impl crate::RegisterSpec for SERNUM8_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sernum8::R`](R) reader structure"]
impl crate::Readable for SERNUM8_SPEC {}
#[doc = "`reset()` method sets SERNUM8 to value 0"]
impl crate::Resettable for SERNUM8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
