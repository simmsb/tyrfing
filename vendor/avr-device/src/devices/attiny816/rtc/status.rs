#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `CTRLABUSY` reader - CTRLA Synchronization Busy Flag"]
pub type CTRLABUSY_R = crate::BitReader;
#[doc = "Field `CNTBUSY` reader - Count Synchronization Busy Flag"]
pub type CNTBUSY_R = crate::BitReader;
#[doc = "Field `PERBUSY` reader - Period Synchronization Busy Flag"]
pub type PERBUSY_R = crate::BitReader;
#[doc = "Field `CMPBUSY` reader - Comparator Synchronization Busy Flag"]
pub type CMPBUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CTRLA Synchronization Busy Flag"]
    #[inline(always)]
    pub fn ctrlabusy(&self) -> CTRLABUSY_R {
        CTRLABUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Count Synchronization Busy Flag"]
    #[inline(always)]
    pub fn cntbusy(&self) -> CNTBUSY_R {
        CNTBUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Period Synchronization Busy Flag"]
    #[inline(always)]
    pub fn perbusy(&self) -> PERBUSY_R {
        PERBUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator Synchronization Busy Flag"]
    #[inline(always)]
    pub fn cmpbusy(&self) -> CMPBUSY_R {
        CMPBUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
