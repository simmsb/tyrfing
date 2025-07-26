#[doc = "Register `CCLROUTEA` reader"]
pub type R = crate::R<CCLROUTEA_SPEC>;
#[doc = "Register `CCLROUTEA` writer"]
pub type W = crate::W<CCLROUTEA_SPEC>;
#[doc = "Field `LUT0` reader - CCL Look-Up Table 0 Signals"]
pub type LUT0_R = crate::BitReader<LUT0_A>;
#[doc = "CCL Look-Up Table 0 Signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LUT0_A {
    #[doc = "0: INn: PA0, PA1, PA2. OUT: PA3."]
    DEFAULT = 0,
    #[doc = "1: INn: PA0, PA1, PA2. OUT: PA6."]
    ALT1 = 1,
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
            true => LUT0_A::ALT1,
        }
    }
    #[doc = "INn: PA0, PA1, PA2. OUT: PA3."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == LUT0_A::DEFAULT
    }
    #[doc = "INn: PA0, PA1, PA2. OUT: PA6."]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == LUT0_A::ALT1
    }
}
#[doc = "Field `LUT0` writer - CCL Look-Up Table 0 Signals"]
pub type LUT0_W<'a, REG> = crate::BitWriter<'a, REG, LUT0_A>;
impl<'a, REG> LUT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "INn: PA0, PA1, PA2. OUT: PA3."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(LUT0_A::DEFAULT)
    }
    #[doc = "INn: PA0, PA1, PA2. OUT: PA6."]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut crate::W<REG> {
        self.variant(LUT0_A::ALT1)
    }
}
#[doc = "Field `LUT1` reader - CCL Look-Up Table 1 Signals"]
pub type LUT1_R = crate::BitReader<LUT1_A>;
#[doc = "CCL Look-Up Table 1 Signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LUT1_A {
    #[doc = "0: INn: -, PC1, PC2 OUT: PC3"]
    DEFAULT = 0,
    #[doc = "1: INn: -, PC1, PC2 OUT: -"]
    ALT1 = 1,
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
            true => LUT1_A::ALT1,
        }
    }
    #[doc = "INn: -, PC1, PC2 OUT: PC3"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == LUT1_A::DEFAULT
    }
    #[doc = "INn: -, PC1, PC2 OUT: -"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == LUT1_A::ALT1
    }
}
#[doc = "Field `LUT1` writer - CCL Look-Up Table 1 Signals"]
pub type LUT1_W<'a, REG> = crate::BitWriter<'a, REG, LUT1_A>;
impl<'a, REG> LUT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "INn: -, PC1, PC2 OUT: PC3"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(LUT1_A::DEFAULT)
    }
    #[doc = "INn: -, PC1, PC2 OUT: -"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut crate::W<REG> {
        self.variant(LUT1_A::ALT1)
    }
}
#[doc = "Field `LUT2` reader - CCL Look-Up Table 2 Signals"]
pub type LUT2_R = crate::BitReader<LUT2_A>;
#[doc = "CCL Look-Up Table 2 Signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LUT2_A {
    #[doc = "0: Not connected to any pins"]
    DEFAULT = 0,
    #[doc = "1: INn: -, -, -. OUT: PD6."]
    ALT1 = 1,
}
impl From<LUT2_A> for bool {
    #[inline(always)]
    fn from(variant: LUT2_A) -> Self {
        variant as u8 != 0
    }
}
impl LUT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LUT2_A {
        match self.bits {
            false => LUT2_A::DEFAULT,
            true => LUT2_A::ALT1,
        }
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == LUT2_A::DEFAULT
    }
    #[doc = "INn: -, -, -. OUT: PD6."]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == LUT2_A::ALT1
    }
}
#[doc = "Field `LUT2` writer - CCL Look-Up Table 2 Signals"]
pub type LUT2_W<'a, REG> = crate::BitWriter<'a, REG, LUT2_A>;
impl<'a, REG> LUT2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(LUT2_A::DEFAULT)
    }
    #[doc = "INn: -, -, -. OUT: PD6."]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut crate::W<REG> {
        self.variant(LUT2_A::ALT1)
    }
}
impl R {
    #[doc = "Bit 0 - CCL Look-Up Table 0 Signals"]
    #[inline(always)]
    pub fn lut0(&self) -> LUT0_R {
        LUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCL Look-Up Table 1 Signals"]
    #[inline(always)]
    pub fn lut1(&self) -> LUT1_R {
        LUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCL Look-Up Table 2 Signals"]
    #[inline(always)]
    pub fn lut2(&self) -> LUT2_R {
        LUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCL Look-Up Table 0 Signals"]
    #[inline(always)]
    #[must_use]
    pub fn lut0(&mut self) -> LUT0_W<CCLROUTEA_SPEC> {
        LUT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - CCL Look-Up Table 1 Signals"]
    #[inline(always)]
    #[must_use]
    pub fn lut1(&mut self) -> LUT1_W<CCLROUTEA_SPEC> {
        LUT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - CCL Look-Up Table 2 Signals"]
    #[inline(always)]
    #[must_use]
    pub fn lut2(&mut self) -> LUT2_W<CCLROUTEA_SPEC> {
        LUT2_W::new(self, 2)
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
#[doc = "CCL route A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cclroutea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cclroutea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCLROUTEA_SPEC;
impl crate::RegisterSpec for CCLROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cclroutea::R`](R) reader structure"]
impl crate::Readable for CCLROUTEA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cclroutea::W`](W) writer structure"]
impl crate::Writable for CCLROUTEA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCLROUTEA to value 0"]
impl crate::Resettable for CCLROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
