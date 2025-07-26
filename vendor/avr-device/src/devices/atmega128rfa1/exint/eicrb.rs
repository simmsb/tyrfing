#[doc = "Register `EICRB` reader"]
pub type R = crate::R<EICRB_SPEC>;
#[doc = "Register `EICRB` writer"]
pub type W = crate::W<EICRB_SPEC>;
#[doc = "Field `ISC4` reader - External Interrupt 4 Sense Control Bit"]
pub type ISC4_R = crate::FieldReader<ISC4_A>;
#[doc = "External Interrupt 4 Sense Control Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISC4_A {
    #[doc = "0: The low level of INTn generates an interrupt request."]
    THE_LOW_LEVEL_OF_INTN_GENERATES_AN_INTERRUPT_REQUEST = 0,
    #[doc = "1: Any edge of INTn generates asynchronously an interrupt request."]
    ANY_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST = 1,
    #[doc = "2: The falling edge of INTn generates asynchronously an interrupt request."]
    THE_FALLING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST = 2,
    #[doc = "3: The rising edge of INTn generates asynchronously an interrupt request."]
    THE_RISING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST = 3,
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
            0 => ISC4_A::THE_LOW_LEVEL_OF_INTN_GENERATES_AN_INTERRUPT_REQUEST,
            1 => ISC4_A::ANY_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST,
            2 => ISC4_A::THE_FALLING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST,
            3 => ISC4_A::THE_RISING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST,
            _ => unreachable!(),
        }
    }
    #[doc = "The low level of INTn generates an interrupt request."]
    #[inline(always)]
    pub fn is_the_low_level_of_intn_generates_an_interrupt_request(&self) -> bool {
        *self == ISC4_A::THE_LOW_LEVEL_OF_INTN_GENERATES_AN_INTERRUPT_REQUEST
    }
    #[doc = "Any edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn is_any_edge_of_intn_generates_asynchronously_an_interrupt_request(&self) -> bool {
        *self == ISC4_A::ANY_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST
    }
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn is_the_falling_edge_of_intn_generates_asynchronously_an_interrupt_request(
        &self,
    ) -> bool {
        *self == ISC4_A::THE_FALLING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST
    }
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn is_the_rising_edge_of_intn_generates_asynchronously_an_interrupt_request(&self) -> bool {
        *self == ISC4_A::THE_RISING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST
    }
}
#[doc = "Field `ISC4` writer - External Interrupt 4 Sense Control Bit"]
pub type ISC4_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ISC4_A>;
impl<'a, REG> ISC4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The low level of INTn generates an interrupt request."]
    #[inline(always)]
    pub fn the_low_level_of_intn_generates_an_interrupt_request(self) -> &'a mut crate::W<REG> {
        self.variant(ISC4_A::THE_LOW_LEVEL_OF_INTN_GENERATES_AN_INTERRUPT_REQUEST)
    }
    #[doc = "Any edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn any_edge_of_intn_generates_asynchronously_an_interrupt_request(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(ISC4_A::ANY_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST)
    }
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn the_falling_edge_of_intn_generates_asynchronously_an_interrupt_request(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(ISC4_A::THE_FALLING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST)
    }
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn the_rising_edge_of_intn_generates_asynchronously_an_interrupt_request(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(ISC4_A::THE_RISING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST)
    }
}
#[doc = "Field `ISC5` reader - External Interrupt 5 Sense Control Bit"]
pub type ISC5_R = crate::FieldReader<ISC5_A>;
#[doc = "External Interrupt 5 Sense Control Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISC5_A {
    #[doc = "0: The low level of INTn generates an interrupt request."]
    THE_LOW_LEVEL_OF_INTN_GENERATES_AN_INTERRUPT_REQUEST = 0,
    #[doc = "1: Any edge of INTn generates asynchronously an interrupt request."]
    ANY_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST = 1,
    #[doc = "2: The falling edge of INTn generates asynchronously an interrupt request."]
    THE_FALLING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST = 2,
    #[doc = "3: The rising edge of INTn generates asynchronously an interrupt request."]
    THE_RISING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST = 3,
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
            0 => ISC5_A::THE_LOW_LEVEL_OF_INTN_GENERATES_AN_INTERRUPT_REQUEST,
            1 => ISC5_A::ANY_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST,
            2 => ISC5_A::THE_FALLING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST,
            3 => ISC5_A::THE_RISING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST,
            _ => unreachable!(),
        }
    }
    #[doc = "The low level of INTn generates an interrupt request."]
    #[inline(always)]
    pub fn is_the_low_level_of_intn_generates_an_interrupt_request(&self) -> bool {
        *self == ISC5_A::THE_LOW_LEVEL_OF_INTN_GENERATES_AN_INTERRUPT_REQUEST
    }
    #[doc = "Any edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn is_any_edge_of_intn_generates_asynchronously_an_interrupt_request(&self) -> bool {
        *self == ISC5_A::ANY_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST
    }
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn is_the_falling_edge_of_intn_generates_asynchronously_an_interrupt_request(
        &self,
    ) -> bool {
        *self == ISC5_A::THE_FALLING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST
    }
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn is_the_rising_edge_of_intn_generates_asynchronously_an_interrupt_request(&self) -> bool {
        *self == ISC5_A::THE_RISING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST
    }
}
#[doc = "Field `ISC5` writer - External Interrupt 5 Sense Control Bit"]
pub type ISC5_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ISC5_A>;
impl<'a, REG> ISC5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The low level of INTn generates an interrupt request."]
    #[inline(always)]
    pub fn the_low_level_of_intn_generates_an_interrupt_request(self) -> &'a mut crate::W<REG> {
        self.variant(ISC5_A::THE_LOW_LEVEL_OF_INTN_GENERATES_AN_INTERRUPT_REQUEST)
    }
    #[doc = "Any edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn any_edge_of_intn_generates_asynchronously_an_interrupt_request(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(ISC5_A::ANY_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST)
    }
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn the_falling_edge_of_intn_generates_asynchronously_an_interrupt_request(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(ISC5_A::THE_FALLING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST)
    }
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn the_rising_edge_of_intn_generates_asynchronously_an_interrupt_request(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(ISC5_A::THE_RISING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST)
    }
}
#[doc = "Field `ISC6` reader - External Interrupt 6 Sense Control Bit"]
pub type ISC6_R = crate::FieldReader<ISC6_A>;
#[doc = "External Interrupt 6 Sense Control Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISC6_A {
    #[doc = "0: The low level of INTn generates an interrupt request."]
    THE_LOW_LEVEL_OF_INTN_GENERATES_AN_INTERRUPT_REQUEST = 0,
    #[doc = "1: Any edge of INTn generates asynchronously an interrupt request."]
    ANY_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST = 1,
    #[doc = "2: The falling edge of INTn generates asynchronously an interrupt request."]
    THE_FALLING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST = 2,
    #[doc = "3: The rising edge of INTn generates asynchronously an interrupt request."]
    THE_RISING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST = 3,
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
            0 => ISC6_A::THE_LOW_LEVEL_OF_INTN_GENERATES_AN_INTERRUPT_REQUEST,
            1 => ISC6_A::ANY_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST,
            2 => ISC6_A::THE_FALLING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST,
            3 => ISC6_A::THE_RISING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST,
            _ => unreachable!(),
        }
    }
    #[doc = "The low level of INTn generates an interrupt request."]
    #[inline(always)]
    pub fn is_the_low_level_of_intn_generates_an_interrupt_request(&self) -> bool {
        *self == ISC6_A::THE_LOW_LEVEL_OF_INTN_GENERATES_AN_INTERRUPT_REQUEST
    }
    #[doc = "Any edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn is_any_edge_of_intn_generates_asynchronously_an_interrupt_request(&self) -> bool {
        *self == ISC6_A::ANY_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST
    }
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn is_the_falling_edge_of_intn_generates_asynchronously_an_interrupt_request(
        &self,
    ) -> bool {
        *self == ISC6_A::THE_FALLING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST
    }
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn is_the_rising_edge_of_intn_generates_asynchronously_an_interrupt_request(&self) -> bool {
        *self == ISC6_A::THE_RISING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST
    }
}
#[doc = "Field `ISC6` writer - External Interrupt 6 Sense Control Bit"]
pub type ISC6_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ISC6_A>;
impl<'a, REG> ISC6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The low level of INTn generates an interrupt request."]
    #[inline(always)]
    pub fn the_low_level_of_intn_generates_an_interrupt_request(self) -> &'a mut crate::W<REG> {
        self.variant(ISC6_A::THE_LOW_LEVEL_OF_INTN_GENERATES_AN_INTERRUPT_REQUEST)
    }
    #[doc = "Any edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn any_edge_of_intn_generates_asynchronously_an_interrupt_request(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(ISC6_A::ANY_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST)
    }
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn the_falling_edge_of_intn_generates_asynchronously_an_interrupt_request(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(ISC6_A::THE_FALLING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST)
    }
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn the_rising_edge_of_intn_generates_asynchronously_an_interrupt_request(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(ISC6_A::THE_RISING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST)
    }
}
#[doc = "Field `ISC7` reader - External Interrupt 7 Sense Control Bit"]
pub type ISC7_R = crate::FieldReader<ISC7_A>;
#[doc = "External Interrupt 7 Sense Control Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISC7_A {
    #[doc = "0: The low level of INTn generates an interrupt request."]
    THE_LOW_LEVEL_OF_INTN_GENERATES_AN_INTERRUPT_REQUEST = 0,
    #[doc = "1: Any edge of INTn generates asynchronously an interrupt request."]
    ANY_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST = 1,
    #[doc = "2: The falling edge of INTn generates asynchronously an interrupt request."]
    THE_FALLING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST = 2,
    #[doc = "3: The rising edge of INTn generates asynchronously an interrupt request."]
    THE_RISING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST = 3,
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
            0 => ISC7_A::THE_LOW_LEVEL_OF_INTN_GENERATES_AN_INTERRUPT_REQUEST,
            1 => ISC7_A::ANY_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST,
            2 => ISC7_A::THE_FALLING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST,
            3 => ISC7_A::THE_RISING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST,
            _ => unreachable!(),
        }
    }
    #[doc = "The low level of INTn generates an interrupt request."]
    #[inline(always)]
    pub fn is_the_low_level_of_intn_generates_an_interrupt_request(&self) -> bool {
        *self == ISC7_A::THE_LOW_LEVEL_OF_INTN_GENERATES_AN_INTERRUPT_REQUEST
    }
    #[doc = "Any edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn is_any_edge_of_intn_generates_asynchronously_an_interrupt_request(&self) -> bool {
        *self == ISC7_A::ANY_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST
    }
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn is_the_falling_edge_of_intn_generates_asynchronously_an_interrupt_request(
        &self,
    ) -> bool {
        *self == ISC7_A::THE_FALLING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST
    }
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn is_the_rising_edge_of_intn_generates_asynchronously_an_interrupt_request(&self) -> bool {
        *self == ISC7_A::THE_RISING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST
    }
}
#[doc = "Field `ISC7` writer - External Interrupt 7 Sense Control Bit"]
pub type ISC7_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ISC7_A>;
impl<'a, REG> ISC7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The low level of INTn generates an interrupt request."]
    #[inline(always)]
    pub fn the_low_level_of_intn_generates_an_interrupt_request(self) -> &'a mut crate::W<REG> {
        self.variant(ISC7_A::THE_LOW_LEVEL_OF_INTN_GENERATES_AN_INTERRUPT_REQUEST)
    }
    #[doc = "Any edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn any_edge_of_intn_generates_asynchronously_an_interrupt_request(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(ISC7_A::ANY_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST)
    }
    #[doc = "The falling edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn the_falling_edge_of_intn_generates_asynchronously_an_interrupt_request(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(ISC7_A::THE_FALLING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST)
    }
    #[doc = "The rising edge of INTn generates asynchronously an interrupt request."]
    #[inline(always)]
    pub fn the_rising_edge_of_intn_generates_asynchronously_an_interrupt_request(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(ISC7_A::THE_RISING_EDGE_OF_INTN_GENERATES_ASYNCHRONOUSLY_AN_INTERRUPT_REQUEST)
    }
}
impl R {
    #[doc = "Bits 0:1 - External Interrupt 4 Sense Control Bit"]
    #[inline(always)]
    pub fn isc4(&self) -> ISC4_R {
        ISC4_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - External Interrupt 5 Sense Control Bit"]
    #[inline(always)]
    pub fn isc5(&self) -> ISC5_R {
        ISC5_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - External Interrupt 6 Sense Control Bit"]
    #[inline(always)]
    pub fn isc6(&self) -> ISC6_R {
        ISC6_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - External Interrupt 7 Sense Control Bit"]
    #[inline(always)]
    pub fn isc7(&self) -> ISC7_R {
        ISC7_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt 4 Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc4(&mut self) -> ISC4_W<EICRB_SPEC> {
        ISC4_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - External Interrupt 5 Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc5(&mut self) -> ISC5_W<EICRB_SPEC> {
        ISC5_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - External Interrupt 6 Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc6(&mut self) -> ISC6_W<EICRB_SPEC> {
        ISC6_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - External Interrupt 7 Sense Control Bit"]
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
