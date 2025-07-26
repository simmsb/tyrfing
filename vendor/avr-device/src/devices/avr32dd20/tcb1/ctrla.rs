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
    #[doc = "0: CLK_PER"]
    DIV1 = 0,
    #[doc = "1: CLK_PER/2"]
    DIV2 = 1,
    #[doc = "2: Use CLK_TCA from TCA0"]
    TCA0 = 2,
    #[doc = "7: Count on event edge"]
    EVENT = 7,
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
            0 => Some(CLKSEL_A::DIV1),
            1 => Some(CLKSEL_A::DIV2),
            2 => Some(CLKSEL_A::TCA0),
            7 => Some(CLKSEL_A::EVENT),
            _ => None,
        }
    }
    #[doc = "CLK_PER"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CLKSEL_A::DIV1
    }
    #[doc = "CLK_PER/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CLKSEL_A::DIV2
    }
    #[doc = "Use CLK_TCA from TCA0"]
    #[inline(always)]
    pub fn is_tca0(&self) -> bool {
        *self == CLKSEL_A::TCA0
    }
    #[doc = "Count on event edge"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CLKSEL_A::EVENT
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type CLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CLKSEL_A>;
impl<'a, REG> CLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_PER"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::DIV1)
    }
    #[doc = "CLK_PER/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::DIV2)
    }
    #[doc = "Use CLK_TCA from TCA0"]
    #[inline(always)]
    pub fn tca0(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::TCA0)
    }
    #[doc = "Count on event edge"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::EVENT)
    }
}
#[doc = "Field `SYNCUPD` reader - Synchronize Update"]
pub type SYNCUPD_R = crate::BitReader;
#[doc = "Field `SYNCUPD` writer - Synchronize Update"]
pub type SYNCUPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CASCADE` reader - Cascade two timers"]
pub type CASCADE_R = crate::BitReader;
#[doc = "Field `CASCADE` writer - Cascade two timers"]
pub type CASCADE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bits 1:3 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits >> 1) & 7)
    }
    #[doc = "Bit 4 - Synchronize Update"]
    #[inline(always)]
    pub fn syncupd(&self) -> SYNCUPD_R {
        SYNCUPD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Cascade two timers"]
    #[inline(always)]
    pub fn cascade(&self) -> CASCADE_R {
        CASCADE_R::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bits 1:3 - Clock Select"]
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
    #[doc = "Bit 5 - Cascade two timers"]
    #[inline(always)]
    #[must_use]
    pub fn cascade(&mut self) -> CASCADE_W<CTRLA_SPEC> {
        CASCADE_W::new(self, 5)
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
