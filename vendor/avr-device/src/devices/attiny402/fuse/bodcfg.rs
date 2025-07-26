#[doc = "Register `BODCFG` reader"]
pub type R = crate::R<BODCFG_SPEC>;
#[doc = "Field `SLEEP` reader - BOD Operation in Sleep Mode"]
pub type SLEEP_R = crate::FieldReader<SLEEP_A>;
#[doc = "BOD Operation in Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SLEEP_A {
    #[doc = "0: Disabled"]
    DIS = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "2: Sampled"]
    SAMPLED = 2,
}
impl From<SLEEP_A> for u8 {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SLEEP_A {
    type Ux = u8;
}
impl SLEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SLEEP_A> {
        match self.bits {
            0 => Some(SLEEP_A::DIS),
            1 => Some(SLEEP_A::ENABLED),
            2 => Some(SLEEP_A::SAMPLED),
            _ => None,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SLEEP_A::DIS
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLEEP_A::ENABLED
    }
    #[doc = "Sampled"]
    #[inline(always)]
    pub fn is_sampled(&self) -> bool {
        *self == SLEEP_A::SAMPLED
    }
}
#[doc = "Field `ACTIVE` reader - BOD Operation in Active Mode"]
pub type ACTIVE_R = crate::FieldReader<ACTIVE_A>;
#[doc = "BOD Operation in Active Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACTIVE_A {
    #[doc = "0: Disabled"]
    DIS = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "2: Sampled"]
    SAMPLED = 2,
    #[doc = "3: Enabled with wake-up halted until BOD is ready"]
    ENWAKE = 3,
}
impl From<ACTIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTIVE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACTIVE_A {
    type Ux = u8;
}
impl ACTIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACTIVE_A {
        match self.bits {
            0 => ACTIVE_A::DIS,
            1 => ACTIVE_A::ENABLED,
            2 => ACTIVE_A::SAMPLED,
            3 => ACTIVE_A::ENWAKE,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ACTIVE_A::DIS
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACTIVE_A::ENABLED
    }
    #[doc = "Sampled"]
    #[inline(always)]
    pub fn is_sampled(&self) -> bool {
        *self == ACTIVE_A::SAMPLED
    }
    #[doc = "Enabled with wake-up halted until BOD is ready"]
    #[inline(always)]
    pub fn is_enwake(&self) -> bool {
        *self == ACTIVE_A::ENWAKE
    }
}
#[doc = "Field `SAMPFREQ` reader - BOD Sample Frequency"]
pub type SAMPFREQ_R = crate::BitReader<SAMPFREQ_A>;
#[doc = "BOD Sample Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAMPFREQ_A {
    #[doc = "0: 1kHz sampling frequency"]
    _1KHZ = 0,
    #[doc = "1: 125Hz sampling frequency"]
    _125HZ = 1,
}
impl From<SAMPFREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPFREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SAMPFREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SAMPFREQ_A {
        match self.bits {
            false => SAMPFREQ_A::_1KHZ,
            true => SAMPFREQ_A::_125HZ,
        }
    }
    #[doc = "1kHz sampling frequency"]
    #[inline(always)]
    pub fn is_1khz(&self) -> bool {
        *self == SAMPFREQ_A::_1KHZ
    }
    #[doc = "125Hz sampling frequency"]
    #[inline(always)]
    pub fn is_125hz(&self) -> bool {
        *self == SAMPFREQ_A::_125HZ
    }
}
#[doc = "Field `LVL` reader - BOD Level"]
pub type LVL_R = crate::FieldReader<LVL_A>;
#[doc = "BOD Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVL_A {
    #[doc = "0: 1.8 V"]
    BODLEVEL0 = 0,
    #[doc = "2: 2.6 V"]
    BODLEVEL2 = 2,
    #[doc = "7: 4.2 V"]
    BODLEVEL7 = 7,
}
impl From<LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LVL_A {
    type Ux = u8;
}
impl LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LVL_A> {
        match self.bits {
            0 => Some(LVL_A::BODLEVEL0),
            2 => Some(LVL_A::BODLEVEL2),
            7 => Some(LVL_A::BODLEVEL7),
            _ => None,
        }
    }
    #[doc = "1.8 V"]
    #[inline(always)]
    pub fn is_bodlevel0(&self) -> bool {
        *self == LVL_A::BODLEVEL0
    }
    #[doc = "2.6 V"]
    #[inline(always)]
    pub fn is_bodlevel2(&self) -> bool {
        *self == LVL_A::BODLEVEL2
    }
    #[doc = "4.2 V"]
    #[inline(always)]
    pub fn is_bodlevel7(&self) -> bool {
        *self == LVL_A::BODLEVEL7
    }
}
impl R {
    #[doc = "Bits 0:1 - BOD Operation in Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - BOD Operation in Active Mode"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - BOD Sample Frequency"]
    #[inline(always)]
    pub fn sampfreq(&self) -> SAMPFREQ_R {
        SAMPFREQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - BOD Level"]
    #[inline(always)]
    pub fn lvl(&self) -> LVL_R {
        LVL_R::new((self.bits >> 5) & 7)
    }
}
#[doc = "BOD Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bodcfg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BODCFG_SPEC;
impl crate::RegisterSpec for BODCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bodcfg::R`](R) reader structure"]
impl crate::Readable for BODCFG_SPEC {}
#[doc = "`reset()` method sets BODCFG to value 0"]
impl crate::Resettable for BODCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
