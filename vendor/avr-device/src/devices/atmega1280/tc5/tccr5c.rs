#[doc = "Register `TCCR5C` reader"]
pub type R = crate::R<TCCR5C_SPEC>;
#[doc = "Register `TCCR5C` writer"]
pub type W = crate::W<TCCR5C_SPEC>;
#[doc = "Field `FOC5C` writer - Force Output Compare 5C"]
pub type FOC5C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOC5B` writer - Force Output Compare 5B"]
pub type FOC5B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOC5A` writer - Force Output Compare 5A"]
pub type FOC5A_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 5 - Force Output Compare 5C"]
    #[inline(always)]
    #[must_use]
    pub fn foc5c(&mut self) -> FOC5C_W<TCCR5C_SPEC> {
        FOC5C_W::new(self, 5)
    }
    #[doc = "Bit 6 - Force Output Compare 5B"]
    #[inline(always)]
    #[must_use]
    pub fn foc5b(&mut self) -> FOC5B_W<TCCR5C_SPEC> {
        FOC5B_W::new(self, 6)
    }
    #[doc = "Bit 7 - Force Output Compare 5A"]
    #[inline(always)]
    #[must_use]
    pub fn foc5a(&mut self) -> FOC5A_W<TCCR5C_SPEC> {
        FOC5A_W::new(self, 7)
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
#[doc = "Timer/Counter 5 Control Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr5c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr5c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR5C_SPEC;
impl crate::RegisterSpec for TCCR5C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr5c::R`](R) reader structure"]
impl crate::Readable for TCCR5C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr5c::W`](W) writer structure"]
impl crate::Writable for TCCR5C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR5C to value 0"]
impl crate::Resettable for TCCR5C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
