#[doc = "Register `MUXCTRLA` reader"]
pub type R = crate::R<MUXCTRLA_SPEC>;
#[doc = "Register `MUXCTRLA` writer"]
pub type W = crate::W<MUXCTRLA_SPEC>;
#[doc = "Field `MUXNEG` reader - Negative Input MUX Selection"]
pub type MUXNEG_R = crate::FieldReader<MUXNEG_A>;
#[doc = "Negative Input MUX Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUXNEG_A {
    #[doc = "0: Negative Pin 0"]
    PIN0 = 0,
    #[doc = "1: Negative Pin 1"]
    PIN1 = 1,
    #[doc = "2: Voltage Reference"]
    VREF = 2,
    #[doc = "3: DAC output"]
    DAC = 3,
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
    pub const fn variant(&self) -> MUXNEG_A {
        match self.bits {
            0 => MUXNEG_A::PIN0,
            1 => MUXNEG_A::PIN1,
            2 => MUXNEG_A::VREF,
            3 => MUXNEG_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Negative Pin 0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == MUXNEG_A::PIN0
    }
    #[doc = "Negative Pin 1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == MUXNEG_A::PIN1
    }
    #[doc = "Voltage Reference"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == MUXNEG_A::VREF
    }
    #[doc = "DAC output"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == MUXNEG_A::DAC
    }
}
#[doc = "Field `MUXNEG` writer - Negative Input MUX Selection"]
pub type MUXNEG_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MUXNEG_A>;
impl<'a, REG> MUXNEG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Negative Pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::PIN0)
    }
    #[doc = "Negative Pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::PIN1)
    }
    #[doc = "Voltage Reference"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::VREF)
    }
    #[doc = "DAC output"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::DAC)
    }
}
#[doc = "Field `MUXPOS` reader - Positive Input MUX Selection"]
pub type MUXPOS_R = crate::FieldReader<MUXPOS_A>;
#[doc = "Positive Input MUX Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUXPOS_A {
    #[doc = "0: Positive Pin 0"]
    PIN0 = 0,
    #[doc = "1: Positive Pin 1"]
    PIN1 = 1,
    #[doc = "2: Positive Pin 2"]
    PIN2 = 2,
    #[doc = "3: Positive Pin 3"]
    PIN3 = 3,
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
    pub const fn variant(&self) -> MUXPOS_A {
        match self.bits {
            0 => MUXPOS_A::PIN0,
            1 => MUXPOS_A::PIN1,
            2 => MUXPOS_A::PIN2,
            3 => MUXPOS_A::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Positive Pin 0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == MUXPOS_A::PIN0
    }
    #[doc = "Positive Pin 1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == MUXPOS_A::PIN1
    }
    #[doc = "Positive Pin 2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == MUXPOS_A::PIN2
    }
    #[doc = "Positive Pin 3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == MUXPOS_A::PIN3
    }
}
#[doc = "Field `MUXPOS` writer - Positive Input MUX Selection"]
pub type MUXPOS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MUXPOS_A>;
impl<'a, REG> MUXPOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Positive Pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::PIN0)
    }
    #[doc = "Positive Pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::PIN1)
    }
    #[doc = "Positive Pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::PIN2)
    }
    #[doc = "Positive Pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::PIN3)
    }
}
#[doc = "Field `INVERT` reader - Invert AC Output"]
pub type INVERT_R = crate::BitReader;
#[doc = "Field `INVERT` writer - Invert AC Output"]
pub type INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Negative Input MUX Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(self.bits & 3)
    }
    #[doc = "Bits 3:4 - Positive Input MUX Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 7 - Invert AC Output"]
    #[inline(always)]
    pub fn invert(&self) -> INVERT_R {
        INVERT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Negative Input MUX Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxneg(&mut self) -> MUXNEG_W<MUXCTRLA_SPEC> {
        MUXNEG_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Positive Input MUX Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxpos(&mut self) -> MUXPOS_W<MUXCTRLA_SPEC> {
        MUXPOS_W::new(self, 3)
    }
    #[doc = "Bit 7 - Invert AC Output"]
    #[inline(always)]
    #[must_use]
    pub fn invert(&mut self) -> INVERT_W<MUXCTRLA_SPEC> {
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
#[doc = "Mux Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXCTRLA_SPEC;
impl crate::RegisterSpec for MUXCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`muxctrla::R`](R) reader structure"]
impl crate::Readable for MUXCTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxctrla::W`](W) writer structure"]
impl crate::Writable for MUXCTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXCTRLA to value 0"]
impl crate::Resettable for MUXCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
