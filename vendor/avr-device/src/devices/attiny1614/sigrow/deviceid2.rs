#[doc = "Register `DEVICEID2` reader"]
pub type R = crate::R<DEVICEID2_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DEVICEID2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Device ID Byte 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deviceid2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVICEID2_SPEC;
impl crate::RegisterSpec for DEVICEID2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`deviceid2::R`](R) reader structure"]
impl crate::Readable for DEVICEID2_SPEC {}
#[doc = "`reset()` method sets DEVICEID2 to value 0"]
impl crate::Resettable for DEVICEID2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
