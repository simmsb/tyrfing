#[doc = "Register `TIMSK0` reader"]
pub type R = crate::R<TIMSK0_SPEC>;
#[doc = "Register `TIMSK0` writer"]
pub type W = crate::W<TIMSK0_SPEC>;
#[doc = "Field `TOIE0` reader - Timer/Counter0 Overflow Interrupt Enable"]
pub type TOIE0_R = crate::BitReader;
#[doc = "Field `TOIE0` writer - Timer/Counter0 Overflow Interrupt Enable"]
pub type TOIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIE0A` reader - Timer/Counter0 Output Compare Match A Interrupt Enable"]
pub type OCIE0A_R = crate::BitReader;
#[doc = "Field `OCIE0A` writer - Timer/Counter0 Output Compare Match A Interrupt Enable"]
pub type OCIE0A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIE0B` reader - Timer/Counter0 Output Compare Match B Interrupt Enable"]
pub type OCIE0B_R = crate::BitReader;
#[doc = "Field `OCIE0B` writer - Timer/Counter0 Output Compare Match B Interrupt Enable"]
pub type OCIE0B_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer/Counter0 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie0(&self) -> TOIE0_R {
        TOIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter0 Output Compare Match A Interrupt Enable"]
    #[inline(always)]
    pub fn ocie0a(&self) -> OCIE0A_R {
        OCIE0A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter0 Output Compare Match B Interrupt Enable"]
    #[inline(always)]
    pub fn ocie0b(&self) -> OCIE0B_R {
        OCIE0B_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter0 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie0(&mut self) -> TOIE0_W<TIMSK0_SPEC> {
        TOIE0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer/Counter0 Output Compare Match A Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie0a(&mut self) -> OCIE0A_W<TIMSK0_SPEC> {
        OCIE0A_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer/Counter0 Output Compare Match B Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie0b(&mut self) -> OCIE0B_W<TIMSK0_SPEC> {
        OCIE0B_W::new(self, 2)
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
#[doc = "Timer/Counter0 Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timsk0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timsk0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMSK0_SPEC;
impl crate::RegisterSpec for TIMSK0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`timsk0::R`](R) reader structure"]
impl crate::Readable for TIMSK0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timsk0::W`](W) writer structure"]
impl crate::Writable for TIMSK0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMSK0 to value 0"]
impl crate::Resettable for TIMSK0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
