#[doc = "Register `TEMPSENSE1` reader"]
pub type R = crate::R<TEMPSENSE1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TEMPSENSE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Temperature Sensor Calibration: Offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tempsense1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEMPSENSE1_SPEC;
impl crate::RegisterSpec for TEMPSENSE1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tempsense1::R`](R) reader structure"]
impl crate::Readable for TEMPSENSE1_SPEC {}
#[doc = "`reset()` method sets TEMPSENSE1 to value 0"]
impl crate::Resettable for TEMPSENSE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
