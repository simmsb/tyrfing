#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYSMODE` reader - Hysteresis Mode"]
pub type HYSMODE_R = crate::FieldReader<HYSMODE_A>;
#[doc = "Hysteresis Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HYSMODE_A {
    #[doc = "0: No hysteresis"]
    NONE = 0,
    #[doc = "1: Small hysteresis"]
    SMALL = 1,
    #[doc = "2: Medium hysteresis"]
    MEDIUM = 2,
    #[doc = "3: Large hysteresis"]
    LARGE = 3,
}
impl From<HYSMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: HYSMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HYSMODE_A {
    type Ux = u8;
}
impl HYSMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HYSMODE_A {
        match self.bits {
            0 => HYSMODE_A::NONE,
            1 => HYSMODE_A::SMALL,
            2 => HYSMODE_A::MEDIUM,
            3 => HYSMODE_A::LARGE,
            _ => unreachable!(),
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == HYSMODE_A::NONE
    }
    #[doc = "Small hysteresis"]
    #[inline(always)]
    pub fn is_small(&self) -> bool {
        *self == HYSMODE_A::SMALL
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == HYSMODE_A::MEDIUM
    }
    #[doc = "Large hysteresis"]
    #[inline(always)]
    pub fn is_large(&self) -> bool {
        *self == HYSMODE_A::LARGE
    }
}
#[doc = "Field `HYSMODE` writer - Hysteresis Mode"]
pub type HYSMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, HYSMODE_A>;
impl<'a, REG> HYSMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(HYSMODE_A::NONE)
    }
    #[doc = "Small hysteresis"]
    #[inline(always)]
    pub fn small(self) -> &'a mut crate::W<REG> {
        self.variant(HYSMODE_A::SMALL)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(HYSMODE_A::MEDIUM)
    }
    #[doc = "Large hysteresis"]
    #[inline(always)]
    pub fn large(self) -> &'a mut crate::W<REG> {
        self.variant(HYSMODE_A::LARGE)
    }
}
#[doc = "Field `POWER` reader - Power profile"]
pub type POWER_R = crate::FieldReader<POWER_A>;
#[doc = "Power profile\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POWER_A {
    #[doc = "0: Power profile 0, Shortest response time, highest consumption"]
    PROFILE0 = 0,
    #[doc = "1: Power profile 1"]
    PROFILE1 = 1,
    #[doc = "2: Power profile 2"]
    PROFILE2 = 2,
    #[doc = "3: Power profile 3"]
    PROFILE3 = 3,
}
impl From<POWER_A> for u8 {
    #[inline(always)]
    fn from(variant: POWER_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for POWER_A {
    type Ux = u8;
}
impl POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POWER_A {
        match self.bits {
            0 => POWER_A::PROFILE0,
            1 => POWER_A::PROFILE1,
            2 => POWER_A::PROFILE2,
            3 => POWER_A::PROFILE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Power profile 0, Shortest response time, highest consumption"]
    #[inline(always)]
    pub fn is_profile0(&self) -> bool {
        *self == POWER_A::PROFILE0
    }
    #[doc = "Power profile 1"]
    #[inline(always)]
    pub fn is_profile1(&self) -> bool {
        *self == POWER_A::PROFILE1
    }
    #[doc = "Power profile 2"]
    #[inline(always)]
    pub fn is_profile2(&self) -> bool {
        *self == POWER_A::PROFILE2
    }
    #[doc = "Power profile 3"]
    #[inline(always)]
    pub fn is_profile3(&self) -> bool {
        *self == POWER_A::PROFILE3
    }
}
#[doc = "Field `POWER` writer - Power profile"]
pub type POWER_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, POWER_A>;
impl<'a, REG> POWER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Power profile 0, Shortest response time, highest consumption"]
    #[inline(always)]
    pub fn profile0(self) -> &'a mut crate::W<REG> {
        self.variant(POWER_A::PROFILE0)
    }
    #[doc = "Power profile 1"]
    #[inline(always)]
    pub fn profile1(self) -> &'a mut crate::W<REG> {
        self.variant(POWER_A::PROFILE1)
    }
    #[doc = "Power profile 2"]
    #[inline(always)]
    pub fn profile2(self) -> &'a mut crate::W<REG> {
        self.variant(POWER_A::PROFILE2)
    }
    #[doc = "Power profile 3"]
    #[inline(always)]
    pub fn profile3(self) -> &'a mut crate::W<REG> {
        self.variant(POWER_A::PROFILE3)
    }
}
#[doc = "Field `OUTEN` reader - Output Pad Enable"]
pub type OUTEN_R = crate::BitReader;
#[doc = "Field `OUTEN` writer - Output Pad Enable"]
pub type OUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby Mode"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby Mode"]
pub type RUNSTDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Hysteresis Mode"]
    #[inline(always)]
    pub fn hysmode(&self) -> HYSMODE_R {
        HYSMODE_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bits 3:4 - Power profile"]
    #[inline(always)]
    pub fn power(&self) -> POWER_R {
        POWER_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 6 - Output Pad Enable"]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Run in Standby Mode"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRLA_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Hysteresis Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hysmode(&mut self) -> HYSMODE_W<CTRLA_SPEC> {
        HYSMODE_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - Power profile"]
    #[inline(always)]
    #[must_use]
    pub fn power(&mut self) -> POWER_W<CTRLA_SPEC> {
        POWER_W::new(self, 3)
    }
    #[doc = "Bit 6 - Output Pad Enable"]
    #[inline(always)]
    #[must_use]
    pub fn outen(&mut self) -> OUTEN_W<CTRLA_SPEC> {
        OUTEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Run in Standby Mode"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<CTRLA_SPEC> {
        RUNSTDBY_W::new(self, 7)
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
