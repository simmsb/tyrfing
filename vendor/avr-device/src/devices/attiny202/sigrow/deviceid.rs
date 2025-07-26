#[doc = "Register `DEVICEID%s` reader"]
pub type R = crate::R<DEVICEID_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DEVICEID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Device IO Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deviceid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVICEID_SPEC;
impl crate::RegisterSpec for DEVICEID_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`deviceid::R`](R) reader structure"]
impl crate::Readable for DEVICEID_SPEC {}
#[doc = "`reset()` method sets DEVICEID%s to value 0"]
impl crate::Resettable for DEVICEID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
