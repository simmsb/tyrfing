#[doc = "Register `OSCCFG` reader"]
pub type R = crate::R<OSCCFG_SPEC>;
#[doc = "Field `FREQSEL` reader - Frequency Select"]
pub type FREQSEL_R = crate::FieldReader<FREQSEL_A>;
#[doc = "Frequency Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FREQSEL_A {
    #[doc = "1: 16 MHz"]
    _16MHZ = 1,
    #[doc = "2: 20 MHz"]
    _20MHZ = 2,
}
impl From<FREQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FREQSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FREQSEL_A {
    type Ux = u8;
}
impl FREQSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FREQSEL_A> {
        match self.bits {
            1 => Some(FREQSEL_A::_16MHZ),
            2 => Some(FREQSEL_A::_20MHZ),
            _ => None,
        }
    }
    #[doc = "16 MHz"]
    #[inline(always)]
    pub fn is_16mhz(&self) -> bool {
        *self == FREQSEL_A::_16MHZ
    }
    #[doc = "20 MHz"]
    #[inline(always)]
    pub fn is_20mhz(&self) -> bool {
        *self == FREQSEL_A::_20MHZ
    }
}
#[doc = "Field `OSCLOCK` reader - Oscillator Lock"]
pub type OSCLOCK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Frequency Select"]
    #[inline(always)]
    pub fn freqsel(&self) -> FREQSEL_R {
        FREQSEL_R::new(self.bits & 3)
    }
    #[doc = "Bit 7 - Oscillator Lock"]
    #[inline(always)]
    pub fn osclock(&self) -> OSCLOCK_R {
        OSCLOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Oscillator Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccfg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCCFG_SPEC;
impl crate::RegisterSpec for OSCCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`osccfg::R`](R) reader structure"]
impl crate::Readable for OSCCFG_SPEC {}
#[doc = "`reset()` method sets OSCCFG to value 0"]
impl crate::Resettable for OSCCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
