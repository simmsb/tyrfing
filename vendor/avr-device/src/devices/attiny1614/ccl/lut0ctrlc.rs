#[doc = "Register `LUT0CTRLC` reader"]
pub type R = crate::R<LUT0CTRLC_SPEC>;
#[doc = "Register `LUT0CTRLC` writer"]
pub type W = crate::W<LUT0CTRLC_SPEC>;
#[doc = "Field `INSEL2` reader - LUT Input 2 Source Selection"]
pub type INSEL2_R = crate::FieldReader<INSEL2_A>;
#[doc = "LUT Input 2 Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INSEL2_A {
    #[doc = "0: Masked input"]
    MASK = 0,
    #[doc = "1: Feedback input source"]
    FEEDBACK = 1,
    #[doc = "2: Linked LUT input source"]
    LINK = 2,
    #[doc = "3: Event input source 0"]
    EVENT0 = 3,
    #[doc = "4: Event input source 1"]
    EVENT1 = 4,
    #[doc = "5: IO pin LUTn-IN2 input source"]
    IO = 5,
    #[doc = "6: AC0 OUT input source"]
    AC0 = 6,
    #[doc = "7: TCB0 WO input source"]
    TCB0 = 7,
    #[doc = "8: TCA0 WO2 input source"]
    TCA0 = 8,
    #[doc = "9: TCD0 WOA input source"]
    TCD0 = 9,
    #[doc = "11: SPI0 MISO source"]
    SPI0 = 11,
    #[doc = "12: AC1 OUT input source"]
    AC1 = 12,
    #[doc = "13: TCB1 WO input source"]
    TCB1 = 13,
    #[doc = "14: AC2 OUT input source"]
    AC2 = 14,
}
impl From<INSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: INSEL2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INSEL2_A {
    type Ux = u8;
}
impl INSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INSEL2_A> {
        match self.bits {
            0 => Some(INSEL2_A::MASK),
            1 => Some(INSEL2_A::FEEDBACK),
            2 => Some(INSEL2_A::LINK),
            3 => Some(INSEL2_A::EVENT0),
            4 => Some(INSEL2_A::EVENT1),
            5 => Some(INSEL2_A::IO),
            6 => Some(INSEL2_A::AC0),
            7 => Some(INSEL2_A::TCB0),
            8 => Some(INSEL2_A::TCA0),
            9 => Some(INSEL2_A::TCD0),
            11 => Some(INSEL2_A::SPI0),
            12 => Some(INSEL2_A::AC1),
            13 => Some(INSEL2_A::TCB1),
            14 => Some(INSEL2_A::AC2),
            _ => None,
        }
    }
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == INSEL2_A::MASK
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn is_feedback(&self) -> bool {
        *self == INSEL2_A::FEEDBACK
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        *self == INSEL2_A::LINK
    }
    #[doc = "Event input source 0"]
    #[inline(always)]
    pub fn is_event0(&self) -> bool {
        *self == INSEL2_A::EVENT0
    }
    #[doc = "Event input source 1"]
    #[inline(always)]
    pub fn is_event1(&self) -> bool {
        *self == INSEL2_A::EVENT1
    }
    #[doc = "IO pin LUTn-IN2 input source"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == INSEL2_A::IO
    }
    #[doc = "AC0 OUT input source"]
    #[inline(always)]
    pub fn is_ac0(&self) -> bool {
        *self == INSEL2_A::AC0
    }
    #[doc = "TCB0 WO input source"]
    #[inline(always)]
    pub fn is_tcb0(&self) -> bool {
        *self == INSEL2_A::TCB0
    }
    #[doc = "TCA0 WO2 input source"]
    #[inline(always)]
    pub fn is_tca0(&self) -> bool {
        *self == INSEL2_A::TCA0
    }
    #[doc = "TCD0 WOA input source"]
    #[inline(always)]
    pub fn is_tcd0(&self) -> bool {
        *self == INSEL2_A::TCD0
    }
    #[doc = "SPI0 MISO source"]
    #[inline(always)]
    pub fn is_spi0(&self) -> bool {
        *self == INSEL2_A::SPI0
    }
    #[doc = "AC1 OUT input source"]
    #[inline(always)]
    pub fn is_ac1(&self) -> bool {
        *self == INSEL2_A::AC1
    }
    #[doc = "TCB1 WO input source"]
    #[inline(always)]
    pub fn is_tcb1(&self) -> bool {
        *self == INSEL2_A::TCB1
    }
    #[doc = "AC2 OUT input source"]
    #[inline(always)]
    pub fn is_ac2(&self) -> bool {
        *self == INSEL2_A::AC2
    }
}
#[doc = "Field `INSEL2` writer - LUT Input 2 Source Selection"]
pub type INSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 4, INSEL2_A>;
impl<'a, REG> INSEL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL2_A::MASK)
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn feedback(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL2_A::FEEDBACK)
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn link(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL2_A::LINK)
    }
    #[doc = "Event input source 0"]
    #[inline(always)]
    pub fn event0(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL2_A::EVENT0)
    }
    #[doc = "Event input source 1"]
    #[inline(always)]
    pub fn event1(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL2_A::EVENT1)
    }
    #[doc = "IO pin LUTn-IN2 input source"]
    #[inline(always)]
    pub fn io(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL2_A::IO)
    }
    #[doc = "AC0 OUT input source"]
    #[inline(always)]
    pub fn ac0(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL2_A::AC0)
    }
    #[doc = "TCB0 WO input source"]
    #[inline(always)]
    pub fn tcb0(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL2_A::TCB0)
    }
    #[doc = "TCA0 WO2 input source"]
    #[inline(always)]
    pub fn tca0(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL2_A::TCA0)
    }
    #[doc = "TCD0 WOA input source"]
    #[inline(always)]
    pub fn tcd0(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL2_A::TCD0)
    }
    #[doc = "SPI0 MISO source"]
    #[inline(always)]
    pub fn spi0(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL2_A::SPI0)
    }
    #[doc = "AC1 OUT input source"]
    #[inline(always)]
    pub fn ac1(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL2_A::AC1)
    }
    #[doc = "TCB1 WO input source"]
    #[inline(always)]
    pub fn tcb1(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL2_A::TCB1)
    }
    #[doc = "AC2 OUT input source"]
    #[inline(always)]
    pub fn ac2(self) -> &'a mut crate::W<REG> {
        self.variant(INSEL2_A::AC2)
    }
}
impl R {
    #[doc = "Bits 0:3 - LUT Input 2 Source Selection"]
    #[inline(always)]
    pub fn insel2(&self) -> INSEL2_R {
        INSEL2_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - LUT Input 2 Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn insel2(&mut self) -> INSEL2_W<LUT0CTRLC_SPEC> {
        INSEL2_W::new(self, 0)
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
#[doc = "LUT Control 0 C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut0ctrlc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut0ctrlc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LUT0CTRLC_SPEC;
impl crate::RegisterSpec for LUT0CTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lut0ctrlc::R`](R) reader structure"]
impl crate::Readable for LUT0CTRLC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lut0ctrlc::W`](W) writer structure"]
impl crate::Writable for LUT0CTRLC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LUT0CTRLC to value 0"]
impl crate::Resettable for LUT0CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
