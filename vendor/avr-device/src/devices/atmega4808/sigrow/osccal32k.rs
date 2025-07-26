#[doc = "Register `OSCCAL32K` reader"]
pub type R = crate::R<OSCCAL32K_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<OSCCAL32K_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Oscillator Calibration for 32kHz ULP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccal32k::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCCAL32K_SPEC;
impl crate::RegisterSpec for OSCCAL32K_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`osccal32k::R`](R) reader structure"]
impl crate::Readable for OSCCAL32K_SPEC {}
#[doc = "`reset()` method sets OSCCAL32K to value 0"]
impl crate::Resettable for OSCCAL32K_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
