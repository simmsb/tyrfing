#[doc = "Register `SPCR0` reader"]
pub type R = crate::R<SPCR0_SPEC>;
#[doc = "Register `SPCR0` writer"]
pub type W = crate::W<SPCR0_SPEC>;
#[doc = "Field `SPR00` reader - SPI Clock Rate Select 0"]
pub type SPR00_R = crate::BitReader<SPR00_A>;
#[doc = "SPI Clock Rate Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPR00_A {
    #[doc = "0: fosc/4"]
    FOSC_4 = 0,
    #[doc = "1: fosc/16"]
    FOSC_16 = 1,
}
impl From<SPR00_A> for bool {
    #[inline(always)]
    fn from(variant: SPR00_A) -> Self {
        variant as u8 != 0
    }
}
impl SPR00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPR00_A {
        match self.bits {
            false => SPR00_A::FOSC_4,
            true => SPR00_A::FOSC_16,
        }
    }
    #[doc = "fosc/4"]
    #[inline(always)]
    pub fn is_fosc_4(&self) -> bool {
        *self == SPR00_A::FOSC_4
    }
    #[doc = "fosc/16"]
    #[inline(always)]
    pub fn is_fosc_16(&self) -> bool {
        *self == SPR00_A::FOSC_16
    }
}
#[doc = "Field `SPR00` writer - SPI Clock Rate Select 0"]
pub type SPR00_W<'a, REG> = crate::BitWriter<'a, REG, SPR00_A>;
impl<'a, REG> SPR00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fosc/4"]
    #[inline(always)]
    pub fn fosc_4(self) -> &'a mut crate::W<REG> {
        self.variant(SPR00_A::FOSC_4)
    }
    #[doc = "fosc/16"]
    #[inline(always)]
    pub fn fosc_16(self) -> &'a mut crate::W<REG> {
        self.variant(SPR00_A::FOSC_16)
    }
}
#[doc = "Field `SPR10` reader - SPI Clock Rate Select 1"]
pub type SPR10_R = crate::BitReader;
#[doc = "Field `SPR10` writer - SPI Clock Rate Select 1"]
pub type SPR10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHA0` reader - Clock Phase"]
pub type CPHA0_R = crate::BitReader;
#[doc = "Field `CPHA0` writer - Clock Phase"]
pub type CPHA0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL0` reader - Clock polarity"]
pub type CPOL0_R = crate::BitReader;
#[doc = "Field `CPOL0` writer - Clock polarity"]
pub type CPOL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTR0` reader - Master/Slave Select"]
pub type MSTR0_R = crate::BitReader;
#[doc = "Field `MSTR0` writer - Master/Slave Select"]
pub type MSTR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DORD0` reader - Data Order"]
pub type DORD0_R = crate::BitReader;
#[doc = "Field `DORD0` writer - Data Order"]
pub type DORD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPE0` reader - SPI Enable"]
pub type SPE0_R = crate::BitReader;
#[doc = "Field `SPE0` writer - SPI Enable"]
pub type SPE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIE0` reader - SPI Interrupt Enable"]
pub type SPIE0_R = crate::BitReader;
#[doc = "Field `SPIE0` writer - SPI Interrupt Enable"]
pub type SPIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI Clock Rate Select 0"]
    #[inline(always)]
    pub fn spr00(&self) -> SPR00_R {
        SPR00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Clock Rate Select 1"]
    #[inline(always)]
    pub fn spr10(&self) -> SPR10_R {
        SPR10_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Phase"]
    #[inline(always)]
    pub fn cpha0(&self) -> CPHA0_R {
        CPHA0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock polarity"]
    #[inline(always)]
    pub fn cpol0(&self) -> CPOL0_R {
        CPOL0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master/Slave Select"]
    #[inline(always)]
    pub fn mstr0(&self) -> MSTR0_R {
        MSTR0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Order"]
    #[inline(always)]
    pub fn dord0(&self) -> DORD0_R {
        DORD0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI Enable"]
    #[inline(always)]
    pub fn spe0(&self) -> SPE0_R {
        SPE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Interrupt Enable"]
    #[inline(always)]
    pub fn spie0(&self) -> SPIE0_R {
        SPIE0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Clock Rate Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn spr00(&mut self) -> SPR00_W<SPCR0_SPEC> {
        SPR00_W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI Clock Rate Select 1"]
    #[inline(always)]
    #[must_use]
    pub fn spr10(&mut self) -> SPR10_W<SPCR0_SPEC> {
        SPR10_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha0(&mut self) -> CPHA0_W<SPCR0_SPEC> {
        CPHA0_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol0(&mut self) -> CPOL0_W<SPCR0_SPEC> {
        CPOL0_W::new(self, 3)
    }
    #[doc = "Bit 4 - Master/Slave Select"]
    #[inline(always)]
    #[must_use]
    pub fn mstr0(&mut self) -> MSTR0_W<SPCR0_SPEC> {
        MSTR0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Data Order"]
    #[inline(always)]
    #[must_use]
    pub fn dord0(&mut self) -> DORD0_W<SPCR0_SPEC> {
        DORD0_W::new(self, 5)
    }
    #[doc = "Bit 6 - SPI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spe0(&mut self) -> SPE0_W<SPCR0_SPEC> {
        SPE0_W::new(self, 6)
    }
    #[doc = "Bit 7 - SPI Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spie0(&mut self) -> SPIE0_W<SPCR0_SPEC> {
        SPIE0_W::new(self, 7)
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
#[doc = "SPI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spcr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spcr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPCR0_SPEC;
impl crate::RegisterSpec for SPCR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spcr0::R`](R) reader structure"]
impl crate::Readable for SPCR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spcr0::W`](W) writer structure"]
impl crate::Writable for SPCR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPCR0 to value 0"]
impl crate::Resettable for SPCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
