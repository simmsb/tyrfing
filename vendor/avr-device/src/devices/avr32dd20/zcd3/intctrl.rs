#[doc = "Register `INTCTRL` reader"]
pub type R = crate::R<INTCTRL_SPEC>;
#[doc = "Register `INTCTRL` writer"]
pub type W = crate::W<INTCTRL_SPEC>;
#[doc = "Field `INTMODE` reader - Interrupt Mode"]
pub type INTMODE_R = crate::FieldReader<INTMODE_A>;
#[doc = "Interrupt Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTMODE_A {
    #[doc = "0: No interrupt"]
    NONE = 0,
    #[doc = "1: Interrupt on rising input signal"]
    RISING = 1,
    #[doc = "2: Interrupt on falling input signal"]
    FALLING = 2,
    #[doc = "3: Interrupt on both rising and falling input signal"]
    BOTH = 3,
}
impl From<INTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: INTMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INTMODE_A {
    type Ux = u8;
}
impl INTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTMODE_A {
        match self.bits {
            0 => INTMODE_A::NONE,
            1 => INTMODE_A::RISING,
            2 => INTMODE_A::FALLING,
            3 => INTMODE_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == INTMODE_A::NONE
    }
    #[doc = "Interrupt on rising input signal"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == INTMODE_A::RISING
    }
    #[doc = "Interrupt on falling input signal"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == INTMODE_A::FALLING
    }
    #[doc = "Interrupt on both rising and falling input signal"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == INTMODE_A::BOTH
    }
}
#[doc = "Field `INTMODE` writer - Interrupt Mode"]
pub type INTMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, INTMODE_A>;
impl<'a, REG> INTMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE_A::NONE)
    }
    #[doc = "Interrupt on rising input signal"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE_A::RISING)
    }
    #[doc = "Interrupt on falling input signal"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE_A::FALLING)
    }
    #[doc = "Interrupt on both rising and falling input signal"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE_A::BOTH)
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt Mode"]
    #[inline(always)]
    pub fn intmode(&self) -> INTMODE_R {
        INTMODE_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interrupt Mode"]
    #[inline(always)]
    #[must_use]
    pub fn intmode(&mut self) -> INTMODE_W<INTCTRL_SPEC> {
        INTMODE_W::new(self, 0)
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
#[doc = "Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTCTRL_SPEC;
impl crate::RegisterSpec for INTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intctrl::R`](R) reader structure"]
impl crate::Readable for INTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intctrl::W`](W) writer structure"]
impl crate::Writable for INTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCTRL to value 0"]
impl crate::Resettable for INTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
