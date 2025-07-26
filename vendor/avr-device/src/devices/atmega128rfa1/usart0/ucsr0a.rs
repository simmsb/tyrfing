#[doc = "Register `UCSR0A` reader"]
pub type R = crate::R<UCSR0A_SPEC>;
#[doc = "Register `UCSR0A` writer"]
pub type W = crate::W<UCSR0A_SPEC>;
#[doc = "Field `MPCM0` reader - Multi-processor Communication Mode"]
pub type MPCM0_R = crate::BitReader;
#[doc = "Field `MPCM0` writer - Multi-processor Communication Mode"]
pub type MPCM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `U2X0` reader - Double the USART Transmission Speed"]
pub type U2X0_R = crate::BitReader;
#[doc = "Field `U2X0` writer - Double the USART Transmission Speed"]
pub type U2X0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPE0` reader - USART Parity Error"]
pub type UPE0_R = crate::BitReader;
#[doc = "Field `DOR0` reader - Data OverRun"]
pub type DOR0_R = crate::BitReader;
#[doc = "Field `FE0` reader - Frame Error"]
pub type FE0_R = crate::BitReader;
#[doc = "Field `UDRE0` reader - USART Data Register Empty"]
pub type UDRE0_R = crate::BitReader;
#[doc = "Field `TXC0` reader - USART Transmit Complete"]
pub type TXC0_R = crate::BitReader;
#[doc = "Field `TXC0` writer - USART Transmit Complete"]
pub type TXC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXC0` reader - USART Receive Complete"]
pub type RXC0_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Multi-processor Communication Mode"]
    #[inline(always)]
    pub fn mpcm0(&self) -> MPCM0_R {
        MPCM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Double the USART Transmission Speed"]
    #[inline(always)]
    pub fn u2x0(&self) -> U2X0_R {
        U2X0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USART Parity Error"]
    #[inline(always)]
    pub fn upe0(&self) -> UPE0_R {
        UPE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data OverRun"]
    #[inline(always)]
    pub fn dor0(&self) -> DOR0_R {
        DOR0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Frame Error"]
    #[inline(always)]
    pub fn fe0(&self) -> FE0_R {
        FE0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART Data Register Empty"]
    #[inline(always)]
    pub fn udre0(&self) -> UDRE0_R {
        UDRE0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USART Transmit Complete"]
    #[inline(always)]
    pub fn txc0(&self) -> TXC0_R {
        TXC0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USART Receive Complete"]
    #[inline(always)]
    pub fn rxc0(&self) -> RXC0_R {
        RXC0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Multi-processor Communication Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mpcm0(&mut self) -> MPCM0_W<UCSR0A_SPEC> {
        MPCM0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Double the USART Transmission Speed"]
    #[inline(always)]
    #[must_use]
    pub fn u2x0(&mut self) -> U2X0_W<UCSR0A_SPEC> {
        U2X0_W::new(self, 1)
    }
    #[doc = "Bit 6 - USART Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn txc0(&mut self) -> TXC0_W<UCSR0A_SPEC> {
        TXC0_W::new(self, 6)
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
#[doc = "USART0 Control and Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr0a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr0a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSR0A_SPEC;
impl crate::RegisterSpec for UCSR0A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucsr0a::R`](R) reader structure"]
impl crate::Readable for UCSR0A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsr0a::W`](W) writer structure"]
impl crate::Writable for UCSR0A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR0A to value 0"]
impl crate::Resettable for UCSR0A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
