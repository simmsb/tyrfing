#[doc = "Register `UCSR1D` reader"]
pub type R = crate::R<UCSR1D_SPEC>;
#[doc = "Register `UCSR1D` writer"]
pub type W = crate::W<UCSR1D_SPEC>;
#[doc = "Field `SFDE1` reader - USART RX Start Frame Detection Enable"]
pub type SFDE1_R = crate::BitReader;
#[doc = "Field `SFDE1` writer - USART RX Start Frame Detection Enable"]
pub type SFDE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXS1` reader - USART RX Start Flag"]
pub type RXS1_R = crate::BitReader;
#[doc = "Field `RXS1` writer - USART RX Start Flag"]
pub type RXS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSIE1` reader - USART RX Start Interrupt Enable"]
pub type RXSIE1_R = crate::BitReader;
#[doc = "Field `RXSIE1` writer - USART RX Start Interrupt Enable"]
pub type RXSIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - USART RX Start Frame Detection Enable"]
    #[inline(always)]
    pub fn sfde1(&self) -> SFDE1_R {
        SFDE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USART RX Start Flag"]
    #[inline(always)]
    pub fn rxs1(&self) -> RXS1_R {
        RXS1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USART RX Start Interrupt Enable"]
    #[inline(always)]
    pub fn rxsie1(&self) -> RXSIE1_R {
        RXSIE1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - USART RX Start Frame Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sfde1(&mut self) -> SFDE1_W<UCSR1D_SPEC> {
        SFDE1_W::new(self, 5)
    }
    #[doc = "Bit 6 - USART RX Start Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxs1(&mut self) -> RXS1_W<UCSR1D_SPEC> {
        RXS1_W::new(self, 6)
    }
    #[doc = "Bit 7 - USART RX Start Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxsie1(&mut self) -> RXSIE1_W<UCSR1D_SPEC> {
        RXSIE1_W::new(self, 7)
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
#[doc = "USART Control and Status Register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr1d::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr1d::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSR1D_SPEC;
impl crate::RegisterSpec for UCSR1D_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucsr1d::R`](R) reader structure"]
impl crate::Readable for UCSR1D_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsr1d::W`](W) writer structure"]
impl crate::Writable for UCSR1D_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR1D to value 0"]
impl crate::Resettable for UCSR1D_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
