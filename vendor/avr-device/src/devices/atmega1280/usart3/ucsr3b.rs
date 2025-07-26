#[doc = "Register `UCSR3B` reader"]
pub type R = crate::R<UCSR3B_SPEC>;
#[doc = "Register `UCSR3B` writer"]
pub type W = crate::W<UCSR3B_SPEC>;
#[doc = "Field `TXB83` reader - Transmit Data Bit 8"]
pub type TXB83_R = crate::BitReader;
#[doc = "Field `TXB83` writer - Transmit Data Bit 8"]
pub type TXB83_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXB83` reader - Receive Data Bit 8"]
pub type RXB83_R = crate::BitReader;
#[doc = "Field `UCSZ32` reader - Character Size"]
pub type UCSZ32_R = crate::BitReader;
#[doc = "Field `UCSZ32` writer - Character Size"]
pub type UCSZ32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN3` reader - Transmitter Enable"]
pub type TXEN3_R = crate::BitReader;
#[doc = "Field `TXEN3` writer - Transmitter Enable"]
pub type TXEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN3` reader - Receiver Enable"]
pub type RXEN3_R = crate::BitReader;
#[doc = "Field `RXEN3` writer - Receiver Enable"]
pub type RXEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDRIE3` reader - USART Data register Empty Interrupt Enable"]
pub type UDRIE3_R = crate::BitReader;
#[doc = "Field `UDRIE3` writer - USART Data register Empty Interrupt Enable"]
pub type UDRIE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCIE3` reader - TX Complete Interrupt Enable"]
pub type TXCIE3_R = crate::BitReader;
#[doc = "Field `TXCIE3` writer - TX Complete Interrupt Enable"]
pub type TXCIE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCIE3` reader - RX Complete Interrupt Enable"]
pub type RXCIE3_R = crate::BitReader;
#[doc = "Field `RXCIE3` writer - RX Complete Interrupt Enable"]
pub type RXCIE3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Data Bit 8"]
    #[inline(always)]
    pub fn txb83(&self) -> TXB83_R {
        TXB83_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Data Bit 8"]
    #[inline(always)]
    pub fn rxb83(&self) -> RXB83_R {
        RXB83_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Character Size"]
    #[inline(always)]
    pub fn ucsz32(&self) -> UCSZ32_R {
        UCSZ32_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen3(&self) -> TXEN3_R {
        TXEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen3(&self) -> RXEN3_R {
        RXEN3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART Data register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn udrie3(&self) -> UDRIE3_R {
        UDRIE3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Complete Interrupt Enable"]
    #[inline(always)]
    pub fn txcie3(&self) -> TXCIE3_R {
        TXCIE3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Complete Interrupt Enable"]
    #[inline(always)]
    pub fn rxcie3(&self) -> RXCIE3_R {
        RXCIE3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn txb83(&mut self) -> TXB83_W<UCSR3B_SPEC> {
        TXB83_W::new(self, 0)
    }
    #[doc = "Bit 2 - Character Size"]
    #[inline(always)]
    #[must_use]
    pub fn ucsz32(&mut self) -> UCSZ32_W<UCSR3B_SPEC> {
        UCSZ32_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen3(&mut self) -> TXEN3_W<UCSR3B_SPEC> {
        TXEN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen3(&mut self) -> RXEN3_W<UCSR3B_SPEC> {
        RXEN3_W::new(self, 4)
    }
    #[doc = "Bit 5 - USART Data register Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn udrie3(&mut self) -> UDRIE3_W<UCSR3B_SPEC> {
        UDRIE3_W::new(self, 5)
    }
    #[doc = "Bit 6 - TX Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txcie3(&mut self) -> TXCIE3_W<UCSR3B_SPEC> {
        TXCIE3_W::new(self, 6)
    }
    #[doc = "Bit 7 - RX Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxcie3(&mut self) -> RXCIE3_W<UCSR3B_SPEC> {
        RXCIE3_W::new(self, 7)
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
#[doc = "USART Control and Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr3b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr3b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSR3B_SPEC;
impl crate::RegisterSpec for UCSR3B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucsr3b::R`](R) reader structure"]
impl crate::Readable for UCSR3B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsr3b::W`](W) writer structure"]
impl crate::Writable for UCSR3B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR3B to value 0"]
impl crate::Resettable for UCSR3B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
