#[doc = "Register `TIMSK3` reader"]
pub type R = crate::R<TIMSK3_SPEC>;
#[doc = "Register `TIMSK3` writer"]
pub type W = crate::W<TIMSK3_SPEC>;
#[doc = "Field `TOIE3` reader - Timer/Counter3 Overflow Interrupt Enable"]
pub type TOIE3_R = crate::BitReader;
#[doc = "Field `TOIE3` writer - Timer/Counter3 Overflow Interrupt Enable"]
pub type TOIE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIE3A` reader - Timer/Counter3 Output Compare A Match Interrupt Enable"]
pub type OCIE3A_R = crate::BitReader;
#[doc = "Field `OCIE3A` writer - Timer/Counter3 Output Compare A Match Interrupt Enable"]
pub type OCIE3A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIE3B` reader - Timer/Counter3 Output Compare B Match Interrupt Enable"]
pub type OCIE3B_R = crate::BitReader;
#[doc = "Field `OCIE3B` writer - Timer/Counter3 Output Compare B Match Interrupt Enable"]
pub type OCIE3B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIE3C` reader - Timer/Counter3 Output Compare C Match Interrupt Enable"]
pub type OCIE3C_R = crate::BitReader;
#[doc = "Field `OCIE3C` writer - Timer/Counter3 Output Compare C Match Interrupt Enable"]
pub type OCIE3C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICIE3` reader - Timer/Counter3 Input Capture Interrupt Enable"]
pub type ICIE3_R = crate::BitReader;
#[doc = "Field `ICIE3` writer - Timer/Counter3 Input Capture Interrupt Enable"]
pub type ICIE3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer/Counter3 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie3(&self) -> TOIE3_R {
        TOIE3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter3 Output Compare A Match Interrupt Enable"]
    #[inline(always)]
    pub fn ocie3a(&self) -> OCIE3A_R {
        OCIE3A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter3 Output Compare B Match Interrupt Enable"]
    #[inline(always)]
    pub fn ocie3b(&self) -> OCIE3B_R {
        OCIE3B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer/Counter3 Output Compare C Match Interrupt Enable"]
    #[inline(always)]
    pub fn ocie3c(&self) -> OCIE3C_R {
        OCIE3C_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer/Counter3 Input Capture Interrupt Enable"]
    #[inline(always)]
    pub fn icie3(&self) -> ICIE3_R {
        ICIE3_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter3 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie3(&mut self) -> TOIE3_W<TIMSK3_SPEC> {
        TOIE3_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer/Counter3 Output Compare A Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie3a(&mut self) -> OCIE3A_W<TIMSK3_SPEC> {
        OCIE3A_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer/Counter3 Output Compare B Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie3b(&mut self) -> OCIE3B_W<TIMSK3_SPEC> {
        OCIE3B_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timer/Counter3 Output Compare C Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie3c(&mut self) -> OCIE3C_W<TIMSK3_SPEC> {
        OCIE3C_W::new(self, 3)
    }
    #[doc = "Bit 5 - Timer/Counter3 Input Capture Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icie3(&mut self) -> ICIE3_W<TIMSK3_SPEC> {
        ICIE3_W::new(self, 5)
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
#[doc = "Timer/Counter3 Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timsk3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timsk3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMSK3_SPEC;
impl crate::RegisterSpec for TIMSK3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`timsk3::R`](R) reader structure"]
impl crate::Readable for TIMSK3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timsk3::W`](W) writer structure"]
impl crate::Writable for TIMSK3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMSK3 to value 0"]
impl crate::Resettable for TIMSK3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
