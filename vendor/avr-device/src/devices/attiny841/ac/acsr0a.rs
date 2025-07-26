#[doc = "Register `ACSR0A` reader"]
pub type R = crate::R<ACSR0A_SPEC>;
#[doc = "Register `ACSR0A` writer"]
pub type W = crate::W<ACSR0A_SPEC>;
#[doc = "Field `ACIS0` reader - Analog Comparator Interrupt Mode Select"]
pub type ACIS0_R = crate::FieldReader<ACIS0_A>;
#[doc = "Analog Comparator Interrupt Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACIS0_A {
    #[doc = "0: Interrupt on Toggle"]
    ON_TOGGLE = 0,
    #[doc = "2: Interrupt on Falling Edge"]
    ON_FALLING_EDGE = 2,
    #[doc = "3: Interrupt on Rising Edge"]
    ON_RISING_EDGE = 3,
}
impl From<ACIS0_A> for u8 {
    #[inline(always)]
    fn from(variant: ACIS0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACIS0_A {
    type Ux = u8;
}
impl ACIS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ACIS0_A> {
        match self.bits {
            0 => Some(ACIS0_A::ON_TOGGLE),
            2 => Some(ACIS0_A::ON_FALLING_EDGE),
            3 => Some(ACIS0_A::ON_RISING_EDGE),
            _ => None,
        }
    }
    #[doc = "Interrupt on Toggle"]
    #[inline(always)]
    pub fn is_on_toggle(&self) -> bool {
        *self == ACIS0_A::ON_TOGGLE
    }
    #[doc = "Interrupt on Falling Edge"]
    #[inline(always)]
    pub fn is_on_falling_edge(&self) -> bool {
        *self == ACIS0_A::ON_FALLING_EDGE
    }
    #[doc = "Interrupt on Rising Edge"]
    #[inline(always)]
    pub fn is_on_rising_edge(&self) -> bool {
        *self == ACIS0_A::ON_RISING_EDGE
    }
}
#[doc = "Field `ACIS0` writer - Analog Comparator Interrupt Mode Select"]
pub type ACIS0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ACIS0_A>;
impl<'a, REG> ACIS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt on Toggle"]
    #[inline(always)]
    pub fn on_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(ACIS0_A::ON_TOGGLE)
    }
    #[doc = "Interrupt on Falling Edge"]
    #[inline(always)]
    pub fn on_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(ACIS0_A::ON_FALLING_EDGE)
    }
    #[doc = "Interrupt on Rising Edge"]
    #[inline(always)]
    pub fn on_rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(ACIS0_A::ON_RISING_EDGE)
    }
}
#[doc = "Field `ACIC0` reader - Analog Comparator 0 Input Capture Enable"]
pub type ACIC0_R = crate::BitReader;
#[doc = "Field `ACIC0` writer - Analog Comparator 0 Input Capture Enable"]
pub type ACIC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACIE0` reader - Analog Comparator 0 Interrupt Enable"]
pub type ACIE0_R = crate::BitReader;
#[doc = "Field `ACIE0` writer - Analog Comparator 0 Interrupt Enable"]
pub type ACIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACI0` reader - Analog Comparator 0 Interrupt Flag"]
pub type ACI0_R = crate::BitReader;
#[doc = "Field `ACI0` writer - Analog Comparator 0 Interrupt Flag"]
pub type ACI0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACO0` reader - Analog Comparator 0 Output"]
pub type ACO0_R = crate::BitReader;
#[doc = "Field `ACO0` writer - Analog Comparator 0 Output"]
pub type ACO0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACPMUX2` reader - Analog Comparator 0 Positive Input Multiplexer Bit 2"]
pub type ACPMUX2_R = crate::BitReader;
#[doc = "Field `ACPMUX2` writer - Analog Comparator 0 Positive Input Multiplexer Bit 2"]
pub type ACPMUX2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACD0` reader - Analog Comparator 0 Disable"]
pub type ACD0_R = crate::BitReader;
#[doc = "Field `ACD0` writer - Analog Comparator 0 Disable"]
pub type ACD0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Analog Comparator Interrupt Mode Select"]
    #[inline(always)]
    pub fn acis0(&self) -> ACIS0_R {
        ACIS0_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Analog Comparator 0 Input Capture Enable"]
    #[inline(always)]
    pub fn acic0(&self) -> ACIC0_R {
        ACIC0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog Comparator 0 Interrupt Enable"]
    #[inline(always)]
    pub fn acie0(&self) -> ACIE0_R {
        ACIE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog Comparator 0 Interrupt Flag"]
    #[inline(always)]
    pub fn aci0(&self) -> ACI0_R {
        ACI0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog Comparator 0 Output"]
    #[inline(always)]
    pub fn aco0(&self) -> ACO0_R {
        ACO0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog Comparator 0 Positive Input Multiplexer Bit 2"]
    #[inline(always)]
    pub fn acpmux2(&self) -> ACPMUX2_R {
        ACPMUX2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator 0 Disable"]
    #[inline(always)]
    pub fn acd0(&self) -> ACD0_R {
        ACD0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog Comparator Interrupt Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn acis0(&mut self) -> ACIS0_W<ACSR0A_SPEC> {
        ACIS0_W::new(self, 0)
    }
    #[doc = "Bit 2 - Analog Comparator 0 Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acic0(&mut self) -> ACIC0_W<ACSR0A_SPEC> {
        ACIC0_W::new(self, 2)
    }
    #[doc = "Bit 3 - Analog Comparator 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acie0(&mut self) -> ACIE0_W<ACSR0A_SPEC> {
        ACIE0_W::new(self, 3)
    }
    #[doc = "Bit 4 - Analog Comparator 0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aci0(&mut self) -> ACI0_W<ACSR0A_SPEC> {
        ACI0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Analog Comparator 0 Output"]
    #[inline(always)]
    #[must_use]
    pub fn aco0(&mut self) -> ACO0_W<ACSR0A_SPEC> {
        ACO0_W::new(self, 5)
    }
    #[doc = "Bit 6 - Analog Comparator 0 Positive Input Multiplexer Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn acpmux2(&mut self) -> ACPMUX2_W<ACSR0A_SPEC> {
        ACPMUX2_W::new(self, 6)
    }
    #[doc = "Bit 7 - Analog Comparator 0 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn acd0(&mut self) -> ACD0_W<ACSR0A_SPEC> {
        ACD0_W::new(self, 7)
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
#[doc = "Analog Comparator 0 Control And Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsr0a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsr0a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACSR0A_SPEC;
impl crate::RegisterSpec for ACSR0A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`acsr0a::R`](R) reader structure"]
impl crate::Readable for ACSR0A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acsr0a::W`](W) writer structure"]
impl crate::Writable for ACSR0A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACSR0A to value 0"]
impl crate::Resettable for ACSR0A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
