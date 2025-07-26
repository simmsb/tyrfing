#[doc = "Register `TIMSK` reader"]
pub type R = crate::R<TIMSK_SPEC>;
#[doc = "Register `TIMSK` writer"]
pub type W = crate::W<TIMSK_SPEC>;
#[doc = "Field `TOIE1` reader - Timer/Counter1 Overflow Interrupt Enable"]
pub type TOIE1_R = crate::BitReader;
#[doc = "Field `TOIE1` writer - Timer/Counter1 Overflow Interrupt Enable"]
pub type TOIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIE1B` reader - OCIE1A: Timer/Counter1 Output Compare B Interrupt Enable"]
pub type OCIE1B_R = crate::BitReader;
#[doc = "Field `OCIE1B` writer - OCIE1A: Timer/Counter1 Output Compare B Interrupt Enable"]
pub type OCIE1B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIE1A` reader - OCIE1A: Timer/Counter1 Output Compare Interrupt Enable"]
pub type OCIE1A_R = crate::BitReader;
#[doc = "Field `OCIE1A` writer - OCIE1A: Timer/Counter1 Output Compare Interrupt Enable"]
pub type OCIE1A_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Timer/Counter1 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie1(&self) -> TOIE1_R {
        TOIE1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - OCIE1A: Timer/Counter1 Output Compare B Interrupt Enable"]
    #[inline(always)]
    pub fn ocie1b(&self) -> OCIE1B_R {
        OCIE1B_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OCIE1A: Timer/Counter1 Output Compare Interrupt Enable"]
    #[inline(always)]
    pub fn ocie1a(&self) -> OCIE1A_R {
        OCIE1A_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Timer/Counter1 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie1(&mut self) -> TOIE1_W<TIMSK_SPEC> {
        TOIE1_W::new(self, 2)
    }
    #[doc = "Bit 5 - OCIE1A: Timer/Counter1 Output Compare B Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie1b(&mut self) -> OCIE1B_W<TIMSK_SPEC> {
        OCIE1B_W::new(self, 5)
    }
    #[doc = "Bit 6 - OCIE1A: Timer/Counter1 Output Compare Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie1a(&mut self) -> OCIE1A_W<TIMSK_SPEC> {
        OCIE1A_W::new(self, 6)
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
#[doc = "Timer/Counter Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMSK_SPEC;
impl crate::RegisterSpec for TIMSK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`timsk::R`](R) reader structure"]
impl crate::Readable for TIMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timsk::W`](W) writer structure"]
impl crate::Writable for TIMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMSK to value 0"]
impl crate::Resettable for TIMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
