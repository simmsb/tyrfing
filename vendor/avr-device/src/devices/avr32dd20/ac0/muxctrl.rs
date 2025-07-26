#[doc = "Register `MUXCTRL` reader"]
pub type R = crate::R<MUXCTRL_SPEC>;
#[doc = "Register `MUXCTRL` writer"]
pub type W = crate::W<MUXCTRL_SPEC>;
#[doc = "Field `MUXNEG` reader - Negative Input MUX Selection"]
pub type MUXNEG_R = crate::FieldReader<MUXNEG_A>;
#[doc = "Negative Input MUX Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUXNEG_A {
    #[doc = "0: Negative Pin 0"]
    AINN0 = 0,
    #[doc = "2: Negative Pin 2"]
    AINN2 = 2,
    #[doc = "3: Negative Pin 3"]
    AINN3 = 3,
    #[doc = "4: DAC Reference"]
    DACREF = 4,
}
impl From<MUXNEG_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXNEG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MUXNEG_A {
    type Ux = u8;
}
impl MUXNEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MUXNEG_A> {
        match self.bits {
            0 => Some(MUXNEG_A::AINN0),
            2 => Some(MUXNEG_A::AINN2),
            3 => Some(MUXNEG_A::AINN3),
            4 => Some(MUXNEG_A::DACREF),
            _ => None,
        }
    }
    #[doc = "Negative Pin 0"]
    #[inline(always)]
    pub fn is_ainn0(&self) -> bool {
        *self == MUXNEG_A::AINN0
    }
    #[doc = "Negative Pin 2"]
    #[inline(always)]
    pub fn is_ainn2(&self) -> bool {
        *self == MUXNEG_A::AINN2
    }
    #[doc = "Negative Pin 3"]
    #[inline(always)]
    pub fn is_ainn3(&self) -> bool {
        *self == MUXNEG_A::AINN3
    }
    #[doc = "DAC Reference"]
    #[inline(always)]
    pub fn is_dacref(&self) -> bool {
        *self == MUXNEG_A::DACREF
    }
}
#[doc = "Field `MUXNEG` writer - Negative Input MUX Selection"]
pub type MUXNEG_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MUXNEG_A>;
impl<'a, REG> MUXNEG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Negative Pin 0"]
    #[inline(always)]
    pub fn ainn0(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AINN0)
    }
    #[doc = "Negative Pin 2"]
    #[inline(always)]
    pub fn ainn2(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AINN2)
    }
    #[doc = "Negative Pin 3"]
    #[inline(always)]
    pub fn ainn3(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AINN3)
    }
    #[doc = "DAC Reference"]
    #[inline(always)]
    pub fn dacref(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::DACREF)
    }
}
#[doc = "Field `MUXPOS` reader - Positive Input MUX Selection"]
pub type MUXPOS_R = crate::FieldReader<MUXPOS_A>;
#[doc = "Positive Input MUX Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUXPOS_A {
    #[doc = "0: Positive Pin 0"]
    AINP0 = 0,
    #[doc = "3: Positive Pin 3"]
    AINP3 = 3,
    #[doc = "4: Positive Pin 4"]
    AINP4 = 4,
}
impl From<MUXPOS_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXPOS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MUXPOS_A {
    type Ux = u8;
}
impl MUXPOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MUXPOS_A> {
        match self.bits {
            0 => Some(MUXPOS_A::AINP0),
            3 => Some(MUXPOS_A::AINP3),
            4 => Some(MUXPOS_A::AINP4),
            _ => None,
        }
    }
    #[doc = "Positive Pin 0"]
    #[inline(always)]
    pub fn is_ainp0(&self) -> bool {
        *self == MUXPOS_A::AINP0
    }
    #[doc = "Positive Pin 3"]
    #[inline(always)]
    pub fn is_ainp3(&self) -> bool {
        *self == MUXPOS_A::AINP3
    }
    #[doc = "Positive Pin 4"]
    #[inline(always)]
    pub fn is_ainp4(&self) -> bool {
        *self == MUXPOS_A::AINP4
    }
}
#[doc = "Field `MUXPOS` writer - Positive Input MUX Selection"]
pub type MUXPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MUXPOS_A>;
impl<'a, REG> MUXPOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Positive Pin 0"]
    #[inline(always)]
    pub fn ainp0(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AINP0)
    }
    #[doc = "Positive Pin 3"]
    #[inline(always)]
    pub fn ainp3(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AINP3)
    }
    #[doc = "Positive Pin 4"]
    #[inline(always)]
    pub fn ainp4(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AINP4)
    }
}
#[doc = "Field `INITVAL` reader - AC Output Initial Value"]
pub type INITVAL_R = crate::BitReader<INITVAL_A>;
#[doc = "AC Output Initial Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITVAL_A {
    #[doc = "0: Output initialized to 0"]
    LOW = 0,
    #[doc = "1: Output initialized to 1"]
    HIGH = 1,
}
impl From<INITVAL_A> for bool {
    #[inline(always)]
    fn from(variant: INITVAL_A) -> Self {
        variant as u8 != 0
    }
}
impl INITVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INITVAL_A {
        match self.bits {
            false => INITVAL_A::LOW,
            true => INITVAL_A::HIGH,
        }
    }
    #[doc = "Output initialized to 0"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == INITVAL_A::LOW
    }
    #[doc = "Output initialized to 1"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == INITVAL_A::HIGH
    }
}
#[doc = "Field `INITVAL` writer - AC Output Initial Value"]
pub type INITVAL_W<'a, REG> = crate::BitWriter<'a, REG, INITVAL_A>;
impl<'a, REG> INITVAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output initialized to 0"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(INITVAL_A::LOW)
    }
    #[doc = "Output initialized to 1"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(INITVAL_A::HIGH)
    }
}
#[doc = "Field `INVERT` reader - Invert AC Output"]
pub type INVERT_R = crate::BitReader;
#[doc = "Field `INVERT` writer - Invert AC Output"]
pub type INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Negative Input MUX Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:5 - Positive Input MUX Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new((self.bits >> 3) & 7)
    }
    #[doc = "Bit 6 - AC Output Initial Value"]
    #[inline(always)]
    pub fn initval(&self) -> INITVAL_R {
        INITVAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Invert AC Output"]
    #[inline(always)]
    pub fn invert(&self) -> INVERT_R {
        INVERT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Negative Input MUX Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxneg(&mut self) -> MUXNEG_W<MUXCTRL_SPEC> {
        MUXNEG_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Positive Input MUX Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxpos(&mut self) -> MUXPOS_W<MUXCTRL_SPEC> {
        MUXPOS_W::new(self, 3)
    }
    #[doc = "Bit 6 - AC Output Initial Value"]
    #[inline(always)]
    #[must_use]
    pub fn initval(&mut self) -> INITVAL_W<MUXCTRL_SPEC> {
        INITVAL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Invert AC Output"]
    #[inline(always)]
    #[must_use]
    pub fn invert(&mut self) -> INVERT_W<MUXCTRL_SPEC> {
        INVERT_W::new(self, 7)
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
#[doc = "Mux Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXCTRL_SPEC;
impl crate::RegisterSpec for MUXCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`muxctrl::R`](R) reader structure"]
impl crate::Readable for MUXCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxctrl::W`](W) writer structure"]
impl crate::Writable for MUXCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXCTRL to value 0"]
impl crate::Resettable for MUXCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
