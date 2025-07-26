#[doc = "Register `TIMSK1` reader"]
pub type R = crate::R<TIMSK1_SPEC>;
#[doc = "Register `TIMSK1` writer"]
pub type W = crate::W<TIMSK1_SPEC>;
#[doc = "Field `TOIE1` reader - Timer/Counter1 Overflow Interrupt Enable"]
pub type TOIE1_R = crate::BitReader;
#[doc = "Field `TOIE1` writer - Timer/Counter1 Overflow Interrupt Enable"]
pub type TOIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIE1A` reader - Timer/Counter1 Output CompareA Match Interrupt Enable"]
pub type OCIE1A_R = crate::BitReader;
#[doc = "Field `OCIE1A` writer - Timer/Counter1 Output CompareA Match Interrupt Enable"]
pub type OCIE1A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIE1B` reader - Timer/Counter1 Output CompareB Match Interrupt Enable"]
pub type OCIE1B_R = crate::BitReader;
#[doc = "Field `OCIE1B` writer - Timer/Counter1 Output CompareB Match Interrupt Enable"]
pub type OCIE1B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICIE1` reader - Timer/Counter1 Input Capture Interrupt Enable"]
pub type ICIE1_R = crate::BitReader;
#[doc = "Field `ICIE1` writer - Timer/Counter1 Input Capture Interrupt Enable"]
pub type ICIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer/Counter1 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie1(&self) -> TOIE1_R {
        TOIE1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter1 Output CompareA Match Interrupt Enable"]
    #[inline(always)]
    pub fn ocie1a(&self) -> OCIE1A_R {
        OCIE1A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter1 Output CompareB Match Interrupt Enable"]
    #[inline(always)]
    pub fn ocie1b(&self) -> OCIE1B_R {
        OCIE1B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer/Counter1 Input Capture Interrupt Enable"]
    #[inline(always)]
    pub fn icie1(&self) -> ICIE1_R {
        ICIE1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter1 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie1(&mut self) -> TOIE1_W<TIMSK1_SPEC> {
        TOIE1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer/Counter1 Output CompareA Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie1a(&mut self) -> OCIE1A_W<TIMSK1_SPEC> {
        OCIE1A_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer/Counter1 Output CompareB Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie1b(&mut self) -> OCIE1B_W<TIMSK1_SPEC> {
        OCIE1B_W::new(self, 2)
    }
    #[doc = "Bit 5 - Timer/Counter1 Input Capture Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icie1(&mut self) -> ICIE1_W<TIMSK1_SPEC> {
        ICIE1_W::new(self, 5)
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
#[doc = "Timer/Counter Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timsk1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timsk1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMSK1_SPEC;
impl crate::RegisterSpec for TIMSK1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`timsk1::R`](R) reader structure"]
impl crate::Readable for TIMSK1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timsk1::W`](W) writer structure"]
impl crate::Writable for TIMSK1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMSK1 to value 0"]
impl crate::Resettable for TIMSK1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
