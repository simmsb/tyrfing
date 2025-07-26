#[doc = "Register `TIMSK4` reader"]
pub type R = crate::R<TIMSK4_SPEC>;
#[doc = "Register `TIMSK4` writer"]
pub type W = crate::W<TIMSK4_SPEC>;
#[doc = "Field `TOIE4` reader - Timer/Counter4 Overflow Interrupt Enable"]
pub type TOIE4_R = crate::BitReader;
#[doc = "Field `TOIE4` writer - Timer/Counter4 Overflow Interrupt Enable"]
pub type TOIE4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIE4A` reader - Timer/Counter4 Output Compare Match A Interrupt Enable"]
pub type OCIE4A_R = crate::BitReader;
#[doc = "Field `OCIE4A` writer - Timer/Counter4 Output Compare Match A Interrupt Enable"]
pub type OCIE4A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIE4B` reader - Timer/Counter4 Output Compare Match B Interrupt Enable"]
pub type OCIE4B_R = crate::BitReader;
#[doc = "Field `OCIE4B` writer - Timer/Counter4 Output Compare Match B Interrupt Enable"]
pub type OCIE4B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICIE4` reader - Timer/Counter4 Input Capture Interrupt Enable"]
pub type ICIE4_R = crate::BitReader;
#[doc = "Field `ICIE4` writer - Timer/Counter4 Input Capture Interrupt Enable"]
pub type ICIE4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer/Counter4 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie4(&self) -> TOIE4_R {
        TOIE4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter4 Output Compare Match A Interrupt Enable"]
    #[inline(always)]
    pub fn ocie4a(&self) -> OCIE4A_R {
        OCIE4A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter4 Output Compare Match B Interrupt Enable"]
    #[inline(always)]
    pub fn ocie4b(&self) -> OCIE4B_R {
        OCIE4B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer/Counter4 Input Capture Interrupt Enable"]
    #[inline(always)]
    pub fn icie4(&self) -> ICIE4_R {
        ICIE4_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter4 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie4(&mut self) -> TOIE4_W<TIMSK4_SPEC> {
        TOIE4_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer/Counter4 Output Compare Match A Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie4a(&mut self) -> OCIE4A_W<TIMSK4_SPEC> {
        OCIE4A_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer/Counter4 Output Compare Match B Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie4b(&mut self) -> OCIE4B_W<TIMSK4_SPEC> {
        OCIE4B_W::new(self, 2)
    }
    #[doc = "Bit 5 - Timer/Counter4 Input Capture Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icie4(&mut self) -> ICIE4_W<TIMSK4_SPEC> {
        ICIE4_W::new(self, 5)
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
#[doc = "Timer/Counter4 Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timsk4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timsk4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMSK4_SPEC;
impl crate::RegisterSpec for TIMSK4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`timsk4::R`](R) reader structure"]
impl crate::Readable for TIMSK4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timsk4::W`](W) writer structure"]
impl crate::Writable for TIMSK4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMSK4 to value 0"]
impl crate::Resettable for TIMSK4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
