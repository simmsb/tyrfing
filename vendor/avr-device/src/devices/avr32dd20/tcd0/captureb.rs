#[doc = "Register `CAPTUREB` reader"]
pub type R = crate::R<CAPTUREB_SPEC>;
#[doc = "Field `CAPTUREB` reader - Capture B"]
pub type CAPTUREB_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Capture B"]
    #[inline(always)]
    pub fn captureb(&self) -> CAPTUREB_R {
        CAPTUREB_R::new(self.bits & 0x0fff)
    }
}
#[doc = "Capture B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`captureb::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPTUREB_SPEC;
impl crate::RegisterSpec for CAPTUREB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`captureb::R`](R) reader structure"]
impl crate::Readable for CAPTUREB_SPEC {}
#[doc = "`reset()` method sets CAPTUREB to value 0"]
impl crate::Resettable for CAPTUREB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
