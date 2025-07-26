#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSEL` reader - Clock Select"]
pub type CLKSEL_R = crate::FieldReader<CLKSEL_A>;
#[doc = "Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: CLK_PER (No Prescaling)"]
    CLKDIV1 = 0,
    #[doc = "1: CLK_PER/2 (From Prescaler)"]
    CLKDIV2 = 1,
    #[doc = "2: Use Clock from TCA"]
    CLKTCA = 2,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKSEL_A {
    type Ux = u8;
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::CLKDIV1),
            1 => Some(CLKSEL_A::CLKDIV2),
            2 => Some(CLKSEL_A::CLKTCA),
            _ => None,
        }
    }
    #[doc = "CLK_PER (No Prescaling)"]
    #[inline(always)]
    pub fn is_clkdiv1(&self) -> bool {
        *self == CLKSEL_A::CLKDIV1
    }
    #[doc = "CLK_PER/2 (From Prescaler)"]
    #[inline(always)]
    pub fn is_clkdiv2(&self) -> bool {
        *self == CLKSEL_A::CLKDIV2
    }
    #[doc = "Use Clock from TCA"]
    #[inline(always)]
    pub fn is_clktca(&self) -> bool {
        *self == CLKSEL_A::CLKTCA
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type CLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CLKSEL_A>;
impl<'a, REG> CLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_PER (No Prescaling)"]
    #[inline(always)]
    pub fn clkdiv1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::CLKDIV1)
    }
    #[doc = "CLK_PER/2 (From Prescaler)"]
    #[inline(always)]
    pub fn clkdiv2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::CLKDIV2)
    }
    #[doc = "Use Clock from TCA"]
    #[inline(always)]
    pub fn clktca(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::CLKTCA)
    }
}
#[doc = "Field `SYNCUPD` reader - Synchronize Update"]
pub type SYNCUPD_R = crate::BitReader;
#[doc = "Field `SYNCUPD` writer - Synchronize Update"]
pub type SYNCUPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run Standby"]
pub type RUNSTDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 4 - Synchronize Update"]
    #[inline(always)]
    pub fn syncupd(&self) -> SYNCUPD_R {
        SYNCUPD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Run Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRLA_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<CTRLA_SPEC> {
        CLKSEL_W::new(self, 1)
    }
    #[doc = "Bit 4 - Synchronize Update"]
    #[inline(always)]
    #[must_use]
    pub fn syncupd(&mut self) -> SYNCUPD_W<CTRLA_SPEC> {
        SYNCUPD_W::new(self, 4)
    }
    #[doc = "Bit 6 - Run Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<CTRLA_SPEC> {
        RUNSTDBY_W::new(self, 6)
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
#[doc = "Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
