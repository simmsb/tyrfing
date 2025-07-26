#[doc = "Register `UCSR0A` reader"]
pub type R = crate::R<UCSR0A_SPEC>;
#[doc = "Register `UCSR0A` writer"]
pub type W = crate::W<UCSR0A_SPEC>;
#[doc = "Field `UDRE0` reader - USART Data Register Empty"]
pub type UDRE0_R = crate::BitReader;
#[doc = "Field `UDRE0` writer - USART Data Register Empty"]
pub type UDRE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC0` reader - USART Transmit Complete"]
pub type TXC0_R = crate::BitReader;
#[doc = "Field `TXC0` writer - USART Transmit Complete"]
pub type TXC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXC0` reader - USART Receive Complete"]
pub type RXC0_R = crate::BitReader;
#[doc = "Field `RXC0` writer - USART Receive Complete"]
pub type RXC0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
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
    #[doc = "Bit 5 - USART Data Register Empty"]
    #[inline(always)]
    #[must_use]
    pub fn udre0(&mut self) -> UDRE0_W<UCSR0A_SPEC> {
        UDRE0_W::new(self, 5)
    }
    #[doc = "Bit 6 - USART Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn txc0(&mut self) -> TXC0_W<UCSR0A_SPEC> {
        TXC0_W::new(self, 6)
    }
    #[doc = "Bit 7 - USART Receive Complete"]
    #[inline(always)]
    #[must_use]
    pub fn rxc0(&mut self) -> RXC0_W<UCSR0A_SPEC> {
        RXC0_W::new(self, 7)
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
#[doc = "USART0 MSPIM Control and Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr0a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr0a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
