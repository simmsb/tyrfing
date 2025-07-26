#[doc = "Register `ADMUX` reader"]
pub type R = crate::R<ADMUX_SPEC>;
#[doc = "Register `ADMUX` writer"]
pub type W = crate::W<ADMUX_SPEC>;
#[doc = "Field `MUX` reader - Analog Channel and Gain Selection Bits"]
pub type MUX_R = crate::FieldReader<MUX_A>;
#[doc = "Analog Channel and Gain Selection Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUX_A {
    #[doc = "0: Single-ended Input ADC0"]
    ADC0 = 0,
    #[doc = "1: Single-ended Input ADC1"]
    ADC1 = 1,
    #[doc = "2: Single-ended Input ADC2"]
    ADC2 = 2,
    #[doc = "3: Single-ended Input ADC3"]
    ADC3 = 3,
    #[doc = "4: Differential Inputs Positive ADC2 Negative ADC2 1x Gain"]
    ADC2_ADC2_1X = 4,
    #[doc = "5: Differential Inputs Positive ADC2 Negative ADC2 20x Gain"]
    ADC2_ADC2_20X = 5,
    #[doc = "6: Differential Inputs Positive ADC2 Negative ADC3 1x Gain"]
    ADC2_ADC3_1X = 6,
    #[doc = "7: Differential Inputs Positive ADC2 Negative ADC3 20x Gain"]
    ADC2_ADC3_20X = 7,
    #[doc = "8: Differential Inputs Positive ADC0 Negative ADC0 1x Gain"]
    ADC0_ADC0_1X = 8,
    #[doc = "9: Differential Inputs Positive ADC0 Negative ADC0 20x Gain"]
    ADC0_ADC0_20X = 9,
    #[doc = "10: Differential Inputs Positive ADC0 Negative ADC1 1x Gain"]
    ADC0_ADC1_1X = 10,
    #[doc = "11: Differential Inputs Positive ADC0 Negative ADC1 20x Gain"]
    ADC0_ADC1_20X = 11,
    #[doc = "12: Internal Reference (VBG)"]
    ADC_VBG = 12,
    #[doc = "13: 0V (GND)"]
    ADC_GND = 13,
    #[doc = "15: Temperature sensor"]
    TEMPSENS = 15,
}
impl From<MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MUX_A {
    type Ux = u8;
}
impl MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MUX_A> {
        match self.bits {
            0 => Some(MUX_A::ADC0),
            1 => Some(MUX_A::ADC1),
            2 => Some(MUX_A::ADC2),
            3 => Some(MUX_A::ADC3),
            4 => Some(MUX_A::ADC2_ADC2_1X),
            5 => Some(MUX_A::ADC2_ADC2_20X),
            6 => Some(MUX_A::ADC2_ADC3_1X),
            7 => Some(MUX_A::ADC2_ADC3_20X),
            8 => Some(MUX_A::ADC0_ADC0_1X),
            9 => Some(MUX_A::ADC0_ADC0_20X),
            10 => Some(MUX_A::ADC0_ADC1_1X),
            11 => Some(MUX_A::ADC0_ADC1_20X),
            12 => Some(MUX_A::ADC_VBG),
            13 => Some(MUX_A::ADC_GND),
            15 => Some(MUX_A::TEMPSENS),
            _ => None,
        }
    }
    #[doc = "Single-ended Input ADC0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == MUX_A::ADC0
    }
    #[doc = "Single-ended Input ADC1"]
    #[inline(always)]
    pub fn is_adc1(&self) -> bool {
        *self == MUX_A::ADC1
    }
    #[doc = "Single-ended Input ADC2"]
    #[inline(always)]
    pub fn is_adc2(&self) -> bool {
        *self == MUX_A::ADC2
    }
    #[doc = "Single-ended Input ADC3"]
    #[inline(always)]
    pub fn is_adc3(&self) -> bool {
        *self == MUX_A::ADC3
    }
    #[doc = "Differential Inputs Positive ADC2 Negative ADC2 1x Gain"]
    #[inline(always)]
    pub fn is_adc2_adc2_1x(&self) -> bool {
        *self == MUX_A::ADC2_ADC2_1X
    }
    #[doc = "Differential Inputs Positive ADC2 Negative ADC2 20x Gain"]
    #[inline(always)]
    pub fn is_adc2_adc2_20x(&self) -> bool {
        *self == MUX_A::ADC2_ADC2_20X
    }
    #[doc = "Differential Inputs Positive ADC2 Negative ADC3 1x Gain"]
    #[inline(always)]
    pub fn is_adc2_adc3_1x(&self) -> bool {
        *self == MUX_A::ADC2_ADC3_1X
    }
    #[doc = "Differential Inputs Positive ADC2 Negative ADC3 20x Gain"]
    #[inline(always)]
    pub fn is_adc2_adc3_20x(&self) -> bool {
        *self == MUX_A::ADC2_ADC3_20X
    }
    #[doc = "Differential Inputs Positive ADC0 Negative ADC0 1x Gain"]
    #[inline(always)]
    pub fn is_adc0_adc0_1x(&self) -> bool {
        *self == MUX_A::ADC0_ADC0_1X
    }
    #[doc = "Differential Inputs Positive ADC0 Negative ADC0 20x Gain"]
    #[inline(always)]
    pub fn is_adc0_adc0_20x(&self) -> bool {
        *self == MUX_A::ADC0_ADC0_20X
    }
    #[doc = "Differential Inputs Positive ADC0 Negative ADC1 1x Gain"]
    #[inline(always)]
    pub fn is_adc0_adc1_1x(&self) -> bool {
        *self == MUX_A::ADC0_ADC1_1X
    }
    #[doc = "Differential Inputs Positive ADC0 Negative ADC1 20x Gain"]
    #[inline(always)]
    pub fn is_adc0_adc1_20x(&self) -> bool {
        *self == MUX_A::ADC0_ADC1_20X
    }
    #[doc = "Internal Reference (VBG)"]
    #[inline(always)]
    pub fn is_adc_vbg(&self) -> bool {
        *self == MUX_A::ADC_VBG
    }
    #[doc = "0V (GND)"]
    #[inline(always)]
    pub fn is_adc_gnd(&self) -> bool {
        *self == MUX_A::ADC_GND
    }
    #[doc = "Temperature sensor"]
    #[inline(always)]
    pub fn is_tempsens(&self) -> bool {
        *self == MUX_A::TEMPSENS
    }
}
#[doc = "Field `MUX` writer - Analog Channel and Gain Selection Bits"]
pub type MUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MUX_A>;
impl<'a, REG> MUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single-ended Input ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC0)
    }
    #[doc = "Single-ended Input ADC1"]
    #[inline(always)]
    pub fn adc1(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC1)
    }
    #[doc = "Single-ended Input ADC2"]
    #[inline(always)]
    pub fn adc2(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC2)
    }
    #[doc = "Single-ended Input ADC3"]
    #[inline(always)]
    pub fn adc3(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3)
    }
    #[doc = "Differential Inputs Positive ADC2 Negative ADC2 1x Gain"]
    #[inline(always)]
    pub fn adc2_adc2_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC2_ADC2_1X)
    }
    #[doc = "Differential Inputs Positive ADC2 Negative ADC2 20x Gain"]
    #[inline(always)]
    pub fn adc2_adc2_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC2_ADC2_20X)
    }
    #[doc = "Differential Inputs Positive ADC2 Negative ADC3 1x Gain"]
    #[inline(always)]
    pub fn adc2_adc3_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC2_ADC3_1X)
    }
    #[doc = "Differential Inputs Positive ADC2 Negative ADC3 20x Gain"]
    #[inline(always)]
    pub fn adc2_adc3_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC2_ADC3_20X)
    }
    #[doc = "Differential Inputs Positive ADC0 Negative ADC0 1x Gain"]
    #[inline(always)]
    pub fn adc0_adc0_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC0_ADC0_1X)
    }
    #[doc = "Differential Inputs Positive ADC0 Negative ADC0 20x Gain"]
    #[inline(always)]
    pub fn adc0_adc0_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC0_ADC0_20X)
    }
    #[doc = "Differential Inputs Positive ADC0 Negative ADC1 1x Gain"]
    #[inline(always)]
    pub fn adc0_adc1_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC0_ADC1_1X)
    }
    #[doc = "Differential Inputs Positive ADC0 Negative ADC1 20x Gain"]
    #[inline(always)]
    pub fn adc0_adc1_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC0_ADC1_20X)
    }
    #[doc = "Internal Reference (VBG)"]
    #[inline(always)]
    pub fn adc_vbg(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC_VBG)
    }
    #[doc = "0V (GND)"]
    #[inline(always)]
    pub fn adc_gnd(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC_GND)
    }
    #[doc = "Temperature sensor"]
    #[inline(always)]
    pub fn tempsens(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::TEMPSENS)
    }
}
#[doc = "Field `REFS2` reader - Reference Selection Bit 2"]
pub type REFS2_R = crate::BitReader;
#[doc = "Field `REFS2` writer - Reference Selection Bit 2"]
pub type REFS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADLAR` reader - Left Adjust Result"]
pub type ADLAR_R = crate::BitReader;
#[doc = "Field `ADLAR` writer - Left Adjust Result"]
pub type ADLAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFS` reader - Reference Selection Bits"]
pub type REFS_R = crate::FieldReader<REFS_A>;
#[doc = "Reference Selection Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFS_A {
    #[doc = "0: Vcc used as Voltage Reference, disconnected from Aref"]
    VCC = 0,
    #[doc = "1: External Voltage Reference at AREF pin, Internal Voltage Reference turned off"]
    AREF = 1,
    #[doc = "2: Internal Voltage Reference (1.1V when REFS2 is cleared, 2.56V when REFS2 is set) without external bypass"]
    INTERNAL = 2,
    #[doc = "3: Internal 2.56V Voltage Reference with external bypass capacitor at AREF pin (REFS2 must be set)"]
    INTERNAL_BYPASS = 3,
}
impl From<REFS_A> for u8 {
    #[inline(always)]
    fn from(variant: REFS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REFS_A {
    type Ux = u8;
}
impl REFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REFS_A {
        match self.bits {
            0 => REFS_A::VCC,
            1 => REFS_A::AREF,
            2 => REFS_A::INTERNAL,
            3 => REFS_A::INTERNAL_BYPASS,
            _ => unreachable!(),
        }
    }
    #[doc = "Vcc used as Voltage Reference, disconnected from Aref"]
    #[inline(always)]
    pub fn is_vcc(&self) -> bool {
        *self == REFS_A::VCC
    }
    #[doc = "External Voltage Reference at AREF pin, Internal Voltage Reference turned off"]
    #[inline(always)]
    pub fn is_aref(&self) -> bool {
        *self == REFS_A::AREF
    }
    #[doc = "Internal Voltage Reference (1.1V when REFS2 is cleared, 2.56V when REFS2 is set) without external bypass"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == REFS_A::INTERNAL
    }
    #[doc = "Internal 2.56V Voltage Reference with external bypass capacitor at AREF pin (REFS2 must be set)"]
    #[inline(always)]
    pub fn is_internal_bypass(&self) -> bool {
        *self == REFS_A::INTERNAL_BYPASS
    }
}
#[doc = "Field `REFS` writer - Reference Selection Bits"]
pub type REFS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, REFS_A>;
impl<'a, REG> REFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Vcc used as Voltage Reference, disconnected from Aref"]
    #[inline(always)]
    pub fn vcc(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::VCC)
    }
    #[doc = "External Voltage Reference at AREF pin, Internal Voltage Reference turned off"]
    #[inline(always)]
    pub fn aref(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::AREF)
    }
    #[doc = "Internal Voltage Reference (1.1V when REFS2 is cleared, 2.56V when REFS2 is set) without external bypass"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::INTERNAL)
    }
    #[doc = "Internal 2.56V Voltage Reference with external bypass capacitor at AREF pin (REFS2 must be set)"]
    #[inline(always)]
    pub fn internal_bypass(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::INTERNAL_BYPASS)
    }
}
impl R {
    #[doc = "Bits 0:3 - Analog Channel and Gain Selection Bits"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Reference Selection Bit 2"]
    #[inline(always)]
    pub fn refs2(&self) -> REFS2_R {
        REFS2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Left Adjust Result"]
    #[inline(always)]
    pub fn adlar(&self) -> ADLAR_R {
        ADLAR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reference Selection Bits"]
    #[inline(always)]
    pub fn refs(&self) -> REFS_R {
        REFS_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:3 - Analog Channel and Gain Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mux(&mut self) -> MUX_W<ADMUX_SPEC> {
        MUX_W::new(self, 0)
    }
    #[doc = "Bit 4 - Reference Selection Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn refs2(&mut self) -> REFS2_W<ADMUX_SPEC> {
        REFS2_W::new(self, 4)
    }
    #[doc = "Bit 5 - Left Adjust Result"]
    #[inline(always)]
    #[must_use]
    pub fn adlar(&mut self) -> ADLAR_W<ADMUX_SPEC> {
        ADLAR_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Reference Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn refs(&mut self) -> REFS_W<ADMUX_SPEC> {
        REFS_W::new(self, 6)
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
#[doc = "The ADC multiplexer Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`admux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`admux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADMUX_SPEC;
impl crate::RegisterSpec for ADMUX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`admux::R`](R) reader structure"]
impl crate::Readable for ADMUX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`admux::W`](W) writer structure"]
impl crate::Writable for ADMUX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADMUX to value 0"]
impl crate::Resettable for ADMUX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
