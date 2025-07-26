#[doc = "Register `UCSR1A` reader"]
pub type R = crate::R<UCSR1A_SPEC>;
#[doc = "Register `UCSR1A` writer"]
pub type W = crate::W<UCSR1A_SPEC>;
#[doc = "Field `UDRE1` reader - USART Data Register Empty"]
pub type UDRE1_R = crate::BitReader;
#[doc = "Field `UDRE1` writer - USART Data Register Empty"]
pub type UDRE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC1` reader - USART Transmit Complete"]
pub type TXC1_R = crate::BitReader;
#[doc = "Field `TXC1` writer - USART Transmit Complete"]
pub type TXC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXC1` reader - USART Receive Complete"]
pub type RXC1_R = crate::BitReader;
#[doc = "Field `RXC1` writer - USART Receive Complete"]
pub type RXC1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - USART Data Register Empty"]
    #[inline(always)]
    pub fn udre1(&self) -> UDRE1_R {
        UDRE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USART Transmit Complete"]
    #[inline(always)]
    pub fn txc1(&self) -> TXC1_R {
        TXC1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USART Receive Complete"]
    #[inline(always)]
    pub fn rxc1(&self) -> RXC1_R {
        RXC1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - USART Data Register Empty"]
    #[inline(always)]
    #[must_use]
    pub fn udre1(&mut self) -> UDRE1_W<UCSR1A_SPEC> {
        UDRE1_W::new(self, 5)
    }
    #[doc = "Bit 6 - USART Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn txc1(&mut self) -> TXC1_W<UCSR1A_SPEC> {
        TXC1_W::new(self, 6)
    }
    #[doc = "Bit 7 - USART Receive Complete"]
    #[inline(always)]
    #[must_use]
    pub fn rxc1(&mut self) -> RXC1_W<UCSR1A_SPEC> {
        RXC1_W::new(self, 7)
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
#[doc = "USART1 MSPIM Control and Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr1a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr1a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSR1A_SPEC;
impl crate::RegisterSpec for UCSR1A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucsr1a::R`](R) reader structure"]
impl crate::Readable for UCSR1A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsr1a::W`](W) writer structure"]
impl crate::Writable for UCSR1A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR1A to value 0"]
impl crate::Resettable for UCSR1A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
