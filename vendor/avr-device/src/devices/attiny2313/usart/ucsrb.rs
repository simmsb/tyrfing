#[doc = "Register `UCSRB` reader"]
pub type R = crate::R<UCSRB_SPEC>;
#[doc = "Register `UCSRB` writer"]
pub type W = crate::W<UCSRB_SPEC>;
#[doc = "Field `TXB8` reader - Transmit Data Bit 8"]
pub type TXB8_R = crate::BitReader;
#[doc = "Field `TXB8` writer - Transmit Data Bit 8"]
pub type TXB8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXB8` reader - Receive Data Bit 8"]
pub type RXB8_R = crate::BitReader;
#[doc = "Field `UCSZ2` reader - Character Size"]
pub type UCSZ2_R = crate::BitReader;
#[doc = "Field `UCSZ2` writer - Character Size"]
pub type UCSZ2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` reader - Transmitter Enable"]
pub type TXEN_R = crate::BitReader;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` reader - Receiver Enable"]
pub type RXEN_R = crate::BitReader;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDRIE` reader - USART Data register Empty Interrupt Enable"]
pub type UDRIE_R = crate::BitReader;
#[doc = "Field `UDRIE` writer - USART Data register Empty Interrupt Enable"]
pub type UDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCIE` reader - TX Complete Interrupt Enable"]
pub type TXCIE_R = crate::BitReader;
#[doc = "Field `TXCIE` writer - TX Complete Interrupt Enable"]
pub type TXCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCIE` reader - RX Complete Interrupt Enable"]
pub type RXCIE_R = crate::BitReader;
#[doc = "Field `RXCIE` writer - RX Complete Interrupt Enable"]
pub type RXCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Data Bit 8"]
    #[inline(always)]
    pub fn txb8(&self) -> TXB8_R {
        TXB8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Data Bit 8"]
    #[inline(always)]
    pub fn rxb8(&self) -> RXB8_R {
        RXB8_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Character Size"]
    #[inline(always)]
    pub fn ucsz2(&self) -> UCSZ2_R {
        UCSZ2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART Data register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn udrie(&self) -> UDRIE_R {
        UDRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Complete Interrupt Enable"]
    #[inline(always)]
    pub fn txcie(&self) -> TXCIE_R {
        TXCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Complete Interrupt Enable"]
    #[inline(always)]
    pub fn rxcie(&self) -> RXCIE_R {
        RXCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn txb8(&mut self) -> TXB8_W<UCSRB_SPEC> {
        TXB8_W::new(self, 0)
    }
    #[doc = "Bit 2 - Character Size"]
    #[inline(always)]
    #[must_use]
    pub fn ucsz2(&mut self) -> UCSZ2_W<UCSRB_SPEC> {
        UCSZ2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<UCSRB_SPEC> {
        TXEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<UCSRB_SPEC> {
        RXEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - USART Data register Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn udrie(&mut self) -> UDRIE_W<UCSRB_SPEC> {
        UDRIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - TX Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txcie(&mut self) -> TXCIE_W<UCSRB_SPEC> {
        TXCIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - RX Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxcie(&mut self) -> RXCIE_W<UCSRB_SPEC> {
        RXCIE_W::new(self, 7)
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
#[doc = "USART Control and Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsrb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsrb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSRB_SPEC;
impl crate::RegisterSpec for UCSRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucsrb::R`](R) reader structure"]
impl crate::Readable for UCSRB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsrb::W`](W) writer structure"]
impl crate::Writable for UCSRB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSRB to value 0"]
impl crate::Resettable for UCSRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
