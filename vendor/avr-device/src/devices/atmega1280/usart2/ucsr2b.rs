#[doc = "Register `UCSR2B` reader"]
pub type R = crate::R<UCSR2B_SPEC>;
#[doc = "Register `UCSR2B` writer"]
pub type W = crate::W<UCSR2B_SPEC>;
#[doc = "Field `TXB82` reader - Transmit Data Bit 8"]
pub type TXB82_R = crate::BitReader;
#[doc = "Field `TXB82` writer - Transmit Data Bit 8"]
pub type TXB82_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXB82` reader - Receive Data Bit 8"]
pub type RXB82_R = crate::BitReader;
#[doc = "Field `UCSZ22` reader - Character Size"]
pub type UCSZ22_R = crate::BitReader;
#[doc = "Field `UCSZ22` writer - Character Size"]
pub type UCSZ22_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN2` reader - Transmitter Enable"]
pub type TXEN2_R = crate::BitReader;
#[doc = "Field `TXEN2` writer - Transmitter Enable"]
pub type TXEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN2` reader - Receiver Enable"]
pub type RXEN2_R = crate::BitReader;
#[doc = "Field `RXEN2` writer - Receiver Enable"]
pub type RXEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDRIE2` reader - USART Data register Empty Interrupt Enable"]
pub type UDRIE2_R = crate::BitReader;
#[doc = "Field `UDRIE2` writer - USART Data register Empty Interrupt Enable"]
pub type UDRIE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCIE2` reader - TX Complete Interrupt Enable"]
pub type TXCIE2_R = crate::BitReader;
#[doc = "Field `TXCIE2` writer - TX Complete Interrupt Enable"]
pub type TXCIE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCIE2` reader - RX Complete Interrupt Enable"]
pub type RXCIE2_R = crate::BitReader;
#[doc = "Field `RXCIE2` writer - RX Complete Interrupt Enable"]
pub type RXCIE2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Data Bit 8"]
    #[inline(always)]
    pub fn txb82(&self) -> TXB82_R {
        TXB82_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Data Bit 8"]
    #[inline(always)]
    pub fn rxb82(&self) -> RXB82_R {
        RXB82_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Character Size"]
    #[inline(always)]
    pub fn ucsz22(&self) -> UCSZ22_R {
        UCSZ22_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen2(&self) -> TXEN2_R {
        TXEN2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen2(&self) -> RXEN2_R {
        RXEN2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART Data register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn udrie2(&self) -> UDRIE2_R {
        UDRIE2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Complete Interrupt Enable"]
    #[inline(always)]
    pub fn txcie2(&self) -> TXCIE2_R {
        TXCIE2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Complete Interrupt Enable"]
    #[inline(always)]
    pub fn rxcie2(&self) -> RXCIE2_R {
        RXCIE2_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn txb82(&mut self) -> TXB82_W<UCSR2B_SPEC> {
        TXB82_W::new(self, 0)
    }
    #[doc = "Bit 2 - Character Size"]
    #[inline(always)]
    #[must_use]
    pub fn ucsz22(&mut self) -> UCSZ22_W<UCSR2B_SPEC> {
        UCSZ22_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen2(&mut self) -> TXEN2_W<UCSR2B_SPEC> {
        TXEN2_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen2(&mut self) -> RXEN2_W<UCSR2B_SPEC> {
        RXEN2_W::new(self, 4)
    }
    #[doc = "Bit 5 - USART Data register Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn udrie2(&mut self) -> UDRIE2_W<UCSR2B_SPEC> {
        UDRIE2_W::new(self, 5)
    }
    #[doc = "Bit 6 - TX Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txcie2(&mut self) -> TXCIE2_W<UCSR2B_SPEC> {
        TXCIE2_W::new(self, 6)
    }
    #[doc = "Bit 7 - RX Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxcie2(&mut self) -> RXCIE2_W<UCSR2B_SPEC> {
        RXCIE2_W::new(self, 7)
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
#[doc = "USART Control and Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr2b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr2b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSR2B_SPEC;
impl crate::RegisterSpec for UCSR2B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucsr2b::R`](R) reader structure"]
impl crate::Readable for UCSR2B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsr2b::W`](W) writer structure"]
impl crate::Writable for UCSR2B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR2B to value 0"]
impl crate::Resettable for UCSR2B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
