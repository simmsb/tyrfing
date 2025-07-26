#[doc = "Register `MCUCR` reader"]
pub type R = crate::R<MCUCR_SPEC>;
#[doc = "Register `MCUCR` writer"]
pub type W = crate::W<MCUCR_SPEC>;
#[doc = "Field `ISC0` reader - Interrupt Sense Control 0 Bits"]
pub type ISC0_R = crate::FieldReader<ISC0_A>;
#[doc = "Interrupt Sense Control 0 Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISC0_A {
    #[doc = "0: Low Level of INTX"]
    VAL_0X00 = 0,
    #[doc = "1: Any Logical Change of INTX"]
    VAL_0X01 = 1,
    #[doc = "2: Falling Edge of INTX"]
    VAL_0X02 = 2,
    #[doc = "3: Rising Edge of INTX"]
    VAL_0X03 = 3,
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
            0 => ISC0_A::VAL_0X00,
            1 => ISC0_A::VAL_0X01,
            2 => ISC0_A::VAL_0X02,
            3 => ISC0_A::VAL_0X03,
            _ => unreachable!(),
        }
    }
    #[doc = "Low Level of INTX"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == ISC0_A::VAL_0X00
    }
    #[doc = "Any Logical Change of INTX"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == ISC0_A::VAL_0X01
    }
    #[doc = "Falling Edge of INTX"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == ISC0_A::VAL_0X02
    }
    #[doc = "Rising Edge of INTX"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == ISC0_A::VAL_0X03
    }
}
#[doc = "Field `ISC0` writer - Interrupt Sense Control 0 Bits"]
pub type ISC0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ISC0_A>;
impl<'a, REG> ISC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low Level of INTX"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(ISC0_A::VAL_0X00)
    }
    #[doc = "Any Logical Change of INTX"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(ISC0_A::VAL_0X01)
    }
    #[doc = "Falling Edge of INTX"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(ISC0_A::VAL_0X02)
    }
    #[doc = "Rising Edge of INTX"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(ISC0_A::VAL_0X03)
    }
}
#[doc = "Field `ISC1` reader - Interrupt Sense Control 1 Bits"]
pub type ISC1_R = crate::FieldReader<ISC1_A>;
#[doc = "Interrupt Sense Control 1 Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISC1_A {
    #[doc = "0: Low Level of INTX"]
    VAL_0X00 = 0,
    #[doc = "1: Any Logical Change of INTX"]
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
    #[doc = "Any Logical Change of INTX"]
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
#[doc = "Field `ISC1` writer - Interrupt Sense Control 1 Bits"]
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
    #[doc = "Any Logical Change of INTX"]
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
    #[doc = "Bits 0:1 - Interrupt Sense Control 0 Bits"]
    #[inline(always)]
    pub fn isc0(&self) -> ISC0_R {
        ISC0_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Interrupt Sense Control 1 Bits"]
    #[inline(always)]
    pub fn isc1(&self) -> ISC1_R {
        ISC1_R::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interrupt Sense Control 0 Bits"]
    #[inline(always)]
    #[must_use]
    pub fn isc0(&mut self) -> ISC0_W<MCUCR_SPEC> {
        ISC0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Interrupt Sense Control 1 Bits"]
    #[inline(always)]
    #[must_use]
    pub fn isc1(&mut self) -> ISC1_W<MCUCR_SPEC> {
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
#[doc = "MCU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCUCR_SPEC;
impl crate::RegisterSpec for MCUCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mcucr::R`](R) reader structure"]
impl crate::Readable for MCUCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcucr::W`](W) writer structure"]
impl crate::Writable for MCUCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUCR to value 0"]
impl crate::Resettable for MCUCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
