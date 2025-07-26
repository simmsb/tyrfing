#[doc = "Register `LOW` reader"]
pub type R = crate::R<LOW_SPEC>;
#[doc = "Register `LOW` writer"]
pub type W = crate::W<LOW_SPEC>;
#[doc = "Field `SUT_CKSEL` reader - Select Clock source"]
pub type SUT_CKSEL_R = crate::FieldReader<SUT_CKSEL_A>;
#[doc = "Select Clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SUT_CKSEL_A {
    #[doc = "0: Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms"]
    EXTCLK_6CK_14CK_0MS = 0,
    #[doc = "1: PLL Clock; Start-up time PWRDWN/RESET: 1K CK/14 CK + 8 ms"]
    PLLCLK_1KCK_14CK_8MS = 1,
    #[doc = "2: Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms"]
    INTRCOSC_8MHZ_6CK_14CK_0MS = 2,
    #[doc = "3: WD. Osc. 128 kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms"]
    WDOSC_128KHZ_6CK_14CK_0MS = 3,
    #[doc = "4: Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1 CK 4 ms"]
    EXTLOFXTAL_1CK_4MS = 4,
    #[doc = "8: Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms"]
    EXTCRES_0MHZ4_0MHZ9_258CK_14CK_4MS1 = 8,
    #[doc = "9: Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms"]
    EXTCRES_0MHZ4_0MHZ9_1KCK_14CK_65MS = 9,
    #[doc = "10: Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms"]
    EXTCRES_0MHZ9_3MHZ_258CK_14CK_4MS1 = 10,
    #[doc = "11: Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms"]
    EXTCRES_0MHZ9_3MHZ_1KCK_14CK_65MS = 11,
    #[doc = "12: Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms"]
    EXTCRES_3MHZ_8MHZ_258CK_14CK_4MS1 = 12,
    #[doc = "13: Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms"]
    EXTCRES_3MHZ_8MHZ_1KCK_14CK_65MS = 13,
    #[doc = "14: Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms"]
    EXTCRES_8MHZ_XX_258CK_14CK_4MS1 = 14,
    #[doc = "15: Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms"]
    EXTCRES_8MHZ_XX_1KCK_14CK_65MS = 15,
    #[doc = "16: Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4 ms"]
    EXTCLK_6CK_14CK_4MS = 16,
    #[doc = "17: PLL Clock; Start-up time PWRDWN/RESET: 16K CK/14 CK + 8 ms"]
    PLLCLK_16KCK_14CK_8MS = 17,
    #[doc = "18: Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4 ms"]
    INTRCOSC_8MHZ_6CK_14CK_4MS = 18,
    #[doc = "19: WD. Osc. 128 kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4 ms"]
    WDOSC_128KHZ_6CK_14CK_4MS = 19,
    #[doc = "20: Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1 CK + 64 ms"]
    EXTLOFXTAL_1CK_64MS = 20,
    #[doc = "24: Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms"]
    EXTCRES_0MHZ4_0MHZ9_258CK_14CK_65MS = 24,
    #[doc = "25: Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms"]
    EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_0MS = 25,
    #[doc = "26: Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms"]
    EXTCRES_0MHZ9_3MHZ_258CK_14CK_65MS = 26,
    #[doc = "27: Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms"]
    EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_0MS = 27,
    #[doc = "28: Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms"]
    EXTCRES_3MHZ_8MHZ_258CK_14CK_65MS = 28,
    #[doc = "29: Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms"]
    EXTXOSC_3MHZ_8MHZ_16KCK_14CK_0MS = 29,
    #[doc = "30: Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms"]
    EXTCRES_8MHZ_XX_258CK_14CK_65MS = 30,
    #[doc = "31: Ext. Crystal Osc. 8.0- MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms"]
    EXTXOSC_8MHZ_XX_16KCK_14CK_0MS = 31,
    #[doc = "32: Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 64 ms"]
    EXTCLK_6CK_14CK_64MS = 32,
    #[doc = "33: PLL Clock; Start-up time PWRDWN/RESET: 1K CK/14 CK + 68 ms"]
    PLLCLK_1KCK_14CK_68MS = 33,
    #[doc = "34: Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 64 ms"]
    INTRCOSC_8MHZ_6CK_14CK_64MS = 34,
    #[doc = "35: WD. Osc. 128 kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 64 ms"]
    WDOSC_128KHZ_6CK_14CK_64MS = 35,
    #[doc = "36: Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 32 CK + 64 ms"]
    EXTLOFXTAL_32CK_64MS = 36,
    #[doc = "40: Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms"]
    EXTCRES_0MHZ4_0MHZ9_1KCK_14CK_0MS = 40,
    #[doc = "41: Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms"]
    EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_4MS1 = 41,
    #[doc = "42: Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms"]
    EXTCRES_0MHZ9_3MHZ_1KCK_14CK_0MS = 42,
    #[doc = "43: Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms"]
    EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_4MS1 = 43,
    #[doc = "44: Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms"]
    EXTCRES_3MHZ_8MHZ_1KCK_14CK_0MS = 44,
    #[doc = "45: Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms"]
    EXTXOSC_3MHZ_8MHZ_16KCK_14CK_4MS1 = 45,
    #[doc = "46: Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms"]
    EXTCRES_8MHZ_XX_1KCK_14CK_0MS = 46,
    #[doc = "47: Ext. Crystal Osc. 8.0- MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms"]
    EXTXOSC_8MHZ_XX_16KCK_14CK_4MS1 = 47,
    #[doc = "49: PLL Clock; Start-up time PWRDWN/RESET: 16K CK/14 CK + 68 ms"]
    PLLCLK_16KCK_14CK_68MS = 49,
    #[doc = "56: Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms"]
    EXTCRES_0MHZ4_0MHZ9_1KCK_14CK_4MS1 = 56,
    #[doc = "57: Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms"]
    EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_65MS = 57,
    #[doc = "58: Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms"]
    EXTCRES_0MHZ9_3MHZ_1KCK_14CK_4MS1 = 58,
    #[doc = "59: Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms"]
    EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_65MS = 59,
    #[doc = "60: Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms"]
    EXTCRES_3MHZ_8MHZ_1KCK_14CK_4MS1 = 60,
    #[doc = "61: Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms"]
    EXTXOSC_3MHZ_8MHZ_16KCK_14CK_65MS = 61,
    #[doc = "62: Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms"]
    EXTCRES_8MHZ_XX_1KCK_14CK_4MS1 = 62,
    #[doc = "63: Ext. Crystal Osc. 8.0- MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms"]
    EXTXOSC_8MHZ_XX_16KCK_14CK_65MS = 63,
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
            0 => Some(SUT_CKSEL_A::EXTCLK_6CK_14CK_0MS),
            1 => Some(SUT_CKSEL_A::PLLCLK_1KCK_14CK_8MS),
            2 => Some(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_14CK_0MS),
            3 => Some(SUT_CKSEL_A::WDOSC_128KHZ_6CK_14CK_0MS),
            4 => Some(SUT_CKSEL_A::EXTLOFXTAL_1CK_4MS),
            8 => Some(SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_258CK_14CK_4MS1),
            9 => Some(SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_1KCK_14CK_65MS),
            10 => Some(SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_258CK_14CK_4MS1),
            11 => Some(SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_1KCK_14CK_65MS),
            12 => Some(SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_258CK_14CK_4MS1),
            13 => Some(SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_1KCK_14CK_65MS),
            14 => Some(SUT_CKSEL_A::EXTCRES_8MHZ_XX_258CK_14CK_4MS1),
            15 => Some(SUT_CKSEL_A::EXTCRES_8MHZ_XX_1KCK_14CK_65MS),
            16 => Some(SUT_CKSEL_A::EXTCLK_6CK_14CK_4MS),
            17 => Some(SUT_CKSEL_A::PLLCLK_16KCK_14CK_8MS),
            18 => Some(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_14CK_4MS),
            19 => Some(SUT_CKSEL_A::WDOSC_128KHZ_6CK_14CK_4MS),
            20 => Some(SUT_CKSEL_A::EXTLOFXTAL_1CK_64MS),
            24 => Some(SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_258CK_14CK_65MS),
            25 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_0MS),
            26 => Some(SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_258CK_14CK_65MS),
            27 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_0MS),
            28 => Some(SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_258CK_14CK_65MS),
            29 => Some(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_14CK_0MS),
            30 => Some(SUT_CKSEL_A::EXTCRES_8MHZ_XX_258CK_14CK_65MS),
            31 => Some(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_14CK_0MS),
            32 => Some(SUT_CKSEL_A::EXTCLK_6CK_14CK_64MS),
            33 => Some(SUT_CKSEL_A::PLLCLK_1KCK_14CK_68MS),
            34 => Some(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_14CK_64MS),
            35 => Some(SUT_CKSEL_A::WDOSC_128KHZ_6CK_14CK_64MS),
            36 => Some(SUT_CKSEL_A::EXTLOFXTAL_32CK_64MS),
            40 => Some(SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_1KCK_14CK_0MS),
            41 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_4MS1),
            42 => Some(SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_1KCK_14CK_0MS),
            43 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_4MS1),
            44 => Some(SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_1KCK_14CK_0MS),
            45 => Some(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_14CK_4MS1),
            46 => Some(SUT_CKSEL_A::EXTCRES_8MHZ_XX_1KCK_14CK_0MS),
            47 => Some(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_14CK_4MS1),
            49 => Some(SUT_CKSEL_A::PLLCLK_16KCK_14CK_68MS),
            56 => Some(SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_1KCK_14CK_4MS1),
            57 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_65MS),
            58 => Some(SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_1KCK_14CK_4MS1),
            59 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_65MS),
            60 => Some(SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_1KCK_14CK_4MS1),
            61 => Some(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_14CK_65MS),
            62 => Some(SUT_CKSEL_A::EXTCRES_8MHZ_XX_1KCK_14CK_4MS1),
            63 => Some(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_14CK_65MS),
            _ => None,
        }
    }
    #[doc = "Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extclk_6ck_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCLK_6CK_14CK_0MS
    }
    #[doc = "PLL Clock; Start-up time PWRDWN/RESET: 1K CK/14 CK + 8 ms"]
    #[inline(always)]
    pub fn is_pllclk_1kck_14ck_8ms(&self) -> bool {
        *self == SUT_CKSEL_A::PLLCLK_1KCK_14CK_8MS
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_intrcosc_8mhz_6ck_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_14CK_0MS
    }
    #[doc = "WD. Osc. 128 kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_wdosc_128khz_6ck_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::WDOSC_128KHZ_6CK_14CK_0MS
    }
    #[doc = "Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1 CK 4 ms"]
    #[inline(always)]
    pub fn is_extlofxtal_1ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTLOFXTAL_1CK_4MS
    }
    #[doc = "Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_extcres_0mhz4_0mhz9_258ck_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_258CK_14CK_4MS1
    }
    #[doc = "Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_extcres_0mhz4_0mhz9_1kck_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_1KCK_14CK_65MS
    }
    #[doc = "Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_extcres_0mhz9_3mhz_258ck_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_258CK_14CK_4MS1
    }
    #[doc = "Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_extcres_0mhz9_3mhz_1kck_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_1KCK_14CK_65MS
    }
    #[doc = "Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_extcres_3mhz_8mhz_258ck_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_258CK_14CK_4MS1
    }
    #[doc = "Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_extcres_3mhz_8mhz_1kck_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_1KCK_14CK_65MS
    }
    #[doc = "Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_extcres_8mhz_xx_258ck_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_8MHZ_XX_258CK_14CK_4MS1
    }
    #[doc = "Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_extcres_8mhz_xx_1kck_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_8MHZ_XX_1KCK_14CK_65MS
    }
    #[doc = "Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4 ms"]
    #[inline(always)]
    pub fn is_extclk_6ck_14ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCLK_6CK_14CK_4MS
    }
    #[doc = "PLL Clock; Start-up time PWRDWN/RESET: 16K CK/14 CK + 8 ms"]
    #[inline(always)]
    pub fn is_pllclk_16kck_14ck_8ms(&self) -> bool {
        *self == SUT_CKSEL_A::PLLCLK_16KCK_14CK_8MS
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4 ms"]
    #[inline(always)]
    pub fn is_intrcosc_8mhz_6ck_14ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_14CK_4MS
    }
    #[doc = "WD. Osc. 128 kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4 ms"]
    #[inline(always)]
    pub fn is_wdosc_128khz_6ck_14ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::WDOSC_128KHZ_6CK_14CK_4MS
    }
    #[doc = "Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1 CK + 64 ms"]
    #[inline(always)]
    pub fn is_extlofxtal_1ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTLOFXTAL_1CK_64MS
    }
    #[doc = "Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_extcres_0mhz4_0mhz9_258ck_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_258CK_14CK_65MS
    }
    #[doc = "Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extxosc_0mhz4_0mhz9_16kck_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_0MS
    }
    #[doc = "Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_extcres_0mhz9_3mhz_258ck_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_258CK_14CK_65MS
    }
    #[doc = "Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extxosc_0mhz9_3mhz_16kck_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_0MS
    }
    #[doc = "Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_extcres_3mhz_8mhz_258ck_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_258CK_14CK_65MS
    }
    #[doc = "Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extxosc_3mhz_8mhz_16kck_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_14CK_0MS
    }
    #[doc = "Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_extcres_8mhz_xx_258ck_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_8MHZ_XX_258CK_14CK_65MS
    }
    #[doc = "Ext. Crystal Osc. 8.0- MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extxosc_8mhz_xx_16kck_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_14CK_0MS
    }
    #[doc = "Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 64 ms"]
    #[inline(always)]
    pub fn is_extclk_6ck_14ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCLK_6CK_14CK_64MS
    }
    #[doc = "PLL Clock; Start-up time PWRDWN/RESET: 1K CK/14 CK + 68 ms"]
    #[inline(always)]
    pub fn is_pllclk_1kck_14ck_68ms(&self) -> bool {
        *self == SUT_CKSEL_A::PLLCLK_1KCK_14CK_68MS
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 64 ms"]
    #[inline(always)]
    pub fn is_intrcosc_8mhz_6ck_14ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_14CK_64MS
    }
    #[doc = "WD. Osc. 128 kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 64 ms"]
    #[inline(always)]
    pub fn is_wdosc_128khz_6ck_14ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::WDOSC_128KHZ_6CK_14CK_64MS
    }
    #[doc = "Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 32 CK + 64 ms"]
    #[inline(always)]
    pub fn is_extlofxtal_32ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTLOFXTAL_32CK_64MS
    }
    #[doc = "Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extcres_0mhz4_0mhz9_1kck_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_1KCK_14CK_0MS
    }
    #[doc = "Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_extxosc_0mhz4_0mhz9_16kck_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_4MS1
    }
    #[doc = "Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extcres_0mhz9_3mhz_1kck_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_1KCK_14CK_0MS
    }
    #[doc = "Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_extxosc_0mhz9_3mhz_16kck_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_4MS1
    }
    #[doc = "Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extcres_3mhz_8mhz_1kck_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_1KCK_14CK_0MS
    }
    #[doc = "Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_extxosc_3mhz_8mhz_16kck_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_14CK_4MS1
    }
    #[doc = "Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extcres_8mhz_xx_1kck_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_8MHZ_XX_1KCK_14CK_0MS
    }
    #[doc = "Ext. Crystal Osc. 8.0- MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_extxosc_8mhz_xx_16kck_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_14CK_4MS1
    }
    #[doc = "PLL Clock; Start-up time PWRDWN/RESET: 16K CK/14 CK + 68 ms"]
    #[inline(always)]
    pub fn is_pllclk_16kck_14ck_68ms(&self) -> bool {
        *self == SUT_CKSEL_A::PLLCLK_16KCK_14CK_68MS
    }
    #[doc = "Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_extcres_0mhz4_0mhz9_1kck_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_1KCK_14CK_4MS1
    }
    #[doc = "Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_extxosc_0mhz4_0mhz9_16kck_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_65MS
    }
    #[doc = "Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_extcres_0mhz9_3mhz_1kck_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_1KCK_14CK_4MS1
    }
    #[doc = "Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_extxosc_0mhz9_3mhz_16kck_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_65MS
    }
    #[doc = "Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_extcres_3mhz_8mhz_1kck_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_1KCK_14CK_4MS1
    }
    #[doc = "Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_extxosc_3mhz_8mhz_16kck_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_14CK_65MS
    }
    #[doc = "Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_extcres_8mhz_xx_1kck_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_8MHZ_XX_1KCK_14CK_4MS1
    }
    #[doc = "Ext. Crystal Osc. 8.0- MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_extxosc_8mhz_xx_16kck_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_14CK_65MS
    }
}
#[doc = "Field `SUT_CKSEL` writer - Select Clock source"]
pub type SUT_CKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6, SUT_CKSEL_A>;
impl<'a, REG> SUT_CKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms"]
    #[inline(always)]
    pub fn extclk_6ck_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCLK_6CK_14CK_0MS)
    }
    #[doc = "PLL Clock; Start-up time PWRDWN/RESET: 1K CK/14 CK + 8 ms"]
    #[inline(always)]
    pub fn pllclk_1kck_14ck_8ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::PLLCLK_1KCK_14CK_8MS)
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms"]
    #[inline(always)]
    pub fn intrcosc_8mhz_6ck_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_14CK_0MS)
    }
    #[doc = "WD. Osc. 128 kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms"]
    #[inline(always)]
    pub fn wdosc_128khz_6ck_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::WDOSC_128KHZ_6CK_14CK_0MS)
    }
    #[doc = "Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1 CK 4 ms"]
    #[inline(always)]
    pub fn extlofxtal_1ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTLOFXTAL_1CK_4MS)
    }
    #[doc = "Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extcres_0mhz4_0mhz9_258ck_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_258CK_14CK_4MS1)
    }
    #[doc = "Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms"]
    #[inline(always)]
    pub fn extcres_0mhz4_0mhz9_1kck_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_1KCK_14CK_65MS)
    }
    #[doc = "Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extcres_0mhz9_3mhz_258ck_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_258CK_14CK_4MS1)
    }
    #[doc = "Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms"]
    #[inline(always)]
    pub fn extcres_0mhz9_3mhz_1kck_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_1KCK_14CK_65MS)
    }
    #[doc = "Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extcres_3mhz_8mhz_258ck_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_258CK_14CK_4MS1)
    }
    #[doc = "Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms"]
    #[inline(always)]
    pub fn extcres_3mhz_8mhz_1kck_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_1KCK_14CK_65MS)
    }
    #[doc = "Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extcres_8mhz_xx_258ck_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_8MHZ_XX_258CK_14CK_4MS1)
    }
    #[doc = "Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms"]
    #[inline(always)]
    pub fn extcres_8mhz_xx_1kck_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_8MHZ_XX_1KCK_14CK_65MS)
    }
    #[doc = "Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4 ms"]
    #[inline(always)]
    pub fn extclk_6ck_14ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCLK_6CK_14CK_4MS)
    }
    #[doc = "PLL Clock; Start-up time PWRDWN/RESET: 16K CK/14 CK + 8 ms"]
    #[inline(always)]
    pub fn pllclk_16kck_14ck_8ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::PLLCLK_16KCK_14CK_8MS)
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4 ms"]
    #[inline(always)]
    pub fn intrcosc_8mhz_6ck_14ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_14CK_4MS)
    }
    #[doc = "WD. Osc. 128 kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4 ms"]
    #[inline(always)]
    pub fn wdosc_128khz_6ck_14ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::WDOSC_128KHZ_6CK_14CK_4MS)
    }
    #[doc = "Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1 CK + 64 ms"]
    #[inline(always)]
    pub fn extlofxtal_1ck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTLOFXTAL_1CK_64MS)
    }
    #[doc = "Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms"]
    #[inline(always)]
    pub fn extcres_0mhz4_0mhz9_258ck_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_258CK_14CK_65MS)
    }
    #[doc = "Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms"]
    #[inline(always)]
    pub fn extxosc_0mhz4_0mhz9_16kck_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_0MS)
    }
    #[doc = "Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms"]
    #[inline(always)]
    pub fn extcres_0mhz9_3mhz_258ck_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_258CK_14CK_65MS)
    }
    #[doc = "Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms"]
    #[inline(always)]
    pub fn extxosc_0mhz9_3mhz_16kck_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_0MS)
    }
    #[doc = "Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms"]
    #[inline(always)]
    pub fn extcres_3mhz_8mhz_258ck_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_258CK_14CK_65MS)
    }
    #[doc = "Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms"]
    #[inline(always)]
    pub fn extxosc_3mhz_8mhz_16kck_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_14CK_0MS)
    }
    #[doc = "Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms"]
    #[inline(always)]
    pub fn extcres_8mhz_xx_258ck_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_8MHZ_XX_258CK_14CK_65MS)
    }
    #[doc = "Ext. Crystal Osc. 8.0- MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms"]
    #[inline(always)]
    pub fn extxosc_8mhz_xx_16kck_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_14CK_0MS)
    }
    #[doc = "Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 64 ms"]
    #[inline(always)]
    pub fn extclk_6ck_14ck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCLK_6CK_14CK_64MS)
    }
    #[doc = "PLL Clock; Start-up time PWRDWN/RESET: 1K CK/14 CK + 68 ms"]
    #[inline(always)]
    pub fn pllclk_1kck_14ck_68ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::PLLCLK_1KCK_14CK_68MS)
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 64 ms"]
    #[inline(always)]
    pub fn intrcosc_8mhz_6ck_14ck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_14CK_64MS)
    }
    #[doc = "WD. Osc. 128 kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 64 ms"]
    #[inline(always)]
    pub fn wdosc_128khz_6ck_14ck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::WDOSC_128KHZ_6CK_14CK_64MS)
    }
    #[doc = "Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 32 CK + 64 ms"]
    #[inline(always)]
    pub fn extlofxtal_32ck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTLOFXTAL_32CK_64MS)
    }
    #[doc = "Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms"]
    #[inline(always)]
    pub fn extcres_0mhz4_0mhz9_1kck_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_1KCK_14CK_0MS)
    }
    #[doc = "Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extxosc_0mhz4_0mhz9_16kck_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_4MS1)
    }
    #[doc = "Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms"]
    #[inline(always)]
    pub fn extcres_0mhz9_3mhz_1kck_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_1KCK_14CK_0MS)
    }
    #[doc = "Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extxosc_0mhz9_3mhz_16kck_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_4MS1)
    }
    #[doc = "Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms"]
    #[inline(always)]
    pub fn extcres_3mhz_8mhz_1kck_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_1KCK_14CK_0MS)
    }
    #[doc = "Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extxosc_3mhz_8mhz_16kck_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_14CK_4MS1)
    }
    #[doc = "Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms"]
    #[inline(always)]
    pub fn extcres_8mhz_xx_1kck_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_8MHZ_XX_1KCK_14CK_0MS)
    }
    #[doc = "Ext. Crystal Osc. 8.0- MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extxosc_8mhz_xx_16kck_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_14CK_4MS1)
    }
    #[doc = "PLL Clock; Start-up time PWRDWN/RESET: 16K CK/14 CK + 68 ms"]
    #[inline(always)]
    pub fn pllclk_16kck_14ck_68ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::PLLCLK_16KCK_14CK_68MS)
    }
    #[doc = "Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extcres_0mhz4_0mhz9_1kck_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_1KCK_14CK_4MS1)
    }
    #[doc = "Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms"]
    #[inline(always)]
    pub fn extxosc_0mhz4_0mhz9_16kck_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_65MS)
    }
    #[doc = "Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extcres_0mhz9_3mhz_1kck_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_1KCK_14CK_4MS1)
    }
    #[doc = "Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms"]
    #[inline(always)]
    pub fn extxosc_0mhz9_3mhz_16kck_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_65MS)
    }
    #[doc = "Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extcres_3mhz_8mhz_1kck_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_1KCK_14CK_4MS1)
    }
    #[doc = "Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms"]
    #[inline(always)]
    pub fn extxosc_3mhz_8mhz_16kck_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_14CK_65MS)
    }
    #[doc = "Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extcres_8mhz_xx_1kck_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_8MHZ_XX_1KCK_14CK_4MS1)
    }
    #[doc = "Ext. Crystal Osc. 8.0- MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms"]
    #[inline(always)]
    pub fn extxosc_8mhz_xx_16kck_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_14CK_65MS)
    }
}
#[doc = "Field `CKOUT` reader - Clock output on PORTB5"]
pub type CKOUT_R = crate::BitReader;
#[doc = "Field `CKOUT` writer - Clock output on PORTB5"]
pub type CKOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKDIV8` reader - Divide clock by 8 internally"]
pub type CKDIV8_R = crate::BitReader;
#[doc = "Field `CKDIV8` writer - Divide clock by 8 internally"]
pub type CKDIV8_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Select Clock source"]
    #[inline(always)]
    pub fn sut_cksel(&self) -> SUT_CKSEL_R {
        SUT_CKSEL_R::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Clock output on PORTB5"]
    #[inline(always)]
    pub fn ckout(&self) -> CKOUT_R {
        CKOUT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Divide clock by 8 internally"]
    #[inline(always)]
    pub fn ckdiv8(&self) -> CKDIV8_R {
        CKDIV8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Select Clock source"]
    #[inline(always)]
    #[must_use]
    pub fn sut_cksel(&mut self) -> SUT_CKSEL_W<LOW_SPEC> {
        SUT_CKSEL_W::new(self, 0)
    }
    #[doc = "Bit 6 - Clock output on PORTB5"]
    #[inline(always)]
    #[must_use]
    pub fn ckout(&mut self) -> CKOUT_W<LOW_SPEC> {
        CKOUT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Divide clock by 8 internally"]
    #[inline(always)]
    #[must_use]
    pub fn ckdiv8(&mut self) -> CKDIV8_W<LOW_SPEC> {
        CKDIV8_W::new(self, 7)
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
