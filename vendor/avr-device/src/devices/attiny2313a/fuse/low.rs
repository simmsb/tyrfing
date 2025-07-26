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
    #[doc = "0: Ext. Clock; Start-up time: 14 CK + 0 ms"]
    EXTCLK_14CK_0MS = 0,
    #[doc = "2: Int. RC Osc. 4 MHz; Start-up time: 14 CK + 0 ms"]
    INTRCOSC_4MHZ_14CK_0MS = 2,
    #[doc = "4: Int. RC Osc. 8 MHz; Start-up time: 14 CK + 0 ms"]
    INTRCOSC_8MHZ_14CK_0MS = 4,
    #[doc = "6: Int. RC Osc. 128 kHz; Start-up time: 14 CK + 0 ms"]
    INTRCOSC_128KHZ_14CK_0MS = 6,
    #[doc = "16: Ext. Clock; Start-up time: 14 CK + 4.1 ms"]
    EXTCLK_14CK_4MS1 = 16,
    #[doc = "18: Int. RC Osc. 4 MHz; Start-up time: 14 CK + 4.1 ms"]
    INTRCOSC_4MHZ_14CK_4MS1 = 18,
    #[doc = "20: Int. RC Osc. 8 MHz; Start-up time: 14 CK + 4.1 ms"]
    INTRCOSC_8MHZ_14CK_4MS1 = 20,
    #[doc = "22: Int. RC Osc. 128 kHz; Start-up time: 14 CK + 4 ms"]
    INTRCOSC_128KHZ_14CK_4MS = 22,
    #[doc = "25: Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 14 CK + 0 ms"]
    EXTXOSC_0MHZ4_0MHZ9_14CK_0MS = 25,
    #[doc = "27: Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 14 CK + 0 ms"]
    EXTXOSC_0MHZ9_3MHZ_14CK_0MS = 27,
    #[doc = "29: Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 14 CK + 0 ms"]
    EXTXOSC_3MHZ_8MHZ_14CK_0MS = 29,
    #[doc = "31: Ext. Crystal Osc. 8.0- MHz; Start-up time: 14 CK + 0 ms"]
    EXTXOSC_8MHZ_XX_14CK_0MS = 31,
    #[doc = "32: Ext. Clock; Start-up time: 14 CK + 65 ms"]
    EXTCLK_14CK_65MS = 32,
    #[doc = "34: Int. RC Osc. 4 MHz; Start-up time: 14 CK + 65 ms"]
    INTRCOSC_4MHZ_14CK_65MS = 34,
    #[doc = "36: Int. RC Osc. 8 MHz; Start-up time: 14 CK + 65 ms"]
    INTRCOSC_8MHZ_14CK_65MS = 36,
    #[doc = "38: Int. RC Osc. 128 kHz; Start-up time: 14 CK + 64 ms"]
    INTRCOSC_128KHZ_14CK_64MS = 38,
    #[doc = "41: Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 14 CK + 4.1 ms"]
    EXTXOSC_0MHZ4_0MHZ9_14CK_4MS1 = 41,
    #[doc = "43: Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 14 CK + 4.1 ms"]
    EXTXOSC_0MHZ9_3MHZ_14CK_4MS1 = 43,
    #[doc = "45: Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 14 CK + 4.1 ms"]
    EXTXOSC_3MHZ_8MHZ_14CK_4MS1 = 45,
    #[doc = "47: Ext. Crystal Osc. 8.0- MHz; Start-up time: 14 CK + 4.1 ms"]
    EXTXOSC_8MHZ_XX_14CK_4MS1 = 47,
    #[doc = "57: Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 14 CK + 65 ms"]
    EXTXOSC_0MHZ4_0MHZ9_14CK_65MS = 57,
    #[doc = "59: Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 14 CK + 65 ms"]
    EXTXOSC_0MHZ9_3MHZ_14CK_65MS = 59,
    #[doc = "61: Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 14 CK + 65 ms"]
    EXTXOSC_3MHZ_8MHZ_14CK_65MS = 61,
    #[doc = "63: Ext. Crystal Osc. 8.0- MHz; Start-up time: 14 CK + 65 ms"]
    EXTXOSC_8MHZ_XX_14CK_65MS = 63,
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
            0 => Some(SUT_CKSEL_A::EXTCLK_14CK_0MS),
            2 => Some(SUT_CKSEL_A::INTRCOSC_4MHZ_14CK_0MS),
            4 => Some(SUT_CKSEL_A::INTRCOSC_8MHZ_14CK_0MS),
            6 => Some(SUT_CKSEL_A::INTRCOSC_128KHZ_14CK_0MS),
            16 => Some(SUT_CKSEL_A::EXTCLK_14CK_4MS1),
            18 => Some(SUT_CKSEL_A::INTRCOSC_4MHZ_14CK_4MS1),
            20 => Some(SUT_CKSEL_A::INTRCOSC_8MHZ_14CK_4MS1),
            22 => Some(SUT_CKSEL_A::INTRCOSC_128KHZ_14CK_4MS),
            25 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_14CK_0MS),
            27 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_14CK_0MS),
            29 => Some(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_14CK_0MS),
            31 => Some(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_14CK_0MS),
            32 => Some(SUT_CKSEL_A::EXTCLK_14CK_65MS),
            34 => Some(SUT_CKSEL_A::INTRCOSC_4MHZ_14CK_65MS),
            36 => Some(SUT_CKSEL_A::INTRCOSC_8MHZ_14CK_65MS),
            38 => Some(SUT_CKSEL_A::INTRCOSC_128KHZ_14CK_64MS),
            41 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_14CK_4MS1),
            43 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_14CK_4MS1),
            45 => Some(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_14CK_4MS1),
            47 => Some(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_14CK_4MS1),
            57 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_14CK_65MS),
            59 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_14CK_65MS),
            61 => Some(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_14CK_65MS),
            63 => Some(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_14CK_65MS),
            _ => None,
        }
    }
    #[doc = "Ext. Clock; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extclk_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCLK_14CK_0MS
    }
    #[doc = "Int. RC Osc. 4 MHz; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_intrcosc_4mhz_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_4MHZ_14CK_0MS
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_intrcosc_8mhz_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_8MHZ_14CK_0MS
    }
    #[doc = "Int. RC Osc. 128 kHz; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_intrcosc_128khz_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_128KHZ_14CK_0MS
    }
    #[doc = "Ext. Clock; Start-up time: 14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_extclk_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCLK_14CK_4MS1
    }
    #[doc = "Int. RC Osc. 4 MHz; Start-up time: 14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_intrcosc_4mhz_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_4MHZ_14CK_4MS1
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time: 14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_intrcosc_8mhz_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_8MHZ_14CK_4MS1
    }
    #[doc = "Int. RC Osc. 128 kHz; Start-up time: 14 CK + 4 ms"]
    #[inline(always)]
    pub fn is_intrcosc_128khz_14ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_128KHZ_14CK_4MS
    }
    #[doc = "Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extxosc_0mhz4_0mhz9_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_14CK_0MS
    }
    #[doc = "Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extxosc_0mhz9_3mhz_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_14CK_0MS
    }
    #[doc = "Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extxosc_3mhz_8mhz_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_14CK_0MS
    }
    #[doc = "Ext. Crystal Osc. 8.0- MHz; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn is_extxosc_8mhz_xx_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_8MHZ_XX_14CK_0MS
    }
    #[doc = "Ext. Clock; Start-up time: 14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_extclk_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCLK_14CK_65MS
    }
    #[doc = "Int. RC Osc. 4 MHz; Start-up time: 14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_intrcosc_4mhz_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_4MHZ_14CK_65MS
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time: 14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_intrcosc_8mhz_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_8MHZ_14CK_65MS
    }
    #[doc = "Int. RC Osc. 128 kHz; Start-up time: 14 CK + 64 ms"]
    #[inline(always)]
    pub fn is_intrcosc_128khz_14ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_128KHZ_14CK_64MS
    }
    #[doc = "Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_extxosc_0mhz4_0mhz9_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_14CK_4MS1
    }
    #[doc = "Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_extxosc_0mhz9_3mhz_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_14CK_4MS1
    }
    #[doc = "Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_extxosc_3mhz_8mhz_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_14CK_4MS1
    }
    #[doc = "Ext. Crystal Osc. 8.0- MHz; Start-up time: 14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn is_extxosc_8mhz_xx_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_8MHZ_XX_14CK_4MS1
    }
    #[doc = "Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_extxosc_0mhz4_0mhz9_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_14CK_65MS
    }
    #[doc = "Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_extxosc_0mhz9_3mhz_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_14CK_65MS
    }
    #[doc = "Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_extxosc_3mhz_8mhz_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_14CK_65MS
    }
    #[doc = "Ext. Crystal Osc. 8.0- MHz; Start-up time: 14 CK + 65 ms"]
    #[inline(always)]
    pub fn is_extxosc_8mhz_xx_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_8MHZ_XX_14CK_65MS
    }
}
#[doc = "Field `SUT_CKSEL` writer - Select Clock Source"]
pub type SUT_CKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6, SUT_CKSEL_A>;
impl<'a, REG> SUT_CKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ext. Clock; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn extclk_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCLK_14CK_0MS)
    }
    #[doc = "Int. RC Osc. 4 MHz; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn intrcosc_4mhz_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_4MHZ_14CK_0MS)
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn intrcosc_8mhz_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_8MHZ_14CK_0MS)
    }
    #[doc = "Int. RC Osc. 128 kHz; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn intrcosc_128khz_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_128KHZ_14CK_0MS)
    }
    #[doc = "Ext. Clock; Start-up time: 14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extclk_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCLK_14CK_4MS1)
    }
    #[doc = "Int. RC Osc. 4 MHz; Start-up time: 14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn intrcosc_4mhz_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_4MHZ_14CK_4MS1)
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time: 14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn intrcosc_8mhz_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_8MHZ_14CK_4MS1)
    }
    #[doc = "Int. RC Osc. 128 kHz; Start-up time: 14 CK + 4 ms"]
    #[inline(always)]
    pub fn intrcosc_128khz_14ck_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_128KHZ_14CK_4MS)
    }
    #[doc = "Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn extxosc_0mhz4_0mhz9_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_14CK_0MS)
    }
    #[doc = "Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn extxosc_0mhz9_3mhz_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_14CK_0MS)
    }
    #[doc = "Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn extxosc_3mhz_8mhz_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_14CK_0MS)
    }
    #[doc = "Ext. Crystal Osc. 8.0- MHz; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn extxosc_8mhz_xx_14ck_0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_14CK_0MS)
    }
    #[doc = "Ext. Clock; Start-up time: 14 CK + 65 ms"]
    #[inline(always)]
    pub fn extclk_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCLK_14CK_65MS)
    }
    #[doc = "Int. RC Osc. 4 MHz; Start-up time: 14 CK + 65 ms"]
    #[inline(always)]
    pub fn intrcosc_4mhz_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_4MHZ_14CK_65MS)
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time: 14 CK + 65 ms"]
    #[inline(always)]
    pub fn intrcosc_8mhz_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_8MHZ_14CK_65MS)
    }
    #[doc = "Int. RC Osc. 128 kHz; Start-up time: 14 CK + 64 ms"]
    #[inline(always)]
    pub fn intrcosc_128khz_14ck_64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_128KHZ_14CK_64MS)
    }
    #[doc = "Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extxosc_0mhz4_0mhz9_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_14CK_4MS1)
    }
    #[doc = "Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extxosc_0mhz9_3mhz_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_14CK_4MS1)
    }
    #[doc = "Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extxosc_3mhz_8mhz_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_14CK_4MS1)
    }
    #[doc = "Ext. Crystal Osc. 8.0- MHz; Start-up time: 14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extxosc_8mhz_xx_14ck_4ms1(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_14CK_4MS1)
    }
    #[doc = "Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 14 CK + 65 ms"]
    #[inline(always)]
    pub fn extxosc_0mhz4_0mhz9_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_14CK_65MS)
    }
    #[doc = "Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 14 CK + 65 ms"]
    #[inline(always)]
    pub fn extxosc_0mhz9_3mhz_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_14CK_65MS)
    }
    #[doc = "Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 14 CK + 65 ms"]
    #[inline(always)]
    pub fn extxosc_3mhz_8mhz_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_14CK_65MS)
    }
    #[doc = "Ext. Crystal Osc. 8.0- MHz; Start-up time: 14 CK + 65 ms"]
    #[inline(always)]
    pub fn extxosc_8mhz_xx_14ck_65ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_14CK_65MS)
    }
}
#[doc = "Field `CKOUT` reader - Clock output on PORTD2"]
pub type CKOUT_R = crate::BitReader;
#[doc = "Field `CKOUT` writer - Clock output on PORTD2"]
pub type CKOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKDIV8` reader - Divide clock by 8 internally"]
pub type CKDIV8_R = crate::BitReader;
#[doc = "Field `CKDIV8` writer - Divide clock by 8 internally"]
pub type CKDIV8_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Select Clock Source"]
    #[inline(always)]
    pub fn sut_cksel(&self) -> SUT_CKSEL_R {
        SUT_CKSEL_R::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Clock output on PORTD2"]
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
    #[doc = "Bits 0:5 - Select Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn sut_cksel(&mut self) -> SUT_CKSEL_W<LOW_SPEC> {
        SUT_CKSEL_W::new(self, 0)
    }
    #[doc = "Bit 6 - Clock output on PORTD2"]
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
