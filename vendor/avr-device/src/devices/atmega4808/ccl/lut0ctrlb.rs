#[doc = "Register `LUT0CTRLB` reader"]
pub type R = crate::R<LUT0CTRLB_SPEC>;
#[doc = "Register `LUT0CTRLB` writer"]
pub type W = crate::W<LUT0CTRLB_SPEC>;
#[doc = "Field `INSEL0` reader - LUT Input 0 Source Selection"]
pub type INSEL0_R = crate::FieldReader<INSEL0_A>;
#[doc = "LUT Input 0 Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INSEL0_A {
    #[doc = "0: Masked input"]
    MASK = 0,
    #[doc = "1: Feedback input source"]
    FEEDBACK = 1,
    #[doc = "2: Linked LUT input source"]
    LINK = 2,
    #[doc = "3: Event input source A"]
    EVENTA = 3,
    #[doc = "4: Event input source B"]
    EVENTB = 4,
    #[doc = "5: IO pin LUTn-IN0 input source"]
    IO = 5,
    #[doc = "6: AC0 OUT input source"]
    AC0 = 6,
    #[doc = "8: USART0 TXD input source"]
    USART0 = 8,
    #[doc = "9: SPI0 MOSI input source"]
    SPI0 = 9,
    #[doc = "10: TCA0 WO0 input source"]
    TCA0 = 10,
    #[doc = "12: TCB0 WO input source"]
    TCB0 = 12,
}
impl From<INSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: INSEL0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INSEL0_A {
    type Ux = u8;
}
impl INSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INSEL0_A> {
        match self.bits {
            0 => Some(INSEL0_A::MASK),
            1 => Some(INSEL0_A::FEEDBACK),
            2 => Some(INSEL0_A::LINK),
            3 => Some(INSEL0_A::EVENTA),
            4 => Some(INSEL0_A::EVENTB),
            5 => Some(INSEL0_A::IO),
            6 => Some(INSEL0_A::AC0),
            8 => Some(INSEL0_A::USART0),
            9 => Some(INSEL0_A::SPI0),
            10 => Some(INSEL0_A::TCA0),
            12 => Some(INSEL0_A::TCB0),
            _ => None,
        }
    }
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == INSEL0_A::MASK
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn is_feedback(&self) -> bool {
        *self == INSEL0_A::FEEDBACK
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        *self == INSEL0_A::LINK
    }
    #[doc = "Event input source A"]
    #[inline(always)]
    pub fn is_eventa(&self) -> bool {
        *self == INSEL0_A::EVENTA
    }
    #[doc = "Event input source B"]
    #[inline(always)]
    pub fn is_eventb(&self) -> bool {
        *self == INSEL0_A::EVENTB
    }
    #[doc = "IO pin LUTn-IN0 input source"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == INSEL0_A::IO
    }
    #[doc = "AC0 OUT input source"]
    #[inline(always)]
    pub fn is_ac0(&self) -> bool {
        *self == INSEL0_A::AC0
    }
    #[doc = "USART0 TXD input source"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == INSEL0_A::USART0
    }
    #[doc = "SPI0 MOSI input source"]
    #[inline(always)]
    pub fn is_spi0(&self) -> bool {
        *self == INSEL0_A::SPI0
    }
    #[doc = "TCA0 WO0 input source"]
    #[inline(always)]
    pub fn is_tca0(&self) -> bool {
        *self == INSEL0_A::TCA0
    }
    #[doc = "TCB0 WO input source"]
    #[inline(always)]
    pub fn is_tcb0(&self) -> bool {
        *self == INSEL0_A::TCB0
    }
}
#[doc = "Field `INSEL0` writer - LUT Input 0 Source Selection"]
pub type INSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 4, INSEL0_A>;
impl<'a, REG> INSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL0_A::MASK)
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn feedback(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL0_A::FEEDBACK)
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn link(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL0_A::LINK)
    }
    #[doc = "Event input source A"]
    #[inline(always)]
    pub fn eventa(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL0_A::EVENTA)
    }
    #[doc = "Event input source B"]
    #[inline(always)]
    pub fn eventb(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL0_A::EVENTB)
    }
    #[doc = "IO pin LUTn-IN0 input source"]
    #[inline(always)]
    pub fn io(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL0_A::IO)
    }
    #[doc = "AC0 OUT input source"]
    #[inline(always)]
    pub fn ac0(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL0_A::AC0)
    }
    #[doc = "USART0 TXD input source"]
    #[inline(always)]
    pub fn usart0(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL0_A::USART0)
    }
    #[doc = "SPI0 MOSI input source"]
    #[inline(always)]
    pub fn spi0(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL0_A::SPI0)
    }
    #[doc = "TCA0 WO0 input source"]
    #[inline(always)]
    pub fn tca0(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL0_A::TCA0)
    }
    #[doc = "TCB0 WO input source"]
    #[inline(always)]
    pub fn tcb0(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL0_A::TCB0)
    }
}
#[doc = "Field `INSEL1` reader - LUT Input 1 Source Selection"]
pub type INSEL1_R = crate::FieldReader<INSEL1_A>;
#[doc = "LUT Input 1 Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INSEL1_A {
    #[doc = "0: Masked input"]
    MASK = 0,
    #[doc = "1: Feedback input source"]
    FEEDBACK = 1,
    #[doc = "2: Linked LUT input source"]
    LINK = 2,
    #[doc = "3: Event input source A"]
    EVENTA = 3,
    #[doc = "4: Event input source B"]
    EVENTB = 4,
    #[doc = "5: IO pin LUTn-N1 input source"]
    IO = 5,
    #[doc = "6: AC0 OUT input source"]
    AC0 = 6,
    #[doc = "8: USART1 TXD input source"]
    USART1 = 8,
    #[doc = "9: SPI0 MOSI input source"]
    SPI0 = 9,
    #[doc = "10: TCA0 WO1 input source"]
    TCA0 = 10,
    #[doc = "12: TCB1 WO input source"]
    TCB1 = 12,
}
impl From<INSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: INSEL1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INSEL1_A {
    type Ux = u8;
}
impl INSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INSEL1_A> {
        match self.bits {
            0 => Some(INSEL1_A::MASK),
            1 => Some(INSEL1_A::FEEDBACK),
            2 => Some(INSEL1_A::LINK),
            3 => Some(INSEL1_A::EVENTA),
            4 => Some(INSEL1_A::EVENTB),
            5 => Some(INSEL1_A::IO),
            6 => Some(INSEL1_A::AC0),
            8 => Some(INSEL1_A::USART1),
            9 => Some(INSEL1_A::SPI0),
            10 => Some(INSEL1_A::TCA0),
            12 => Some(INSEL1_A::TCB1),
            _ => None,
        }
    }
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == INSEL1_A::MASK
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn is_feedback(&self) -> bool {
        *self == INSEL1_A::FEEDBACK
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        *self == INSEL1_A::LINK
    }
    #[doc = "Event input source A"]
    #[inline(always)]
    pub fn is_eventa(&self) -> bool {
        *self == INSEL1_A::EVENTA
    }
    #[doc = "Event input source B"]
    #[inline(always)]
    pub fn is_eventb(&self) -> bool {
        *self == INSEL1_A::EVENTB
    }
    #[doc = "IO pin LUTn-N1 input source"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == INSEL1_A::IO
    }
    #[doc = "AC0 OUT input source"]
    #[inline(always)]
    pub fn is_ac0(&self) -> bool {
        *self == INSEL1_A::AC0
    }
    #[doc = "USART1 TXD input source"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == INSEL1_A::USART1
    }
    #[doc = "SPI0 MOSI input source"]
    #[inline(always)]
    pub fn is_spi0(&self) -> bool {
        *self == INSEL1_A::SPI0
    }
    #[doc = "TCA0 WO1 input source"]
    #[inline(always)]
    pub fn is_tca0(&self) -> bool {
        *self == INSEL1_A::TCA0
    }
    #[doc = "TCB1 WO input source"]
    #[inline(always)]
    pub fn is_tcb1(&self) -> bool {
        *self == INSEL1_A::TCB1
    }
}
#[doc = "Field `INSEL1` writer - LUT Input 1 Source Selection"]
pub type INSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, INSEL1_A>;
impl<'a, REG> INSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL1_A::MASK)
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn feedback(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL1_A::FEEDBACK)
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn link(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL1_A::LINK)
    }
    #[doc = "Event input source A"]
    #[inline(always)]
    pub fn eventa(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL1_A::EVENTA)
    }
    #[doc = "Event input source B"]
    #[inline(always)]
    pub fn eventb(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL1_A::EVENTB)
    }
    #[doc = "IO pin LUTn-N1 input source"]
    #[inline(always)]
    pub fn io(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL1_A::IO)
    }
    #[doc = "AC0 OUT input source"]
    #[inline(always)]
    pub fn ac0(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL1_A::AC0)
    }
    #[doc = "USART1 TXD input source"]
    #[inline(always)]
    pub fn usart1(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL1_A::USART1)
    }
    #[doc = "SPI0 MOSI input source"]
    #[inline(always)]
    pub fn spi0(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL1_A::SPI0)
    }
    #[doc = "TCA0 WO1 input source"]
    #[inline(always)]
    pub fn tca0(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL1_A::TCA0)
    }
    #[doc = "TCB1 WO input source"]
    #[inline(always)]
    pub fn tcb1(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL1_A::TCB1)
    }
}
impl R {
    #[doc = "Bits 0:3 - LUT Input 0 Source Selection"]
    #[inline(always)]
    pub fn insel0(&self) -> INSEL0_R {
        INSEL0_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - LUT Input 1 Source Selection"]
    #[inline(always)]
    pub fn insel1(&self) -> INSEL1_R {
        INSEL1_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - LUT Input 0 Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn insel0(&mut self) -> INSEL0_W<LUT0CTRLB_SPEC> {
        INSEL0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - LUT Input 1 Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn insel1(&mut self) -> INSEL1_W<LUT0CTRLB_SPEC> {
        INSEL1_W::new(self, 4)
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
#[doc = "LUT Control 0 B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut0ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut0ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LUT0CTRLB_SPEC;
impl crate::RegisterSpec for LUT0CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lut0ctrlb::R`](R) reader structure"]
impl crate::Readable for LUT0CTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lut0ctrlb::W`](W) writer structure"]
impl crate::Writable for LUT0CTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LUT0CTRLB to value 0"]
impl crate::Resettable for LUT0CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
