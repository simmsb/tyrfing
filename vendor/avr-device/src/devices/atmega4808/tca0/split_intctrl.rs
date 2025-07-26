#[doc = "Register `INTCTRL` reader"]
pub type R = crate::R<SPLIT_INTCTRL_SPEC>;
#[doc = "Register `INTCTRL` writer"]
pub type W = crate::W<SPLIT_INTCTRL_SPEC>;
#[doc = "Field `LUNF` reader - Low Underflow Interrupt Enable"]
pub type LUNF_R = crate::BitReader;
#[doc = "Field `LUNF` writer - Low Underflow Interrupt Enable"]
pub type LUNF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUNF` reader - High Underflow Interrupt Enable"]
pub type HUNF_R = crate::BitReader;
#[doc = "Field `HUNF` writer - High Underflow Interrupt Enable"]
pub type HUNF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCMP0` reader - Low Compare 0 Interrupt Enable"]
pub type LCMP0_R = crate::BitReader;
#[doc = "Field `LCMP0` writer - Low Compare 0 Interrupt Enable"]
pub type LCMP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCMP1` reader - Low Compare 1 Interrupt Enable"]
pub type LCMP1_R = crate::BitReader;
#[doc = "Field `LCMP1` writer - Low Compare 1 Interrupt Enable"]
pub type LCMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCMP2` reader - Low Compare 2 Interrupt Enable"]
pub type LCMP2_R = crate::BitReader;
#[doc = "Field `LCMP2` writer - Low Compare 2 Interrupt Enable"]
pub type LCMP2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn lunf(&self) -> LUNF_R {
        LUNF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn hunf(&self) -> HUNF_R {
        HUNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Compare 0 Interrupt Enable"]
    #[inline(always)]
    pub fn lcmp0(&self) -> LCMP0_R {
        LCMP0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Low Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub fn lcmp1(&self) -> LCMP1_R {
        LCMP1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low Compare 2 Interrupt Enable"]
    #[inline(always)]
    pub fn lcmp2(&self) -> LCMP2_R {
        LCMP2_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lunf(&mut self) -> LUNF_W<SPLIT_INTCTRL_SPEC> {
        LUNF_W::new(self, 0)
    }
    #[doc = "Bit 1 - High Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hunf(&mut self) -> HUNF_W<SPLIT_INTCTRL_SPEC> {
        HUNF_W::new(self, 1)
    }
    #[doc = "Bit 4 - Low Compare 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcmp0(&mut self) -> LCMP0_W<SPLIT_INTCTRL_SPEC> {
        LCMP0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Low Compare 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcmp1(&mut self) -> LCMP1_W<SPLIT_INTCTRL_SPEC> {
        LCMP1_W::new(self, 5)
    }
    #[doc = "Bit 6 - Low Compare 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcmp2(&mut self) -> LCMP2_W<SPLIT_INTCTRL_SPEC> {
        LCMP2_W::new(self, 6)
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
#[doc = "Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_intctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_intctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPLIT_INTCTRL_SPEC;
impl crate::RegisterSpec for SPLIT_INTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`split_intctrl::R`](R) reader structure"]
impl crate::Readable for SPLIT_INTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`split_intctrl::W`](W) writer structure"]
impl crate::Writable for SPLIT_INTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCTRL to value 0"]
impl crate::Resettable for SPLIT_INTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
