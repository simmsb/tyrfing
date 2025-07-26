#[doc = "Register `TIMSK2` reader"]
pub type R = crate::R<TIMSK2_SPEC>;
#[doc = "Register `TIMSK2` writer"]
pub type W = crate::W<TIMSK2_SPEC>;
#[doc = "Field `TOIE2` reader - Timer/Counter2 Overflow Interrupt Enable"]
pub type TOIE2_R = crate::BitReader;
#[doc = "Field `TOIE2` writer - Timer/Counter2 Overflow Interrupt Enable"]
pub type TOIE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIE2A` reader - Timer/Counter2 Output Compare Match A Interrupt Enable"]
pub type OCIE2A_R = crate::BitReader;
#[doc = "Field `OCIE2A` writer - Timer/Counter2 Output Compare Match A Interrupt Enable"]
pub type OCIE2A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIE2B` reader - Timer/Counter2 Output Compare Match B Interrupt Enable"]
pub type OCIE2B_R = crate::BitReader;
#[doc = "Field `OCIE2B` writer - Timer/Counter2 Output Compare Match B Interrupt Enable"]
pub type OCIE2B_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer/Counter2 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie2(&self) -> TOIE2_R {
        TOIE2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter2 Output Compare Match A Interrupt Enable"]
    #[inline(always)]
    pub fn ocie2a(&self) -> OCIE2A_R {
        OCIE2A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter2 Output Compare Match B Interrupt Enable"]
    #[inline(always)]
    pub fn ocie2b(&self) -> OCIE2B_R {
        OCIE2B_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter2 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie2(&mut self) -> TOIE2_W<TIMSK2_SPEC> {
        TOIE2_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer/Counter2 Output Compare Match A Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie2a(&mut self) -> OCIE2A_W<TIMSK2_SPEC> {
        OCIE2A_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer/Counter2 Output Compare Match B Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie2b(&mut self) -> OCIE2B_W<TIMSK2_SPEC> {
        OCIE2B_W::new(self, 2)
    }
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
#[doc = "Timer/Counter Interrupt Mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timsk2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timsk2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMSK2_SPEC;
impl crate::RegisterSpec for TIMSK2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`timsk2::R`](R) reader structure"]
impl crate::Readable for TIMSK2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timsk2::W`](W) writer structure"]
impl crate::Writable for TIMSK2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMSK2 to value 0"]
impl crate::Resettable for TIMSK2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
