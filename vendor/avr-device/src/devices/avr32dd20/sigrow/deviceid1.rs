#[doc = "Register `DEVICEID1` reader"]
pub type R = crate::R<DEVICEID1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DEVICEID1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Device ID Byte 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deviceid1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVICEID1_SPEC;
impl crate::RegisterSpec for DEVICEID1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`deviceid1::R`](R) reader structure"]
impl crate::Readable for DEVICEID1_SPEC {}
#[doc = "`reset()` method sets DEVICEID1 to value 0"]
impl crate::Resettable for DEVICEID1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
