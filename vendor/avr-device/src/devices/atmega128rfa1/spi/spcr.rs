#[doc = "Register `SPCR` reader"]
pub type R = crate::R<SPCR_SPEC>;
#[doc = "Register `SPCR` writer"]
pub type W = crate::W<SPCR_SPEC>;
#[doc = "Field `SPR` reader - SPI Clock Rate Select 1 and 0"]
pub type SPR_R = crate::FieldReader<SPR_A>;
#[doc = "SPI Clock Rate Select 1 and 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPR_A {
    #[doc = "0: Fosc/4 if SPI2X == 0 else Fosc/2"]
    FOSC_4_2 = 0,
    #[doc = "1: Fosc/16 if SPI2X == 0 else Fosc/8"]
    FOSC_16_8 = 1,
    #[doc = "2: Fosc/64 if SPI2X == 0 else Fosc/32"]
    FOSC_64_32 = 2,
    #[doc = "3: Fosc/128 if SPI2X == 0 else Fosc/64"]
    FOSC_128_64 = 3,
}
impl From<SPR_A> for u8 {
    #[inline(always)]
    fn from(variant: SPR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPR_A {
    type Ux = u8;
}
impl SPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPR_A {
        match self.bits {
            0 => SPR_A::FOSC_4_2,
            1 => SPR_A::FOSC_16_8,
            2 => SPR_A::FOSC_64_32,
            3 => SPR_A::FOSC_128_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Fosc/4 if SPI2X == 0 else Fosc/2"]
    #[inline(always)]
    pub fn is_fosc_4_2(&self) -> bool {
        *self == SPR_A::FOSC_4_2
    }
    #[doc = "Fosc/16 if SPI2X == 0 else Fosc/8"]
    #[inline(always)]
    pub fn is_fosc_16_8(&self) -> bool {
        *self == SPR_A::FOSC_16_8
    }
    #[doc = "Fosc/64 if SPI2X == 0 else Fosc/32"]
    #[inline(always)]
    pub fn is_fosc_64_32(&self) -> bool {
        *self == SPR_A::FOSC_64_32
    }
    #[doc = "Fosc/128 if SPI2X == 0 else Fosc/64"]
    #[inline(always)]
    pub fn is_fosc_128_64(&self) -> bool {
        *self == SPR_A::FOSC_128_64
    }
}
#[doc = "Field `SPR` writer - SPI Clock Rate Select 1 and 0"]
pub type SPR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SPR_A>;
impl<'a, REG> SPR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Fosc/4 if SPI2X == 0 else Fosc/2"]
    #[inline(always)]
    pub fn fosc_4_2(self) -> &'a mut crate::W<REG> {
        self.variant(SPR_A::FOSC_4_2)
    }
    #[doc = "Fosc/16 if SPI2X == 0 else Fosc/8"]
    #[inline(always)]
    pub fn fosc_16_8(self) -> &'a mut crate::W<REG> {
        self.variant(SPR_A::FOSC_16_8)
    }
    #[doc = "Fosc/64 if SPI2X == 0 else Fosc/32"]
    #[inline(always)]
    pub fn fosc_64_32(self) -> &'a mut crate::W<REG> {
        self.variant(SPR_A::FOSC_64_32)
    }
    #[doc = "Fosc/128 if SPI2X == 0 else Fosc/64"]
    #[inline(always)]
    pub fn fosc_128_64(self) -> &'a mut crate::W<REG> {
        self.variant(SPR_A::FOSC_128_64)
    }
}
#[doc = "Field `CPHA` reader - Clock Phase"]
pub type CPHA_R = crate::BitReader<CPHA_A>;
#[doc = "Clock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA_A {
    #[doc = "0: Sample (Leading Edge), Setup (Trailing Edge)"]
    SAMPLE_LEADING_EDGE_SETUP_TRAILING_EDGE = 0,
    #[doc = "1: Setup (Leading Edge), Sample (Trailing Edge)"]
    SETUP_LEADING_EDGE_SAMPLE_TRAILING_EDGE = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::SAMPLE_LEADING_EDGE_SETUP_TRAILING_EDGE,
            true => CPHA_A::SETUP_LEADING_EDGE_SAMPLE_TRAILING_EDGE,
        }
    }
    #[doc = "Sample (Leading Edge), Setup (Trailing Edge)"]
    #[inline(always)]
    pub fn is_sample_leading_edge_setup_trailing_edge(&self) -> bool {
        *self == CPHA_A::SAMPLE_LEADING_EDGE_SETUP_TRAILING_EDGE
    }
    #[doc = "Setup (Leading Edge), Sample (Trailing Edge)"]
    #[inline(always)]
    pub fn is_setup_leading_edge_sample_trailing_edge(&self) -> bool {
        *self == CPHA_A::SETUP_LEADING_EDGE_SAMPLE_TRAILING_EDGE
    }
}
#[doc = "Field `CPHA` writer - Clock Phase"]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG, CPHA_A>;
impl<'a, REG> CPHA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sample (Leading Edge), Setup (Trailing Edge)"]
    #[inline(always)]
    pub fn sample_leading_edge_setup_trailing_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA_A::SAMPLE_LEADING_EDGE_SETUP_TRAILING_EDGE)
    }
    #[doc = "Setup (Leading Edge), Sample (Trailing Edge)"]
    #[inline(always)]
    pub fn setup_leading_edge_sample_trailing_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA_A::SETUP_LEADING_EDGE_SAMPLE_TRAILING_EDGE)
    }
}
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CPOL_R = crate::BitReader<CPOL_A>;
#[doc = "Clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    #[doc = "0: Rising (Leading Edge), Falling (Trailing Edge)"]
    RISING_LEADING_EDGE_FALLING_TRAILING_EDGE = 0,
    #[doc = "1: Falling (Leading Egde), Rising (Trailing Edge)"]
    FALLING_LEADING_EGDE_RISING_TRAILING_EDGE = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::RISING_LEADING_EDGE_FALLING_TRAILING_EDGE,
            true => CPOL_A::FALLING_LEADING_EGDE_RISING_TRAILING_EDGE,
        }
    }
    #[doc = "Rising (Leading Edge), Falling (Trailing Edge)"]
    #[inline(always)]
    pub fn is_rising_leading_edge_falling_trailing_edge(&self) -> bool {
        *self == CPOL_A::RISING_LEADING_EDGE_FALLING_TRAILING_EDGE
    }
    #[doc = "Falling (Leading Egde), Rising (Trailing Edge)"]
    #[inline(always)]
    pub fn is_falling_leading_egde_rising_trailing_edge(&self) -> bool {
        *self == CPOL_A::FALLING_LEADING_EGDE_RISING_TRAILING_EDGE
    }
}
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG, CPOL_A>;
impl<'a, REG> CPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising (Leading Edge), Falling (Trailing Edge)"]
    #[inline(always)]
    pub fn rising_leading_edge_falling_trailing_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL_A::RISING_LEADING_EDGE_FALLING_TRAILING_EDGE)
    }
    #[doc = "Falling (Leading Egde), Rising (Trailing Edge)"]
    #[inline(always)]
    pub fn falling_leading_egde_rising_trailing_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL_A::FALLING_LEADING_EGDE_RISING_TRAILING_EDGE)
    }
}
#[doc = "Field `MSTR` reader - Master/Slave Select"]
pub type MSTR_R = crate::BitReader;
#[doc = "Field `MSTR` writer - Master/Slave Select"]
pub type MSTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DORD` reader - Data Order"]
pub type DORD_R = crate::BitReader;
#[doc = "Field `DORD` writer - Data Order"]
pub type DORD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPE` reader - SPI Enable"]
pub type SPE_R = crate::BitReader;
#[doc = "Field `SPE` writer - SPI Enable"]
pub type SPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIE` reader - SPI Interrupt Enable"]
pub type SPIE_R = crate::BitReader;
#[doc = "Field `SPIE` writer - SPI Interrupt Enable"]
pub type SPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SPI Clock Rate Select 1 and 0"]
    #[inline(always)]
    pub fn spr(&self) -> SPR_R {
        SPR_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master/Slave Select"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Order"]
    #[inline(always)]
    pub fn dord(&self) -> DORD_R {
        DORD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI Enable"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Interrupt Enable"]
    #[inline(always)]
    pub fn spie(&self) -> SPIE_R {
        SPIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI Clock Rate Select 1 and 0"]
    #[inline(always)]
    #[must_use]
    pub fn spr(&mut self) -> SPR_W<SPCR_SPEC> {
        SPR_W::new(self, 0)
    }
    #[doc = "Bit 2 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<SPCR_SPEC> {
        CPHA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<SPCR_SPEC> {
        CPOL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Master/Slave Select"]
    #[inline(always)]
    #[must_use]
    pub fn mstr(&mut self) -> MSTR_W<SPCR_SPEC> {
        MSTR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Data Order"]
    #[inline(always)]
    #[must_use]
    pub fn dord(&mut self) -> DORD_W<SPCR_SPEC> {
        DORD_W::new(self, 5)
    }
    #[doc = "Bit 6 - SPI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SPE_W<SPCR_SPEC> {
        SPE_W::new(self, 6)
    }
    #[doc = "Bit 7 - SPI Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spie(&mut self) -> SPIE_W<SPCR_SPEC> {
        SPIE_W::new(self, 7)
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
#[doc = "SPI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPCR_SPEC;
impl crate::RegisterSpec for SPCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spcr::R`](R) reader structure"]
impl crate::Readable for SPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spcr::W`](W) writer structure"]
impl crate::Writable for SPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPCR to value 0"]
impl crate::Resettable for SPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
