#[doc = "Register `TIMSK` reader"]
pub type R = crate::R<TIMSK_SPEC>;
#[doc = "Register `TIMSK` writer"]
pub type W = crate::W<TIMSK_SPEC>;
#[doc = "Field `TOIE0` reader - Timer/Counter0 Overflow Interrupt Enable"]
pub type TOIE0_R = crate::BitReader;
#[doc = "Field `TOIE0` writer - Timer/Counter0 Overflow Interrupt Enable"]
pub type TOIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer/Counter0 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie0(&self) -> TOIE0_R {
        TOIE0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter0 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie0(&mut self) -> TOIE0_W<TIMSK_SPEC> {
        TOIE0_W::new(self, 0)
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
