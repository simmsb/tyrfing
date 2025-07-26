#[doc = "Register `DEVICEID0` reader"]
pub type R = crate::R<DEVICEID0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DEVICEID0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Device ID Byte 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deviceid0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVICEID0_SPEC;
impl crate::RegisterSpec for DEVICEID0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`deviceid0::R`](R) reader structure"]
impl crate::Readable for DEVICEID0_SPEC {}
#[doc = "`reset()` method sets DEVICEID0 to value 0"]
impl crate::Resettable for DEVICEID0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
