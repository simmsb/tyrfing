#[doc = "Register `SYNCCH1` reader"]
pub type R = crate::R<SYNCCH1_SPEC>;
#[doc = "Register `SYNCCH1` writer"]
pub type W = crate::W<SYNCCH1_SPEC>;
#[doc = "Field `SYNCCH1` reader - Synchronous Channel 1 Generator Selection"]
pub type SYNCCH1_R = crate::FieldReader<SYNCCH1_A>;
#[doc = "Synchronous Channel 1 Generator Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCCH1_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Timer/Counter B0"]
    TCB0 = 1,
    #[doc = "2: Timer/Counter A0 overflow"]
    TCA0_OVF_LUNF = 2,
    #[doc = "3: Timer/Counter A0 underflow high byte (split mode)"]
    TCA0_HUNF = 3,
    #[doc = "4: Timer/Counter A0 compare 0"]
    TCA0_CMP0 = 4,
    #[doc = "5: Timer/Counter A0 compare 1"]
    TCA0_CMP1 = 5,
    #[doc = "6: Timer/Counter A0 compare 2"]
    TCA0_CMP2 = 6,
    #[doc = "8: Synchronous Event from Pin PB0"]
    PORTB_PIN0 = 8,
    #[doc = "9: Synchronous Event from Pin PB1"]
    PORTB_PIN1 = 9,
    #[doc = "10: Synchronous Event from Pin PB2"]
    PORTB_PIN2 = 10,
    #[doc = "11: Synchronous Event from Pin PB3"]
    PORTB_PIN3 = 11,
    #[doc = "12: Synchronous Event from Pin PB4"]
    PORTB_PIN4 = 12,
    #[doc = "13: Synchronous Event from Pin PB5"]
    PORTB_PIN5 = 13,
    #[doc = "14: Synchronous Event from Pin PB6"]
    PORTB_PIN6 = 14,
    #[doc = "15: Synchronous Event from Pin PB7"]
    PORTB_PIN7 = 15,
    #[doc = "16: Timer/Counter B1"]
    TCB1 = 16,
}
impl From<SYNCCH1_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCCH1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNCCH1_A {
    type Ux = u8;
}
impl SYNCCH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYNCCH1_A> {
        match self.bits {
            0 => Some(SYNCCH1_A::OFF),
            1 => Some(SYNCCH1_A::TCB0),
            2 => Some(SYNCCH1_A::TCA0_OVF_LUNF),
            3 => Some(SYNCCH1_A::TCA0_HUNF),
            4 => Some(SYNCCH1_A::TCA0_CMP0),
            5 => Some(SYNCCH1_A::TCA0_CMP1),
            6 => Some(SYNCCH1_A::TCA0_CMP2),
            8 => Some(SYNCCH1_A::PORTB_PIN0),
            9 => Some(SYNCCH1_A::PORTB_PIN1),
            10 => Some(SYNCCH1_A::PORTB_PIN2),
            11 => Some(SYNCCH1_A::PORTB_PIN3),
            12 => Some(SYNCCH1_A::PORTB_PIN4),
            13 => Some(SYNCCH1_A::PORTB_PIN5),
            14 => Some(SYNCCH1_A::PORTB_PIN6),
            15 => Some(SYNCCH1_A::PORTB_PIN7),
            16 => Some(SYNCCH1_A::TCB1),
            _ => None,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SYNCCH1_A::OFF
    }
    #[doc = "Timer/Counter B0"]
    #[inline(always)]
    pub fn is_tcb0(&self) -> bool {
        *self == SYNCCH1_A::TCB0
    }
    #[doc = "Timer/Counter A0 overflow"]
    #[inline(always)]
    pub fn is_tca0_ovf_lunf(&self) -> bool {
        *self == SYNCCH1_A::TCA0_OVF_LUNF
    }
    #[doc = "Timer/Counter A0 underflow high byte (split mode)"]
    #[inline(always)]
    pub fn is_tca0_hunf(&self) -> bool {
        *self == SYNCCH1_A::TCA0_HUNF
    }
    #[doc = "Timer/Counter A0 compare 0"]
    #[inline(always)]
    pub fn is_tca0_cmp0(&self) -> bool {
        *self == SYNCCH1_A::TCA0_CMP0
    }
    #[doc = "Timer/Counter A0 compare 1"]
    #[inline(always)]
    pub fn is_tca0_cmp1(&self) -> bool {
        *self == SYNCCH1_A::TCA0_CMP1
    }
    #[doc = "Timer/Counter A0 compare 2"]
    #[inline(always)]
    pub fn is_tca0_cmp2(&self) -> bool {
        *self == SYNCCH1_A::TCA0_CMP2
    }
    #[doc = "Synchronous Event from Pin PB0"]
    #[inline(always)]
    pub fn is_portb_pin0(&self) -> bool {
        *self == SYNCCH1_A::PORTB_PIN0
    }
    #[doc = "Synchronous Event from Pin PB1"]
    #[inline(always)]
    pub fn is_portb_pin1(&self) -> bool {
        *self == SYNCCH1_A::PORTB_PIN1
    }
    #[doc = "Synchronous Event from Pin PB2"]
    #[inline(always)]
    pub fn is_portb_pin2(&self) -> bool {
        *self == SYNCCH1_A::PORTB_PIN2
    }
    #[doc = "Synchronous Event from Pin PB3"]
    #[inline(always)]
    pub fn is_portb_pin3(&self) -> bool {
        *self == SYNCCH1_A::PORTB_PIN3
    }
    #[doc = "Synchronous Event from Pin PB4"]
    #[inline(always)]
    pub fn is_portb_pin4(&self) -> bool {
        *self == SYNCCH1_A::PORTB_PIN4
    }
    #[doc = "Synchronous Event from Pin PB5"]
    #[inline(always)]
    pub fn is_portb_pin5(&self) -> bool {
        *self == SYNCCH1_A::PORTB_PIN5
    }
    #[doc = "Synchronous Event from Pin PB6"]
    #[inline(always)]
    pub fn is_portb_pin6(&self) -> bool {
        *self == SYNCCH1_A::PORTB_PIN6
    }
    #[doc = "Synchronous Event from Pin PB7"]
    #[inline(always)]
    pub fn is_portb_pin7(&self) -> bool {
        *self == SYNCCH1_A::PORTB_PIN7
    }
    #[doc = "Timer/Counter B1"]
    #[inline(always)]
    pub fn is_tcb1(&self) -> bool {
        *self == SYNCCH1_A::TCB1
    }
}
#[doc = "Field `SYNCCH1` writer - Synchronous Channel 1 Generator Selection"]
pub type SYNCCH1_W<'a, REG> = crate::FieldWriter<'a, REG, 8, SYNCCH1_A>;
impl<'a, REG> SYNCCH1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCCH1_A::OFF)
    }
    #[doc = "Timer/Counter B0"]
    #[inline(always)]
    pub fn tcb0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCCH1_A::TCB0)
    }
    #[doc = "Timer/Counter A0 overflow"]
    #[inline(always)]
    pub fn tca0_ovf_lunf(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCCH1_A::TCA0_OVF_LUNF)
    }
    #[doc = "Timer/Counter A0 underflow high byte (split mode)"]
    #[inline(always)]
    pub fn tca0_hunf(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCCH1_A::TCA0_HUNF)
    }
    #[doc = "Timer/Counter A0 compare 0"]
    #[inline(always)]
    pub fn tca0_cmp0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCCH1_A::TCA0_CMP0)
    }
    #[doc = "Timer/Counter A0 compare 1"]
    #[inline(always)]
    pub fn tca0_cmp1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCCH1_A::TCA0_CMP1)
    }
    #[doc = "Timer/Counter A0 compare 2"]
    #[inline(always)]
    pub fn tca0_cmp2(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCCH1_A::TCA0_CMP2)
    }
    #[doc = "Synchronous Event from Pin PB0"]
    #[inline(always)]
    pub fn portb_pin0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCCH1_A::PORTB_PIN0)
    }
    #[doc = "Synchronous Event from Pin PB1"]
    #[inline(always)]
    pub fn portb_pin1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCCH1_A::PORTB_PIN1)
    }
    #[doc = "Synchronous Event from Pin PB2"]
    #[inline(always)]
    pub fn portb_pin2(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCCH1_A::PORTB_PIN2)
    }
    #[doc = "Synchronous Event from Pin PB3"]
    #[inline(always)]
    pub fn portb_pin3(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCCH1_A::PORTB_PIN3)
    }
    #[doc = "Synchronous Event from Pin PB4"]
    #[inline(always)]
    pub fn portb_pin4(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCCH1_A::PORTB_PIN4)
    }
    #[doc = "Synchronous Event from Pin PB5"]
    #[inline(always)]
    pub fn portb_pin5(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCCH1_A::PORTB_PIN5)
    }
    #[doc = "Synchronous Event from Pin PB6"]
    #[inline(always)]
    pub fn portb_pin6(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCCH1_A::PORTB_PIN6)
    }
    #[doc = "Synchronous Event from Pin PB7"]
    #[inline(always)]
    pub fn portb_pin7(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCCH1_A::PORTB_PIN7)
    }
    #[doc = "Timer/Counter B1"]
    #[inline(always)]
    pub fn tcb1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCCH1_A::TCB1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Synchronous Channel 1 Generator Selection"]
    #[inline(always)]
    pub fn syncch1(&self) -> SYNCCH1_R {
        SYNCCH1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronous Channel 1 Generator Selection"]
    #[inline(always)]
    #[must_use]
    pub fn syncch1(&mut self) -> SYNCCH1_W<SYNCCH1_SPEC> {
        SYNCCH1_W::new(self, 0)
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
#[doc = "Synchronous Channel 1 Generator Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncch1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syncch1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNCCH1_SPEC;
impl crate::RegisterSpec for SYNCCH1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`syncch1::R`](R) reader structure"]
impl crate::Readable for SYNCCH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syncch1::W`](W) writer structure"]
impl crate::Writable for SYNCCH1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCCH1 to value 0"]
impl crate::Resettable for SYNCCH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
