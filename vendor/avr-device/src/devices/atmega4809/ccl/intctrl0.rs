#[doc = "Register `INTCTRL0` reader"]
pub type R = crate::R<INTCTRL0_SPEC>;
#[doc = "Register `INTCTRL0` writer"]
pub type W = crate::W<INTCTRL0_SPEC>;
#[doc = "Field `INTMODE0` reader - Interrupt Mode for LUT0"]
pub type INTMODE0_R = crate::FieldReader<INTMODE0_A>;
#[doc = "Interrupt Mode for LUT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTMODE0_A {
    #[doc = "0: Interrupt disabled"]
    INTDISABLE = 0,
    #[doc = "1: Sense rising edge"]
    RISING = 1,
    #[doc = "2: Sense falling edge"]
    FALLING = 2,
    #[doc = "3: Sense both edges"]
    BOTH = 3,
}
impl From<INTMODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: INTMODE0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INTMODE0_A {
    type Ux = u8;
}
impl INTMODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTMODE0_A {
        match self.bits {
            0 => INTMODE0_A::INTDISABLE,
            1 => INTMODE0_A::RISING,
            2 => INTMODE0_A::FALLING,
            3 => INTMODE0_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_intdisable(&self) -> bool {
        *self == INTMODE0_A::INTDISABLE
    }
    #[doc = "Sense rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == INTMODE0_A::RISING
    }
    #[doc = "Sense falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == INTMODE0_A::FALLING
    }
    #[doc = "Sense both edges"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == INTMODE0_A::BOTH
    }
}
#[doc = "Field `INTMODE0` writer - Interrupt Mode for LUT0"]
pub type INTMODE0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, INTMODE0_A>;
impl<'a, REG> INTMODE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn intdisable(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE0_A::INTDISABLE)
    }
    #[doc = "Sense rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE0_A::RISING)
    }
    #[doc = "Sense falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE0_A::FALLING)
    }
    #[doc = "Sense both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE0_A::BOTH)
    }
}
#[doc = "Field `INTMODE1` reader - Interrupt Mode for LUT1"]
pub type INTMODE1_R = crate::FieldReader<INTMODE1_A>;
#[doc = "Interrupt Mode for LUT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTMODE1_A {
    #[doc = "0: Interrupt disabled"]
    INTDISABLE = 0,
    #[doc = "1: Sense rising edge"]
    RISING = 1,
    #[doc = "2: Sense falling edge"]
    FALLING = 2,
    #[doc = "3: Sense both edges"]
    BOTH = 3,
}
impl From<INTMODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: INTMODE1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INTMODE1_A {
    type Ux = u8;
}
impl INTMODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTMODE1_A {
        match self.bits {
            0 => INTMODE1_A::INTDISABLE,
            1 => INTMODE1_A::RISING,
            2 => INTMODE1_A::FALLING,
            3 => INTMODE1_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_intdisable(&self) -> bool {
        *self == INTMODE1_A::INTDISABLE
    }
    #[doc = "Sense rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == INTMODE1_A::RISING
    }
    #[doc = "Sense falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == INTMODE1_A::FALLING
    }
    #[doc = "Sense both edges"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == INTMODE1_A::BOTH
    }
}
#[doc = "Field `INTMODE1` writer - Interrupt Mode for LUT1"]
pub type INTMODE1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, INTMODE1_A>;
impl<'a, REG> INTMODE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn intdisable(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE1_A::INTDISABLE)
    }
    #[doc = "Sense rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE1_A::RISING)
    }
    #[doc = "Sense falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE1_A::FALLING)
    }
    #[doc = "Sense both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE1_A::BOTH)
    }
}
#[doc = "Field `INTMODE2` reader - Interrupt Mode for LUT2"]
pub type INTMODE2_R = crate::FieldReader<INTMODE2_A>;
#[doc = "Interrupt Mode for LUT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTMODE2_A {
    #[doc = "0: Interrupt disabled"]
    INTDISABLE = 0,
    #[doc = "1: Sense rising edge"]
    RISING = 1,
    #[doc = "2: Sense falling edge"]
    FALLING = 2,
    #[doc = "3: Sense both edges"]
    BOTH = 3,
}
impl From<INTMODE2_A> for u8 {
    #[inline(always)]
    fn from(variant: INTMODE2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INTMODE2_A {
    type Ux = u8;
}
impl INTMODE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTMODE2_A {
        match self.bits {
            0 => INTMODE2_A::INTDISABLE,
            1 => INTMODE2_A::RISING,
            2 => INTMODE2_A::FALLING,
            3 => INTMODE2_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_intdisable(&self) -> bool {
        *self == INTMODE2_A::INTDISABLE
    }
    #[doc = "Sense rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == INTMODE2_A::RISING
    }
    #[doc = "Sense falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == INTMODE2_A::FALLING
    }
    #[doc = "Sense both edges"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == INTMODE2_A::BOTH
    }
}
#[doc = "Field `INTMODE2` writer - Interrupt Mode for LUT2"]
pub type INTMODE2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, INTMODE2_A>;
impl<'a, REG> INTMODE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn intdisable(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE2_A::INTDISABLE)
    }
    #[doc = "Sense rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE2_A::RISING)
    }
    #[doc = "Sense falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE2_A::FALLING)
    }
    #[doc = "Sense both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE2_A::BOTH)
    }
}
#[doc = "Field `INTMODE3` reader - Interrupt Mode for LUT3"]
pub type INTMODE3_R = crate::FieldReader<INTMODE3_A>;
#[doc = "Interrupt Mode for LUT3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTMODE3_A {
    #[doc = "0: Interrupt disabled"]
    INTDISABLE = 0,
    #[doc = "1: Sense rising edge"]
    RISING = 1,
    #[doc = "2: Sense falling edge"]
    FALLING = 2,
    #[doc = "3: Sense both edges"]
    BOTH = 3,
}
impl From<INTMODE3_A> for u8 {
    #[inline(always)]
    fn from(variant: INTMODE3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INTMODE3_A {
    type Ux = u8;
}
impl INTMODE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTMODE3_A {
        match self.bits {
            0 => INTMODE3_A::INTDISABLE,
            1 => INTMODE3_A::RISING,
            2 => INTMODE3_A::FALLING,
            3 => INTMODE3_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_intdisable(&self) -> bool {
        *self == INTMODE3_A::INTDISABLE
    }
    #[doc = "Sense rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == INTMODE3_A::RISING
    }
    #[doc = "Sense falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == INTMODE3_A::FALLING
    }
    #[doc = "Sense both edges"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == INTMODE3_A::BOTH
    }
}
#[doc = "Field `INTMODE3` writer - Interrupt Mode for LUT3"]
pub type INTMODE3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, INTMODE3_A>;
impl<'a, REG> INTMODE3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn intdisable(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE3_A::INTDISABLE)
    }
    #[doc = "Sense rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE3_A::RISING)
    }
    #[doc = "Sense falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE3_A::FALLING)
    }
    #[doc = "Sense both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE3_A::BOTH)
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt Mode for LUT0"]
    #[inline(always)]
    pub fn intmode0(&self) -> INTMODE0_R {
        INTMODE0_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Interrupt Mode for LUT1"]
    #[inline(always)]
    pub fn intmode1(&self) -> INTMODE1_R {
        INTMODE1_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Interrupt Mode for LUT2"]
    #[inline(always)]
    pub fn intmode2(&self) -> INTMODE2_R {
        INTMODE2_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Interrupt Mode for LUT3"]
    #[inline(always)]
    pub fn intmode3(&self) -> INTMODE3_R {
        INTMODE3_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interrupt Mode for LUT0"]
    #[inline(always)]
    #[must_use]
    pub fn intmode0(&mut self) -> INTMODE0_W<INTCTRL0_SPEC> {
        INTMODE0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Interrupt Mode for LUT1"]
    #[inline(always)]
    #[must_use]
    pub fn intmode1(&mut self) -> INTMODE1_W<INTCTRL0_SPEC> {
        INTMODE1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Interrupt Mode for LUT2"]
    #[inline(always)]
    #[must_use]
    pub fn intmode2(&mut self) -> INTMODE2_W<INTCTRL0_SPEC> {
        INTMODE2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Interrupt Mode for LUT3"]
    #[inline(always)]
    #[must_use]
    pub fn intmode3(&mut self) -> INTMODE3_W<INTCTRL0_SPEC> {
        INTMODE3_W::new(self, 6)
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
#[doc = "Interrupt Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTCTRL0_SPEC;
impl crate::RegisterSpec for INTCTRL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intctrl0::R`](R) reader structure"]
impl crate::Readable for INTCTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intctrl0::W`](W) writer structure"]
impl crate::Writable for INTCTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCTRL0 to value 0"]
impl crate::Resettable for INTCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
