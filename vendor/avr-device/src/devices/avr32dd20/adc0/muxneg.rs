#[doc = "Register `MUXNEG` reader"]
pub type R = crate::R<MUXNEG_SPEC>;
#[doc = "Register `MUXNEG` writer"]
pub type W = crate::W<MUXNEG_SPEC>;
#[doc = "Field `MUXNEG` reader - Analog Channel Selection Bits"]
pub type MUXNEG_R = crate::FieldReader<MUXNEG_A>;
#[doc = "Analog Channel Selection Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUXNEG_A {
    #[doc = "1: ADC input pin 1"]
    AIN1 = 1,
    #[doc = "2: ADC input pin 2"]
    AIN2 = 2,
    #[doc = "3: ADC input pin 3"]
    AIN3 = 3,
    #[doc = "4: ADC input pin 4"]
    AIN4 = 4,
    #[doc = "5: ADC input pin 5"]
    AIN5 = 5,
    #[doc = "6: ADC input pin 6"]
    AIN6 = 6,
    #[doc = "7: ADC input pin 7"]
    AIN7 = 7,
    #[doc = "16: ADC input pin 16"]
    AIN16 = 16,
    #[doc = "17: ADC input pin 17"]
    AIN17 = 17,
    #[doc = "18: ADC input pin 18"]
    AIN18 = 18,
    #[doc = "19: ADC input pin 19"]
    AIN19 = 19,
    #[doc = "20: ADC input pin 20"]
    AIN20 = 20,
    #[doc = "21: ADC input pin 21"]
    AIN21 = 21,
    #[doc = "22: ADC input pin 22"]
    AIN22 = 22,
    #[doc = "23: ADC input pin 23"]
    AIN23 = 23,
    #[doc = "24: ADC input pin 24"]
    AIN24 = 24,
    #[doc = "25: ADC input pin 25"]
    AIN25 = 25,
    #[doc = "26: ADC input pin 26"]
    AIN26 = 26,
    #[doc = "27: ADC input pin 27"]
    AIN27 = 27,
    #[doc = "28: ADC input pin 28"]
    AIN28 = 28,
    #[doc = "29: ADC input pin 29"]
    AIN29 = 29,
    #[doc = "30: ADC input pin 30"]
    AIN30 = 30,
    #[doc = "31: ADC input pin 31"]
    AIN31 = 31,
    #[doc = "64: Ground"]
    GND = 64,
    #[doc = "72: DAC0"]
    DAC0 = 72,
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
            1 => Some(MUXNEG_A::AIN1),
            2 => Some(MUXNEG_A::AIN2),
            3 => Some(MUXNEG_A::AIN3),
            4 => Some(MUXNEG_A::AIN4),
            5 => Some(MUXNEG_A::AIN5),
            6 => Some(MUXNEG_A::AIN6),
            7 => Some(MUXNEG_A::AIN7),
            16 => Some(MUXNEG_A::AIN16),
            17 => Some(MUXNEG_A::AIN17),
            18 => Some(MUXNEG_A::AIN18),
            19 => Some(MUXNEG_A::AIN19),
            20 => Some(MUXNEG_A::AIN20),
            21 => Some(MUXNEG_A::AIN21),
            22 => Some(MUXNEG_A::AIN22),
            23 => Some(MUXNEG_A::AIN23),
            24 => Some(MUXNEG_A::AIN24),
            25 => Some(MUXNEG_A::AIN25),
            26 => Some(MUXNEG_A::AIN26),
            27 => Some(MUXNEG_A::AIN27),
            28 => Some(MUXNEG_A::AIN28),
            29 => Some(MUXNEG_A::AIN29),
            30 => Some(MUXNEG_A::AIN30),
            31 => Some(MUXNEG_A::AIN31),
            64 => Some(MUXNEG_A::GND),
            72 => Some(MUXNEG_A::DAC0),
            _ => None,
        }
    }
    #[doc = "ADC input pin 1"]
    #[inline(always)]
    pub fn is_ain1(&self) -> bool {
        *self == MUXNEG_A::AIN1
    }
    #[doc = "ADC input pin 2"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == MUXNEG_A::AIN2
    }
    #[doc = "ADC input pin 3"]
    #[inline(always)]
    pub fn is_ain3(&self) -> bool {
        *self == MUXNEG_A::AIN3
    }
    #[doc = "ADC input pin 4"]
    #[inline(always)]
    pub fn is_ain4(&self) -> bool {
        *self == MUXNEG_A::AIN4
    }
    #[doc = "ADC input pin 5"]
    #[inline(always)]
    pub fn is_ain5(&self) -> bool {
        *self == MUXNEG_A::AIN5
    }
    #[doc = "ADC input pin 6"]
    #[inline(always)]
    pub fn is_ain6(&self) -> bool {
        *self == MUXNEG_A::AIN6
    }
    #[doc = "ADC input pin 7"]
    #[inline(always)]
    pub fn is_ain7(&self) -> bool {
        *self == MUXNEG_A::AIN7
    }
    #[doc = "ADC input pin 16"]
    #[inline(always)]
    pub fn is_ain16(&self) -> bool {
        *self == MUXNEG_A::AIN16
    }
    #[doc = "ADC input pin 17"]
    #[inline(always)]
    pub fn is_ain17(&self) -> bool {
        *self == MUXNEG_A::AIN17
    }
    #[doc = "ADC input pin 18"]
    #[inline(always)]
    pub fn is_ain18(&self) -> bool {
        *self == MUXNEG_A::AIN18
    }
    #[doc = "ADC input pin 19"]
    #[inline(always)]
    pub fn is_ain19(&self) -> bool {
        *self == MUXNEG_A::AIN19
    }
    #[doc = "ADC input pin 20"]
    #[inline(always)]
    pub fn is_ain20(&self) -> bool {
        *self == MUXNEG_A::AIN20
    }
    #[doc = "ADC input pin 21"]
    #[inline(always)]
    pub fn is_ain21(&self) -> bool {
        *self == MUXNEG_A::AIN21
    }
    #[doc = "ADC input pin 22"]
    #[inline(always)]
    pub fn is_ain22(&self) -> bool {
        *self == MUXNEG_A::AIN22
    }
    #[doc = "ADC input pin 23"]
    #[inline(always)]
    pub fn is_ain23(&self) -> bool {
        *self == MUXNEG_A::AIN23
    }
    #[doc = "ADC input pin 24"]
    #[inline(always)]
    pub fn is_ain24(&self) -> bool {
        *self == MUXNEG_A::AIN24
    }
    #[doc = "ADC input pin 25"]
    #[inline(always)]
    pub fn is_ain25(&self) -> bool {
        *self == MUXNEG_A::AIN25
    }
    #[doc = "ADC input pin 26"]
    #[inline(always)]
    pub fn is_ain26(&self) -> bool {
        *self == MUXNEG_A::AIN26
    }
    #[doc = "ADC input pin 27"]
    #[inline(always)]
    pub fn is_ain27(&self) -> bool {
        *self == MUXNEG_A::AIN27
    }
    #[doc = "ADC input pin 28"]
    #[inline(always)]
    pub fn is_ain28(&self) -> bool {
        *self == MUXNEG_A::AIN28
    }
    #[doc = "ADC input pin 29"]
    #[inline(always)]
    pub fn is_ain29(&self) -> bool {
        *self == MUXNEG_A::AIN29
    }
    #[doc = "ADC input pin 30"]
    #[inline(always)]
    pub fn is_ain30(&self) -> bool {
        *self == MUXNEG_A::AIN30
    }
    #[doc = "ADC input pin 31"]
    #[inline(always)]
    pub fn is_ain31(&self) -> bool {
        *self == MUXNEG_A::AIN31
    }
    #[doc = "Ground"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == MUXNEG_A::GND
    }
    #[doc = "DAC0"]
    #[inline(always)]
    pub fn is_dac0(&self) -> bool {
        *self == MUXNEG_A::DAC0
    }
}
#[doc = "Field `MUXNEG` writer - Analog Channel Selection Bits"]
pub type MUXNEG_W<'a, REG> = crate::FieldWriter<'a, REG, 7, MUXNEG_A>;
impl<'a, REG> MUXNEG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC input pin 1"]
    #[inline(always)]
    pub fn ain1(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN1)
    }
    #[doc = "ADC input pin 2"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN2)
    }
    #[doc = "ADC input pin 3"]
    #[inline(always)]
    pub fn ain3(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN3)
    }
    #[doc = "ADC input pin 4"]
    #[inline(always)]
    pub fn ain4(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN4)
    }
    #[doc = "ADC input pin 5"]
    #[inline(always)]
    pub fn ain5(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN5)
    }
    #[doc = "ADC input pin 6"]
    #[inline(always)]
    pub fn ain6(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN6)
    }
    #[doc = "ADC input pin 7"]
    #[inline(always)]
    pub fn ain7(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN7)
    }
    #[doc = "ADC input pin 16"]
    #[inline(always)]
    pub fn ain16(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN16)
    }
    #[doc = "ADC input pin 17"]
    #[inline(always)]
    pub fn ain17(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN17)
    }
    #[doc = "ADC input pin 18"]
    #[inline(always)]
    pub fn ain18(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN18)
    }
    #[doc = "ADC input pin 19"]
    #[inline(always)]
    pub fn ain19(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN19)
    }
    #[doc = "ADC input pin 20"]
    #[inline(always)]
    pub fn ain20(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN20)
    }
    #[doc = "ADC input pin 21"]
    #[inline(always)]
    pub fn ain21(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN21)
    }
    #[doc = "ADC input pin 22"]
    #[inline(always)]
    pub fn ain22(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN22)
    }
    #[doc = "ADC input pin 23"]
    #[inline(always)]
    pub fn ain23(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN23)
    }
    #[doc = "ADC input pin 24"]
    #[inline(always)]
    pub fn ain24(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN24)
    }
    #[doc = "ADC input pin 25"]
    #[inline(always)]
    pub fn ain25(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN25)
    }
    #[doc = "ADC input pin 26"]
    #[inline(always)]
    pub fn ain26(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN26)
    }
    #[doc = "ADC input pin 27"]
    #[inline(always)]
    pub fn ain27(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN27)
    }
    #[doc = "ADC input pin 28"]
    #[inline(always)]
    pub fn ain28(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN28)
    }
    #[doc = "ADC input pin 29"]
    #[inline(always)]
    pub fn ain29(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN29)
    }
    #[doc = "ADC input pin 30"]
    #[inline(always)]
    pub fn ain30(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN30)
    }
    #[doc = "ADC input pin 31"]
    #[inline(always)]
    pub fn ain31(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::AIN31)
    }
    #[doc = "Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::GND)
    }
    #[doc = "DAC0"]
    #[inline(always)]
    pub fn dac0(self) -> &'a mut crate::W<REG> {
        self.variant(MUXNEG_A::DAC0)
    }
}
impl R {
    #[doc = "Bits 0:6 - Analog Channel Selection Bits"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Analog Channel Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn muxneg(&mut self) -> MUXNEG_W<MUXNEG_SPEC> {
        MUXNEG_W::new(self, 0)
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
#[doc = "Negative mux input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxneg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxneg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXNEG_SPEC;
impl crate::RegisterSpec for MUXNEG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`muxneg::R`](R) reader structure"]
impl crate::Readable for MUXNEG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxneg::W`](W) writer structure"]
impl crate::Writable for MUXNEG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXNEG to value 0"]
impl crate::Resettable for MUXNEG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
