#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `SLEEP` reader - Operation in sleep mode"]
pub type SLEEP_R = crate::FieldReader<SLEEP_A>;
#[doc = "Operation in sleep mode\n\nValue on reset: 0"]
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
#[doc = "Field `SLEEP` writer - Operation in sleep mode"]
pub type SLEEP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SLEEP_A>;
impl<'a, REG> SLEEP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_A::DIS)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_A::ENABLED)
    }
    #[doc = "Sampled"]
    #[inline(always)]
    pub fn sampled(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_A::SAMPLED)
    }
}
#[doc = "Field `ACTIVE` reader - Operation in active mode"]
pub type ACTIVE_R = crate::FieldReader<ACTIVE_A>;
#[doc = "Operation in active mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACTIVE_A {
    #[doc = "0: Disabled"]
    DIS = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "2: Sampled"]
    SAMPLED = 2,
    #[doc = "3: Enabled with wakeup halt"]
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
    #[doc = "Enabled with wakeup halt"]
    #[inline(always)]
    pub fn is_enwake(&self) -> bool {
        *self == ACTIVE_A::ENWAKE
    }
}
#[doc = "Field `SAMPFREQ` reader - Sample frequency"]
pub type SAMPFREQ_R = crate::BitReader<SAMPFREQ_A>;
#[doc = "Sample frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAMPFREQ_A {
    #[doc = "0: 1kHz sampling"]
    _1KHZ = 0,
    #[doc = "1: 125Hz sampling"]
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
    #[doc = "1kHz sampling"]
    #[inline(always)]
    pub fn is_1khz(&self) -> bool {
        *self == SAMPFREQ_A::_1KHZ
    }
    #[doc = "125Hz sampling"]
    #[inline(always)]
    pub fn is_125hz(&self) -> bool {
        *self == SAMPFREQ_A::_125HZ
    }
}
impl R {
    #[doc = "Bits 0:1 - Operation in sleep mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Operation in active mode"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - Sample frequency"]
    #[inline(always)]
    pub fn sampfreq(&self) -> SAMPFREQ_R {
        SAMPFREQ_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operation in sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<CTRLA_SPEC> {
        SLEEP_W::new(self, 0)
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
#[doc = "Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
