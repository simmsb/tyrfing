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
    #[doc = "4: Single-ended Input ADC4"]
    ADC4 = 4,
    #[doc = "5: Single-ended Input ADC5"]
    ADC5 = 5,
    #[doc = "6: Single-ended Input ADC6"]
    ADC6 = 6,
    #[doc = "7: Single-ended Input ADC7"]
    ADC7 = 7,
    #[doc = "32: 0V (GND)"]
    ADC_GND = 32,
    #[doc = "33: Internal Reference (VBG)"]
    ADC_VBG = 33,
    #[doc = "34: Temperature sensor"]
    TEMPSENS = 34,
    #[doc = "35: Differential Inputs Positive ADC0 Negative ADC0 20x Gain"]
    ADC0_ADC0_20X = 35,
    #[doc = "8: Differential Inputs Positive ADC0 Negative ADC1 1x Gain"]
    ADC0_ADC1_1X = 8,
    #[doc = "9: Differential Inputs Postive ADC0 Negative ADC1 20x Gain"]
    ADC0_ADC1_20X = 9,
    #[doc = "10: Differential Inputs Positive ADC0 Negative ADC3 1x Gain"]
    ADC0_ADC3_1X = 10,
    #[doc = "11: Differential Inputs Positive ADC0 Negative ADC3 20x Gain"]
    ADC0_ADC3_20X = 11,
    #[doc = "40: Differential Inputs Positive ADC1 Negative ADC0 1x Gain"]
    ADC1_ADC0_1X = 40,
    #[doc = "41: Differential Inputs Positive ADC1 Negative ADC0 20x Gain"]
    ADC1_ADC0_20X = 41,
    #[doc = "12: Differential Inputs Positive ADC1 Negative ADC2 1x Gain"]
    ADC1_ADC2_1X = 12,
    #[doc = "13: Differential Inputs Positive ADC1 Negative ADC2 20x Gain"]
    ADC1_ADC2_20X = 13,
    #[doc = "14: Differential Inputs Positive ADC1 Negative ADC3 1x Gain"]
    ADC1_ADC3_1X = 14,
    #[doc = "15: Differential Inputs Positive ADC1 Negative ADC3 20x Gain"]
    ADC1_ADC3_20X = 15,
    #[doc = "44: Differential Inputs Positive ADC2 Negative ADC2 1x Gain"]
    ADC2_ADC1_1X = 44,
    #[doc = "45: Differential Inputs Positive ADC2 Negative ADC2 20x Gain"]
    ADC2_ADC1_20X = 45,
    #[doc = "16: Differential Inputs Positive ADC2 Negative ADC3 1x Gain"]
    ADC2_ADC3_1X = 16,
    #[doc = "17: Differential Inputs Positive ADC2 Negative ADC3 20x Gain"]
    ADC2_ADC3_20X = 17,
    #[doc = "42: Differential Inputs Positive ADC3 Negative ADC0 1x Gain"]
    ADC3_ADC0_1X = 42,
    #[doc = "43: Differential Inputs Positive ADC3 Negative ADC0 20x Gain"]
    ADC3_ADC0_20X = 43,
    #[doc = "46: Differential Inputs Positive ADC3 Negative ADC1 1x Gain"]
    ADC3_ADC1_1X = 46,
    #[doc = "47: Differential Inputs Positive ADC3 Negative ADC1 20x Gain"]
    ADC3_ADC1_20X = 47,
    #[doc = "48: Differential Inputs Positive ADC3 Negative ADC2 1x Gain"]
    ADC3_ADC2_1X = 48,
    #[doc = "49: Differential Inputs Positive ADC3 Negative ADC2 20x Gain"]
    ADC3_ADC2_20X = 49,
    #[doc = "36: Differential Inputs Positive ADC3 Negative ADC3 1x Gain"]
    ADC3_ADC3_1X = 36,
    #[doc = "37: Differential Inputs Positive ADC3 Negative ADC3 20x Gain"]
    ADC3_ADC3_20X = 37,
    #[doc = "18: Differential Inputs Positive ADC4 Negative ADC0 1x Gain"]
    ADC3_ADC4_1X = 18,
    #[doc = "19: Differential Inputs Positive ADC4 Negative ADC0 20x Gain"]
    ADC3_ADC4_20X = 19,
    #[doc = "20: Differential Inputs Positive ADC5 Negative ADC1 1x Gain"]
    ADC3_ADC5_1X = 20,
    #[doc = "21: Differential Inputs Positive ADC5 Negative ADC1 20x Gain"]
    ADC3_ADC5_20X = 21,
    #[doc = "22: Differential Inputs Positive ADC6 Negative ADC2 1x Gain"]
    ADC3_ADC6_1X = 22,
    #[doc = "23: Differential Inputs Positive ADC6 Negative ADC2 20x Gain"]
    ADC3_ADC6_20X = 23,
    #[doc = "24: Differential Inputs Positive ADC7 Negative ADC3 1x Gain"]
    ADC3_ADC7_1X = 24,
    #[doc = "25: Differential Inputs Positive ADC7 Negative ADC3 20x Gain"]
    ADC3_ADC7_20X = 25,
    #[doc = "50: Differential Inputs Positive ADC4 Negative ADC3 1x Gain"]
    ADC4_ADC3_1X = 50,
    #[doc = "51: Differential Inputs Positive ADC4 Negative ADC3 20x Gain"]
    ADC4_ADC3_20X = 51,
    #[doc = "26: Differential Inputs Positive ADC4 Negative ADC5 1x Gain"]
    ADC4_ADC5_1X = 26,
    #[doc = "27: Differential Inputs Positive ADC4 Negative ADC5 20x Gain"]
    ADC4_ADC5_20X = 27,
    #[doc = "52: Differential Inputs Positive ADC5 Negative ADC3 1x Gain"]
    ADC5_ADC3_1X = 52,
    #[doc = "53: Differential Inputs Positive ADC5 Negative ADC3 20x Gain"]
    ADC5_ADC3_20X = 53,
    #[doc = "58: Differential Inputs Positive ADC5 Negative ADC4 1x Gain"]
    ADC5_ADC4_1X = 58,
    #[doc = "59: Differential Inputs Positive ADC5 Negative ADC4 20x Gain"]
    ADC5_ADC4_20X = 59,
    #[doc = "28: Differential Inputs Positive ADC5 Negative ADC6 1x Gain"]
    ADC5_ADC6_1X = 28,
    #[doc = "29: Differential Inputs Positive ADC5 Negative ADC6 20x Gain"]
    ADC5_ADC6_20X = 29,
    #[doc = "54: Differential Inputs Positive ADC6 Negative ADC3 1x Gain"]
    ADC6_ADC3_1X = 54,
    #[doc = "55: Differential Inputs Positive ADC6 Negative ADC3 20x Gain"]
    ADC6_ADC3_20X = 55,
    #[doc = "60: Differential Inputs Positive ADC6 Negative ADC5 1x Gain"]
    ADC6_ADC5_1X = 60,
    #[doc = "61: Differential Inputs Positive ADC6 Negative ADC5 20x Gain"]
    ADC6_ADC5_20X = 61,
    #[doc = "30: Differential Inputs Positive ADC6 Negative ADC7 1x Gain"]
    ADC6_ADC7_1X = 30,
    #[doc = "31: Differential Inputs Positive ADC6 Negative ADC7 20x Gain"]
    ADC6_ADC7_20X = 31,
    #[doc = "56: Differential Inputs Positive ADC7 Negative ADC3 1x Gain"]
    ADC7_ADC3_1X = 56,
    #[doc = "57: Differential Inputs Positive ADC7 Negative ADC3 20x Gain"]
    ADC7_ADC3_20X = 57,
    #[doc = "62: Differential Inputs Positive ADC7 Negative ADC6 1x Gain"]
    ADC7_ADC6_1X = 62,
    #[doc = "63: Differential Inputs Positive ADC7 Negative ADC6 20x Gain"]
    ADC7_ADC6_20X = 63,
    #[doc = "38: Differential Inputs Positive ADC7 Negative ADC7 1x Gain"]
    ADC7_ADC7_1X = 38,
    #[doc = "39: Differential Inputs Positive ADC7 Negative ADC7 20x Gain"]
    ADC7_ADC7_20X = 39,
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
    pub const fn variant(&self) -> MUX_A {
        match self.bits {
            0 => MUX_A::ADC0,
            1 => MUX_A::ADC1,
            2 => MUX_A::ADC2,
            3 => MUX_A::ADC3,
            4 => MUX_A::ADC4,
            5 => MUX_A::ADC5,
            6 => MUX_A::ADC6,
            7 => MUX_A::ADC7,
            32 => MUX_A::ADC_GND,
            33 => MUX_A::ADC_VBG,
            34 => MUX_A::TEMPSENS,
            35 => MUX_A::ADC0_ADC0_20X,
            8 => MUX_A::ADC0_ADC1_1X,
            9 => MUX_A::ADC0_ADC1_20X,
            10 => MUX_A::ADC0_ADC3_1X,
            11 => MUX_A::ADC0_ADC3_20X,
            40 => MUX_A::ADC1_ADC0_1X,
            41 => MUX_A::ADC1_ADC0_20X,
            12 => MUX_A::ADC1_ADC2_1X,
            13 => MUX_A::ADC1_ADC2_20X,
            14 => MUX_A::ADC1_ADC3_1X,
            15 => MUX_A::ADC1_ADC3_20X,
            44 => MUX_A::ADC2_ADC1_1X,
            45 => MUX_A::ADC2_ADC1_20X,
            16 => MUX_A::ADC2_ADC3_1X,
            17 => MUX_A::ADC2_ADC3_20X,
            42 => MUX_A::ADC3_ADC0_1X,
            43 => MUX_A::ADC3_ADC0_20X,
            46 => MUX_A::ADC3_ADC1_1X,
            47 => MUX_A::ADC3_ADC1_20X,
            48 => MUX_A::ADC3_ADC2_1X,
            49 => MUX_A::ADC3_ADC2_20X,
            36 => MUX_A::ADC3_ADC3_1X,
            37 => MUX_A::ADC3_ADC3_20X,
            18 => MUX_A::ADC3_ADC4_1X,
            19 => MUX_A::ADC3_ADC4_20X,
            20 => MUX_A::ADC3_ADC5_1X,
            21 => MUX_A::ADC3_ADC5_20X,
            22 => MUX_A::ADC3_ADC6_1X,
            23 => MUX_A::ADC3_ADC6_20X,
            24 => MUX_A::ADC3_ADC7_1X,
            25 => MUX_A::ADC3_ADC7_20X,
            50 => MUX_A::ADC4_ADC3_1X,
            51 => MUX_A::ADC4_ADC3_20X,
            26 => MUX_A::ADC4_ADC5_1X,
            27 => MUX_A::ADC4_ADC5_20X,
            52 => MUX_A::ADC5_ADC3_1X,
            53 => MUX_A::ADC5_ADC3_20X,
            58 => MUX_A::ADC5_ADC4_1X,
            59 => MUX_A::ADC5_ADC4_20X,
            28 => MUX_A::ADC5_ADC6_1X,
            29 => MUX_A::ADC5_ADC6_20X,
            54 => MUX_A::ADC6_ADC3_1X,
            55 => MUX_A::ADC6_ADC3_20X,
            60 => MUX_A::ADC6_ADC5_1X,
            61 => MUX_A::ADC6_ADC5_20X,
            30 => MUX_A::ADC6_ADC7_1X,
            31 => MUX_A::ADC6_ADC7_20X,
            56 => MUX_A::ADC7_ADC3_1X,
            57 => MUX_A::ADC7_ADC3_20X,
            62 => MUX_A::ADC7_ADC6_1X,
            63 => MUX_A::ADC7_ADC6_20X,
            38 => MUX_A::ADC7_ADC7_1X,
            39 => MUX_A::ADC7_ADC7_20X,
            _ => unreachable!(),
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
    #[doc = "Single-ended Input ADC4"]
    #[inline(always)]
    pub fn is_adc4(&self) -> bool {
        *self == MUX_A::ADC4
    }
    #[doc = "Single-ended Input ADC5"]
    #[inline(always)]
    pub fn is_adc5(&self) -> bool {
        *self == MUX_A::ADC5
    }
    #[doc = "Single-ended Input ADC6"]
    #[inline(always)]
    pub fn is_adc6(&self) -> bool {
        *self == MUX_A::ADC6
    }
    #[doc = "Single-ended Input ADC7"]
    #[inline(always)]
    pub fn is_adc7(&self) -> bool {
        *self == MUX_A::ADC7
    }
    #[doc = "0V (GND)"]
    #[inline(always)]
    pub fn is_adc_gnd(&self) -> bool {
        *self == MUX_A::ADC_GND
    }
    #[doc = "Internal Reference (VBG)"]
    #[inline(always)]
    pub fn is_adc_vbg(&self) -> bool {
        *self == MUX_A::ADC_VBG
    }
    #[doc = "Temperature sensor"]
    #[inline(always)]
    pub fn is_tempsens(&self) -> bool {
        *self == MUX_A::TEMPSENS
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
    #[doc = "Differential Inputs Postive ADC0 Negative ADC1 20x Gain"]
    #[inline(always)]
    pub fn is_adc0_adc1_20x(&self) -> bool {
        *self == MUX_A::ADC0_ADC1_20X
    }
    #[doc = "Differential Inputs Positive ADC0 Negative ADC3 1x Gain"]
    #[inline(always)]
    pub fn is_adc0_adc3_1x(&self) -> bool {
        *self == MUX_A::ADC0_ADC3_1X
    }
    #[doc = "Differential Inputs Positive ADC0 Negative ADC3 20x Gain"]
    #[inline(always)]
    pub fn is_adc0_adc3_20x(&self) -> bool {
        *self == MUX_A::ADC0_ADC3_20X
    }
    #[doc = "Differential Inputs Positive ADC1 Negative ADC0 1x Gain"]
    #[inline(always)]
    pub fn is_adc1_adc0_1x(&self) -> bool {
        *self == MUX_A::ADC1_ADC0_1X
    }
    #[doc = "Differential Inputs Positive ADC1 Negative ADC0 20x Gain"]
    #[inline(always)]
    pub fn is_adc1_adc0_20x(&self) -> bool {
        *self == MUX_A::ADC1_ADC0_20X
    }
    #[doc = "Differential Inputs Positive ADC1 Negative ADC2 1x Gain"]
    #[inline(always)]
    pub fn is_adc1_adc2_1x(&self) -> bool {
        *self == MUX_A::ADC1_ADC2_1X
    }
    #[doc = "Differential Inputs Positive ADC1 Negative ADC2 20x Gain"]
    #[inline(always)]
    pub fn is_adc1_adc2_20x(&self) -> bool {
        *self == MUX_A::ADC1_ADC2_20X
    }
    #[doc = "Differential Inputs Positive ADC1 Negative ADC3 1x Gain"]
    #[inline(always)]
    pub fn is_adc1_adc3_1x(&self) -> bool {
        *self == MUX_A::ADC1_ADC3_1X
    }
    #[doc = "Differential Inputs Positive ADC1 Negative ADC3 20x Gain"]
    #[inline(always)]
    pub fn is_adc1_adc3_20x(&self) -> bool {
        *self == MUX_A::ADC1_ADC3_20X
    }
    #[doc = "Differential Inputs Positive ADC2 Negative ADC2 1x Gain"]
    #[inline(always)]
    pub fn is_adc2_adc1_1x(&self) -> bool {
        *self == MUX_A::ADC2_ADC1_1X
    }
    #[doc = "Differential Inputs Positive ADC2 Negative ADC2 20x Gain"]
    #[inline(always)]
    pub fn is_adc2_adc1_20x(&self) -> bool {
        *self == MUX_A::ADC2_ADC1_20X
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
    #[doc = "Differential Inputs Positive ADC3 Negative ADC0 1x Gain"]
    #[inline(always)]
    pub fn is_adc3_adc0_1x(&self) -> bool {
        *self == MUX_A::ADC3_ADC0_1X
    }
    #[doc = "Differential Inputs Positive ADC3 Negative ADC0 20x Gain"]
    #[inline(always)]
    pub fn is_adc3_adc0_20x(&self) -> bool {
        *self == MUX_A::ADC3_ADC0_20X
    }
    #[doc = "Differential Inputs Positive ADC3 Negative ADC1 1x Gain"]
    #[inline(always)]
    pub fn is_adc3_adc1_1x(&self) -> bool {
        *self == MUX_A::ADC3_ADC1_1X
    }
    #[doc = "Differential Inputs Positive ADC3 Negative ADC1 20x Gain"]
    #[inline(always)]
    pub fn is_adc3_adc1_20x(&self) -> bool {
        *self == MUX_A::ADC3_ADC1_20X
    }
    #[doc = "Differential Inputs Positive ADC3 Negative ADC2 1x Gain"]
    #[inline(always)]
    pub fn is_adc3_adc2_1x(&self) -> bool {
        *self == MUX_A::ADC3_ADC2_1X
    }
    #[doc = "Differential Inputs Positive ADC3 Negative ADC2 20x Gain"]
    #[inline(always)]
    pub fn is_adc3_adc2_20x(&self) -> bool {
        *self == MUX_A::ADC3_ADC2_20X
    }
    #[doc = "Differential Inputs Positive ADC3 Negative ADC3 1x Gain"]
    #[inline(always)]
    pub fn is_adc3_adc3_1x(&self) -> bool {
        *self == MUX_A::ADC3_ADC3_1X
    }
    #[doc = "Differential Inputs Positive ADC3 Negative ADC3 20x Gain"]
    #[inline(always)]
    pub fn is_adc3_adc3_20x(&self) -> bool {
        *self == MUX_A::ADC3_ADC3_20X
    }
    #[doc = "Differential Inputs Positive ADC4 Negative ADC0 1x Gain"]
    #[inline(always)]
    pub fn is_adc3_adc4_1x(&self) -> bool {
        *self == MUX_A::ADC3_ADC4_1X
    }
    #[doc = "Differential Inputs Positive ADC4 Negative ADC0 20x Gain"]
    #[inline(always)]
    pub fn is_adc3_adc4_20x(&self) -> bool {
        *self == MUX_A::ADC3_ADC4_20X
    }
    #[doc = "Differential Inputs Positive ADC5 Negative ADC1 1x Gain"]
    #[inline(always)]
    pub fn is_adc3_adc5_1x(&self) -> bool {
        *self == MUX_A::ADC3_ADC5_1X
    }
    #[doc = "Differential Inputs Positive ADC5 Negative ADC1 20x Gain"]
    #[inline(always)]
    pub fn is_adc3_adc5_20x(&self) -> bool {
        *self == MUX_A::ADC3_ADC5_20X
    }
    #[doc = "Differential Inputs Positive ADC6 Negative ADC2 1x Gain"]
    #[inline(always)]
    pub fn is_adc3_adc6_1x(&self) -> bool {
        *self == MUX_A::ADC3_ADC6_1X
    }
    #[doc = "Differential Inputs Positive ADC6 Negative ADC2 20x Gain"]
    #[inline(always)]
    pub fn is_adc3_adc6_20x(&self) -> bool {
        *self == MUX_A::ADC3_ADC6_20X
    }
    #[doc = "Differential Inputs Positive ADC7 Negative ADC3 1x Gain"]
    #[inline(always)]
    pub fn is_adc3_adc7_1x(&self) -> bool {
        *self == MUX_A::ADC3_ADC7_1X
    }
    #[doc = "Differential Inputs Positive ADC7 Negative ADC3 20x Gain"]
    #[inline(always)]
    pub fn is_adc3_adc7_20x(&self) -> bool {
        *self == MUX_A::ADC3_ADC7_20X
    }
    #[doc = "Differential Inputs Positive ADC4 Negative ADC3 1x Gain"]
    #[inline(always)]
    pub fn is_adc4_adc3_1x(&self) -> bool {
        *self == MUX_A::ADC4_ADC3_1X
    }
    #[doc = "Differential Inputs Positive ADC4 Negative ADC3 20x Gain"]
    #[inline(always)]
    pub fn is_adc4_adc3_20x(&self) -> bool {
        *self == MUX_A::ADC4_ADC3_20X
    }
    #[doc = "Differential Inputs Positive ADC4 Negative ADC5 1x Gain"]
    #[inline(always)]
    pub fn is_adc4_adc5_1x(&self) -> bool {
        *self == MUX_A::ADC4_ADC5_1X
    }
    #[doc = "Differential Inputs Positive ADC4 Negative ADC5 20x Gain"]
    #[inline(always)]
    pub fn is_adc4_adc5_20x(&self) -> bool {
        *self == MUX_A::ADC4_ADC5_20X
    }
    #[doc = "Differential Inputs Positive ADC5 Negative ADC3 1x Gain"]
    #[inline(always)]
    pub fn is_adc5_adc3_1x(&self) -> bool {
        *self == MUX_A::ADC5_ADC3_1X
    }
    #[doc = "Differential Inputs Positive ADC5 Negative ADC3 20x Gain"]
    #[inline(always)]
    pub fn is_adc5_adc3_20x(&self) -> bool {
        *self == MUX_A::ADC5_ADC3_20X
    }
    #[doc = "Differential Inputs Positive ADC5 Negative ADC4 1x Gain"]
    #[inline(always)]
    pub fn is_adc5_adc4_1x(&self) -> bool {
        *self == MUX_A::ADC5_ADC4_1X
    }
    #[doc = "Differential Inputs Positive ADC5 Negative ADC4 20x Gain"]
    #[inline(always)]
    pub fn is_adc5_adc4_20x(&self) -> bool {
        *self == MUX_A::ADC5_ADC4_20X
    }
    #[doc = "Differential Inputs Positive ADC5 Negative ADC6 1x Gain"]
    #[inline(always)]
    pub fn is_adc5_adc6_1x(&self) -> bool {
        *self == MUX_A::ADC5_ADC6_1X
    }
    #[doc = "Differential Inputs Positive ADC5 Negative ADC6 20x Gain"]
    #[inline(always)]
    pub fn is_adc5_adc6_20x(&self) -> bool {
        *self == MUX_A::ADC5_ADC6_20X
    }
    #[doc = "Differential Inputs Positive ADC6 Negative ADC3 1x Gain"]
    #[inline(always)]
    pub fn is_adc6_adc3_1x(&self) -> bool {
        *self == MUX_A::ADC6_ADC3_1X
    }
    #[doc = "Differential Inputs Positive ADC6 Negative ADC3 20x Gain"]
    #[inline(always)]
    pub fn is_adc6_adc3_20x(&self) -> bool {
        *self == MUX_A::ADC6_ADC3_20X
    }
    #[doc = "Differential Inputs Positive ADC6 Negative ADC5 1x Gain"]
    #[inline(always)]
    pub fn is_adc6_adc5_1x(&self) -> bool {
        *self == MUX_A::ADC6_ADC5_1X
    }
    #[doc = "Differential Inputs Positive ADC6 Negative ADC5 20x Gain"]
    #[inline(always)]
    pub fn is_adc6_adc5_20x(&self) -> bool {
        *self == MUX_A::ADC6_ADC5_20X
    }
    #[doc = "Differential Inputs Positive ADC6 Negative ADC7 1x Gain"]
    #[inline(always)]
    pub fn is_adc6_adc7_1x(&self) -> bool {
        *self == MUX_A::ADC6_ADC7_1X
    }
    #[doc = "Differential Inputs Positive ADC6 Negative ADC7 20x Gain"]
    #[inline(always)]
    pub fn is_adc6_adc7_20x(&self) -> bool {
        *self == MUX_A::ADC6_ADC7_20X
    }
    #[doc = "Differential Inputs Positive ADC7 Negative ADC3 1x Gain"]
    #[inline(always)]
    pub fn is_adc7_adc3_1x(&self) -> bool {
        *self == MUX_A::ADC7_ADC3_1X
    }
    #[doc = "Differential Inputs Positive ADC7 Negative ADC3 20x Gain"]
    #[inline(always)]
    pub fn is_adc7_adc3_20x(&self) -> bool {
        *self == MUX_A::ADC7_ADC3_20X
    }
    #[doc = "Differential Inputs Positive ADC7 Negative ADC6 1x Gain"]
    #[inline(always)]
    pub fn is_adc7_adc6_1x(&self) -> bool {
        *self == MUX_A::ADC7_ADC6_1X
    }
    #[doc = "Differential Inputs Positive ADC7 Negative ADC6 20x Gain"]
    #[inline(always)]
    pub fn is_adc7_adc6_20x(&self) -> bool {
        *self == MUX_A::ADC7_ADC6_20X
    }
    #[doc = "Differential Inputs Positive ADC7 Negative ADC7 1x Gain"]
    #[inline(always)]
    pub fn is_adc7_adc7_1x(&self) -> bool {
        *self == MUX_A::ADC7_ADC7_1X
    }
    #[doc = "Differential Inputs Positive ADC7 Negative ADC7 20x Gain"]
    #[inline(always)]
    pub fn is_adc7_adc7_20x(&self) -> bool {
        *self == MUX_A::ADC7_ADC7_20X
    }
}
#[doc = "Field `MUX` writer - Analog Channel and Gain Selection Bits"]
pub type MUX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6, MUX_A>;
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
    #[doc = "Single-ended Input ADC4"]
    #[inline(always)]
    pub fn adc4(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC4)
    }
    #[doc = "Single-ended Input ADC5"]
    #[inline(always)]
    pub fn adc5(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC5)
    }
    #[doc = "Single-ended Input ADC6"]
    #[inline(always)]
    pub fn adc6(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC6)
    }
    #[doc = "Single-ended Input ADC7"]
    #[inline(always)]
    pub fn adc7(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC7)
    }
    #[doc = "0V (GND)"]
    #[inline(always)]
    pub fn adc_gnd(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC_GND)
    }
    #[doc = "Internal Reference (VBG)"]
    #[inline(always)]
    pub fn adc_vbg(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC_VBG)
    }
    #[doc = "Temperature sensor"]
    #[inline(always)]
    pub fn tempsens(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::TEMPSENS)
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
    #[doc = "Differential Inputs Postive ADC0 Negative ADC1 20x Gain"]
    #[inline(always)]
    pub fn adc0_adc1_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC0_ADC1_20X)
    }
    #[doc = "Differential Inputs Positive ADC0 Negative ADC3 1x Gain"]
    #[inline(always)]
    pub fn adc0_adc3_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC0_ADC3_1X)
    }
    #[doc = "Differential Inputs Positive ADC0 Negative ADC3 20x Gain"]
    #[inline(always)]
    pub fn adc0_adc3_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC0_ADC3_20X)
    }
    #[doc = "Differential Inputs Positive ADC1 Negative ADC0 1x Gain"]
    #[inline(always)]
    pub fn adc1_adc0_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC1_ADC0_1X)
    }
    #[doc = "Differential Inputs Positive ADC1 Negative ADC0 20x Gain"]
    #[inline(always)]
    pub fn adc1_adc0_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC1_ADC0_20X)
    }
    #[doc = "Differential Inputs Positive ADC1 Negative ADC2 1x Gain"]
    #[inline(always)]
    pub fn adc1_adc2_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC1_ADC2_1X)
    }
    #[doc = "Differential Inputs Positive ADC1 Negative ADC2 20x Gain"]
    #[inline(always)]
    pub fn adc1_adc2_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC1_ADC2_20X)
    }
    #[doc = "Differential Inputs Positive ADC1 Negative ADC3 1x Gain"]
    #[inline(always)]
    pub fn adc1_adc3_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC1_ADC3_1X)
    }
    #[doc = "Differential Inputs Positive ADC1 Negative ADC3 20x Gain"]
    #[inline(always)]
    pub fn adc1_adc3_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC1_ADC3_20X)
    }
    #[doc = "Differential Inputs Positive ADC2 Negative ADC2 1x Gain"]
    #[inline(always)]
    pub fn adc2_adc1_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC2_ADC1_1X)
    }
    #[doc = "Differential Inputs Positive ADC2 Negative ADC2 20x Gain"]
    #[inline(always)]
    pub fn adc2_adc1_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC2_ADC1_20X)
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
    #[doc = "Differential Inputs Positive ADC3 Negative ADC0 1x Gain"]
    #[inline(always)]
    pub fn adc3_adc0_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3_ADC0_1X)
    }
    #[doc = "Differential Inputs Positive ADC3 Negative ADC0 20x Gain"]
    #[inline(always)]
    pub fn adc3_adc0_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3_ADC0_20X)
    }
    #[doc = "Differential Inputs Positive ADC3 Negative ADC1 1x Gain"]
    #[inline(always)]
    pub fn adc3_adc1_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3_ADC1_1X)
    }
    #[doc = "Differential Inputs Positive ADC3 Negative ADC1 20x Gain"]
    #[inline(always)]
    pub fn adc3_adc1_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3_ADC1_20X)
    }
    #[doc = "Differential Inputs Positive ADC3 Negative ADC2 1x Gain"]
    #[inline(always)]
    pub fn adc3_adc2_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3_ADC2_1X)
    }
    #[doc = "Differential Inputs Positive ADC3 Negative ADC2 20x Gain"]
    #[inline(always)]
    pub fn adc3_adc2_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3_ADC2_20X)
    }
    #[doc = "Differential Inputs Positive ADC3 Negative ADC3 1x Gain"]
    #[inline(always)]
    pub fn adc3_adc3_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3_ADC3_1X)
    }
    #[doc = "Differential Inputs Positive ADC3 Negative ADC3 20x Gain"]
    #[inline(always)]
    pub fn adc3_adc3_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3_ADC3_20X)
    }
    #[doc = "Differential Inputs Positive ADC4 Negative ADC0 1x Gain"]
    #[inline(always)]
    pub fn adc3_adc4_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3_ADC4_1X)
    }
    #[doc = "Differential Inputs Positive ADC4 Negative ADC0 20x Gain"]
    #[inline(always)]
    pub fn adc3_adc4_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3_ADC4_20X)
    }
    #[doc = "Differential Inputs Positive ADC5 Negative ADC1 1x Gain"]
    #[inline(always)]
    pub fn adc3_adc5_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3_ADC5_1X)
    }
    #[doc = "Differential Inputs Positive ADC5 Negative ADC1 20x Gain"]
    #[inline(always)]
    pub fn adc3_adc5_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3_ADC5_20X)
    }
    #[doc = "Differential Inputs Positive ADC6 Negative ADC2 1x Gain"]
    #[inline(always)]
    pub fn adc3_adc6_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3_ADC6_1X)
    }
    #[doc = "Differential Inputs Positive ADC6 Negative ADC2 20x Gain"]
    #[inline(always)]
    pub fn adc3_adc6_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3_ADC6_20X)
    }
    #[doc = "Differential Inputs Positive ADC7 Negative ADC3 1x Gain"]
    #[inline(always)]
    pub fn adc3_adc7_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3_ADC7_1X)
    }
    #[doc = "Differential Inputs Positive ADC7 Negative ADC3 20x Gain"]
    #[inline(always)]
    pub fn adc3_adc7_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3_ADC7_20X)
    }
    #[doc = "Differential Inputs Positive ADC4 Negative ADC3 1x Gain"]
    #[inline(always)]
    pub fn adc4_adc3_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC4_ADC3_1X)
    }
    #[doc = "Differential Inputs Positive ADC4 Negative ADC3 20x Gain"]
    #[inline(always)]
    pub fn adc4_adc3_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC4_ADC3_20X)
    }
    #[doc = "Differential Inputs Positive ADC4 Negative ADC5 1x Gain"]
    #[inline(always)]
    pub fn adc4_adc5_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC4_ADC5_1X)
    }
    #[doc = "Differential Inputs Positive ADC4 Negative ADC5 20x Gain"]
    #[inline(always)]
    pub fn adc4_adc5_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC4_ADC5_20X)
    }
    #[doc = "Differential Inputs Positive ADC5 Negative ADC3 1x Gain"]
    #[inline(always)]
    pub fn adc5_adc3_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC5_ADC3_1X)
    }
    #[doc = "Differential Inputs Positive ADC5 Negative ADC3 20x Gain"]
    #[inline(always)]
    pub fn adc5_adc3_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC5_ADC3_20X)
    }
    #[doc = "Differential Inputs Positive ADC5 Negative ADC4 1x Gain"]
    #[inline(always)]
    pub fn adc5_adc4_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC5_ADC4_1X)
    }
    #[doc = "Differential Inputs Positive ADC5 Negative ADC4 20x Gain"]
    #[inline(always)]
    pub fn adc5_adc4_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC5_ADC4_20X)
    }
    #[doc = "Differential Inputs Positive ADC5 Negative ADC6 1x Gain"]
    #[inline(always)]
    pub fn adc5_adc6_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC5_ADC6_1X)
    }
    #[doc = "Differential Inputs Positive ADC5 Negative ADC6 20x Gain"]
    #[inline(always)]
    pub fn adc5_adc6_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC5_ADC6_20X)
    }
    #[doc = "Differential Inputs Positive ADC6 Negative ADC3 1x Gain"]
    #[inline(always)]
    pub fn adc6_adc3_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC6_ADC3_1X)
    }
    #[doc = "Differential Inputs Positive ADC6 Negative ADC3 20x Gain"]
    #[inline(always)]
    pub fn adc6_adc3_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC6_ADC3_20X)
    }
    #[doc = "Differential Inputs Positive ADC6 Negative ADC5 1x Gain"]
    #[inline(always)]
    pub fn adc6_adc5_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC6_ADC5_1X)
    }
    #[doc = "Differential Inputs Positive ADC6 Negative ADC5 20x Gain"]
    #[inline(always)]
    pub fn adc6_adc5_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC6_ADC5_20X)
    }
    #[doc = "Differential Inputs Positive ADC6 Negative ADC7 1x Gain"]
    #[inline(always)]
    pub fn adc6_adc7_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC6_ADC7_1X)
    }
    #[doc = "Differential Inputs Positive ADC6 Negative ADC7 20x Gain"]
    #[inline(always)]
    pub fn adc6_adc7_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC6_ADC7_20X)
    }
    #[doc = "Differential Inputs Positive ADC7 Negative ADC3 1x Gain"]
    #[inline(always)]
    pub fn adc7_adc3_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC7_ADC3_1X)
    }
    #[doc = "Differential Inputs Positive ADC7 Negative ADC3 20x Gain"]
    #[inline(always)]
    pub fn adc7_adc3_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC7_ADC3_20X)
    }
    #[doc = "Differential Inputs Positive ADC7 Negative ADC6 1x Gain"]
    #[inline(always)]
    pub fn adc7_adc6_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC7_ADC6_1X)
    }
    #[doc = "Differential Inputs Positive ADC7 Negative ADC6 20x Gain"]
    #[inline(always)]
    pub fn adc7_adc6_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC7_ADC6_20X)
    }
    #[doc = "Differential Inputs Positive ADC7 Negative ADC7 1x Gain"]
    #[inline(always)]
    pub fn adc7_adc7_1x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC7_ADC7_1X)
    }
    #[doc = "Differential Inputs Positive ADC7 Negative ADC7 20x Gain"]
    #[inline(always)]
    pub fn adc7_adc7_20x(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC7_ADC7_20X)
    }
}
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
    #[doc = "2: Internal 1.1V Voltage Reference"]
    INTERNAL = 2,
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
    pub const fn variant(&self) -> Option<REFS_A> {
        match self.bits {
            0 => Some(REFS_A::VCC),
            1 => Some(REFS_A::AREF),
            2 => Some(REFS_A::INTERNAL),
            _ => None,
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
    #[doc = "Internal 1.1V Voltage Reference"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == REFS_A::INTERNAL
    }
}
#[doc = "Field `REFS` writer - Reference Selection Bits"]
pub type REFS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, REFS_A>;
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
    #[doc = "Internal 1.1V Voltage Reference"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::INTERNAL)
    }
}
impl R {
    #[doc = "Bits 0:5 - Analog Channel and Gain Selection Bits"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(self.bits & 0x3f)
    }
    #[doc = "Bits 6:7 - Reference Selection Bits"]
    #[inline(always)]
    pub fn refs(&self) -> REFS_R {
        REFS_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:5 - Analog Channel and Gain Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mux(&mut self) -> MUX_W<ADMUX_SPEC> {
        MUX_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - Reference Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn refs(&mut self) -> REFS_W<ADMUX_SPEC> {
        REFS_W::new(self, 6)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC Multiplexer Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`admux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`admux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
