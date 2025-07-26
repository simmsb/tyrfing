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
    #[doc = "0: Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/16 CK + 16 ms"]
    EXTCLK_6CK_16CK_16MS = 0,
    #[doc = "2: Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/16 CK + 16 ms"]
    INTRCOSC_8MHZ_6CK_16CK_16MS = 2,
    #[doc = "4: Int. ULP Osc.; Start-up time PWRDWN/RESET: 6 CK/16 CK + 16 ms"]
    INTULPOSC_32KHZ_6CK_16CK_16MS = 4,
    #[doc = "6: Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms"]
    EXTLOFXTAL_1KCK_16CK_16MS = 6,
    #[doc = "8: Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms"]
    EXTCRES_0MHZ4_0MHZ9_258CK_16CK_16MS = 8,
    #[doc = "9: Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms"]
    EXTXOSC_0MHZ4_0MHZ9_16KCK_16CK_16MS = 9,
    #[doc = "10: Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms"]
    EXTCRES_0MHZ9_3MHZ_258CK_16CK_16MS = 10,
    #[doc = "11: Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms"]
    EXTXOSC_0MHZ9_3MHZ_16KCK_16CK_16MS = 11,
    #[doc = "12: Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms"]
    EXTCRES_3MHZ_8MHZ_258CK_16CK_16MS = 12,
    #[doc = "13: Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms"]
    EXTXOSC_3MHZ_8MHZ_16KCK_16CK_16MS = 13,
    #[doc = "14: Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms"]
    EXTCRES_8MHZ_XX_258CK_16CK_16MS = 14,
    #[doc = "15: Ext. Crystal Osc. 8.0- MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms"]
    EXTXOSC_8MHZ_XX_16KCK_16CK_16MS = 15,
    #[doc = "22: Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 32K CK/16 CK + 16 ms"]
    EXTLOFXTAL_32KCK_14CK_16MS = 22,
    #[doc = "24: Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms"]
    EXTCRES_0MHZ4_0MHZ9_1KCK_16CK_16MS = 24,
    #[doc = "26: Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms"]
    EXTCRES_0MHZ9_3MHZ_1KCK_16CK_16MS = 26,
    #[doc = "28: Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms"]
    EXTCRES_3MHZ_8MHZ_1KCK_16CK_16MS = 28,
    #[doc = "30: Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms"]
    EXTCRES_8MHZ_XX_1KCK_16CK_16MS = 30,
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
            0 => Some(SUT_CKSEL_A::EXTCLK_6CK_16CK_16MS),
            2 => Some(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_16CK_16MS),
            4 => Some(SUT_CKSEL_A::INTULPOSC_32KHZ_6CK_16CK_16MS),
            6 => Some(SUT_CKSEL_A::EXTLOFXTAL_1KCK_16CK_16MS),
            8 => Some(SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_258CK_16CK_16MS),
            9 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_16CK_16MS),
            10 => Some(SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_258CK_16CK_16MS),
            11 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_16CK_16MS),
            12 => Some(SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_258CK_16CK_16MS),
            13 => Some(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_16CK_16MS),
            14 => Some(SUT_CKSEL_A::EXTCRES_8MHZ_XX_258CK_16CK_16MS),
            15 => Some(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_16CK_16MS),
            22 => Some(SUT_CKSEL_A::EXTLOFXTAL_32KCK_14CK_16MS),
            24 => Some(SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_1KCK_16CK_16MS),
            26 => Some(SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_1KCK_16CK_16MS),
            28 => Some(SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_1KCK_16CK_16MS),
            30 => Some(SUT_CKSEL_A::EXTCRES_8MHZ_XX_1KCK_16CK_16MS),
            _ => None,
        }
    }
    #[doc = "Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn is_extclk_6ck_16ck_16ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCLK_6CK_16CK_16MS
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn is_intrcosc_8mhz_6ck_16ck_16ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_16CK_16MS
    }
    #[doc = "Int. ULP Osc.; Start-up time PWRDWN/RESET: 6 CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn is_intulposc_32khz_6ck_16ck_16ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTULPOSC_32KHZ_6CK_16CK_16MS
    }
    #[doc = "Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn is_extlofxtal_1kck_16ck_16ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTLOFXTAL_1KCK_16CK_16MS
    }
    #[doc = "Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn is_extcres_0mhz4_0mhz9_258ck_16ck_16ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_258CK_16CK_16MS
    }
    #[doc = "Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn is_extxosc_0mhz4_0mhz9_16kck_16ck_16ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_16CK_16MS
    }
    #[doc = "Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn is_extcres_0mhz9_3mhz_258ck_16ck_16ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_258CK_16CK_16MS
    }
    #[doc = "Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn is_extxosc_0mhz9_3mhz_16kck_16ck_16ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_16CK_16MS
    }
    #[doc = "Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn is_extcres_3mhz_8mhz_258ck_16ck_16ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_258CK_16CK_16MS
    }
    #[doc = "Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn is_extxosc_3mhz_8mhz_16kck_16ck_16ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_16CK_16MS
    }
    #[doc = "Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn is_extcres_8mhz_xx_258ck_16ck_16ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_8MHZ_XX_258CK_16CK_16MS
    }
    #[doc = "Ext. Crystal Osc. 8.0- MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn is_extxosc_8mhz_xx_16kck_16ck_16ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_16CK_16MS
    }
    #[doc = "Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 32K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn is_extlofxtal_32kck_14ck_16ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTLOFXTAL_32KCK_14CK_16MS
    }
    #[doc = "Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn is_extcres_0mhz4_0mhz9_1kck_16ck_16ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_1KCK_16CK_16MS
    }
    #[doc = "Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn is_extcres_0mhz9_3mhz_1kck_16ck_16ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_1KCK_16CK_16MS
    }
    #[doc = "Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn is_extcres_3mhz_8mhz_1kck_16ck_16ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_1KCK_16CK_16MS
    }
    #[doc = "Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn is_extcres_8mhz_xx_1kck_16ck_16ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCRES_8MHZ_XX_1KCK_16CK_16MS
    }
}
#[doc = "Field `SUT_CKSEL` writer - Select Clock Source"]
pub type SUT_CKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, SUT_CKSEL_A>;
impl<'a, REG> SUT_CKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn extclk_6ck_16ck_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCLK_6CK_16CK_16MS)
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn intrcosc_8mhz_6ck_16ck_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_16CK_16MS)
    }
    #[doc = "Int. ULP Osc.; Start-up time PWRDWN/RESET: 6 CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn intulposc_32khz_6ck_16ck_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::INTULPOSC_32KHZ_6CK_16CK_16MS)
    }
    #[doc = "Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn extlofxtal_1kck_16ck_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTLOFXTAL_1KCK_16CK_16MS)
    }
    #[doc = "Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn extcres_0mhz4_0mhz9_258ck_16ck_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_258CK_16CK_16MS)
    }
    #[doc = "Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn extxosc_0mhz4_0mhz9_16kck_16ck_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_16CK_16MS)
    }
    #[doc = "Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn extcres_0mhz9_3mhz_258ck_16ck_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_258CK_16CK_16MS)
    }
    #[doc = "Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn extxosc_0mhz9_3mhz_16kck_16ck_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_16CK_16MS)
    }
    #[doc = "Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn extcres_3mhz_8mhz_258ck_16ck_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_258CK_16CK_16MS)
    }
    #[doc = "Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn extxosc_3mhz_8mhz_16kck_16ck_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_16CK_16MS)
    }
    #[doc = "Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn extcres_8mhz_xx_258ck_16ck_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_8MHZ_XX_258CK_16CK_16MS)
    }
    #[doc = "Ext. Crystal Osc. 8.0- MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn extxosc_8mhz_xx_16kck_16ck_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_16CK_16MS)
    }
    #[doc = "Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 32K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn extlofxtal_32kck_14ck_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTLOFXTAL_32KCK_14CK_16MS)
    }
    #[doc = "Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn extcres_0mhz4_0mhz9_1kck_16ck_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_0MHZ4_0MHZ9_1KCK_16CK_16MS)
    }
    #[doc = "Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn extcres_0mhz9_3mhz_1kck_16ck_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_0MHZ9_3MHZ_1KCK_16CK_16MS)
    }
    #[doc = "Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn extcres_3mhz_8mhz_1kck_16ck_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_3MHZ_8MHZ_1KCK_16CK_16MS)
    }
    #[doc = "Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms"]
    #[inline(always)]
    pub fn extcres_8mhz_xx_1kck_16ck_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_CKSEL_A::EXTCRES_8MHZ_XX_1KCK_16CK_16MS)
    }
}
#[doc = "Field `CKOUT` reader - Clock output on PORTC2"]
pub type CKOUT_R = crate::BitReader;
#[doc = "Field `CKOUT` writer - Clock output on PORTC2"]
pub type CKOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKDIV8` reader - Divide clock by 8 internally"]
pub type CKDIV8_R = crate::BitReader;
#[doc = "Field `CKDIV8` writer - Divide clock by 8 internally"]
pub type CKDIV8_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Select Clock Source"]
    #[inline(always)]
    pub fn sut_cksel(&self) -> SUT_CKSEL_R {
        SUT_CKSEL_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 6 - Clock output on PORTC2"]
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
    #[doc = "Bits 0:4 - Select Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn sut_cksel(&mut self) -> SUT_CKSEL_W<LOW_SPEC> {
        SUT_CKSEL_W::new(self, 0)
    }
    #[doc = "Bit 6 - Clock output on PORTC2"]
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
