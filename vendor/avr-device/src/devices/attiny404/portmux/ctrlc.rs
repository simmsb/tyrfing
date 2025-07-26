#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CTRLC_SPEC>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CTRLC_SPEC>;
#[doc = "Field `TCA00` reader - Port Multiplexer TCA0 Output 0"]
pub type TCA00_R = crate::BitReader<TCA00_A>;
#[doc = "Port Multiplexer TCA0 Output 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCA00_A {
    #[doc = "0: Default pin"]
    DEFAULT = 0,
    #[doc = "1: Alternate pin"]
    ALTERNATE = 1,
}
impl From<TCA00_A> for bool {
    #[inline(always)]
    fn from(variant: TCA00_A) -> Self {
        variant as u8 != 0
    }
}
impl TCA00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCA00_A {
        match self.bits {
            false => TCA00_A::DEFAULT,
            true => TCA00_A::ALTERNATE,
        }
    }
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCA00_A::DEFAULT
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TCA00_A::ALTERNATE
    }
}
#[doc = "Field `TCA00` writer - Port Multiplexer TCA0 Output 0"]
pub type TCA00_W<'a, REG> = crate::BitWriter<'a, REG, TCA00_A>;
impl<'a, REG> TCA00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(TCA00_A::DEFAULT)
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(TCA00_A::ALTERNATE)
    }
}
#[doc = "Field `TCA01` reader - Port Multiplexer TCA0 Output 1"]
pub type TCA01_R = crate::BitReader<TCA01_A>;
#[doc = "Port Multiplexer TCA0 Output 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCA01_A {
    #[doc = "0: Default pin"]
    DEFAULT = 0,
    #[doc = "1: Alternate pin"]
    ALTERNATE = 1,
}
impl From<TCA01_A> for bool {
    #[inline(always)]
    fn from(variant: TCA01_A) -> Self {
        variant as u8 != 0
    }
}
impl TCA01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCA01_A {
        match self.bits {
            false => TCA01_A::DEFAULT,
            true => TCA01_A::ALTERNATE,
        }
    }
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCA01_A::DEFAULT
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TCA01_A::ALTERNATE
    }
}
#[doc = "Field `TCA01` writer - Port Multiplexer TCA0 Output 1"]
pub type TCA01_W<'a, REG> = crate::BitWriter<'a, REG, TCA01_A>;
impl<'a, REG> TCA01_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(TCA01_A::DEFAULT)
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(TCA01_A::ALTERNATE)
    }
}
#[doc = "Field `TCA02` reader - Port Multiplexer TCA0 Output 2"]
pub type TCA02_R = crate::BitReader<TCA02_A>;
#[doc = "Port Multiplexer TCA0 Output 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCA02_A {
    #[doc = "0: Default pin"]
    DEFAULT = 0,
    #[doc = "1: Alternate pin"]
    ALTERNATE = 1,
}
impl From<TCA02_A> for bool {
    #[inline(always)]
    fn from(variant: TCA02_A) -> Self {
        variant as u8 != 0
    }
}
impl TCA02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCA02_A {
        match self.bits {
            false => TCA02_A::DEFAULT,
            true => TCA02_A::ALTERNATE,
        }
    }
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCA02_A::DEFAULT
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TCA02_A::ALTERNATE
    }
}
#[doc = "Field `TCA02` writer - Port Multiplexer TCA0 Output 2"]
pub type TCA02_W<'a, REG> = crate::BitWriter<'a, REG, TCA02_A>;
impl<'a, REG> TCA02_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(TCA02_A::DEFAULT)
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(TCA02_A::ALTERNATE)
    }
}
#[doc = "Field `TCA03` reader - Port Multiplexer TCA0 Output 3"]
pub type TCA03_R = crate::BitReader<TCA03_A>;
#[doc = "Port Multiplexer TCA0 Output 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCA03_A {
    #[doc = "0: Default pin"]
    DEFAULT = 0,
    #[doc = "1: Alternate pin"]
    ALTERNATE = 1,
}
impl From<TCA03_A> for bool {
    #[inline(always)]
    fn from(variant: TCA03_A) -> Self {
        variant as u8 != 0
    }
}
impl TCA03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCA03_A {
        match self.bits {
            false => TCA03_A::DEFAULT,
            true => TCA03_A::ALTERNATE,
        }
    }
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCA03_A::DEFAULT
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TCA03_A::ALTERNATE
    }
}
#[doc = "Field `TCA03` writer - Port Multiplexer TCA0 Output 3"]
pub type TCA03_W<'a, REG> = crate::BitWriter<'a, REG, TCA03_A>;
impl<'a, REG> TCA03_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(TCA03_A::DEFAULT)
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(TCA03_A::ALTERNATE)
    }
}
#[doc = "Field `TCA04` reader - Port Multiplexer TCA0 Output 4"]
pub type TCA04_R = crate::BitReader<TCA04_A>;
#[doc = "Port Multiplexer TCA0 Output 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCA04_A {
    #[doc = "0: Default pin"]
    DEFAULT = 0,
    #[doc = "1: Alternate pin"]
    ALTERNATE = 1,
}
impl From<TCA04_A> for bool {
    #[inline(always)]
    fn from(variant: TCA04_A) -> Self {
        variant as u8 != 0
    }
}
impl TCA04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCA04_A {
        match self.bits {
            false => TCA04_A::DEFAULT,
            true => TCA04_A::ALTERNATE,
        }
    }
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCA04_A::DEFAULT
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TCA04_A::ALTERNATE
    }
}
#[doc = "Field `TCA04` writer - Port Multiplexer TCA0 Output 4"]
pub type TCA04_W<'a, REG> = crate::BitWriter<'a, REG, TCA04_A>;
impl<'a, REG> TCA04_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(TCA04_A::DEFAULT)
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(TCA04_A::ALTERNATE)
    }
}
#[doc = "Field `TCA05` reader - Port Multiplexer TCA0 Output 5"]
pub type TCA05_R = crate::BitReader<TCA05_A>;
#[doc = "Port Multiplexer TCA0 Output 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCA05_A {
    #[doc = "0: Default pin"]
    DEFAULT = 0,
    #[doc = "1: Alternate pin"]
    ALTERNATE = 1,
}
impl From<TCA05_A> for bool {
    #[inline(always)]
    fn from(variant: TCA05_A) -> Self {
        variant as u8 != 0
    }
}
impl TCA05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCA05_A {
        match self.bits {
            false => TCA05_A::DEFAULT,
            true => TCA05_A::ALTERNATE,
        }
    }
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCA05_A::DEFAULT
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TCA05_A::ALTERNATE
    }
}
#[doc = "Field `TCA05` writer - Port Multiplexer TCA0 Output 5"]
pub type TCA05_W<'a, REG> = crate::BitWriter<'a, REG, TCA05_A>;
impl<'a, REG> TCA05_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(TCA05_A::DEFAULT)
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(TCA05_A::ALTERNATE)
    }
}
impl R {
    #[doc = "Bit 0 - Port Multiplexer TCA0 Output 0"]
    #[inline(always)]
    pub fn tca00(&self) -> TCA00_R {
        TCA00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Multiplexer TCA0 Output 1"]
    #[inline(always)]
    pub fn tca01(&self) -> TCA01_R {
        TCA01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Multiplexer TCA0 Output 2"]
    #[inline(always)]
    pub fn tca02(&self) -> TCA02_R {
        TCA02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Multiplexer TCA0 Output 3"]
    #[inline(always)]
    pub fn tca03(&self) -> TCA03_R {
        TCA03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Multiplexer TCA0 Output 4"]
    #[inline(always)]
    pub fn tca04(&self) -> TCA04_R {
        TCA04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Multiplexer TCA0 Output 5"]
    #[inline(always)]
    pub fn tca05(&self) -> TCA05_R {
        TCA05_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Multiplexer TCA0 Output 0"]
    #[inline(always)]
    #[must_use]
    pub fn tca00(&mut self) -> TCA00_W<CTRLC_SPEC> {
        TCA00_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port Multiplexer TCA0 Output 1"]
    #[inline(always)]
    #[must_use]
    pub fn tca01(&mut self) -> TCA01_W<CTRLC_SPEC> {
        TCA01_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port Multiplexer TCA0 Output 2"]
    #[inline(always)]
    #[must_use]
    pub fn tca02(&mut self) -> TCA02_W<CTRLC_SPEC> {
        TCA02_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port Multiplexer TCA0 Output 3"]
    #[inline(always)]
    #[must_use]
    pub fn tca03(&mut self) -> TCA03_W<CTRLC_SPEC> {
        TCA03_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port Multiplexer TCA0 Output 4"]
    #[inline(always)]
    #[must_use]
    pub fn tca04(&mut self) -> TCA04_W<CTRLC_SPEC> {
        TCA04_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port Multiplexer TCA0 Output 5"]
    #[inline(always)]
    #[must_use]
    pub fn tca05(&mut self) -> TCA05_W<CTRLC_SPEC> {
        TCA05_W::new(self, 5)
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
#[doc = "Port Multiplexer Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLC_SPEC;
impl crate::RegisterSpec for CTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CTRLC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CTRLC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
