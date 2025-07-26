#[doc = "Register `CAPTUREA` reader"]
pub type R = crate::R<CAPTUREA_SPEC>;
#[doc = "Field `CAPTUREA` reader - Capture A"]
pub type CAPTUREA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Capture A"]
    #[inline(always)]
    pub fn capturea(&self) -> CAPTUREA_R {
        CAPTUREA_R::new(self.bits & 0x0fff)
    }
}
#[doc = "Capture A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capturea::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPTUREA_SPEC;
impl crate::RegisterSpec for CAPTUREA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`capturea::R`](R) reader structure"]
impl crate::Readable for CAPTUREA_SPEC {}
#[doc = "`reset()` method sets CAPTUREA to value 0"]
impl crate::Resettable for CAPTUREA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
