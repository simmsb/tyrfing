#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `ENABLE` reader - ADC Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - ADC Enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREERUN` reader - ADC Freerun mode"]
pub type FREERUN_R = crate::BitReader;
#[doc = "Field `FREERUN` writer - ADC Freerun mode"]
pub type FREERUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESSEL` reader - ADC Resolution"]
pub type RESSEL_R = crate::BitReader<RESSEL_A>;
#[doc = "ADC Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESSEL_A {
    #[doc = "0: 10-bit mode"]
    _10BIT = 0,
    #[doc = "1: 8-bit mode"]
    _8BIT = 1,
}
impl From<RESSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RESSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl RESSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESSEL_A {
        match self.bits {
            false => RESSEL_A::_10BIT,
            true => RESSEL_A::_8BIT,
        }
    }
    #[doc = "10-bit mode"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == RESSEL_A::_10BIT
    }
    #[doc = "8-bit mode"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == RESSEL_A::_8BIT
    }
}
#[doc = "Field `RESSEL` writer - ADC Resolution"]
pub type RESSEL_W<'a, REG> = crate::BitWriter<'a, REG, RESSEL_A>;
impl<'a, REG> RESSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "10-bit mode"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(RESSEL_A::_10BIT)
    }
    #[doc = "8-bit mode"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(RESSEL_A::_8BIT)
    }
}
#[doc = "Field `RUNSTBY` reader - Run standby mode"]
pub type RUNSTBY_R = crate::BitReader;
#[doc = "Field `RUNSTBY` writer - Run standby mode"]
pub type RUNSTBY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Freerun mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FREERUN_R {
        FREERUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Resolution"]
    #[inline(always)]
    pub fn ressel(&self) -> RESSEL_R {
        RESSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Run standby mode"]
    #[inline(always)]
    pub fn runstby(&self) -> RUNSTBY_R {
        RUNSTBY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRLA_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC Freerun mode"]
    #[inline(always)]
    #[must_use]
    pub fn freerun(&mut self) -> FREERUN_W<CTRLA_SPEC> {
        FREERUN_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn ressel(&mut self) -> RESSEL_W<CTRLA_SPEC> {
        RESSEL_W::new(self, 2)
    }
    #[doc = "Bit 7 - Run standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn runstby(&mut self) -> RUNSTBY_W<CTRLA_SPEC> {
        RUNSTBY_W::new(self, 7)
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
