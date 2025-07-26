#[doc = "Register `EICRA` reader"]
pub type R = crate::R<EICRA_SPEC>;
#[doc = "Register `EICRA` writer"]
pub type W = crate::W<EICRA_SPEC>;
#[doc = "Field `ISC0` reader - External Interrupt Sense Control 0 Bits"]
pub type ISC0_R = crate::FieldReader<ISC0_A>;
#[doc = "External Interrupt Sense Control 0 Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISC0_A {
    #[doc = "0: The low level of INTx generates an interrupt request"]
    LOW = 0,
    #[doc = "1: Any logical change on INTx generates an interrupt request"]
    TOGGLE = 1,
    #[doc = "2: The falling edge of INTx generates an interrupt request"]
    FALLING = 2,
    #[doc = "3: The rising edge of INTx generates an interrupt request"]
    RISING = 3,
}
impl From<ISC0_A> for u8 {
    #[inline(always)]
    fn from(variant: ISC0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ISC0_A {
    type Ux = u8;
}
impl ISC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ISC0_A {
        match self.bits {
            0 => ISC0_A::LOW,
            1 => ISC0_A::TOGGLE,
            2 => ISC0_A::FALLING,
            3 => ISC0_A::RISING,
            _ => unreachable!(),
        }
    }
    #[doc = "The low level of INTx generates an interrupt request"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ISC0_A::LOW
    }
    #[doc = "Any logical change on INTx generates an interrupt request"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == ISC0_A::TOGGLE
    }
    #[doc = "The falling edge of INTx generates an interrupt request"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ISC0_A::FALLING
    }
    #[doc = "The rising edge of INTx generates an interrupt request"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ISC0_A::RISING
    }
}
#[doc = "Field `ISC0` writer - External Interrupt Sense Control 0 Bits"]
pub type ISC0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ISC0_A>;
impl<'a, REG> ISC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The low level of INTx generates an interrupt request"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(ISC0_A::LOW)
    }
    #[doc = "Any logical change on INTx generates an interrupt request"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(ISC0_A::TOGGLE)
    }
    #[doc = "The falling edge of INTx generates an interrupt request"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(ISC0_A::FALLING)
    }
    #[doc = "The rising edge of INTx generates an interrupt request"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(ISC0_A::RISING)
    }
}
#[doc = "Field `ISC1` reader - External Interrupt Sense Control 1 Bits"]
pub type ISC1_R = crate::FieldReader<ISC1_A>;
#[doc = "External Interrupt Sense Control 1 Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISC1_A {
    #[doc = "0: Low Level of INTX"]
    VAL_0X00 = 0,
    #[doc = "1: Any Logical Change in INTX"]
    VAL_0X01 = 1,
    #[doc = "2: Falling Edge of INTX"]
    VAL_0X02 = 2,
    #[doc = "3: Rising Edge of INTX"]
    VAL_0X03 = 3,
}
impl From<ISC1_A> for u8 {
    #[inline(always)]
    fn from(variant: ISC1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ISC1_A {
    type Ux = u8;
}
impl ISC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ISC1_A {
        match self.bits {
            0 => ISC1_A::VAL_0X00,
            1 => ISC1_A::VAL_0X01,
            2 => ISC1_A::VAL_0X02,
            3 => ISC1_A::VAL_0X03,
            _ => unreachable!(),
        }
    }
    #[doc = "Low Level of INTX"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == ISC1_A::VAL_0X00
    }
    #[doc = "Any Logical Change in INTX"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == ISC1_A::VAL_0X01
    }
    #[doc = "Falling Edge of INTX"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == ISC1_A::VAL_0X02
    }
    #[doc = "Rising Edge of INTX"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == ISC1_A::VAL_0X03
    }
}
#[doc = "Field `ISC1` writer - External Interrupt Sense Control 1 Bits"]
pub type ISC1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ISC1_A>;
impl<'a, REG> ISC1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low Level of INTX"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(ISC1_A::VAL_0X00)
    }
    #[doc = "Any Logical Change in INTX"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(ISC1_A::VAL_0X01)
    }
    #[doc = "Falling Edge of INTX"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(ISC1_A::VAL_0X02)
    }
    #[doc = "Rising Edge of INTX"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(ISC1_A::VAL_0X03)
    }
}
impl R {
    #[doc = "Bits 0:1 - External Interrupt Sense Control 0 Bits"]
    #[inline(always)]
    pub fn isc0(&self) -> ISC0_R {
        ISC0_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - External Interrupt Sense Control 1 Bits"]
    #[inline(always)]
    pub fn isc1(&self) -> ISC1_R {
        ISC1_R::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt Sense Control 0 Bits"]
    #[inline(always)]
    #[must_use]
    pub fn isc0(&mut self) -> ISC0_W<EICRA_SPEC> {
        ISC0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - External Interrupt Sense Control 1 Bits"]
    #[inline(always)]
    #[must_use]
    pub fn isc1(&mut self) -> ISC1_W<EICRA_SPEC> {
        ISC1_W::new(self, 2)
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
#[doc = "External Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eicra::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eicra::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EICRA_SPEC;
impl crate::RegisterSpec for EICRA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eicra::R`](R) reader structure"]
impl crate::Readable for EICRA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eicra::W`](W) writer structure"]
impl crate::Writable for EICRA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EICRA to value 0"]
impl crate::Resettable for EICRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
