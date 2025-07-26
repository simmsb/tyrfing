#[doc = "Register `UCSR3A` reader"]
pub type R = crate::R<UCSR3A_SPEC>;
#[doc = "Register `UCSR3A` writer"]
pub type W = crate::W<UCSR3A_SPEC>;
#[doc = "Field `MPCM3` reader - Multi-processor Communication Mode"]
pub type MPCM3_R = crate::BitReader;
#[doc = "Field `MPCM3` writer - Multi-processor Communication Mode"]
pub type MPCM3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `U2X3` reader - Double the USART transmission speed"]
pub type U2X3_R = crate::BitReader;
#[doc = "Field `U2X3` writer - Double the USART transmission speed"]
pub type U2X3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPE3` reader - Parity Error"]
pub type UPE3_R = crate::BitReader;
#[doc = "Field `DOR3` reader - Data overRun"]
pub type DOR3_R = crate::BitReader;
#[doc = "Field `FE3` reader - Framing Error"]
pub type FE3_R = crate::BitReader;
#[doc = "Field `UDRE3` reader - USART Data Register Empty"]
pub type UDRE3_R = crate::BitReader;
#[doc = "Field `TXC3` reader - USART Transmit Complete"]
pub type TXC3_R = crate::BitReader;
#[doc = "Field `TXC3` writer - USART Transmit Complete"]
pub type TXC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXC3` reader - USART Receive Complete"]
pub type RXC3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Multi-processor Communication Mode"]
    #[inline(always)]
    pub fn mpcm3(&self) -> MPCM3_R {
        MPCM3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Double the USART transmission speed"]
    #[inline(always)]
    pub fn u2x3(&self) -> U2X3_R {
        U2X3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error"]
    #[inline(always)]
    pub fn upe3(&self) -> UPE3_R {
        UPE3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data overRun"]
    #[inline(always)]
    pub fn dor3(&self) -> DOR3_R {
        DOR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Framing Error"]
    #[inline(always)]
    pub fn fe3(&self) -> FE3_R {
        FE3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART Data Register Empty"]
    #[inline(always)]
    pub fn udre3(&self) -> UDRE3_R {
        UDRE3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USART Transmit Complete"]
    #[inline(always)]
    pub fn txc3(&self) -> TXC3_R {
        TXC3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USART Receive Complete"]
    #[inline(always)]
    pub fn rxc3(&self) -> RXC3_R {
        RXC3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Multi-processor Communication Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mpcm3(&mut self) -> MPCM3_W<UCSR3A_SPEC> {
        MPCM3_W::new(self, 0)
    }
    #[doc = "Bit 1 - Double the USART transmission speed"]
    #[inline(always)]
    #[must_use]
    pub fn u2x3(&mut self) -> U2X3_W<UCSR3A_SPEC> {
        U2X3_W::new(self, 1)
    }
    #[doc = "Bit 6 - USART Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn txc3(&mut self) -> TXC3_W<UCSR3A_SPEC> {
        TXC3_W::new(self, 6)
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
#[doc = "USART Control and Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr3a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr3a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSR3A_SPEC;
impl crate::RegisterSpec for UCSR3A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucsr3a::R`](R) reader structure"]
impl crate::Readable for UCSR3A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsr3a::W`](W) writer structure"]
impl crate::Writable for UCSR3A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR3A to value 0"]
impl crate::Resettable for UCSR3A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
