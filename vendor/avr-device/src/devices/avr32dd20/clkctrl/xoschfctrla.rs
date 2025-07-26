#[doc = "Register `XOSCHFCTRLA` reader"]
pub type R = crate::R<XOSCHFCTRLA_SPEC>;
#[doc = "Register `XOSCHFCTRLA` writer"]
pub type W = crate::W<XOSCHFCTRLA_SPEC>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELHF` reader - External Source Select"]
pub type SELHF_R = crate::BitReader<SELHF_A>;
#[doc = "External Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELHF_A {
    #[doc = "0: External Crystal"]
    XTAL = 0,
    #[doc = "1: External clock on XTALHF1 pin"]
    EXTCLOCK = 1,
}
impl From<SELHF_A> for bool {
    #[inline(always)]
    fn from(variant: SELHF_A) -> Self {
        variant as u8 != 0
    }
}
impl SELHF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELHF_A {
        match self.bits {
            false => SELHF_A::XTAL,
            true => SELHF_A::EXTCLOCK,
        }
    }
    #[doc = "External Crystal"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == SELHF_A::XTAL
    }
    #[doc = "External clock on XTALHF1 pin"]
    #[inline(always)]
    pub fn is_extclock(&self) -> bool {
        *self == SELHF_A::EXTCLOCK
    }
}
#[doc = "Field `SELHF` writer - External Source Select"]
pub type SELHF_W<'a, REG> = crate::BitWriter<'a, REG, SELHF_A>;
impl<'a, REG> SELHF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External Crystal"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut crate::W<REG> {
        self.variant(SELHF_A::XTAL)
    }
    #[doc = "External clock on XTALHF1 pin"]
    #[inline(always)]
    pub fn extclock(self) -> &'a mut crate::W<REG> {
        self.variant(SELHF_A::EXTCLOCK)
    }
}
#[doc = "Field `FRQRANGE` reader - Frequency Range"]
pub type FRQRANGE_R = crate::FieldReader<FRQRANGE_A>;
#[doc = "Frequency Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRQRANGE_A {
    #[doc = "0: Max 8 MHz XTAL Frequency"]
    _8M = 0,
    #[doc = "1: Max 16 MHz XTAL Frequency"]
    _16M = 1,
    #[doc = "2: Max 24 MHz XTAL Frequency"]
    _24M = 2,
    #[doc = "3: Max 32 MHz XTAL Frequency"]
    _32M = 3,
}
impl From<FRQRANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: FRQRANGE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FRQRANGE_A {
    type Ux = u8;
}
impl FRQRANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FRQRANGE_A {
        match self.bits {
            0 => FRQRANGE_A::_8M,
            1 => FRQRANGE_A::_16M,
            2 => FRQRANGE_A::_24M,
            3 => FRQRANGE_A::_32M,
            _ => unreachable!(),
        }
    }
    #[doc = "Max 8 MHz XTAL Frequency"]
    #[inline(always)]
    pub fn is_8m(&self) -> bool {
        *self == FRQRANGE_A::_8M
    }
    #[doc = "Max 16 MHz XTAL Frequency"]
    #[inline(always)]
    pub fn is_16m(&self) -> bool {
        *self == FRQRANGE_A::_16M
    }
    #[doc = "Max 24 MHz XTAL Frequency"]
    #[inline(always)]
    pub fn is_24m(&self) -> bool {
        *self == FRQRANGE_A::_24M
    }
    #[doc = "Max 32 MHz XTAL Frequency"]
    #[inline(always)]
    pub fn is_32m(&self) -> bool {
        *self == FRQRANGE_A::_32M
    }
}
#[doc = "Field `FRQRANGE` writer - Frequency Range"]
pub type FRQRANGE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, FRQRANGE_A>;
impl<'a, REG> FRQRANGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Max 8 MHz XTAL Frequency"]
    #[inline(always)]
    pub fn _8m(self) -> &'a mut crate::W<REG> {
        self.variant(FRQRANGE_A::_8M)
    }
    #[doc = "Max 16 MHz XTAL Frequency"]
    #[inline(always)]
    pub fn _16m(self) -> &'a mut crate::W<REG> {
        self.variant(FRQRANGE_A::_16M)
    }
    #[doc = "Max 24 MHz XTAL Frequency"]
    #[inline(always)]
    pub fn _24m(self) -> &'a mut crate::W<REG> {
        self.variant(FRQRANGE_A::_24M)
    }
    #[doc = "Max 32 MHz XTAL Frequency"]
    #[inline(always)]
    pub fn _32m(self) -> &'a mut crate::W<REG> {
        self.variant(FRQRANGE_A::_32M)
    }
}
#[doc = "Field `CSUTHF` reader - Start-up Time Select"]
pub type CSUTHF_R = crate::FieldReader<CSUTHF_A>;
#[doc = "Start-up Time Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSUTHF_A {
    #[doc = "0: 256 XOSCHF cycles"]
    _256 = 0,
    #[doc = "1: 1K XOSCHF cycles"]
    _1K = 1,
    #[doc = "2: 4K XOSCHF cycles"]
    _4K = 2,
}
impl From<CSUTHF_A> for u8 {
    #[inline(always)]
    fn from(variant: CSUTHF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSUTHF_A {
    type Ux = u8;
}
impl CSUTHF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CSUTHF_A> {
        match self.bits {
            0 => Some(CSUTHF_A::_256),
            1 => Some(CSUTHF_A::_1K),
            2 => Some(CSUTHF_A::_4K),
            _ => None,
        }
    }
    #[doc = "256 XOSCHF cycles"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == CSUTHF_A::_256
    }
    #[doc = "1K XOSCHF cycles"]
    #[inline(always)]
    pub fn is_1k(&self) -> bool {
        *self == CSUTHF_A::_1K
    }
    #[doc = "4K XOSCHF cycles"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == CSUTHF_A::_4K
    }
}
#[doc = "Field `CSUTHF` writer - Start-up Time Select"]
pub type CSUTHF_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CSUTHF_A>;
impl<'a, REG> CSUTHF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256 XOSCHF cycles"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(CSUTHF_A::_256)
    }
    #[doc = "1K XOSCHF cycles"]
    #[inline(always)]
    pub fn _1k(self) -> &'a mut crate::W<REG> {
        self.variant(CSUTHF_A::_1K)
    }
    #[doc = "4K XOSCHF cycles"]
    #[inline(always)]
    pub fn _4k(self) -> &'a mut crate::W<REG> {
        self.variant(CSUTHF_A::_4K)
    }
}
#[doc = "Field `RUNSTBY` reader - Run Standby"]
pub type RUNSTBY_R = crate::BitReader;
#[doc = "Field `RUNSTBY` writer - Run Standby"]
pub type RUNSTBY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Source Select"]
    #[inline(always)]
    pub fn selhf(&self) -> SELHF_R {
        SELHF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Frequency Range"]
    #[inline(always)]
    pub fn frqrange(&self) -> FRQRANGE_R {
        FRQRANGE_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Start-up Time Select"]
    #[inline(always)]
    pub fn csuthf(&self) -> CSUTHF_R {
        CSUTHF_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 7 - Run Standby"]
    #[inline(always)]
    pub fn runstby(&self) -> RUNSTBY_R {
        RUNSTBY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<XOSCHFCTRLA_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - External Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn selhf(&mut self) -> SELHF_W<XOSCHFCTRLA_SPEC> {
        SELHF_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Frequency Range"]
    #[inline(always)]
    #[must_use]
    pub fn frqrange(&mut self) -> FRQRANGE_W<XOSCHFCTRLA_SPEC> {
        FRQRANGE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Start-up Time Select"]
    #[inline(always)]
    #[must_use]
    pub fn csuthf(&mut self) -> CSUTHF_W<XOSCHFCTRLA_SPEC> {
        CSUTHF_W::new(self, 4)
    }
    #[doc = "Bit 7 - Run Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstby(&mut self) -> RUNSTBY_W<XOSCHFCTRLA_SPEC> {
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
#[doc = "XOSC HF Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xoschfctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xoschfctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XOSCHFCTRLA_SPEC;
impl crate::RegisterSpec for XOSCHFCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xoschfctrla::R`](R) reader structure"]
impl crate::Readable for XOSCHFCTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xoschfctrla::W`](W) writer structure"]
impl crate::Writable for XOSCHFCTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XOSCHFCTRLA to value 0"]
impl crate::Resettable for XOSCHFCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
