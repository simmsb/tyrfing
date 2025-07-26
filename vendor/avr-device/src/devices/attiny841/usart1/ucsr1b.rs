#[doc = "Register `UCSR1B` reader"]
pub type R = crate::R<UCSR1B_SPEC>;
#[doc = "Register `UCSR1B` writer"]
pub type W = crate::W<UCSR1B_SPEC>;
#[doc = "Field `TXB81` reader - Transmit Data Bit 8"]
pub type TXB81_R = crate::BitReader;
#[doc = "Field `TXB81` writer - Transmit Data Bit 8"]
pub type TXB81_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXB81` reader - Receive Data Bit 8"]
pub type RXB81_R = crate::BitReader;
#[doc = "Field `UCSZ12` reader - Character Size"]
pub type UCSZ12_R = crate::BitReader;
#[doc = "Field `UCSZ12` writer - Character Size"]
pub type UCSZ12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN1` reader - Transmitter Enable"]
pub type TXEN1_R = crate::BitReader;
#[doc = "Field `TXEN1` writer - Transmitter Enable"]
pub type TXEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN1` reader - Receiver Enable"]
pub type RXEN1_R = crate::BitReader;
#[doc = "Field `RXEN1` writer - Receiver Enable"]
pub type RXEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDRIE1` reader - USART Data register Empty Interrupt Enable"]
pub type UDRIE1_R = crate::BitReader;
#[doc = "Field `UDRIE1` writer - USART Data register Empty Interrupt Enable"]
pub type UDRIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCIE1` reader - TX Complete Interrupt Enable"]
pub type TXCIE1_R = crate::BitReader;
#[doc = "Field `TXCIE1` writer - TX Complete Interrupt Enable"]
pub type TXCIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCIE1` reader - RX Complete Interrupt Enable"]
pub type RXCIE1_R = crate::BitReader;
#[doc = "Field `RXCIE1` writer - RX Complete Interrupt Enable"]
pub type RXCIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Data Bit 8"]
    #[inline(always)]
    pub fn txb81(&self) -> TXB81_R {
        TXB81_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Data Bit 8"]
    #[inline(always)]
    pub fn rxb81(&self) -> RXB81_R {
        RXB81_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Character Size"]
    #[inline(always)]
    pub fn ucsz12(&self) -> UCSZ12_R {
        UCSZ12_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen1(&self) -> TXEN1_R {
        TXEN1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen1(&self) -> RXEN1_R {
        RXEN1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART Data register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn udrie1(&self) -> UDRIE1_R {
        UDRIE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Complete Interrupt Enable"]
    #[inline(always)]
    pub fn txcie1(&self) -> TXCIE1_R {
        TXCIE1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Complete Interrupt Enable"]
    #[inline(always)]
    pub fn rxcie1(&self) -> RXCIE1_R {
        RXCIE1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn txb81(&mut self) -> TXB81_W<UCSR1B_SPEC> {
        TXB81_W::new(self, 0)
    }
    #[doc = "Bit 2 - Character Size"]
    #[inline(always)]
    #[must_use]
    pub fn ucsz12(&mut self) -> UCSZ12_W<UCSR1B_SPEC> {
        UCSZ12_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen1(&mut self) -> TXEN1_W<UCSR1B_SPEC> {
        TXEN1_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen1(&mut self) -> RXEN1_W<UCSR1B_SPEC> {
        RXEN1_W::new(self, 4)
    }
    #[doc = "Bit 5 - USART Data register Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn udrie1(&mut self) -> UDRIE1_W<UCSR1B_SPEC> {
        UDRIE1_W::new(self, 5)
    }
    #[doc = "Bit 6 - TX Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txcie1(&mut self) -> TXCIE1_W<UCSR1B_SPEC> {
        TXCIE1_W::new(self, 6)
    }
    #[doc = "Bit 7 - RX Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxcie1(&mut self) -> RXCIE1_W<UCSR1B_SPEC> {
        RXCIE1_W::new(self, 7)
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
#[doc = "USART Control and Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr1b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr1b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSR1B_SPEC;
impl crate::RegisterSpec for UCSR1B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucsr1b::R`](R) reader structure"]
impl crate::Readable for UCSR1B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsr1b::W`](W) writer structure"]
impl crate::Writable for UCSR1B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR1B to value 0"]
impl crate::Resettable for UCSR1B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
