#[doc = "Register `LOW` reader"]
pub type R = crate::R<LOW_SPEC>;
#[doc = "Register `LOW` writer"]
pub type W = crate::W<LOW_SPEC>;
#[doc = "Field `SUT_CKSEL` reader - Select Clock Source"]
pub type SUT_CKSEL_R = crate::FieldReader<SUT_CKSEL_A>;
#[doc = "Select Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SUT_CKSEL_A {
    #[doc = "0: Ext. Clock; Start-up time: 6 CK + 0 ms"]
    EXTCLK_6CK_0MS = 0,
    #[doc = "1: Int. RC Osc. 1 MHz; Start-up time: 6 CK + 0 ms"]
    INTRCOSC_1MHZ_6CK_0MS = 1,
    #[doc = "2: Int. RC Osc. 2 MHz; Start-up time: 6 CK + 0 ms"]
    INTRCOSC_2MHZ_6CK_0MS = 2,
    #[doc = "3: Int. RC Osc. 4 MHz; Start-up time: 6 CK + 0 ms"]
    INTRCOSC_4MHZ_6CK_0MS = 3,
    #[doc = "4: Int. RC Osc. 8 MHz; Start-up time: 6 CK + 0 ms"]
    INTRCOSC_8MHZ_6CK_0MS = 4,
    #[doc = "5: Ext. RC Osc. - 0.9 MHz; Start-up time: 18 CK + 0 ms"]
    EXTRCOSC_XX_0MHZ9_18CK_0MS = 5,
    #[doc = "6: Ext. RC Osc. 0.9 MHz - 3.0 MHz; Start-up time: 18 CK + 0 ms"]
    EXTRCOSC_0MHZ9_3MHZ_18CK_0MS = 6,
    #[doc = "7: Ext. RC Osc. 3.0 MHz - 8.0 MHz; Start-up time: 18 CK + 0 ms"]
    EXTRCOSC_3MHZ_8MHZ_18CK_0MS = 7,
    #[doc = "8: Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 0 ms"]
    EXTRCOSC_8MHZ_12MHZ_18CK_0MS = 8,
    #[doc = "9: Ext. Low-Freq. Crystal; Start-up time: 1K CK + 4 ms"]
    EXTLOFXTAL_1KCK_4MS = 9,
    #[doc = "10: Ext. Crystal/Resonator Low Freq.; Start-up time: 258 CK + 4 ms"]
    EXTLOFXTALRES_258CK_4MS = 10,
    #[doc = "11: Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 64 ms"]
    EXTLOFXTALRES_1KCK_64MS = 11,
    #[doc = "12: Ext. Crystal/Resonator Medium Freq.; Start-up time: 258 CK + 4 ms"]
    EXTMEDFXTALRES_258CK_4MS = 12,
    #[doc = "13: Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 64 ms"]
    EXTMEDFXTALRES_1KCK_64MS = 13,
    #[doc = "14: Ext. Crystal/Resonator High Freq.; Start-up time: 258 CK + 4 ms"]
    EXTHIFXTALRES_258CK_4MS = 14,
    #[doc = "15: Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 64 ms"]
    EXTHIFXTALRES_1KCK_64MS = 15,
    #[doc = "16: Ext. Clock; Start-up time: 6 CK + 4 ms"]
    EXTCLK_6CK_4MS = 16,
    #[doc = "17: Int. RC Osc. 1 MHz; Start-up time: 6 CK + 4 ms"]
    INTRCOSC_1MHZ_6CK_4MS = 17,
    #[doc = "18: Int. RC Osc. 2 MHz; Start-up time: 6 CK + 4 ms"]
    INTRCOSC_2MHZ_6CK_4MS = 18,
    #[doc = "19: Int. RC Osc. 4 MHz; Start-up time: 6 CK + 4 ms"]
    INTRCOSC_4MHZ_6CK_4MS = 19,
    #[doc = "20: Int. RC Osc. 8 MHz; Start-up time: 6 CK + 4 ms"]
    INTRCOSC_8MHZ_6CK_4MS = 20,
    #[doc = "21: Ext. RC Osc. - 0.9 MHz; Start-up time: 18 CK + 4 ms"]
    EXTRCOSC_XX_0MHZ9_18CK_4MS = 21,
    #[doc = "22: Ext. RC Osc. 0.9 MHz - 3.0 MHz; Start-up time: 18 CK + 4 ms"]
    EXTRCOSC_0MHZ9_3MHZ_18CK_4MS = 22,
    #[doc = "23: Ext. RC Osc. 3.0 MHz - 8.0 MHz; Start-up time: 18 CK + 4 ms"]
    EXTRCOSC_3MHZ_8MHZ_18CK_4MS = 23,
    #[doc = "24: Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 4 ms"]
    EXTRCOSC_8MHZ_12MHZ_18CK_4MS = 24,
    #[doc = "25: Ext. Low-Freq. Crystal; Start-up time: 1K CK + 64 ms"]
    EXTLOFXTAL_1KCK_64MS = 25,
    #[doc = "26: Ext. Crystal/Resonator Low Freq.; Start-up time: 258 CK + 64 ms"]
    EXTLOFXTALRES_258CK_64MS = 26,
    #[doc = "27: Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 0 ms"]
    EXTLOFXTALRES_16KCK_0MS = 27,
    #[doc = "28: Ext. Crystal/Resonator Medium Freq.; Start-up time: 258 CK + 64 ms"]
    EXTMEDFXTALRES_258CK_64MS = 28,
    #[doc = "29: Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 0 ms"]
    EXTMEDFXTALRES_16KCK_0MS = 29,
    #[doc = "30: Ext. Crystal/Resonator High Freq.; Start-up time: 258 CK + 64 ms"]
    EXTHIFXTALRES_258CK_64MS = 30,
    #[doc = "31: Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 0 ms"]
    EXTHIFXTALRES_16KCK_0MS = 31,
    #[doc = "32: Ext. Clock; Start-up time: 6 CK + 64 ms"]
    EXTCLK_6CK_64MS = 32,
    #[doc = "33: Int. RC Osc. 1 MHz; Start-up time: 6 CK + 64 ms"]
    INTRCOSC_1MHZ_6CK_64MS = 33,
    #[doc = "34: Int. RC Osc. 2 MHz; Start-up time: 6 CK + 64 ms"]
    INTRCOSC_2MHZ_6CK_64MS = 34,
    #[doc = "35: Int. RC Osc. 4 MHz; Start-up time: 6 CK + 64 ms"]
    INTRCOSC_4MHZ_6CK_64MS = 35,
    #[doc = "36: Int. RC Osc. 8 MHz; Start-up time: 6 CK + 64 ms"]
    INTRCOSC_8MHZ_6CK_64MS = 36,
    #[doc = "37: Ext. RC Osc. - 0.9 MHz; Start-up time: 18 CK + 64 ms"]
    EXTRCOSC_XX_0MHZ9_18CK_64MS = 37,
    #[doc = "38: Ext. RC Osc. 0.9 MHz - 3.0 MHz; Start-up time: 18 CK + 64 ms"]
    EXTRCOSC_0MHZ9_3MHZ_18CK_64MS = 38,
    #[doc = "39: Ext. RC Osc. 3.0 MHz - 8.0 MHz; Start-up time: 18 CK + 64 ms"]
    EXTRCOSC_3MHZ_8MHZ_18CK_64MS = 39,
    #[doc = "40: Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 64 ms"]
    EXTRCOSC_8MHZ_12MHZ_18CK_64MS = 40,
    #[doc = "41: Ext. Low-Freq. Crystal; Start-up time: 32K CK + 64 ms"]
    EXTLOFXTAL_32KCK_64MS = 41,
    #[doc = "42: Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 0 ms"]
    EXTLOFXTALRES_1KCK_0MS = 42,
    #[doc = "43: Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 4 ms"]
    EXTLOFXTALRES_16KCK_4MS = 43,
    #[doc = "44: Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 0 ms"]
    EXTMEDFXTALRES_1KCK_0MS = 44,
    #[doc = "45: Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 4 ms"]
    EXTMEDFXTALRES_16KCK_4MS = 45,
    #[doc = "46: Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 0 ms"]
    EXTHIFXTALRES_1KCK_0MS = 46,
    #[doc = "47: Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 4 ms"]
    EXTHIFXTALRES_16KCK_4MS = 47,
    #[doc = "53: Ext. RC Osc. - 0.9 MHz; Start-up time: 6 CK + 4 ms"]
    EXTRCOSC_XX_0MHZ9_6CK_4MS = 53,
    #[doc = "54: Ext. RC Osc. 0.9 MHz - 3.0 MHz; Start-up time: 6 CK + 4 ms"]
    EXTRCOSC_0MHZ9_3MHZ_6CK_4MS = 54,
    #[doc = "55: Ext. RC Osc. 3.0 MHz - 8.0 MHz; Start-up time: 6 CK + 4 ms"]
    EXTRCOSC_3MHZ_8MHZ_6CK_4MS = 55,
    #[doc = "56: Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 6 CK + 4 ms"]
    EXTRCOSC_8MHZ_12MHZ_6CK_4MS = 56,
    #[doc = "58: Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 4 ms"]
    EXTLOFXTALRES_1KCK_4MS = 58,
    #[doc = "59: Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 64 ms"]
    EXTLOFXTALRES_16KCK_64MS = 59,
    #[doc = "60: Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 4 ms"]
    EXTMEDFXTALRES_1KCK_4MS = 60,
    #[doc = "61: Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 64 ms"]
    EXTMEDFXTALRES_16KCK_64MS = 61,
    #[doc = "62: Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 4 ms"]
    EXTHIFXTALRES_1KCK_4MS = 62,
    #[doc = "63: Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 64 ms"]
    EXTHIFXTALRES_16KCK_64MS = 63,
}
impl From<SUT_CKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SUT_CKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SUT_CKSEL_A {
    type Ux = u8;
}
impl SUT_CKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SUT_CKSEL_A> {
        match self.bits {
            0 => Some(SUT_CKSEL_A::EXTCLK_6CK_0MS),
            1 => Some(SUT_CKSEL_A::INTRCOSC_1MHZ_6CK_0MS),
            2 => Some(SUT_CKSEL_A::INTRCOSC_2MHZ_6CK_0MS),
            3 => Some(SUT_CKSEL_A::INTRCOSC_4MHZ_6CK_0MS),
            4 => Some(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_0MS),
            5 => Some(SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_18CK_0MS),
            6 => Some(SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_18CK_0MS),
            7 => Some(SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_18CK_0MS),
            8 => Some(SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_18CK_0MS),
            9 => Some(SUT_CKSEL_A::EXTLOFXTAL_1KCK_4MS),
            10 => Some(SUT_CKSEL_A::EXTLOFXTALRES_258CK_4MS),
            11 => Some(SUT_CKSEL_A::EXTLOFXTALRES_1KCK_64MS),
            12 => Some(SUT_CKSEL_A::EXTMEDFXTALRES_258CK_4MS),
            13 => Some(SUT_CKSEL_A::EXTMEDFXTALRES_1KCK_64MS),
            14 => Some(SUT_CKSEL_A::EXTHIFXTALRES_258CK_4MS),
            15 => Some(SUT_CKSEL_A::EXTHIFXTALRES_1KCK_64MS),
            16 => Some(SUT_CKSEL_A::EXTCLK_6CK_4MS),
            17 => Some(SUT_CKSEL_A::INTRCOSC_1MHZ_6CK_4MS),
            18 => Some(SUT_CKSEL_A::INTRCOSC_2MHZ_6CK_4MS),
            19 => Some(SUT_CKSEL_A::INTRCOSC_4MHZ_6CK_4MS),
            20 => Some(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_4MS),
            21 => Some(SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_18CK_4MS),
            22 => Some(SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_18CK_4MS),
            23 => Some(SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_18CK_4MS),
            24 => Some(SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_18CK_4MS),
            25 => Some(SUT_CKSEL_A::EXTLOFXTAL_1KCK_64MS),
            26 => Some(SUT_CKSEL_A::EXTLOFXTALRES_258CK_64MS),
            27 => Some(SUT_CKSEL_A::EXTLOFXTALRES_16KCK_0MS),
            28 => Some(SUT_CKSEL_A::EXTMEDFXTALRES_258CK_64MS),
            29 => Some(SUT_CKSEL_A::EXTMEDFXTALRES_16KCK_0MS),
            30 => Some(SUT_CKSEL_A::EXTHIFXTALRES_258CK_64MS),
            31 => Some(SUT_CKSEL_A::EXTHIFXTALRES_16KCK_0MS),
            32 => Some(SUT_CKSEL_A::EXTCLK_6CK_64MS),
            33 => Some(SUT_CKSEL_A::INTRCOSC_1MHZ_6CK_64MS),
            34 => Some(SUT_CKSEL_A::INTRCOSC_2MHZ_6CK_64MS),
            35 => Some(SUT_CKSEL_A::INTRCOSC_4MHZ_6CK_64MS),
            36 => Some(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_64MS),
            37 => Some(SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_18CK_64MS),
            38 => Some(SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_18CK_64MS),
            39 => Some(SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_18CK_64MS),
            40 => Some(SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_18CK_64MS),
            41 => Some(SUT_CKSEL_A::EXTLOFXTAL_32KCK_64MS),
            42 => Some(SUT_CKSEL_A::EXTLOFXTALRES_1KCK_0MS),
            43 => Some(SUT_CKSEL_A::EXTLOFXTALRES_16KCK_4MS),
            44 => Some(SUT_CKSEL_A::EXTMEDFXTALRES_1KCK_0MS),
            45 => Some(SUT_CKSEL_A::EXTMEDFXTALRES_16KCK_4MS),
            46 => Some(SUT_CKSEL_A::EXTHIFXTALRES_1KCK_0MS),
            47 => Some(SUT_CKSEL_A::EXTHIFXTALRES_16KCK_4MS),
            53 => Some(SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_6CK_4MS),
            54 => Some(SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_6CK_4MS),
            55 => Some(SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_6CK_4MS),
            56 => Some(SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_6CK_4MS),
            58 => Some(SUT_CKSEL_A::EXTLOFXTALRES_1KCK_4MS),
            59 => Some(SUT_CKSEL_A::EXTLOFXTALRES_16KCK_64MS),
            60 => Some(SUT_CKSEL_A::EXTMEDFXTALRES_1KCK_4MS),
            61 => Some(SUT_CKSEL_A::EXTMEDFXTALRES_16KCK_64MS),
            62 => Some(SUT_CKSEL_A::EXTHIFXTALRES_1KCK_4MS),
            63 => Some(SUT_CKSEL_A::EXTHIFXTALRES_16KCK_64MS),
            _ => None,
        }
    }
    #[doc = "Ext. Clock; Start-up time: 6 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extclk_6ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCLK_6CK_0MS
    }
    #[doc = "Int. RC Osc. 1 MHz; Start-up time: 6 CK + 0 ms"]
    #[inline(always)]
    pub fn is_intrcosc_1mhz_6ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_1MHZ_6CK_0MS
    }
    #[doc = "Int. RC Osc. 2 MHz; Start-up time: 6 CK + 0 ms"]
    #[inline(always)]
    pub fn is_intrcosc_2mhz_6ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_2MHZ_6CK_0MS
    }
    #[doc = "Int. RC Osc. 4 MHz; Start-up time: 6 CK + 0 ms"]
    #[inline(always)]
    pub fn is_intrcosc_4mhz_6ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_4MHZ_6CK_0MS
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time: 6 CK + 0 ms"]
    #[inline(always)]
    pub fn is_intrcosc_8mhz_6ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_0MS
    }
    #[doc = "Ext. RC Osc. - 0.9 MHz; Start-up time: 18 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extrcosc_xx_0mhz9_18ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_18CK_0MS
    }
    #[doc = "Ext. RC Osc. 0.9 MHz - 3.0 MHz; Start-up time: 18 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extrcosc_0mhz9_3mhz_18ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_18CK_0MS
    }
    #[doc = "Ext. RC Osc. 3.0 MHz - 8.0 MHz; Start-up time: 18 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extrcosc_3mhz_8mhz_18ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_18CK_0MS
    }
    #[doc = "Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extrcosc_8mhz_12mhz_18ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_18CK_0MS
    }
    #[doc = "Ext. Low-Freq. Crystal; Start-up time: 1K CK + 4 ms"]
    #[inline(always)]
    pub fn is_extlofxtal_1kck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTLOFXTAL_1KCK_4MS
    }
    #[doc = "Ext. Crystal/Resonator Low Freq.; Start-up time: 258 CK + 4 ms"]
    #[inline(always)]
    pub fn is_extlofxtalres_258ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTLOFXTALRES_258CK_4MS
    }
    #[doc = "Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 64 ms"]
    #[inline(always)]
    pub fn is_extlofxtalres_1kck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTLOFXTALRES_1KCK_64MS
    }
    #[doc = "Ext. Crystal/Resonator Medium Freq.; Start-up time: 258 CK + 4 ms"]
    #[inline(always)]
    pub fn is_extmedfxtalres_258ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTMEDFXTALRES_258CK_4MS
    }
    #[doc = "Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 64 ms"]
    #[inline(always)]
    pub fn is_extmedfxtalres_1kck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTMEDFXTALRES_1KCK_64MS
    }
    #[doc = "Ext. Crystal/Resonator High Freq.; Start-up time: 258 CK + 4 ms"]
    #[inline(always)]
    pub fn is_exthifxtalres_258ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTHIFXTALRES_258CK_4MS
    }
    #[doc = "Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 64 ms"]
    #[inline(always)]
    pub fn is_exthifxtalres_1kck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTHIFXTALRES_1KCK_64MS
    }
    #[doc = "Ext. Clock; Start-up time: 6 CK + 4 ms"]
    #[inline(always)]
    pub fn is_extclk_6ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCLK_6CK_4MS
    }
    #[doc = "Int. RC Osc. 1 MHz; Start-up time: 6 CK + 4 ms"]
    #[inline(always)]
    pub fn is_intrcosc_1mhz_6ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_1MHZ_6CK_4MS
    }
    #[doc = "Int. RC Osc. 2 MHz; Start-up time: 6 CK + 4 ms"]
    #[inline(always)]
    pub fn is_intrcosc_2mhz_6ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_2MHZ_6CK_4MS
    }
    #[doc = "Int. RC Osc. 4 MHz; Start-up time: 6 CK + 4 ms"]
    #[inline(always)]
    pub fn is_intrcosc_4mhz_6ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_4MHZ_6CK_4MS
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time: 6 CK + 4 ms"]
    #[inline(always)]
    pub fn is_intrcosc_8mhz_6ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_4MS
    }
    #[doc = "Ext. RC Osc. - 0.9 MHz; Start-up time: 18 CK + 4 ms"]
    #[inline(always)]
    pub fn is_extrcosc_xx_0mhz9_18ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_18CK_4MS
    }
    #[doc = "Ext. RC Osc. 0.9 MHz - 3.0 MHz; Start-up time: 18 CK + 4 ms"]
    #[inline(always)]
    pub fn is_extrcosc_0mhz9_3mhz_18ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_18CK_4MS
    }
    #[doc = "Ext. RC Osc. 3.0 MHz - 8.0 MHz; Start-up time: 18 CK + 4 ms"]
    #[inline(always)]
    pub fn is_extrcosc_3mhz_8mhz_18ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_18CK_4MS
    }
    #[doc = "Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 4 ms"]
    #[inline(always)]
    pub fn is_extrcosc_8mhz_12mhz_18ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_18CK_4MS
    }
    #[doc = "Ext. Low-Freq. Crystal; Start-up time: 1K CK + 64 ms"]
    #[inline(always)]
    pub fn is_extlofxtal_1kck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTLOFXTAL_1KCK_64MS
    }
    #[doc = "Ext. Crystal/Resonator Low Freq.; Start-up time: 258 CK + 64 ms"]
    #[inline(always)]
    pub fn is_extlofxtalres_258ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTLOFXTALRES_258CK_64MS
    }
    #[doc = "Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 0 ms"]
    #[inline(always)]
    pub fn is_extlofxtalres_16kck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTLOFXTALRES_16KCK_0MS
    }
    #[doc = "Ext. Crystal/Resonator Medium Freq.; Start-up time: 258 CK + 64 ms"]
    #[inline(always)]
    pub fn is_extmedfxtalres_258ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTMEDFXTALRES_258CK_64MS
    }
    #[doc = "Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 0 ms"]
    #[inline(always)]
    pub fn is_extmedfxtalres_16kck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTMEDFXTALRES_16KCK_0MS
    }
    #[doc = "Ext. Crystal/Resonator High Freq.; Start-up time: 258 CK + 64 ms"]
    #[inline(always)]
    pub fn is_exthifxtalres_258ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTHIFXTALRES_258CK_64MS
    }
    #[doc = "Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 0 ms"]
    #[inline(always)]
    pub fn is_exthifxtalres_16kck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTHIFXTALRES_16KCK_0MS
    }
    #[doc = "Ext. Clock; Start-up time: 6 CK + 64 ms"]
    #[inline(always)]
    pub fn is_extclk_6ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCLK_6CK_64MS
    }
    #[doc = "Int. RC Osc. 1 MHz; Start-up time: 6 CK + 64 ms"]
    #[inline(always)]
    pub fn is_intrcosc_1mhz_6ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_1MHZ_6CK_64MS
    }
    #[doc = "Int. RC Osc. 2 MHz; Start-up time: 6 CK + 64 ms"]
    #[inline(always)]
    pub fn is_intrcosc_2mhz_6ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_2MHZ_6CK_64MS
    }
    #[doc = "Int. RC Osc. 4 MHz; Start-up time: 6 CK + 64 ms"]
    #[inline(always)]
    pub fn is_intrcosc_4mhz_6ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_4MHZ_6CK_64MS
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time: 6 CK + 64 ms"]
    #[inline(always)]
    pub fn is_intrcosc_8mhz_6ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_64MS
    }
    #[doc = "Ext. RC Osc. - 0.9 MHz; Start-up time: 18 CK + 64 ms"]
    #[inline(always)]
    pub fn is_extrcosc_xx_0mhz9_18ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_18CK_64MS
    }
    #[doc = "Ext. RC Osc. 0.9 MHz - 3.0 MHz; Start-up time: 18 CK + 64 ms"]
    #[inline(always)]
    pub fn is_extrcosc_0mhz9_3mhz_18ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_18CK_64MS
    }
    #[doc = "Ext. RC Osc. 3.0 MHz - 8.0 MHz; Start-up time: 18 CK + 64 ms"]
    #[inline(always)]
    pub fn is_extrcosc_3mhz_8mhz_18ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_18CK_64MS
    }
    #[doc = "Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 64 ms"]
    #[inline(always)]
    pub fn is_extrcosc_8mhz_12mhz_18ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_18CK_64MS
    }
    #[doc = "Ext. Low-Freq. Crystal; Start-up time: 32K CK + 64 ms"]
    #[inline(always)]
    pub fn is_extlofxtal_32kck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTLOFXTAL_32KCK_64MS
    }
    #[doc = "Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 0 ms"]
    #[inline(always)]
    pub fn is_extlofxtalres_1kck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTLOFXTALRES_1KCK_0MS
    }
    #[doc = "Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 4 ms"]
    #[inline(always)]
    pub fn is_extlofxtalres_16kck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTLOFXTALRES_16KCK_4MS
    }
    #[doc = "Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 0 ms"]
    #[inline(always)]
    pub fn is_extmedfxtalres_1kck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTMEDFXTALRES_1KCK_0MS
    }
    #[doc = "Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 4 ms"]
    #[inline(always)]
    pub fn is_extmedfxtalres_16kck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTMEDFXTALRES_16KCK_4MS
    }
    #[doc = "Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 0 ms"]
    #[inline(always)]
    pub fn is_exthifxtalres_1kck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTHIFXTALRES_1KCK_0MS
    }
    #[doc = "Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 4 ms"]
    #[inline(always)]
    pub fn is_exthifxtalres_16kck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTHIFXTALRES_16KCK_4MS
    }
    #[doc = "Ext. RC Osc. - 0.9 MHz; Start-up time: 6 CK + 4 ms"]
    #[inline(always)]
    pub fn is_extrcosc_xx_0mhz9_6ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_6CK_4MS
    }
    #[doc = "Ext. RC Osc. 0.9 MHz - 3.0 MHz; Start-up time: 6 CK + 4 ms"]
    #[inline(always)]
    pub fn is_extrcosc_0mhz9_3mhz_6ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_6CK_4MS
    }
    #[doc = "Ext. RC Osc. 3.0 MHz - 8.0 MHz; Start-up time: 6 CK + 4 ms"]
    #[inline(always)]
    pub fn is_extrcosc_3mhz_8mhz_6ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_6CK_4MS
    }
    #[doc = "Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 6 CK + 4 ms"]
    #[inline(always)]
    pub fn is_extrcosc_8mhz_12mhz_6ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_6CK_4MS
    }
    #[doc = "Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 4 ms"]
    #[inline(always)]
    pub fn is_extlofxtalres_1kck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTLOFXTALRES_1KCK_4MS
    }
    #[doc = "Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 64 ms"]
    #[inline(always)]
    pub fn is_extlofxtalres_16kck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTLOFXTALRES_16KCK_64MS
    }
    #[doc = "Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 4 ms"]
    #[inline(always)]
    pub fn is_extmedfxtalres_1kck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTMEDFXTALRES_1KCK_4MS
    }
    #[doc = "Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 64 ms"]
    #[inline(always)]
    pub fn is_extmedfxtalres_16kck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTMEDFXTALRES_16KCK_64MS
    }
    #[doc = "Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 4 ms"]
    #[inline(always)]
    pub fn is_exthifxtalres_1kck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTHIFXTALRES_1KCK_4MS
    }
    #[doc = "Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 64 ms"]
    #[inline(always)]
    pub fn is_exthifxtalres_16kck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTHIFXTALRES_16KCK_64MS
    }
}
#[doc = "Field `SUT_CKSEL` writer - Select Clock Source"]
pub type SUT_CKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6, SUT_CKSEL_A>;
impl<'a, REG> SUT_CKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ext. Clock; Start-up time: 6 CK + 0 ms"]
    #[inline(always)]
    pub fn extclk_6ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCLK_6CK_0MS)
    }
    #[doc = "Int. RC Osc. 1 MHz; Start-up time: 6 CK + 0 ms"]
    #[inline(always)]
    pub fn intrcosc_1mhz_6ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_1MHZ_6CK_0MS)
    }
    #[doc = "Int. RC Osc. 2 MHz; Start-up time: 6 CK + 0 ms"]
    #[inline(always)]
    pub fn intrcosc_2mhz_6ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_2MHZ_6CK_0MS)
    }
    #[doc = "Int. RC Osc. 4 MHz; Start-up time: 6 CK + 0 ms"]
    #[inline(always)]
    pub fn intrcosc_4mhz_6ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_4MHZ_6CK_0MS)
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time: 6 CK + 0 ms"]
    #[inline(always)]
    pub fn intrcosc_8mhz_6ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_0MS)
    }
    #[doc = "Ext. RC Osc. - 0.9 MHz; Start-up time: 18 CK + 0 ms"]
    #[inline(always)]
    pub fn extrcosc_xx_0mhz9_18ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_18CK_0MS)
    }
    #[doc = "Ext. RC Osc. 0.9 MHz - 3.0 MHz; Start-up time: 18 CK + 0 ms"]
    #[inline(always)]
    pub fn extrcosc_0mhz9_3mhz_18ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_18CK_0MS)
    }
    #[doc = "Ext. RC Osc. 3.0 MHz - 8.0 MHz; Start-up time: 18 CK + 0 ms"]
    #[inline(always)]
    pub fn extrcosc_3mhz_8mhz_18ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_18CK_0MS)
    }
    #[doc = "Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 0 ms"]
    #[inline(always)]
    pub fn extrcosc_8mhz_12mhz_18ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_18CK_0MS)
    }
    #[doc = "Ext. Low-Freq. Crystal; Start-up time: 1K CK + 4 ms"]
    #[inline(always)]
    pub fn extlofxtal_1kck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTLOFXTAL_1KCK_4MS)
    }
    #[doc = "Ext. Crystal/Resonator Low Freq.; Start-up time: 258 CK + 4 ms"]
    #[inline(always)]
    pub fn extlofxtalres_258ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTLOFXTALRES_258CK_4MS)
    }
    #[doc = "Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 64 ms"]
    #[inline(always)]
    pub fn extlofxtalres_1kck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTLOFXTALRES_1KCK_64MS)
    }
    #[doc = "Ext. Crystal/Resonator Medium Freq.; Start-up time: 258 CK + 4 ms"]
    #[inline(always)]
    pub fn extmedfxtalres_258ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTMEDFXTALRES_258CK_4MS)
    }
    #[doc = "Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 64 ms"]
    #[inline(always)]
    pub fn extmedfxtalres_1kck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTMEDFXTALRES_1KCK_64MS)
    }
    #[doc = "Ext. Crystal/Resonator High Freq.; Start-up time: 258 CK + 4 ms"]
    #[inline(always)]
    pub fn exthifxtalres_258ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTHIFXTALRES_258CK_4MS)
    }
    #[doc = "Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 64 ms"]
    #[inline(always)]
    pub fn exthifxtalres_1kck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTHIFXTALRES_1KCK_64MS)
    }
    #[doc = "Ext. Clock; Start-up time: 6 CK + 4 ms"]
    #[inline(always)]
    pub fn extclk_6ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCLK_6CK_4MS)
    }
    #[doc = "Int. RC Osc. 1 MHz; Start-up time: 6 CK + 4 ms"]
    #[inline(always)]
    pub fn intrcosc_1mhz_6ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_1MHZ_6CK_4MS)
    }
    #[doc = "Int. RC Osc. 2 MHz; Start-up time: 6 CK + 4 ms"]
    #[inline(always)]
    pub fn intrcosc_2mhz_6ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_2MHZ_6CK_4MS)
    }
    #[doc = "Int. RC Osc. 4 MHz; Start-up time: 6 CK + 4 ms"]
    #[inline(always)]
    pub fn intrcosc_4mhz_6ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_4MHZ_6CK_4MS)
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time: 6 CK + 4 ms"]
    #[inline(always)]
    pub fn intrcosc_8mhz_6ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_4MS)
    }
    #[doc = "Ext. RC Osc. - 0.9 MHz; Start-up time: 18 CK + 4 ms"]
    #[inline(always)]
    pub fn extrcosc_xx_0mhz9_18ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_18CK_4MS)
    }
    #[doc = "Ext. RC Osc. 0.9 MHz - 3.0 MHz; Start-up time: 18 CK + 4 ms"]
    #[inline(always)]
    pub fn extrcosc_0mhz9_3mhz_18ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_18CK_4MS)
    }
    #[doc = "Ext. RC Osc. 3.0 MHz - 8.0 MHz; Start-up time: 18 CK + 4 ms"]
    #[inline(always)]
    pub fn extrcosc_3mhz_8mhz_18ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_18CK_4MS)
    }
    #[doc = "Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 4 ms"]
    #[inline(always)]
    pub fn extrcosc_8mhz_12mhz_18ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_18CK_4MS)
    }
    #[doc = "Ext. Low-Freq. Crystal; Start-up time: 1K CK + 64 ms"]
    #[inline(always)]
    pub fn extlofxtal_1kck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTLOFXTAL_1KCK_64MS)
    }
    #[doc = "Ext. Crystal/Resonator Low Freq.; Start-up time: 258 CK + 64 ms"]
    #[inline(always)]
    pub fn extlofxtalres_258ck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTLOFXTALRES_258CK_64MS)
    }
    #[doc = "Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 0 ms"]
    #[inline(always)]
    pub fn extlofxtalres_16kck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTLOFXTALRES_16KCK_0MS)
    }
    #[doc = "Ext. Crystal/Resonator Medium Freq.; Start-up time: 258 CK + 64 ms"]
    #[inline(always)]
    pub fn extmedfxtalres_258ck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTMEDFXTALRES_258CK_64MS)
    }
    #[doc = "Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 0 ms"]
    #[inline(always)]
    pub fn extmedfxtalres_16kck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTMEDFXTALRES_16KCK_0MS)
    }
    #[doc = "Ext. Crystal/Resonator High Freq.; Start-up time: 258 CK + 64 ms"]
    #[inline(always)]
    pub fn exthifxtalres_258ck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTHIFXTALRES_258CK_64MS)
    }
    #[doc = "Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 0 ms"]
    #[inline(always)]
    pub fn exthifxtalres_16kck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTHIFXTALRES_16KCK_0MS)
    }
    #[doc = "Ext. Clock; Start-up time: 6 CK + 64 ms"]
    #[inline(always)]
    pub fn extclk_6ck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCLK_6CK_64MS)
    }
    #[doc = "Int. RC Osc. 1 MHz; Start-up time: 6 CK + 64 ms"]
    #[inline(always)]
    pub fn intrcosc_1mhz_6ck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_1MHZ_6CK_64MS)
    }
    #[doc = "Int. RC Osc. 2 MHz; Start-up time: 6 CK + 64 ms"]
    #[inline(always)]
    pub fn intrcosc_2mhz_6ck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_2MHZ_6CK_64MS)
    }
    #[doc = "Int. RC Osc. 4 MHz; Start-up time: 6 CK + 64 ms"]
    #[inline(always)]
    pub fn intrcosc_4mhz_6ck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_4MHZ_6CK_64MS)
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time: 6 CK + 64 ms"]
    #[inline(always)]
    pub fn intrcosc_8mhz_6ck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_64MS)
    }
    #[doc = "Ext. RC Osc. - 0.9 MHz; Start-up time: 18 CK + 64 ms"]
    #[inline(always)]
    pub fn extrcosc_xx_0mhz9_18ck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_18CK_64MS)
    }
    #[doc = "Ext. RC Osc. 0.9 MHz - 3.0 MHz; Start-up time: 18 CK + 64 ms"]
    #[inline(always)]
    pub fn extrcosc_0mhz9_3mhz_18ck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_18CK_64MS)
    }
    #[doc = "Ext. RC Osc. 3.0 MHz - 8.0 MHz; Start-up time: 18 CK + 64 ms"]
    #[inline(always)]
    pub fn extrcosc_3mhz_8mhz_18ck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_18CK_64MS)
    }
    #[doc = "Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 64 ms"]
    #[inline(always)]
    pub fn extrcosc_8mhz_12mhz_18ck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_18CK_64MS)
    }
    #[doc = "Ext. Low-Freq. Crystal; Start-up time: 32K CK + 64 ms"]
    #[inline(always)]
    pub fn extlofxtal_32kck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTLOFXTAL_32KCK_64MS)
    }
    #[doc = "Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 0 ms"]
    #[inline(always)]
    pub fn extlofxtalres_1kck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTLOFXTALRES_1KCK_0MS)
    }
    #[doc = "Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 4 ms"]
    #[inline(always)]
    pub fn extlofxtalres_16kck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTLOFXTALRES_16KCK_4MS)
    }
    #[doc = "Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 0 ms"]
    #[inline(always)]
    pub fn extmedfxtalres_1kck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTMEDFXTALRES_1KCK_0MS)
    }
    #[doc = "Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 4 ms"]
    #[inline(always)]
    pub fn extmedfxtalres_16kck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTMEDFXTALRES_16KCK_4MS)
    }
    #[doc = "Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 0 ms"]
    #[inline(always)]
    pub fn exthifxtalres_1kck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTHIFXTALRES_1KCK_0MS)
    }
    #[doc = "Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 4 ms"]
    #[inline(always)]
    pub fn exthifxtalres_16kck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTHIFXTALRES_16KCK_4MS)
    }
    #[doc = "Ext. RC Osc. - 0.9 MHz; Start-up time: 6 CK + 4 ms"]
    #[inline(always)]
    pub fn extrcosc_xx_0mhz9_6ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_6CK_4MS)
    }
    #[doc = "Ext. RC Osc. 0.9 MHz - 3.0 MHz; Start-up time: 6 CK + 4 ms"]
    #[inline(always)]
    pub fn extrcosc_0mhz9_3mhz_6ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_6CK_4MS)
    }
    #[doc = "Ext. RC Osc. 3.0 MHz - 8.0 MHz; Start-up time: 6 CK + 4 ms"]
    #[inline(always)]
    pub fn extrcosc_3mhz_8mhz_6ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_6CK_4MS)
    }
    #[doc = "Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 6 CK + 4 ms"]
    #[inline(always)]
    pub fn extrcosc_8mhz_12mhz_6ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_6CK_4MS)
    }
    #[doc = "Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 4 ms"]
    #[inline(always)]
    pub fn extlofxtalres_1kck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTLOFXTALRES_1KCK_4MS)
    }
    #[doc = "Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 64 ms"]
    #[inline(always)]
    pub fn extlofxtalres_16kck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTLOFXTALRES_16KCK_64MS)
    }
    #[doc = "Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 4 ms"]
    #[inline(always)]
    pub fn extmedfxtalres_1kck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTMEDFXTALRES_1KCK_4MS)
    }
    #[doc = "Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 64 ms"]
    #[inline(always)]
    pub fn extmedfxtalres_16kck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTMEDFXTALRES_16KCK_64MS)
    }
    #[doc = "Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 4 ms"]
    #[inline(always)]
    pub fn exthifxtalres_1kck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTHIFXTALRES_1KCK_4MS)
    }
    #[doc = "Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 64 ms"]
    #[inline(always)]
    pub fn exthifxtalres_16kck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTHIFXTALRES_16KCK_64MS)
    }
}
#[doc = "Field `BODEN` reader - Brown-out detection enabled"]
pub type BODEN_R = crate::BitReader;
#[doc = "Field `BODEN` writer - Brown-out detection enabled"]
pub type BODEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODLEVEL` reader - Brownout detector trigger level"]
pub type BODLEVEL_R = crate::BitReader<BODLEVEL_A>;
#[doc = "Brownout detector trigger level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BODLEVEL_A {
    #[doc = "0: Brown-out detection at VCC=4.0 V"]
    _4V0 = 0,
    #[doc = "1: Brown-out detection at VCC=2.7 V"]
    _2V7 = 1,
}
impl From<BODLEVEL_A> for bool {
    #[inline(always)]
    fn from(variant: BODLEVEL_A) -> Self {
        variant as u8 != 0
    }
}
impl BODLEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BODLEVEL_A {
        match self.bits {
            false => BODLEVEL_A::_4V0,
            true => BODLEVEL_A::_2V7,
        }
    }
    #[doc = "Brown-out detection at VCC=4.0 V"]
    #[inline(always)]
    pub fn is_4v0(&self) -> bool {
        *self == BODLEVEL_A::_4V0
    }
    #[doc = "Brown-out detection at VCC=2.7 V"]
    #[inline(always)]
    pub fn is_2v7(&self) -> bool {
        *self == BODLEVEL_A::_2V7
    }
}
#[doc = "Field `BODLEVEL` writer - Brownout detector trigger level"]
pub type BODLEVEL_W<'a, REG> = crate::BitWriter<'a, REG, BODLEVEL_A>;
impl<'a, REG> BODLEVEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Brown-out detection at VCC=4.0 V"]
    #[inline(always)]
    pub fn _4v0(self) -> &'a mut crate::W<REG> {
        self.variant(BODLEVEL_A::_4V0)
    }
    #[doc = "Brown-out detection at VCC=2.7 V"]
    #[inline(always)]
    pub fn _2v7(self) -> &'a mut crate::W<REG> {
        self.variant(BODLEVEL_A::_2V7)
    }
}
impl R {
    #[doc = "Bits 0:5 - Select Clock Source"]
    #[inline(always)]
    pub fn sut_cksel(&self) -> SUT_CKSEL_R {
        SUT_CKSEL_R::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Brown-out detection enabled"]
    #[inline(always)]
    pub fn boden(&self) -> BODEN_R {
        BODEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Brownout detector trigger level"]
    #[inline(always)]
    pub fn bodlevel(&self) -> BODLEVEL_R {
        BODLEVEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Select Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn sut_cksel(&mut self) -> SUT_CKSEL_W<LOW_SPEC> {
        SUT_CKSEL_W::new(self, 0)
    }
    #[doc = "Bit 6 - Brown-out detection enabled"]
    #[inline(always)]
    #[must_use]
    pub fn boden(&mut self) -> BODEN_W<LOW_SPEC> {
        BODEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Brownout detector trigger level"]
    #[inline(always)]
    #[must_use]
    pub fn bodlevel(&mut self) -> BODLEVEL_W<LOW_SPEC> {
        BODLEVEL_W::new(self, 7)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`low::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`low::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOW_SPEC;
impl crate::RegisterSpec for LOW_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`low::R`](R) reader structure"]
impl crate::Readable for LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`low::W`](W) writer structure"]
impl crate::Writable for LOW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOW to value 0"]
impl crate::Resettable for LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
