#[doc = "Register `XOSC_CTRL` reader"]
pub type R = crate::R<XOSC_CTRL_SPEC>;
#[doc = "Register `XOSC_CTRL` writer"]
pub type W = crate::W<XOSC_CTRL_SPEC>;
#[doc = "Field `XTAL_TRIM` reader - Crystal Oscillator Load Capacitance Trimming"]
pub type XTAL_TRIM_R = crate::FieldReader<XTAL_TRIM_A>;
#[doc = "Crystal Oscillator Load Capacitance Trimming\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XTAL_TRIM_A {
    #[doc = "0: 0.0 pF, trimming capacitors disconnected"]
    XTAL_TRIM_MIN = 0,
    #[doc = "1: 0.3 pF, trimming capacitor switched on"]
    VAL_0X1 = 1,
    #[doc = "2: ..."]
    VAL_0X2 = 2,
    #[doc = "15: 4.5 pF, trimming capacitor switched on"]
    XTAL_TRIM_MAX = 15,
}
impl From<XTAL_TRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: XTAL_TRIM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for XTAL_TRIM_A {
    type Ux = u8;
}
impl XTAL_TRIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<XTAL_TRIM_A> {
        match self.bits {
            0 => Some(XTAL_TRIM_A::XTAL_TRIM_MIN),
            1 => Some(XTAL_TRIM_A::VAL_0X1),
            2 => Some(XTAL_TRIM_A::VAL_0X2),
            15 => Some(XTAL_TRIM_A::XTAL_TRIM_MAX),
            _ => None,
        }
    }
    #[doc = "0.0 pF, trimming capacitors disconnected"]
    #[inline(always)]
    pub fn is_xtal_trim_min(&self) -> bool {
        *self == XTAL_TRIM_A::XTAL_TRIM_MIN
    }
    #[doc = "0.3 pF, trimming capacitor switched on"]
    #[inline(always)]
    pub fn is_val_0x1(&self) -> bool {
        *self == XTAL_TRIM_A::VAL_0X1
    }
    #[doc = "..."]
    #[inline(always)]
    pub fn is_val_0x2(&self) -> bool {
        *self == XTAL_TRIM_A::VAL_0X2
    }
    #[doc = "4.5 pF, trimming capacitor switched on"]
    #[inline(always)]
    pub fn is_xtal_trim_max(&self) -> bool {
        *self == XTAL_TRIM_A::XTAL_TRIM_MAX
    }
}
#[doc = "Field `XTAL_TRIM` writer - Crystal Oscillator Load Capacitance Trimming"]
pub type XTAL_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4, XTAL_TRIM_A>;
impl<'a, REG> XTAL_TRIM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.0 pF, trimming capacitors disconnected"]
    #[inline(always)]
    pub fn xtal_trim_min(self) -> &'a mut crate::W<REG> {
        self.variant(XTAL_TRIM_A::XTAL_TRIM_MIN)
    }
    #[doc = "0.3 pF, trimming capacitor switched on"]
    #[inline(always)]
    pub fn val_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(XTAL_TRIM_A::VAL_0X1)
    }
    #[doc = "..."]
    #[inline(always)]
    pub fn val_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(XTAL_TRIM_A::VAL_0X2)
    }
    #[doc = "4.5 pF, trimming capacitor switched on"]
    #[inline(always)]
    pub fn xtal_trim_max(self) -> &'a mut crate::W<REG> {
        self.variant(XTAL_TRIM_A::XTAL_TRIM_MAX)
    }
}
#[doc = "Field `XTAL_MODE` reader - Crystal Oscillator Operating Mode"]
pub type XTAL_MODE_R = crate::FieldReader<XTAL_MODE_A>;
#[doc = "Crystal Oscillator Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XTAL_MODE_A {
    #[doc = "4: Internal crystal oscillator disabled; use external reference frequency."]
    INTERNAL_CRYSTAL_OSCILLATOR_DISABLED_USE_EXTERNAL_REFERENCE_FREQUENCY = 4,
    #[doc = "15: Internal crystal oscillator enabled; amplitude regulation of oscillation enabled."]
    INTERNAL_CRYSTAL_OSCILLATOR_ENABLED_AMPLITUDE_REGULATION_OF_OSCILLATION_ENABLED = 15,
}
impl From<XTAL_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: XTAL_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for XTAL_MODE_A {
    type Ux = u8;
}
impl XTAL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<XTAL_MODE_A> {
        match self . bits { 4 => Some (XTAL_MODE_A :: INTERNAL_CRYSTAL_OSCILLATOR_DISABLED_USE_EXTERNAL_REFERENCE_FREQUENCY) , 15 => Some (XTAL_MODE_A :: INTERNAL_CRYSTAL_OSCILLATOR_ENABLED_AMPLITUDE_REGULATION_OF_OSCILLATION_ENABLED) , _ => None , }
    }
    #[doc = "Internal crystal oscillator disabled; use external reference frequency."]
    #[inline(always)]
    pub fn is_internal_crystal_oscillator_disabled_use_external_reference_frequency(&self) -> bool {
        *self == XTAL_MODE_A::INTERNAL_CRYSTAL_OSCILLATOR_DISABLED_USE_EXTERNAL_REFERENCE_FREQUENCY
    }
    #[doc = "Internal crystal oscillator enabled; amplitude regulation of oscillation enabled."]
    #[inline(always)]
    pub fn is_internal_crystal_oscillator_enabled_amplitude_regulation_of_oscillation_enabled(
        &self,
    ) -> bool {
        * self == XTAL_MODE_A :: INTERNAL_CRYSTAL_OSCILLATOR_ENABLED_AMPLITUDE_REGULATION_OF_OSCILLATION_ENABLED
    }
}
#[doc = "Field `XTAL_MODE` writer - Crystal Oscillator Operating Mode"]
pub type XTAL_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, XTAL_MODE_A>;
impl<'a, REG> XTAL_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal crystal oscillator disabled; use external reference frequency."]
    #[inline(always)]
    pub fn internal_crystal_oscillator_disabled_use_external_reference_frequency(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(
            XTAL_MODE_A::INTERNAL_CRYSTAL_OSCILLATOR_DISABLED_USE_EXTERNAL_REFERENCE_FREQUENCY,
        )
    }
    #[doc = "Internal crystal oscillator enabled; amplitude regulation of oscillation enabled."]
    #[inline(always)]
    pub fn internal_crystal_oscillator_enabled_amplitude_regulation_of_oscillation_enabled(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (XTAL_MODE_A :: INTERNAL_CRYSTAL_OSCILLATOR_ENABLED_AMPLITUDE_REGULATION_OF_OSCILLATION_ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:3 - Crystal Oscillator Load Capacitance Trimming"]
    #[inline(always)]
    pub fn xtal_trim(&self) -> XTAL_TRIM_R {
        XTAL_TRIM_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Crystal Oscillator Operating Mode"]
    #[inline(always)]
    pub fn xtal_mode(&self) -> XTAL_MODE_R {
        XTAL_MODE_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Crystal Oscillator Load Capacitance Trimming"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_trim(&mut self) -> XTAL_TRIM_W<XOSC_CTRL_SPEC> {
        XTAL_TRIM_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Crystal Oscillator Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_mode(&mut self) -> XTAL_MODE_W<XOSC_CTRL_SPEC> {
        XTAL_MODE_W::new(self, 4)
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
#[doc = "Crystal Oscillator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xosc_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xosc_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XOSC_CTRL_SPEC;
impl crate::RegisterSpec for XOSC_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xosc_ctrl::R`](R) reader structure"]
impl crate::Readable for XOSC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xosc_ctrl::W`](W) writer structure"]
impl crate::Writable for XOSC_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XOSC_CTRL to value 0"]
impl crate::Resettable for XOSC_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
