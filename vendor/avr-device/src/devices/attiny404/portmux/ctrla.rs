#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `EVOUT0` reader - Event Output 0"]
pub type EVOUT0_R = crate::BitReader;
#[doc = "Field `EVOUT0` writer - Event Output 0"]
pub type EVOUT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVOUT1` reader - Event Output 1"]
pub type EVOUT1_R = crate::BitReader;
#[doc = "Field `EVOUT1` writer - Event Output 1"]
pub type EVOUT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVOUT2` reader - Event Output 2"]
pub type EVOUT2_R = crate::BitReader;
#[doc = "Field `EVOUT2` writer - Event Output 2"]
pub type EVOUT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LUT0` reader - Configurable Custom Logic LUT0"]
pub type LUT0_R = crate::BitReader<LUT0_A>;
#[doc = "Configurable Custom Logic LUT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LUT0_A {
    #[doc = "0: Default pin"]
    DEFAULT = 0,
    #[doc = "1: Alternate pin"]
    ALTERNATE = 1,
}
impl From<LUT0_A> for bool {
    #[inline(always)]
    fn from(variant: LUT0_A) -> Self {
        variant as u8 != 0
    }
}
impl LUT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LUT0_A {
        match self.bits {
            false => LUT0_A::DEFAULT,
            true => LUT0_A::ALTERNATE,
        }
    }
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == LUT0_A::DEFAULT
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == LUT0_A::ALTERNATE
    }
}
#[doc = "Field `LUT0` writer - Configurable Custom Logic LUT0"]
pub type LUT0_W<'a, REG> = crate::BitWriter<'a, REG, LUT0_A>;
impl<'a, REG> LUT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(LUT0_A::DEFAULT)
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(LUT0_A::ALTERNATE)
    }
}
#[doc = "Field `LUT1` reader - Configurable Custom Logic LUT1"]
pub type LUT1_R = crate::BitReader<LUT1_A>;
#[doc = "Configurable Custom Logic LUT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LUT1_A {
    #[doc = "0: Default pin"]
    DEFAULT = 0,
    #[doc = "1: Alternate pin"]
    ALTERNATE = 1,
}
impl From<LUT1_A> for bool {
    #[inline(always)]
    fn from(variant: LUT1_A) -> Self {
        variant as u8 != 0
    }
}
impl LUT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LUT1_A {
        match self.bits {
            false => LUT1_A::DEFAULT,
            true => LUT1_A::ALTERNATE,
        }
    }
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == LUT1_A::DEFAULT
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == LUT1_A::ALTERNATE
    }
}
#[doc = "Field `LUT1` writer - Configurable Custom Logic LUT1"]
pub type LUT1_W<'a, REG> = crate::BitWriter<'a, REG, LUT1_A>;
impl<'a, REG> LUT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(LUT1_A::DEFAULT)
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(LUT1_A::ALTERNATE)
    }
}
impl R {
    #[doc = "Bit 0 - Event Output 0"]
    #[inline(always)]
    pub fn evout0(&self) -> EVOUT0_R {
        EVOUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event Output 1"]
    #[inline(always)]
    pub fn evout1(&self) -> EVOUT1_R {
        EVOUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event Output 2"]
    #[inline(always)]
    pub fn evout2(&self) -> EVOUT2_R {
        EVOUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Configurable Custom Logic LUT0"]
    #[inline(always)]
    pub fn lut0(&self) -> LUT0_R {
        LUT0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configurable Custom Logic LUT1"]
    #[inline(always)]
    pub fn lut1(&self) -> LUT1_R {
        LUT1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Output 0"]
    #[inline(always)]
    #[must_use]
    pub fn evout0(&mut self) -> EVOUT0_W<CTRLA_SPEC> {
        EVOUT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Event Output 1"]
    #[inline(always)]
    #[must_use]
    pub fn evout1(&mut self) -> EVOUT1_W<CTRLA_SPEC> {
        EVOUT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Event Output 2"]
    #[inline(always)]
    #[must_use]
    pub fn evout2(&mut self) -> EVOUT2_W<CTRLA_SPEC> {
        EVOUT2_W::new(self, 2)
    }
    #[doc = "Bit 4 - Configurable Custom Logic LUT0"]
    #[inline(always)]
    #[must_use]
    pub fn lut0(&mut self) -> LUT0_W<CTRLA_SPEC> {
        LUT0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configurable Custom Logic LUT1"]
    #[inline(always)]
    #[must_use]
    pub fn lut1(&mut self) -> LUT1_W<CTRLA_SPEC> {
        LUT1_W::new(self, 5)
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
#[doc = "Port Multiplexer Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
