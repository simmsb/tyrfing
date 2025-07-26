#[doc = "Register `PITSTATUS` reader"]
pub type R = crate::R<PITSTATUS_SPEC>;
#[doc = "Field `CTRLBUSY` reader - CTRLA Synchronization Busy Flag"]
pub type CTRLBUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CTRLA Synchronization Busy Flag"]
    #[inline(always)]
    pub fn ctrlbusy(&self) -> CTRLBUSY_R {
        CTRLBUSY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "PIT Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pitstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PITSTATUS_SPEC;
impl crate::RegisterSpec for PITSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pitstatus::R`](R) reader structure"]
impl crate::Readable for PITSTATUS_SPEC {}
#[doc = "`reset()` method sets PITSTATUS to value 0"]
impl crate::Resettable for PITSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
