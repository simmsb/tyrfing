#[doc = "Register `OSCCAL16M1` reader"]
pub type R = crate::R<OSCCAL16M1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<OSCCAL16M1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Oscillator Calibration 16 MHz Byte 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccal16m1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCCAL16M1_SPEC;
impl crate::RegisterSpec for OSCCAL16M1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`osccal16m1::R`](R) reader structure"]
impl crate::Readable for OSCCAL16M1_SPEC {}
#[doc = "`reset()` method sets OSCCAL16M1 to value 0"]
impl crate::Resettable for OSCCAL16M1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
