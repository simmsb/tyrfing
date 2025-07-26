#[doc = "Register `TEMPSENSE1` reader"]
pub type R = crate::R<TEMPSENSE1_SPEC>;
#[doc = "Field `TEMPSENSE1` reader - Temperature Calibration 1"]
pub type TEMPSENSE1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Temperature Calibration 1"]
    #[inline(always)]
    pub fn tempsense1(&self) -> TEMPSENSE1_R {
        TEMPSENSE1_R::new(self.bits)
    }
}
#[doc = "Temperature Calibration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tempsense1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEMPSENSE1_SPEC;
impl crate::RegisterSpec for TEMPSENSE1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tempsense1::R`](R) reader structure"]
impl crate::Readable for TEMPSENSE1_SPEC {}
#[doc = "`reset()` method sets TEMPSENSE1 to value 0"]
impl crate::Resettable for TEMPSENSE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
