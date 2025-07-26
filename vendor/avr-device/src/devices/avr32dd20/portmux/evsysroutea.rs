#[doc = "Register `EVSYSROUTEA` reader"]
pub type R = crate::R<EVSYSROUTEA_SPEC>;
#[doc = "Register `EVSYSROUTEA` writer"]
pub type W = crate::W<EVSYSROUTEA_SPEC>;
#[doc = "Field `EVOUTA` reader - Event Output A"]
pub type EVOUTA_R = crate::BitReader<EVOUTA_A>;
#[doc = "Event Output A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVOUTA_A {
    #[doc = "0: EVOUTA: PA2"]
    DEFAULT = 0,
    #[doc = "1: EVOUTA: PA7"]
    ALT1 = 1,
}
impl From<EVOUTA_A> for bool {
    #[inline(always)]
    fn from(variant: EVOUTA_A) -> Self {
        variant as u8 != 0
    }
}
impl EVOUTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EVOUTA_A {
        match self.bits {
            false => EVOUTA_A::DEFAULT,
            true => EVOUTA_A::ALT1,
        }
    }
    #[doc = "EVOUTA: PA2"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == EVOUTA_A::DEFAULT
    }
    #[doc = "EVOUTA: PA7"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == EVOUTA_A::ALT1
    }
}
#[doc = "Field `EVOUTA` writer - Event Output A"]
pub type EVOUTA_W<'a, REG> = crate::BitWriter<'a, REG, EVOUTA_A>;
impl<'a, REG> EVOUTA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EVOUTA: PA2"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(EVOUTA_A::DEFAULT)
    }
    #[doc = "EVOUTA: PA7"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut crate::W<REG> {
        self.variant(EVOUTA_A::ALT1)
    }
}
#[doc = "Field `EVOUTC` reader - Event Output C"]
pub type EVOUTC_R = crate::BitReader<EVOUTC_A>;
#[doc = "Event Output C\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVOUTC_A {
    #[doc = "0: EVOUTC: PC2"]
    DEFAULT = 0,
}
impl From<EVOUTC_A> for bool {
    #[inline(always)]
    fn from(variant: EVOUTC_A) -> Self {
        variant as u8 != 0
    }
}
impl EVOUTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EVOUTC_A> {
        match self.bits {
            false => Some(EVOUTC_A::DEFAULT),
            _ => None,
        }
    }
    #[doc = "EVOUTC: PC2"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == EVOUTC_A::DEFAULT
    }
}
#[doc = "Field `EVOUTC` writer - Event Output C"]
pub type EVOUTC_W<'a, REG> = crate::BitWriter<'a, REG, EVOUTC_A>;
impl<'a, REG> EVOUTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EVOUTC: PC2"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(EVOUTC_A::DEFAULT)
    }
}
#[doc = "Field `EVOUTD` reader - Event Output D"]
pub type EVOUTD_R = crate::BitReader<EVOUTD_A>;
#[doc = "Event Output D\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVOUTD_A {
    #[doc = "0: Not connected to any pins"]
    DEFAULT = 0,
    #[doc = "1: EVOUTD: PD7"]
    ALT1 = 1,
}
impl From<EVOUTD_A> for bool {
    #[inline(always)]
    fn from(variant: EVOUTD_A) -> Self {
        variant as u8 != 0
    }
}
impl EVOUTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EVOUTD_A {
        match self.bits {
            false => EVOUTD_A::DEFAULT,
            true => EVOUTD_A::ALT1,
        }
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == EVOUTD_A::DEFAULT
    }
    #[doc = "EVOUTD: PD7"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == EVOUTD_A::ALT1
    }
}
#[doc = "Field `EVOUTD` writer - Event Output D"]
pub type EVOUTD_W<'a, REG> = crate::BitWriter<'a, REG, EVOUTD_A>;
impl<'a, REG> EVOUTD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(EVOUTD_A::DEFAULT)
    }
    #[doc = "EVOUTD: PD7"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut crate::W<REG> {
        self.variant(EVOUTD_A::ALT1)
    }
}
impl R {
    #[doc = "Bit 0 - Event Output A"]
    #[inline(always)]
    pub fn evouta(&self) -> EVOUTA_R {
        EVOUTA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Event Output C"]
    #[inline(always)]
    pub fn evoutc(&self) -> EVOUTC_R {
        EVOUTC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event Output D"]
    #[inline(always)]
    pub fn evoutd(&self) -> EVOUTD_R {
        EVOUTD_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Output A"]
    #[inline(always)]
    #[must_use]
    pub fn evouta(&mut self) -> EVOUTA_W<EVSYSROUTEA_SPEC> {
        EVOUTA_W::new(self, 0)
    }
    #[doc = "Bit 2 - Event Output C"]
    #[inline(always)]
    #[must_use]
    pub fn evoutc(&mut self) -> EVOUTC_W<EVSYSROUTEA_SPEC> {
        EVOUTC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Event Output D"]
    #[inline(always)]
    #[must_use]
    pub fn evoutd(&mut self) -> EVOUTD_W<EVSYSROUTEA_SPEC> {
        EVOUTD_W::new(self, 3)
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
#[doc = "EVSYS route A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evsysroutea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evsysroutea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVSYSROUTEA_SPEC;
impl crate::RegisterSpec for EVSYSROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`evsysroutea::R`](R) reader structure"]
impl crate::Readable for EVSYSROUTEA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evsysroutea::W`](W) writer structure"]
impl crate::Writable for EVSYSROUTEA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVSYSROUTEA to value 0"]
impl crate::Resettable for EVSYSROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
