#[doc = "Register `PLLCTRLA` reader"]
pub type R = crate::R<PLLCTRLA_SPEC>;
#[doc = "Register `PLLCTRLA` writer"]
pub type W = crate::W<PLLCTRLA_SPEC>;
#[doc = "Field `MULFAC` reader - Multiplication factor"]
pub type MULFAC_R = crate::FieldReader<MULFAC_A>;
#[doc = "Multiplication factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MULFAC_A {
    #[doc = "0: PLL is disabled"]
    DISABLE = 0,
    #[doc = "1: 2 x multiplication factor"]
    _2X = 1,
    #[doc = "2: 3 x multiplication factor"]
    _3X = 2,
}
impl From<MULFAC_A> for u8 {
    #[inline(always)]
    fn from(variant: MULFAC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MULFAC_A {
    type Ux = u8;
}
impl MULFAC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MULFAC_A> {
        match self.bits {
            0 => Some(MULFAC_A::DISABLE),
            1 => Some(MULFAC_A::_2X),
            2 => Some(MULFAC_A::_3X),
            _ => None,
        }
    }
    #[doc = "PLL is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MULFAC_A::DISABLE
    }
    #[doc = "2 x multiplication factor"]
    #[inline(always)]
    pub fn is_2x(&self) -> bool {
        *self == MULFAC_A::_2X
    }
    #[doc = "3 x multiplication factor"]
    #[inline(always)]
    pub fn is_3x(&self) -> bool {
        *self == MULFAC_A::_3X
    }
}
#[doc = "Field `MULFAC` writer - Multiplication factor"]
pub type MULFAC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MULFAC_A>;
impl<'a, REG> MULFAC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLL is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MULFAC_A::DISABLE)
    }
    #[doc = "2 x multiplication factor"]
    #[inline(always)]
    pub fn _2x(self) -> &'a mut crate::W<REG> {
        self.variant(MULFAC_A::_2X)
    }
    #[doc = "3 x multiplication factor"]
    #[inline(always)]
    pub fn _3x(self) -> &'a mut crate::W<REG> {
        self.variant(MULFAC_A::_3X)
    }
}
#[doc = "Field `SOURCE` reader - Source"]
pub type SOURCE_R = crate::BitReader<SOURCE_A>;
#[doc = "Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOURCE_A {
    #[doc = "0: High frequency internal oscillator as PLL source"]
    OSCHF = 0,
    #[doc = "1: High frequency external clock or external high frequency oscillator as PLL source"]
    XOSCHF = 1,
}
impl From<SOURCE_A> for bool {
    #[inline(always)]
    fn from(variant: SOURCE_A) -> Self {
        variant as u8 != 0
    }
}
impl SOURCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOURCE_A {
        match self.bits {
            false => SOURCE_A::OSCHF,
            true => SOURCE_A::XOSCHF,
        }
    }
    #[doc = "High frequency internal oscillator as PLL source"]
    #[inline(always)]
    pub fn is_oschf(&self) -> bool {
        *self == SOURCE_A::OSCHF
    }
    #[doc = "High frequency external clock or external high frequency oscillator as PLL source"]
    #[inline(always)]
    pub fn is_xoschf(&self) -> bool {
        *self == SOURCE_A::XOSCHF
    }
}
#[doc = "Field `SOURCE` writer - Source"]
pub type SOURCE_W<'a, REG> = crate::BitWriter<'a, REG, SOURCE_A>;
impl<'a, REG> SOURCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High frequency internal oscillator as PLL source"]
    #[inline(always)]
    pub fn oschf(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::OSCHF)
    }
    #[doc = "High frequency external clock or external high frequency oscillator as PLL source"]
    #[inline(always)]
    pub fn xoschf(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::XOSCHF)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run standby"]
pub type RUNSTDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Multiplication factor"]
    #[inline(always)]
    pub fn mulfac(&self) -> MULFAC_R {
        MULFAC_R::new(self.bits & 3)
    }
    #[doc = "Bit 6 - Source"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Run standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Multiplication factor"]
    #[inline(always)]
    #[must_use]
    pub fn mulfac(&mut self) -> MULFAC_W<PLLCTRLA_SPEC> {
        MULFAC_W::new(self, 0)
    }
    #[doc = "Bit 6 - Source"]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SOURCE_W<PLLCTRLA_SPEC> {
        SOURCE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Run standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<PLLCTRLA_SPEC> {
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
#[doc = "PLL Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLCTRLA_SPEC;
impl crate::RegisterSpec for PLLCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pllctrla::R`](R) reader structure"]
impl crate::Readable for PLLCTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllctrla::W`](W) writer structure"]
impl crate::Writable for PLLCTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCTRLA to value 0"]
impl crate::Resettable for PLLCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
