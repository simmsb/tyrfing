#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `ENABLE` reader - ADC Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - ADC Enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREERUN` reader - Free running mode"]
pub type FREERUN_R = crate::BitReader;
#[doc = "Field `FREERUN` writer - Free running mode"]
pub type FREERUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESSEL` reader - Resolution selection"]
pub type RESSEL_R = crate::FieldReader<RESSEL_A>;
#[doc = "Resolution selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESSEL_A {
    #[doc = "0: 12-bit mode"]
    _12BIT = 0,
    #[doc = "1: 10-bit mode"]
    _10BIT = 1,
}
impl From<RESSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RESSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RESSEL_A {
    type Ux = u8;
}
impl RESSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RESSEL_A> {
        match self.bits {
            0 => Some(RESSEL_A::_12BIT),
            1 => Some(RESSEL_A::_10BIT),
            _ => None,
        }
    }
    #[doc = "12-bit mode"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == RESSEL_A::_12BIT
    }
    #[doc = "10-bit mode"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == RESSEL_A::_10BIT
    }
}
#[doc = "Field `RESSEL` writer - Resolution selection"]
pub type RESSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RESSEL_A>;
impl<'a, REG> RESSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit mode"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(RESSEL_A::_12BIT)
    }
    #[doc = "10-bit mode"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(RESSEL_A::_10BIT)
    }
}
#[doc = "Field `LEFTADJ` reader - Left adjust result"]
pub type LEFTADJ_R = crate::BitReader;
#[doc = "Field `LEFTADJ` writer - Left adjust result"]
pub type LEFTADJ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONVMODE` reader - Conversion mode"]
pub type CONVMODE_R = crate::BitReader<CONVMODE_A>;
#[doc = "Conversion mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONVMODE_A {
    #[doc = "0: Single-Ended mode"]
    SINGLEENDED = 0,
    #[doc = "1: Differential mode"]
    DIFF = 1,
}
impl From<CONVMODE_A> for bool {
    #[inline(always)]
    fn from(variant: CONVMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl CONVMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CONVMODE_A {
        match self.bits {
            false => CONVMODE_A::SINGLEENDED,
            true => CONVMODE_A::DIFF,
        }
    }
    #[doc = "Single-Ended mode"]
    #[inline(always)]
    pub fn is_singleended(&self) -> bool {
        *self == CONVMODE_A::SINGLEENDED
    }
    #[doc = "Differential mode"]
    #[inline(always)]
    pub fn is_diff(&self) -> bool {
        *self == CONVMODE_A::DIFF
    }
}
#[doc = "Field `CONVMODE` writer - Conversion mode"]
pub type CONVMODE_W<'a, REG> = crate::BitWriter<'a, REG, CONVMODE_A>;
impl<'a, REG> CONVMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single-Ended mode"]
    #[inline(always)]
    pub fn singleended(self) -> &'a mut crate::W<REG> {
        self.variant(CONVMODE_A::SINGLEENDED)
    }
    #[doc = "Differential mode"]
    #[inline(always)]
    pub fn diff(self) -> &'a mut crate::W<REG> {
        self.variant(CONVMODE_A::DIFF)
    }
}
#[doc = "Field `RUNSTBY` reader - Run standby mode"]
pub type RUNSTBY_R = crate::BitReader;
#[doc = "Field `RUNSTBY` writer - Run standby mode"]
pub type RUNSTBY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Free running mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FREERUN_R {
        FREERUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Resolution selection"]
    #[inline(always)]
    pub fn ressel(&self) -> RESSEL_R {
        RESSEL_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - Left adjust result"]
    #[inline(always)]
    pub fn leftadj(&self) -> LEFTADJ_R {
        LEFTADJ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Conversion mode"]
    #[inline(always)]
    pub fn convmode(&self) -> CONVMODE_R {
        CONVMODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Run standby mode"]
    #[inline(always)]
    pub fn runstby(&self) -> RUNSTBY_R {
        RUNSTBY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRLA_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Free running mode"]
    #[inline(always)]
    #[must_use]
    pub fn freerun(&mut self) -> FREERUN_W<CTRLA_SPEC> {
        FREERUN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Resolution selection"]
    #[inline(always)]
    #[must_use]
    pub fn ressel(&mut self) -> RESSEL_W<CTRLA_SPEC> {
        RESSEL_W::new(self, 2)
    }
    #[doc = "Bit 4 - Left adjust result"]
    #[inline(always)]
    #[must_use]
    pub fn leftadj(&mut self) -> LEFTADJ_W<CTRLA_SPEC> {
        LEFTADJ_W::new(self, 4)
    }
    #[doc = "Bit 5 - Conversion mode"]
    #[inline(always)]
    #[must_use]
    pub fn convmode(&mut self) -> CONVMODE_W<CTRLA_SPEC> {
        CONVMODE_W::new(self, 5)
    }
    #[doc = "Bit 7 - Run standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn runstby(&mut self) -> RUNSTBY_W<CTRLA_SPEC> {
        RUNSTBY_W::new(self, 7)
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
