#[doc = "Register `EXTENDED` reader"]
pub type R = crate::R<EXTENDED_SPEC>;
#[doc = "Register `EXTENDED` writer"]
pub type W = crate::W<EXTENDED_SPEC>;
#[doc = "Field `SELFPRGEN` reader - Self Programming enable"]
pub type SELFPRGEN_R = crate::BitReader;
#[doc = "Field `SELFPRGEN` writer - Self Programming enable"]
pub type SELFPRGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODACT` reader - BOD mode of operation when the device is active or idle"]
pub type BODACT_R = crate::FieldReader<BODACT_A>;
#[doc = "BOD mode of operation when the device is active or idle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BODACT_A {
    #[doc = "1: Sampled"]
    BOD_SAMPLED = 1,
    #[doc = "2: Enabled"]
    BOD_ENABLED = 2,
    #[doc = "3: Disabled"]
    BOD_DISABLED = 3,
}
impl From<BODACT_A> for u8 {
    #[inline(always)]
    fn from(variant: BODACT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BODACT_A {
    type Ux = u8;
}
impl BODACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BODACT_A> {
        match self.bits {
            1 => Some(BODACT_A::BOD_SAMPLED),
            2 => Some(BODACT_A::BOD_ENABLED),
            3 => Some(BODACT_A::BOD_DISABLED),
            _ => None,
        }
    }
    #[doc = "Sampled"]
    #[inline(always)]
    pub fn is_bod_sampled(&self) -> bool {
        *self == BODACT_A::BOD_SAMPLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_bod_enabled(&self) -> bool {
        *self == BODACT_A::BOD_ENABLED
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_bod_disabled(&self) -> bool {
        *self == BODACT_A::BOD_DISABLED
    }
}
#[doc = "Field `BODACT` writer - BOD mode of operation when the device is active or idle"]
pub type BODACT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BODACT_A>;
impl<'a, REG> BODACT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sampled"]
    #[inline(always)]
    pub fn bod_sampled(self) -> &'a mut crate::W<REG> {
        self.variant(BODACT_A::BOD_SAMPLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn bod_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BODACT_A::BOD_ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn bod_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BODACT_A::BOD_DISABLED)
    }
}
#[doc = "Field `BODPD` reader - BOD mode of operation when the device is in sleep mode"]
pub type BODPD_R = crate::FieldReader<BODPD_A>;
#[doc = "BOD mode of operation when the device is in sleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BODPD_A {
    #[doc = "1: Sampled"]
    BOD_SAMPLED = 1,
    #[doc = "2: Enabled"]
    BOD_ENABLED = 2,
    #[doc = "3: Disabled"]
    BOD_DISABLED = 3,
}
impl From<BODPD_A> for u8 {
    #[inline(always)]
    fn from(variant: BODPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BODPD_A {
    type Ux = u8;
}
impl BODPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BODPD_A> {
        match self.bits {
            1 => Some(BODPD_A::BOD_SAMPLED),
            2 => Some(BODPD_A::BOD_ENABLED),
            3 => Some(BODPD_A::BOD_DISABLED),
            _ => None,
        }
    }
    #[doc = "Sampled"]
    #[inline(always)]
    pub fn is_bod_sampled(&self) -> bool {
        *self == BODPD_A::BOD_SAMPLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_bod_enabled(&self) -> bool {
        *self == BODPD_A::BOD_ENABLED
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_bod_disabled(&self) -> bool {
        *self == BODPD_A::BOD_DISABLED
    }
}
#[doc = "Field `BODPD` writer - BOD mode of operation when the device is in sleep mode"]
pub type BODPD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BODPD_A>;
impl<'a, REG> BODPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sampled"]
    #[inline(always)]
    pub fn bod_sampled(self) -> &'a mut crate::W<REG> {
        self.variant(BODPD_A::BOD_SAMPLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn bod_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BODPD_A::BOD_ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn bod_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BODPD_A::BOD_DISABLED)
    }
}
#[doc = "Field `ULPOSCSEL` reader - Frequency selection for internal ULP oscillator. The selection only affects system clock, watchdog and reset timeout always use 32 kHz clock."]
pub type ULPOSCSEL_R = crate::FieldReader<ULPOSCSEL_A>;
#[doc = "Frequency selection for internal ULP oscillator. The selection only affects system clock, watchdog and reset timeout always use 32 kHz clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ULPOSCSEL_A {
    #[doc = "3: 512 kHz"]
    ULPOSC_512KHZ = 3,
    #[doc = "4: 256 kHz"]
    ULPOSC_256KHZ = 4,
    #[doc = "5: 128 kHz"]
    ULPOSC_128KHZ = 5,
    #[doc = "6: 64 kHz"]
    ULPOSC_64KHZ = 6,
    #[doc = "7: 32 kHz"]
    ULPOSC_32KHZ = 7,
}
impl From<ULPOSCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ULPOSCSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ULPOSCSEL_A {
    type Ux = u8;
}
impl ULPOSCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ULPOSCSEL_A> {
        match self.bits {
            3 => Some(ULPOSCSEL_A::ULPOSC_512KHZ),
            4 => Some(ULPOSCSEL_A::ULPOSC_256KHZ),
            5 => Some(ULPOSCSEL_A::ULPOSC_128KHZ),
            6 => Some(ULPOSCSEL_A::ULPOSC_64KHZ),
            7 => Some(ULPOSCSEL_A::ULPOSC_32KHZ),
            _ => None,
        }
    }
    #[doc = "512 kHz"]
    #[inline(always)]
    pub fn is_ulposc_512khz(&self) -> bool {
        *self == ULPOSCSEL_A::ULPOSC_512KHZ
    }
    #[doc = "256 kHz"]
    #[inline(always)]
    pub fn is_ulposc_256khz(&self) -> bool {
        *self == ULPOSCSEL_A::ULPOSC_256KHZ
    }
    #[doc = "128 kHz"]
    #[inline(always)]
    pub fn is_ulposc_128khz(&self) -> bool {
        *self == ULPOSCSEL_A::ULPOSC_128KHZ
    }
    #[doc = "64 kHz"]
    #[inline(always)]
    pub fn is_ulposc_64khz(&self) -> bool {
        *self == ULPOSCSEL_A::ULPOSC_64KHZ
    }
    #[doc = "32 kHz"]
    #[inline(always)]
    pub fn is_ulposc_32khz(&self) -> bool {
        *self == ULPOSCSEL_A::ULPOSC_32KHZ
    }
}
#[doc = "Field `ULPOSCSEL` writer - Frequency selection for internal ULP oscillator. The selection only affects system clock, watchdog and reset timeout always use 32 kHz clock."]
pub type ULPOSCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ULPOSCSEL_A>;
impl<'a, REG> ULPOSCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "512 kHz"]
    #[inline(always)]
    pub fn ulposc_512khz(self) -> &'a mut crate::W<REG> {
        self.variant(ULPOSCSEL_A::ULPOSC_512KHZ)
    }
    #[doc = "256 kHz"]
    #[inline(always)]
    pub fn ulposc_256khz(self) -> &'a mut crate::W<REG> {
        self.variant(ULPOSCSEL_A::ULPOSC_256KHZ)
    }
    #[doc = "128 kHz"]
    #[inline(always)]
    pub fn ulposc_128khz(self) -> &'a mut crate::W<REG> {
        self.variant(ULPOSCSEL_A::ULPOSC_128KHZ)
    }
    #[doc = "64 kHz"]
    #[inline(always)]
    pub fn ulposc_64khz(self) -> &'a mut crate::W<REG> {
        self.variant(ULPOSCSEL_A::ULPOSC_64KHZ)
    }
    #[doc = "32 kHz"]
    #[inline(always)]
    pub fn ulposc_32khz(self) -> &'a mut crate::W<REG> {
        self.variant(ULPOSCSEL_A::ULPOSC_32KHZ)
    }
}
impl R {
    #[doc = "Bit 0 - Self Programming enable"]
    #[inline(always)]
    pub fn selfprgen(&self) -> SELFPRGEN_R {
        SELFPRGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - BOD mode of operation when the device is active or idle"]
    #[inline(always)]
    pub fn bodact(&self) -> BODACT_R {
        BODACT_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bits 3:4 - BOD mode of operation when the device is in sleep mode"]
    #[inline(always)]
    pub fn bodpd(&self) -> BODPD_R {
        BODPD_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bits 5:7 - Frequency selection for internal ULP oscillator. The selection only affects system clock, watchdog and reset timeout always use 32 kHz clock."]
    #[inline(always)]
    pub fn ulposcsel(&self) -> ULPOSCSEL_R {
        ULPOSCSEL_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - Self Programming enable"]
    #[inline(always)]
    #[must_use]
    pub fn selfprgen(&mut self) -> SELFPRGEN_W<EXTENDED_SPEC> {
        SELFPRGEN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - BOD mode of operation when the device is active or idle"]
    #[inline(always)]
    #[must_use]
    pub fn bodact(&mut self) -> BODACT_W<EXTENDED_SPEC> {
        BODACT_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - BOD mode of operation when the device is in sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn bodpd(&mut self) -> BODPD_W<EXTENDED_SPEC> {
        BODPD_W::new(self, 3)
    }
    #[doc = "Bits 5:7 - Frequency selection for internal ULP oscillator. The selection only affects system clock, watchdog and reset timeout always use 32 kHz clock."]
    #[inline(always)]
    #[must_use]
    pub fn ulposcsel(&mut self) -> ULPOSCSEL_W<EXTENDED_SPEC> {
        ULPOSCSEL_W::new(self, 5)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extended::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extended::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTENDED_SPEC;
impl crate::RegisterSpec for EXTENDED_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`extended::R`](R) reader structure"]
impl crate::Readable for EXTENDED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extended::W`](W) writer structure"]
impl crate::Writable for EXTENDED_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTENDED to value 0"]
impl crate::Resettable for EXTENDED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
