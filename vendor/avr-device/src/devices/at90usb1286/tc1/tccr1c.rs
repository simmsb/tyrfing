#[doc = "Register `TCCR1C` reader"]
pub type R = crate::R<TCCR1C_SPEC>;
#[doc = "Register `TCCR1C` writer"]
pub type W = crate::W<TCCR1C_SPEC>;
#[doc = "Field `FOC1C` reader - Force Output Compare 1C"]
pub type FOC1C_R = crate::BitReader;
#[doc = "Field `FOC1C` writer - Force Output Compare 1C"]
pub type FOC1C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOC1B` reader - Force Output Compare 1B"]
pub type FOC1B_R = crate::BitReader;
#[doc = "Field `FOC1B` writer - Force Output Compare 1B"]
pub type FOC1B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOC1A` reader - Force Output Compare 1A"]
pub type FOC1A_R = crate::BitReader;
#[doc = "Field `FOC1A` writer - Force Output Compare 1A"]
pub type FOC1A_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Force Output Compare 1C"]
    #[inline(always)]
    pub fn foc1c(&self) -> FOC1C_R {
        FOC1C_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Output Compare 1B"]
    #[inline(always)]
    pub fn foc1b(&self) -> FOC1B_R {
        FOC1B_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Output Compare 1A"]
    #[inline(always)]
    pub fn foc1a(&self) -> FOC1A_R {
        FOC1A_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Force Output Compare 1C"]
    #[inline(always)]
    #[must_use]
    pub fn foc1c(&mut self) -> FOC1C_W<TCCR1C_SPEC> {
        FOC1C_W::new(self, 5)
    }
    #[doc = "Bit 6 - Force Output Compare 1B"]
    #[inline(always)]
    #[must_use]
    pub fn foc1b(&mut self) -> FOC1B_W<TCCR1C_SPEC> {
        FOC1B_W::new(self, 6)
    }
    #[doc = "Bit 7 - Force Output Compare 1A"]
    #[inline(always)]
    #[must_use]
    pub fn foc1a(&mut self) -> FOC1A_W<TCCR1C_SPEC> {
        FOC1A_W::new(self, 7)
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
#[doc = "Timer/Counter 1 Control Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr1c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr1c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR1C_SPEC;
impl crate::RegisterSpec for TCCR1C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr1c::R`](R) reader structure"]
impl crate::Readable for TCCR1C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr1c::W`](W) writer structure"]
impl crate::Writable for TCCR1C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR1C to value 0"]
impl crate::Resettable for TCCR1C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
