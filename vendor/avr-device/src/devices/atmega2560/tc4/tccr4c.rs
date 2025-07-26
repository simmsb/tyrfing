#[doc = "Register `TCCR4C` reader"]
pub type R = crate::R<TCCR4C_SPEC>;
#[doc = "Register `TCCR4C` writer"]
pub type W = crate::W<TCCR4C_SPEC>;
#[doc = "Field `FOC4C` writer - Force Output Compare 4C"]
pub type FOC4C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOC4B` writer - Force Output Compare 4B"]
pub type FOC4B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOC4A` writer - Force Output Compare 4A"]
pub type FOC4A_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 5 - Force Output Compare 4C"]
    #[inline(always)]
    #[must_use]
    pub fn foc4c(&mut self) -> FOC4C_W<TCCR4C_SPEC> {
        FOC4C_W::new(self, 5)
    }
    #[doc = "Bit 6 - Force Output Compare 4B"]
    #[inline(always)]
    #[must_use]
    pub fn foc4b(&mut self) -> FOC4B_W<TCCR4C_SPEC> {
        FOC4B_W::new(self, 6)
    }
    #[doc = "Bit 7 - Force Output Compare 4A"]
    #[inline(always)]
    #[must_use]
    pub fn foc4a(&mut self) -> FOC4A_W<TCCR4C_SPEC> {
        FOC4A_W::new(self, 7)
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
#[doc = "Timer/Counter 4 Control Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr4c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr4c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR4C_SPEC;
impl crate::RegisterSpec for TCCR4C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr4c::R`](R) reader structure"]
impl crate::Readable for TCCR4C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr4c::W`](W) writer structure"]
impl crate::Writable for TCCR4C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR4C to value 0"]
impl crate::Resettable for TCCR4C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
