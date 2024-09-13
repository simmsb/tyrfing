#[doc = "Register `RES` reader"]
pub type R = crate::R<RES_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RES_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "ADC Accumulator Result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`res::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RES_SPEC;
impl crate::RegisterSpec for RES_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`res::R`](R) reader structure"]
impl crate::Readable for RES_SPEC {}
#[doc = "`reset()` method sets RES to value 0"]
impl crate::Resettable for RES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
