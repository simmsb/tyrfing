#[doc = "Register `CHECKSUM1` reader"]
pub type R = crate::R<CHECKSUM1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CHECKSUM1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "CRC Checksum Byte 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`checksum1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHECKSUM1_SPEC;
impl crate::RegisterSpec for CHECKSUM1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`checksum1::R`](R) reader structure"]
impl crate::Readable for CHECKSUM1_SPEC {}
#[doc = "`reset()` method sets CHECKSUM1 to value 0"]
impl crate::Resettable for CHECKSUM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
