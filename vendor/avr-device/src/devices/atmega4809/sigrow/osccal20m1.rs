#[doc = "Register `OSCCAL20M1` reader"]
pub type R = crate::R<OSCCAL20M1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<OSCCAL20M1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Oscillator Calibration 20 MHz Byte 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccal20m1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCCAL20M1_SPEC;
impl crate::RegisterSpec for OSCCAL20M1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`osccal20m1::R`](R) reader structure"]
impl crate::Readable for OSCCAL20M1_SPEC {}
#[doc = "`reset()` method sets OSCCAL20M1 to value 0"]
impl crate::Resettable for OSCCAL20M1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
