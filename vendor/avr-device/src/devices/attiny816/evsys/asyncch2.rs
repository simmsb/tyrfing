#[doc = "Register `ASYNCCH2` reader"]
pub type R = crate::R<ASYNCCH2_SPEC>;
#[doc = "Register `ASYNCCH2` writer"]
pub type W = crate::W<ASYNCCH2_SPEC>;
#[doc = "Field `ASYNCCH2` reader - Asynchronous Channel 2 Generator Selection"]
pub type ASYNCCH2_R = crate::FieldReader<ASYNCCH2_A>;
#[doc = "Asynchronous Channel 2 Generator Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ASYNCCH2_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Configurable Custom Logic LUT0"]
    CCL_LUT0 = 1,
    #[doc = "2: Configurable Custom Logic LUT1"]
    CCL_LUT1 = 2,
    #[doc = "3: Analog Comparator 0 out"]
    AC0_OUT = 3,
    #[doc = "4: Timer/Counter D0 compare B clear"]
    TCD0_CMPBCLR = 4,
    #[doc = "5: Timer/Counter D0 compare A set"]
    TCD0_CMPASET = 5,
    #[doc = "6: Timer/Counter D0 compare B set"]
    TCD0_CMPBSET = 6,
    #[doc = "7: Timer/Counter D0 program event"]
    TCD0_PROGEV = 7,
    #[doc = "8: Real Time Counter overflow"]
    RTC_OVF = 8,
    #[doc = "9: Real Time Counter compare"]
    RTC_CMP = 9,
    #[doc = "10: Asynchronous Event from Pin PC0"]
    PORTC_PIN0 = 10,
    #[doc = "11: Asynchronous Event from Pin PC1"]
    PORTC_PIN1 = 11,
    #[doc = "12: Asynchronous Event from Pin PC2"]
    PORTC_PIN2 = 12,
    #[doc = "13: Asynchronous Event from Pin PC3"]
    PORTC_PIN3 = 13,
    #[doc = "14: Asynchronous Event from Pin PC4"]
    PORTC_PIN4 = 14,
    #[doc = "15: Asynchronous Event from Pin PC5"]
    PORTC_PIN5 = 15,
}
impl From<ASYNCCH2_A> for u8 {
    #[inline(always)]
    fn from(variant: ASYNCCH2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ASYNCCH2_A {
    type Ux = u8;
}
impl ASYNCCH2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ASYNCCH2_A> {
        match self.bits {
            0 => Some(ASYNCCH2_A::OFF),
            1 => Some(ASYNCCH2_A::CCL_LUT0),
            2 => Some(ASYNCCH2_A::CCL_LUT1),
            3 => Some(ASYNCCH2_A::AC0_OUT),
            4 => Some(ASYNCCH2_A::TCD0_CMPBCLR),
            5 => Some(ASYNCCH2_A::TCD0_CMPASET),
            6 => Some(ASYNCCH2_A::TCD0_CMPBSET),
            7 => Some(ASYNCCH2_A::TCD0_PROGEV),
            8 => Some(ASYNCCH2_A::RTC_OVF),
            9 => Some(ASYNCCH2_A::RTC_CMP),
            10 => Some(ASYNCCH2_A::PORTC_PIN0),
            11 => Some(ASYNCCH2_A::PORTC_PIN1),
            12 => Some(ASYNCCH2_A::PORTC_PIN2),
            13 => Some(ASYNCCH2_A::PORTC_PIN3),
            14 => Some(ASYNCCH2_A::PORTC_PIN4),
            15 => Some(ASYNCCH2_A::PORTC_PIN5),
            _ => None,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ASYNCCH2_A::OFF
    }
    #[doc = "Configurable Custom Logic LUT0"]
    #[inline(always)]
    pub fn is_ccl_lut0(&self) -> bool {
        *self == ASYNCCH2_A::CCL_LUT0
    }
    #[doc = "Configurable Custom Logic LUT1"]
    #[inline(always)]
    pub fn is_ccl_lut1(&self) -> bool {
        *self == ASYNCCH2_A::CCL_LUT1
    }
    #[doc = "Analog Comparator 0 out"]
    #[inline(always)]
    pub fn is_ac0_out(&self) -> bool {
        *self == ASYNCCH2_A::AC0_OUT
    }
    #[doc = "Timer/Counter D0 compare B clear"]
    #[inline(always)]
    pub fn is_tcd0_cmpbclr(&self) -> bool {
        *self == ASYNCCH2_A::TCD0_CMPBCLR
    }
    #[doc = "Timer/Counter D0 compare A set"]
    #[inline(always)]
    pub fn is_tcd0_cmpaset(&self) -> bool {
        *self == ASYNCCH2_A::TCD0_CMPASET
    }
    #[doc = "Timer/Counter D0 compare B set"]
    #[inline(always)]
    pub fn is_tcd0_cmpbset(&self) -> bool {
        *self == ASYNCCH2_A::TCD0_CMPBSET
    }
    #[doc = "Timer/Counter D0 program event"]
    #[inline(always)]
    pub fn is_tcd0_progev(&self) -> bool {
        *self == ASYNCCH2_A::TCD0_PROGEV
    }
    #[doc = "Real Time Counter overflow"]
    #[inline(always)]
    pub fn is_rtc_ovf(&self) -> bool {
        *self == ASYNCCH2_A::RTC_OVF
    }
    #[doc = "Real Time Counter compare"]
    #[inline(always)]
    pub fn is_rtc_cmp(&self) -> bool {
        *self == ASYNCCH2_A::RTC_CMP
    }
    #[doc = "Asynchronous Event from Pin PC0"]
    #[inline(always)]
    pub fn is_portc_pin0(&self) -> bool {
        *self == ASYNCCH2_A::PORTC_PIN0
    }
    #[doc = "Asynchronous Event from Pin PC1"]
    #[inline(always)]
    pub fn is_portc_pin1(&self) -> bool {
        *self == ASYNCCH2_A::PORTC_PIN1
    }
    #[doc = "Asynchronous Event from Pin PC2"]
    #[inline(always)]
    pub fn is_portc_pin2(&self) -> bool {
        *self == ASYNCCH2_A::PORTC_PIN2
    }
    #[doc = "Asynchronous Event from Pin PC3"]
    #[inline(always)]
    pub fn is_portc_pin3(&self) -> bool {
        *self == ASYNCCH2_A::PORTC_PIN3
    }
    #[doc = "Asynchronous Event from Pin PC4"]
    #[inline(always)]
    pub fn is_portc_pin4(&self) -> bool {
        *self == ASYNCCH2_A::PORTC_PIN4
    }
    #[doc = "Asynchronous Event from Pin PC5"]
    #[inline(always)]
    pub fn is_portc_pin5(&self) -> bool {
        *self == ASYNCCH2_A::PORTC_PIN5
    }
}
#[doc = "Field `ASYNCCH2` writer - Asynchronous Channel 2 Generator Selection"]
pub type ASYNCCH2_W<'a, REG> = crate::FieldWriter<'a, REG, 8, ASYNCCH2_A>;
impl<'a, REG> ASYNCCH2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCCH2_A::OFF)
    }
    #[doc = "Configurable Custom Logic LUT0"]
    #[inline(always)]
    pub fn ccl_lut0(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCCH2_A::CCL_LUT0)
    }
    #[doc = "Configurable Custom Logic LUT1"]
    #[inline(always)]
    pub fn ccl_lut1(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCCH2_A::CCL_LUT1)
    }
    #[doc = "Analog Comparator 0 out"]
    #[inline(always)]
    pub fn ac0_out(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCCH2_A::AC0_OUT)
    }
    #[doc = "Timer/Counter D0 compare B clear"]
    #[inline(always)]
    pub fn tcd0_cmpbclr(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCCH2_A::TCD0_CMPBCLR)
    }
    #[doc = "Timer/Counter D0 compare A set"]
    #[inline(always)]
    pub fn tcd0_cmpaset(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCCH2_A::TCD0_CMPASET)
    }
    #[doc = "Timer/Counter D0 compare B set"]
    #[inline(always)]
    pub fn tcd0_cmpbset(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCCH2_A::TCD0_CMPBSET)
    }
    #[doc = "Timer/Counter D0 program event"]
    #[inline(always)]
    pub fn tcd0_progev(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCCH2_A::TCD0_PROGEV)
    }
    #[doc = "Real Time Counter overflow"]
    #[inline(always)]
    pub fn rtc_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCCH2_A::RTC_OVF)
    }
    #[doc = "Real Time Counter compare"]
    #[inline(always)]
    pub fn rtc_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCCH2_A::RTC_CMP)
    }
    #[doc = "Asynchronous Event from Pin PC0"]
    #[inline(always)]
    pub fn portc_pin0(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCCH2_A::PORTC_PIN0)
    }
    #[doc = "Asynchronous Event from Pin PC1"]
    #[inline(always)]
    pub fn portc_pin1(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCCH2_A::PORTC_PIN1)
    }
    #[doc = "Asynchronous Event from Pin PC2"]
    #[inline(always)]
    pub fn portc_pin2(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCCH2_A::PORTC_PIN2)
    }
    #[doc = "Asynchronous Event from Pin PC3"]
    #[inline(always)]
    pub fn portc_pin3(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCCH2_A::PORTC_PIN3)
    }
    #[doc = "Asynchronous Event from Pin PC4"]
    #[inline(always)]
    pub fn portc_pin4(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCCH2_A::PORTC_PIN4)
    }
    #[doc = "Asynchronous Event from Pin PC5"]
    #[inline(always)]
    pub fn portc_pin5(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCCH2_A::PORTC_PIN5)
    }
}
impl R {
    #[doc = "Bits 0:7 - Asynchronous Channel 2 Generator Selection"]
    #[inline(always)]
    pub fn asyncch2(&self) -> ASYNCCH2_R {
        ASYNCCH2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Asynchronous Channel 2 Generator Selection"]
    #[inline(always)]
    #[must_use]
    pub fn asyncch2(&mut self) -> ASYNCCH2_W<ASYNCCH2_SPEC> {
        ASYNCCH2_W::new(self, 0)
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
#[doc = "Asynchronous Channel 2 Generator Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncch2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncch2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASYNCCH2_SPEC;
impl crate::RegisterSpec for ASYNCCH2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`asyncch2::R`](R) reader structure"]
impl crate::Readable for ASYNCCH2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`asyncch2::W`](W) writer structure"]
impl crate::Writable for ASYNCCH2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASYNCCH2 to value 0"]
impl crate::Resettable for ASYNCCH2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
