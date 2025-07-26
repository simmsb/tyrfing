#[doc = "Register `ETIMSK` reader"]
pub type R = crate::R<ETIMSK_SPEC>;
#[doc = "Register `ETIMSK` writer"]
pub type W = crate::W<ETIMSK_SPEC>;
#[doc = "Field `OCIE3C` reader - Timer/Counter3, Output Compare Match Interrupt Enable"]
pub type OCIE3C_R = crate::BitReader;
#[doc = "Field `OCIE3C` writer - Timer/Counter3, Output Compare Match Interrupt Enable"]
pub type OCIE3C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOIE3` reader - Timer/Counter3 Overflow Interrupt Enable"]
pub type TOIE3_R = crate::BitReader;
#[doc = "Field `TOIE3` writer - Timer/Counter3 Overflow Interrupt Enable"]
pub type TOIE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIE3B` reader - Timer/Counter3 Output CompareB Match Interrupt Enable"]
pub type OCIE3B_R = crate::BitReader;
#[doc = "Field `OCIE3B` writer - Timer/Counter3 Output CompareB Match Interrupt Enable"]
pub type OCIE3B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIE3A` reader - Timer/Counter3 Output CompareA Match Interrupt Enable"]
pub type OCIE3A_R = crate::BitReader;
#[doc = "Field `OCIE3A` writer - Timer/Counter3 Output CompareA Match Interrupt Enable"]
pub type OCIE3A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TICIE3` reader - Timer/Counter3 Input Capture Interrupt Enable"]
pub type TICIE3_R = crate::BitReader;
#[doc = "Field `TICIE3` writer - Timer/Counter3 Input Capture Interrupt Enable"]
pub type TICIE3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Timer/Counter3, Output Compare Match Interrupt Enable"]
    #[inline(always)]
    pub fn ocie3c(&self) -> OCIE3C_R {
        OCIE3C_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter3 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie3(&self) -> TOIE3_R {
        TOIE3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer/Counter3 Output CompareB Match Interrupt Enable"]
    #[inline(always)]
    pub fn ocie3b(&self) -> OCIE3B_R {
        OCIE3B_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer/Counter3 Output CompareA Match Interrupt Enable"]
    #[inline(always)]
    pub fn ocie3a(&self) -> OCIE3A_R {
        OCIE3A_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer/Counter3 Input Capture Interrupt Enable"]
    #[inline(always)]
    pub fn ticie3(&self) -> TICIE3_R {
        TICIE3_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Timer/Counter3, Output Compare Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie3c(&mut self) -> OCIE3C_W<ETIMSK_SPEC> {
        OCIE3C_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer/Counter3 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie3(&mut self) -> TOIE3_W<ETIMSK_SPEC> {
        TOIE3_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timer/Counter3 Output CompareB Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie3b(&mut self) -> OCIE3B_W<ETIMSK_SPEC> {
        OCIE3B_W::new(self, 3)
    }
    #[doc = "Bit 4 - Timer/Counter3 Output CompareA Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie3a(&mut self) -> OCIE3A_W<ETIMSK_SPEC> {
        OCIE3A_W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer/Counter3 Input Capture Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ticie3(&mut self) -> TICIE3_W<ETIMSK_SPEC> {
        TICIE3_W::new(self, 5)
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
#[doc = "Extended Timer/Counter Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etimsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etimsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETIMSK_SPEC;
impl crate::RegisterSpec for ETIMSK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`etimsk::R`](R) reader structure"]
impl crate::Readable for ETIMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etimsk::W`](W) writer structure"]
impl crate::Writable for ETIMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETIMSK to value 0"]
impl crate::Resettable for ETIMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
