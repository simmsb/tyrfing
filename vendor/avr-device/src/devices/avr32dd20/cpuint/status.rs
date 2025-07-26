#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `LVL0EX` reader - Level 0 Interrupt Executing"]
pub type LVL0EX_R = crate::BitReader;
#[doc = "Field `LVL1EX` reader - Level 1 Interrupt Executing"]
pub type LVL1EX_R = crate::BitReader;
#[doc = "Field `NMIEX` reader - Non-maskable Interrupt Executing"]
pub type NMIEX_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Level 0 Interrupt Executing"]
    #[inline(always)]
    pub fn lvl0ex(&self) -> LVL0EX_R {
        LVL0EX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level 1 Interrupt Executing"]
    #[inline(always)]
    pub fn lvl1ex(&self) -> LVL1EX_R {
        LVL1EX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Non-maskable Interrupt Executing"]
    #[inline(always)]
    pub fn nmiex(&self) -> NMIEX_R {
        NMIEX_R::new(((self.bits >> 7) & 1) != 0)
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
