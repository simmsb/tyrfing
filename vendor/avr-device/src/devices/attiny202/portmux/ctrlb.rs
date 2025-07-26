#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `USART0` reader - Port Multiplexer USART0"]
pub type USART0_R = crate::BitReader<USART0_A>;
#[doc = "Port Multiplexer USART0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART0_A {
    #[doc = "0: Default pins"]
    DEFAULT = 0,
    #[doc = "1: Alternate pins"]
    ALTERNATE = 1,
}
impl From<USART0_A> for bool {
    #[inline(always)]
    fn from(variant: USART0_A) -> Self {
        variant as u8 != 0
    }
}
impl USART0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART0_A {
        match self.bits {
            false => USART0_A::DEFAULT,
            true => USART0_A::ALTERNATE,
        }
    }
    #[doc = "Default pins"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == USART0_A::DEFAULT
    }
    #[doc = "Alternate pins"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == USART0_A::ALTERNATE
    }
}
#[doc = "Field `USART0` writer - Port Multiplexer USART0"]
pub type USART0_W<'a, REG> = crate::BitWriter<'a, REG, USART0_A>;
impl<'a, REG> USART0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default pins"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(USART0_A::DEFAULT)
    }
    #[doc = "Alternate pins"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(USART0_A::ALTERNATE)
    }
}
#[doc = "Field `SPI0` reader - Port Multiplexer SPI0"]
pub type SPI0_R = crate::BitReader<SPI0_A>;
#[doc = "Port Multiplexer SPI0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI0_A {
    #[doc = "0: Default pins"]
    DEFAULT = 0,
    #[doc = "1: Alternate pins"]
    ALTERNATE = 1,
}
impl From<SPI0_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI0_A {
        match self.bits {
            false => SPI0_A::DEFAULT,
            true => SPI0_A::ALTERNATE,
        }
    }
    #[doc = "Default pins"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == SPI0_A::DEFAULT
    }
    #[doc = "Alternate pins"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == SPI0_A::ALTERNATE
    }
}
#[doc = "Field `SPI0` writer - Port Multiplexer SPI0"]
pub type SPI0_W<'a, REG> = crate::BitWriter<'a, REG, SPI0_A>;
impl<'a, REG> SPI0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default pins"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(SPI0_A::DEFAULT)
    }
    #[doc = "Alternate pins"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(SPI0_A::ALTERNATE)
    }
}
impl R {
    #[doc = "Bit 0 - Port Multiplexer USART0"]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Port Multiplexer SPI0"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Multiplexer USART0"]
    #[inline(always)]
    #[must_use]
    pub fn usart0(&mut self) -> USART0_W<CTRLB_SPEC> {
        USART0_W::new(self, 0)
    }
    #[doc = "Bit 2 - Port Multiplexer SPI0"]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<CTRLB_SPEC> {
        SPI0_W::new(self, 2)
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
#[doc = "Port Multiplexer Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
