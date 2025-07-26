#[doc = "Register `TCCR2C` reader"]
pub type R = crate::R<TCCR2C_SPEC>;
#[doc = "Register `TCCR2C` writer"]
pub type W = crate::W<TCCR2C_SPEC>;
#[doc = "Field `FOC2B` writer - Force Output Compare for Channel B"]
pub type FOC2B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOC2A` writer - Force Output Compare for Channel A"]
pub type FOC2A_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 6 - Force Output Compare for Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn foc2b(&mut self) -> FOC2B_W<TCCR2C_SPEC> {
        FOC2B_W::new(self, 6)
    }
    #[doc = "Bit 7 - Force Output Compare for Channel A"]
    #[inline(always)]
    #[must_use]
    pub fn foc2a(&mut self) -> FOC2A_W<TCCR2C_SPEC> {
        FOC2A_W::new(self, 7)
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
#[doc = "Timer/Counter2 Control Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr2c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr2c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR2C_SPEC;
impl crate::RegisterSpec for TCCR2C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr2c::R`](R) reader structure"]
impl crate::Readable for TCCR2C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr2c::W`](W) writer structure"]
impl crate::Writable for TCCR2C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR2C to value 0"]
impl crate::Resettable for TCCR2C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
