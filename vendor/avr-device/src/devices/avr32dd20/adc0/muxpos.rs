#[doc = "Register `MUXPOS` reader"]
pub type R = crate::R<MUXPOS_SPEC>;
#[doc = "Register `MUXPOS` writer"]
pub type W = crate::W<MUXPOS_SPEC>;
#[doc = "Field `MUXPOS` reader - Analog Channel Selection Bits"]
pub type MUXPOS_R = crate::FieldReader<MUXPOS_A>;
#[doc = "Analog Channel Selection Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUXPOS_A {
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
    #[doc = "66: Temperature sensor"]
    TEMPSENSE = 66,
    #[doc = "68: VDD/10"]
    VDDDIV10 = 68,
    #[doc = "69: VDDIO2/10"]
    VDDIO2DIV10 = 69,
    #[doc = "72: DAC0"]
    DAC0 = 72,
    #[doc = "73: DACREF0"]
    DACREF0 = 73,
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
    pub const fn variant(&self) -> Option<MUXPOS_A> {
        match self.bits {
            1 => Some(MUXPOS_A::AIN1),
            2 => Some(MUXPOS_A::AIN2),
            3 => Some(MUXPOS_A::AIN3),
            4 => Some(MUXPOS_A::AIN4),
            5 => Some(MUXPOS_A::AIN5),
            6 => Some(MUXPOS_A::AIN6),
            7 => Some(MUXPOS_A::AIN7),
            16 => Some(MUXPOS_A::AIN16),
            17 => Some(MUXPOS_A::AIN17),
            18 => Some(MUXPOS_A::AIN18),
            19 => Some(MUXPOS_A::AIN19),
            20 => Some(MUXPOS_A::AIN20),
            21 => Some(MUXPOS_A::AIN21),
            22 => Some(MUXPOS_A::AIN22),
            23 => Some(MUXPOS_A::AIN23),
            24 => Some(MUXPOS_A::AIN24),
            25 => Some(MUXPOS_A::AIN25),
            26 => Some(MUXPOS_A::AIN26),
            27 => Some(MUXPOS_A::AIN27),
            28 => Some(MUXPOS_A::AIN28),
            29 => Some(MUXPOS_A::AIN29),
            30 => Some(MUXPOS_A::AIN30),
            31 => Some(MUXPOS_A::AIN31),
            64 => Some(MUXPOS_A::GND),
            66 => Some(MUXPOS_A::TEMPSENSE),
            68 => Some(MUXPOS_A::VDDDIV10),
            69 => Some(MUXPOS_A::VDDIO2DIV10),
            72 => Some(MUXPOS_A::DAC0),
            73 => Some(MUXPOS_A::DACREF0),
            _ => None,
        }
    }
    #[doc = "ADC input pin 1"]
    #[inline(always)]
    pub fn is_ain1(&self) -> bool {
        *self == MUXPOS_A::AIN1
    }
    #[doc = "ADC input pin 2"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == MUXPOS_A::AIN2
    }
    #[doc = "ADC input pin 3"]
    #[inline(always)]
    pub fn is_ain3(&self) -> bool {
        *self == MUXPOS_A::AIN3
    }
    #[doc = "ADC input pin 4"]
    #[inline(always)]
    pub fn is_ain4(&self) -> bool {
        *self == MUXPOS_A::AIN4
    }
    #[doc = "ADC input pin 5"]
    #[inline(always)]
    pub fn is_ain5(&self) -> bool {
        *self == MUXPOS_A::AIN5
    }
    #[doc = "ADC input pin 6"]
    #[inline(always)]
    pub fn is_ain6(&self) -> bool {
        *self == MUXPOS_A::AIN6
    }
    #[doc = "ADC input pin 7"]
    #[inline(always)]
    pub fn is_ain7(&self) -> bool {
        *self == MUXPOS_A::AIN7
    }
    #[doc = "ADC input pin 16"]
    #[inline(always)]
    pub fn is_ain16(&self) -> bool {
        *self == MUXPOS_A::AIN16
    }
    #[doc = "ADC input pin 17"]
    #[inline(always)]
    pub fn is_ain17(&self) -> bool {
        *self == MUXPOS_A::AIN17
    }
    #[doc = "ADC input pin 18"]
    #[inline(always)]
    pub fn is_ain18(&self) -> bool {
        *self == MUXPOS_A::AIN18
    }
    #[doc = "ADC input pin 19"]
    #[inline(always)]
    pub fn is_ain19(&self) -> bool {
        *self == MUXPOS_A::AIN19
    }
    #[doc = "ADC input pin 20"]
    #[inline(always)]
    pub fn is_ain20(&self) -> bool {
        *self == MUXPOS_A::AIN20
    }
    #[doc = "ADC input pin 21"]
    #[inline(always)]
    pub fn is_ain21(&self) -> bool {
        *self == MUXPOS_A::AIN21
    }
    #[doc = "ADC input pin 22"]
    #[inline(always)]
    pub fn is_ain22(&self) -> bool {
        *self == MUXPOS_A::AIN22
    }
    #[doc = "ADC input pin 23"]
    #[inline(always)]
    pub fn is_ain23(&self) -> bool {
        *self == MUXPOS_A::AIN23
    }
    #[doc = "ADC input pin 24"]
    #[inline(always)]
    pub fn is_ain24(&self) -> bool {
        *self == MUXPOS_A::AIN24
    }
    #[doc = "ADC input pin 25"]
    #[inline(always)]
    pub fn is_ain25(&self) -> bool {
        *self == MUXPOS_A::AIN25
    }
    #[doc = "ADC input pin 26"]
    #[inline(always)]
    pub fn is_ain26(&self) -> bool {
        *self == MUXPOS_A::AIN26
    }
    #[doc = "ADC input pin 27"]
    #[inline(always)]
    pub fn is_ain27(&self) -> bool {
        *self == MUXPOS_A::AIN27
    }
    #[doc = "ADC input pin 28"]
    #[inline(always)]
    pub fn is_ain28(&self) -> bool {
        *self == MUXPOS_A::AIN28
    }
    #[doc = "ADC input pin 29"]
    #[inline(always)]
    pub fn is_ain29(&self) -> bool {
        *self == MUXPOS_A::AIN29
    }
    #[doc = "ADC input pin 30"]
    #[inline(always)]
    pub fn is_ain30(&self) -> bool {
        *self == MUXPOS_A::AIN30
    }
    #[doc = "ADC input pin 31"]
    #[inline(always)]
    pub fn is_ain31(&self) -> bool {
        *self == MUXPOS_A::AIN31
    }
    #[doc = "Ground"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == MUXPOS_A::GND
    }
    #[doc = "Temperature sensor"]
    #[inline(always)]
    pub fn is_tempsense(&self) -> bool {
        *self == MUXPOS_A::TEMPSENSE
    }
    #[doc = "VDD/10"]
    #[inline(always)]
    pub fn is_vdddiv10(&self) -> bool {
        *self == MUXPOS_A::VDDDIV10
    }
    #[doc = "VDDIO2/10"]
    #[inline(always)]
    pub fn is_vddio2div10(&self) -> bool {
        *self == MUXPOS_A::VDDIO2DIV10
    }
    #[doc = "DAC0"]
    #[inline(always)]
    pub fn is_dac0(&self) -> bool {
        *self == MUXPOS_A::DAC0
    }
    #[doc = "DACREF0"]
    #[inline(always)]
    pub fn is_dacref0(&self) -> bool {
        *self == MUXPOS_A::DACREF0
    }
}
#[doc = "Field `MUXPOS` writer - Analog Channel Selection Bits"]
pub type MUXPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 7, MUXPOS_A>;
impl<'a, REG> MUXPOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC input pin 1"]
    #[inline(always)]
    pub fn ain1(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN1)
    }
    #[doc = "ADC input pin 2"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN2)
    }
    #[doc = "ADC input pin 3"]
    #[inline(always)]
    pub fn ain3(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN3)
    }
    #[doc = "ADC input pin 4"]
    #[inline(always)]
    pub fn ain4(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN4)
    }
    #[doc = "ADC input pin 5"]
    #[inline(always)]
    pub fn ain5(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN5)
    }
    #[doc = "ADC input pin 6"]
    #[inline(always)]
    pub fn ain6(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN6)
    }
    #[doc = "ADC input pin 7"]
    #[inline(always)]
    pub fn ain7(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN7)
    }
    #[doc = "ADC input pin 16"]
    #[inline(always)]
    pub fn ain16(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN16)
    }
    #[doc = "ADC input pin 17"]
    #[inline(always)]
    pub fn ain17(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN17)
    }
    #[doc = "ADC input pin 18"]
    #[inline(always)]
    pub fn ain18(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN18)
    }
    #[doc = "ADC input pin 19"]
    #[inline(always)]
    pub fn ain19(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN19)
    }
    #[doc = "ADC input pin 20"]
    #[inline(always)]
    pub fn ain20(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN20)
    }
    #[doc = "ADC input pin 21"]
    #[inline(always)]
    pub fn ain21(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN21)
    }
    #[doc = "ADC input pin 22"]
    #[inline(always)]
    pub fn ain22(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN22)
    }
    #[doc = "ADC input pin 23"]
    #[inline(always)]
    pub fn ain23(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN23)
    }
    #[doc = "ADC input pin 24"]
    #[inline(always)]
    pub fn ain24(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN24)
    }
    #[doc = "ADC input pin 25"]
    #[inline(always)]
    pub fn ain25(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN25)
    }
    #[doc = "ADC input pin 26"]
    #[inline(always)]
    pub fn ain26(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN26)
    }
    #[doc = "ADC input pin 27"]
    #[inline(always)]
    pub fn ain27(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN27)
    }
    #[doc = "ADC input pin 28"]
    #[inline(always)]
    pub fn ain28(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN28)
    }
    #[doc = "ADC input pin 29"]
    #[inline(always)]
    pub fn ain29(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN29)
    }
    #[doc = "ADC input pin 30"]
    #[inline(always)]
    pub fn ain30(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN30)
    }
    #[doc = "ADC input pin 31"]
    #[inline(always)]
    pub fn ain31(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::AIN31)
    }
    #[doc = "Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::GND)
    }
    #[doc = "Temperature sensor"]
    #[inline(always)]
    pub fn tempsense(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::TEMPSENSE)
    }
    #[doc = "VDD/10"]
    #[inline(always)]
    pub fn vdddiv10(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::VDDDIV10)
    }
    #[doc = "VDDIO2/10"]
    #[inline(always)]
    pub fn vddio2div10(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::VDDIO2DIV10)
    }
    #[doc = "DAC0"]
    #[inline(always)]
    pub fn dac0(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::DAC0)
    }
    #[doc = "DACREF0"]
    #[inline(always)]
    pub fn dacref0(self) -> &'a mut crate::W<REG> {
        self.variant(MUXPOS_A::DACREF0)
    }
}
impl R {
    #[doc = "Bits 0:6 - Analog Channel Selection Bits"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Analog Channel Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn muxpos(&mut self) -> MUXPOS_W<MUXPOS_SPEC> {
        MUXPOS_W::new(self, 0)
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
#[doc = "Positive mux input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxpos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxpos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXPOS_SPEC;
impl crate::RegisterSpec for MUXPOS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`muxpos::R`](R) reader structure"]
impl crate::Readable for MUXPOS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxpos::W`](W) writer structure"]
impl crate::Writable for MUXPOS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXPOS to value 0"]
impl crate::Resettable for MUXPOS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
