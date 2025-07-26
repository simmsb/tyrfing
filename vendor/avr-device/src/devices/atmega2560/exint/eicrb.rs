#[doc = "Register `EICRB` reader"]
pub type R = crate::R<EICRB_SPEC>;
#[doc = "Register `EICRB` writer"]
pub type W = crate::W<EICRB_SPEC>;
#[doc = "Field `ISC4` reader - External Interrupt 7-4 Sense Control Bit"]
pub type ISC4_R = crate::FieldReader<ISC4_A>;
#[doc = "External Interrupt 7-4 Sense Control Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISC4_A {
    #[doc = "0: Low Level of INTX"]
    VAL_0X00 = 0,
    #[doc = "1: Any Logical Change of INTX"]
    VAL_0X01 = 1,
    #[doc = "2: Falling Edge of INTX"]
    VAL_0X02 = 2,
    #[doc = "3: Rising Edge of INTX"]
    VAL_0X03 = 3,
}
impl From<ISC4_A> for u8 {
    #[inline(always)]
    fn from(variant: ISC4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ISC4_A {
    type Ux = u8;
}
impl ISC4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ISC4_A {
        match self.bits {
            0 => ISC4_A::VAL_0X00,
            1 => ISC4_A::VAL_0X01,
            2 => ISC4_A::VAL_0X02,
            3 => ISC4_A::VAL_0X03,
            _ => unreachable!(),
        }
    }
    #[doc = "Low Level of INTX"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == ISC4_A::VAL_0X00
    }
    #[doc = "Any Logical Change of INTX"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == ISC4_A::VAL_0X01
    }
    #[doc = "Falling Edge of INTX"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == ISC4_A::VAL_0X02
    }
    #[doc = "Rising Edge of INTX"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == ISC4_A::VAL_0X03
    }
}
#[doc = "Field `ISC4` writer - External Interrupt 7-4 Sense Control Bit"]
pub type ISC4_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ISC4_A>;
impl<'a, REG> ISC4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low Level of INTX"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(ISC4_A::VAL_0X00)
    }
    #[doc = "Any Logical Change of INTX"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(ISC4_A::VAL_0X01)
    }
    #[doc = "Falling Edge of INTX"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(ISC4_A::VAL_0X02)
    }
    #[doc = "Rising Edge of INTX"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(ISC4_A::VAL_0X03)
    }
}
#[doc = "Field `ISC5` reader - External Interrupt 7-4 Sense Control Bit"]
pub type ISC5_R = crate::FieldReader<ISC5_A>;
#[doc = "External Interrupt 7-4 Sense Control Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISC5_A {
    #[doc = "0: Low Level of INTX"]
    VAL_0X00 = 0,
    #[doc = "1: Any Logical Change of INTX"]
    VAL_0X01 = 1,
    #[doc = "2: Falling Edge of INTX"]
    VAL_0X02 = 2,
    #[doc = "3: Rising Edge of INTX"]
    VAL_0X03 = 3,
}
impl From<ISC5_A> for u8 {
    #[inline(always)]
    fn from(variant: ISC5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ISC5_A {
    type Ux = u8;
}
impl ISC5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ISC5_A {
        match self.bits {
            0 => ISC5_A::VAL_0X00,
            1 => ISC5_A::VAL_0X01,
            2 => ISC5_A::VAL_0X02,
            3 => ISC5_A::VAL_0X03,
            _ => unreachable!(),
        }
    }
    #[doc = "Low Level of INTX"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == ISC5_A::VAL_0X00
    }
    #[doc = "Any Logical Change of INTX"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == ISC5_A::VAL_0X01
    }
    #[doc = "Falling Edge of INTX"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == ISC5_A::VAL_0X02
    }
    #[doc = "Rising Edge of INTX"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == ISC5_A::VAL_0X03
    }
}
#[doc = "Field `ISC5` writer - External Interrupt 7-4 Sense Control Bit"]
pub type ISC5_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ISC5_A>;
impl<'a, REG> ISC5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low Level of INTX"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(ISC5_A::VAL_0X00)
    }
    #[doc = "Any Logical Change of INTX"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(ISC5_A::VAL_0X01)
    }
    #[doc = "Falling Edge of INTX"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(ISC5_A::VAL_0X02)
    }
    #[doc = "Rising Edge of INTX"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(ISC5_A::VAL_0X03)
    }
}
#[doc = "Field `ISC6` reader - External Interrupt 7-4 Sense Control Bit"]
pub type ISC6_R = crate::FieldReader<ISC6_A>;
#[doc = "External Interrupt 7-4 Sense Control Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISC6_A {
    #[doc = "0: Low Level of INTX"]
    VAL_0X00 = 0,
    #[doc = "1: Any Logical Change of INTX"]
    VAL_0X01 = 1,
    #[doc = "2: Falling Edge of INTX"]
    VAL_0X02 = 2,
    #[doc = "3: Rising Edge of INTX"]
    VAL_0X03 = 3,
}
impl From<ISC6_A> for u8 {
    #[inline(always)]
    fn from(variant: ISC6_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ISC6_A {
    type Ux = u8;
}
impl ISC6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ISC6_A {
        match self.bits {
            0 => ISC6_A::VAL_0X00,
            1 => ISC6_A::VAL_0X01,
            2 => ISC6_A::VAL_0X02,
            3 => ISC6_A::VAL_0X03,
            _ => unreachable!(),
        }
    }
    #[doc = "Low Level of INTX"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == ISC6_A::VAL_0X00
    }
    #[doc = "Any Logical Change of INTX"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == ISC6_A::VAL_0X01
    }
    #[doc = "Falling Edge of INTX"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == ISC6_A::VAL_0X02
    }
    #[doc = "Rising Edge of INTX"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == ISC6_A::VAL_0X03
    }
}
#[doc = "Field `ISC6` writer - External Interrupt 7-4 Sense Control Bit"]
pub type ISC6_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ISC6_A>;
impl<'a, REG> ISC6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low Level of INTX"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(ISC6_A::VAL_0X00)
    }
    #[doc = "Any Logical Change of INTX"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(ISC6_A::VAL_0X01)
    }
    #[doc = "Falling Edge of INTX"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(ISC6_A::VAL_0X02)
    }
    #[doc = "Rising Edge of INTX"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(ISC6_A::VAL_0X03)
    }
}
#[doc = "Field `ISC7` reader - External Interrupt 7-4 Sense Control Bit"]
pub type ISC7_R = crate::FieldReader<ISC7_A>;
#[doc = "External Interrupt 7-4 Sense Control Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISC7_A {
    #[doc = "0: Low Level of INTX"]
    VAL_0X00 = 0,
    #[doc = "1: Any Logical Change of INTX"]
    VAL_0X01 = 1,
    #[doc = "2: Falling Edge of INTX"]
    VAL_0X02 = 2,
    #[doc = "3: Rising Edge of INTX"]
    VAL_0X03 = 3,
}
impl From<ISC7_A> for u8 {
    #[inline(always)]
    fn from(variant: ISC7_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ISC7_A {
    type Ux = u8;
}
impl ISC7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ISC7_A {
        match self.bits {
            0 => ISC7_A::VAL_0X00,
            1 => ISC7_A::VAL_0X01,
            2 => ISC7_A::VAL_0X02,
            3 => ISC7_A::VAL_0X03,
            _ => unreachable!(),
        }
    }
    #[doc = "Low Level of INTX"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == ISC7_A::VAL_0X00
    }
    #[doc = "Any Logical Change of INTX"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == ISC7_A::VAL_0X01
    }
    #[doc = "Falling Edge of INTX"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == ISC7_A::VAL_0X02
    }
    #[doc = "Rising Edge of INTX"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == ISC7_A::VAL_0X03
    }
}
#[doc = "Field `ISC7` writer - External Interrupt 7-4 Sense Control Bit"]
pub type ISC7_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ISC7_A>;
impl<'a, REG> ISC7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low Level of INTX"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(ISC7_A::VAL_0X00)
    }
    #[doc = "Any Logical Change of INTX"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(ISC7_A::VAL_0X01)
    }
    #[doc = "Falling Edge of INTX"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(ISC7_A::VAL_0X02)
    }
    #[doc = "Rising Edge of INTX"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(ISC7_A::VAL_0X03)
    }
}
impl R {
    #[doc = "Bits 0:1 - External Interrupt 7-4 Sense Control Bit"]
    #[inline(always)]
    pub fn isc4(&self) -> ISC4_R {
        ISC4_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - External Interrupt 7-4 Sense Control Bit"]
    #[inline(always)]
    pub fn isc5(&self) -> ISC5_R {
        ISC5_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - External Interrupt 7-4 Sense Control Bit"]
    #[inline(always)]
    pub fn isc6(&self) -> ISC6_R {
        ISC6_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - External Interrupt 7-4 Sense Control Bit"]
    #[inline(always)]
    pub fn isc7(&self) -> ISC7_R {
        ISC7_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt 7-4 Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc4(&mut self) -> ISC4_W<EICRB_SPEC> {
        ISC4_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - External Interrupt 7-4 Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc5(&mut self) -> ISC5_W<EICRB_SPEC> {
        ISC5_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - External Interrupt 7-4 Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc6(&mut self) -> ISC6_W<EICRB_SPEC> {
        ISC6_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - External Interrupt 7-4 Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc7(&mut self) -> ISC7_W<EICRB_SPEC> {
        ISC7_W::new(self, 6)
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
#[doc = "External Interrupt Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eicrb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eicrb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EICRB_SPEC;
impl crate::RegisterSpec for EICRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eicrb::R`](R) reader structure"]
impl crate::Readable for EICRB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eicrb::W`](W) writer structure"]
impl crate::Writable for EICRB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EICRB to value 0"]
impl crate::Resettable for EICRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
