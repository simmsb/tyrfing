#[doc = "Register `SERNUM9` reader"]
pub type R = crate::R<SERNUM9_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SERNUM9_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Serial Number Byte 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sernum9::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SERNUM9_SPEC;
impl crate::RegisterSpec for SERNUM9_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sernum9::R`](R) reader structure"]
impl crate::Readable for SERNUM9_SPEC {}
#[doc = "`reset()` method sets SERNUM9 to value 0"]
impl crate::Resettable for SERNUM9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
