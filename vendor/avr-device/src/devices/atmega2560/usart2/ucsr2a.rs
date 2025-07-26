#[doc = "Register `UCSR2A` reader"]
pub type R = crate::R<UCSR2A_SPEC>;
#[doc = "Register `UCSR2A` writer"]
pub type W = crate::W<UCSR2A_SPEC>;
#[doc = "Field `MPCM2` reader - Multi-processor Communication Mode"]
pub type MPCM2_R = crate::BitReader;
#[doc = "Field `MPCM2` writer - Multi-processor Communication Mode"]
pub type MPCM2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `U2X2` reader - Double the USART transmission speed"]
pub type U2X2_R = crate::BitReader;
#[doc = "Field `U2X2` writer - Double the USART transmission speed"]
pub type U2X2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPE2` reader - Parity Error"]
pub type UPE2_R = crate::BitReader;
#[doc = "Field `DOR2` reader - Data overRun"]
pub type DOR2_R = crate::BitReader;
#[doc = "Field `FE2` reader - Framing Error"]
pub type FE2_R = crate::BitReader;
#[doc = "Field `UDRE2` reader - USART Data Register Empty"]
pub type UDRE2_R = crate::BitReader;
#[doc = "Field `TXC2` reader - USART Transmit Complete"]
pub type TXC2_R = crate::BitReader;
#[doc = "Field `TXC2` writer - USART Transmit Complete"]
pub type TXC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXC2` reader - USART Receive Complete"]
pub type RXC2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Multi-processor Communication Mode"]
    #[inline(always)]
    pub fn mpcm2(&self) -> MPCM2_R {
        MPCM2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Double the USART transmission speed"]
    #[inline(always)]
    pub fn u2x2(&self) -> U2X2_R {
        U2X2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error"]
    #[inline(always)]
    pub fn upe2(&self) -> UPE2_R {
        UPE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data overRun"]
    #[inline(always)]
    pub fn dor2(&self) -> DOR2_R {
        DOR2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Framing Error"]
    #[inline(always)]
    pub fn fe2(&self) -> FE2_R {
        FE2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART Data Register Empty"]
    #[inline(always)]
    pub fn udre2(&self) -> UDRE2_R {
        UDRE2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USART Transmit Complete"]
    #[inline(always)]
    pub fn txc2(&self) -> TXC2_R {
        TXC2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USART Receive Complete"]
    #[inline(always)]
    pub fn rxc2(&self) -> RXC2_R {
        RXC2_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Multi-processor Communication Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mpcm2(&mut self) -> MPCM2_W<UCSR2A_SPEC> {
        MPCM2_W::new(self, 0)
    }
    #[doc = "Bit 1 - Double the USART transmission speed"]
    #[inline(always)]
    #[must_use]
    pub fn u2x2(&mut self) -> U2X2_W<UCSR2A_SPEC> {
        U2X2_W::new(self, 1)
    }
    #[doc = "Bit 6 - USART Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn txc2(&mut self) -> TXC2_W<UCSR2A_SPEC> {
        TXC2_W::new(self, 6)
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
#[doc = "USART Control and Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr2a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr2a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSR2A_SPEC;
impl crate::RegisterSpec for UCSR2A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucsr2a::R`](R) reader structure"]
impl crate::Readable for UCSR2A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsr2a::W`](W) writer structure"]
impl crate::Writable for UCSR2A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR2A to value 0"]
impl crate::Resettable for UCSR2A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
