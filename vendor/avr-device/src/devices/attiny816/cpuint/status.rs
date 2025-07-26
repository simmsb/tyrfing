#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
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
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
