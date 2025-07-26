#[doc = "Register `UCSR0D` reader"]
pub type R = crate::R<UCSR0D_SPEC>;
#[doc = "Register `UCSR0D` writer"]
pub type W = crate::W<UCSR0D_SPEC>;
#[doc = "Field `SFDE0` reader - USART RX Start Frame Detection Enable"]
pub type SFDE0_R = crate::BitReader;
#[doc = "Field `SFDE0` writer - USART RX Start Frame Detection Enable"]
pub type SFDE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXS0` reader - USART RX Start Flag"]
pub type RXS0_R = crate::BitReader;
#[doc = "Field `RXS0` writer - USART RX Start Flag"]
pub type RXS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSIE0` reader - USART RX Start Interrupt Enable"]
pub type RXSIE0_R = crate::BitReader;
#[doc = "Field `RXSIE0` writer - USART RX Start Interrupt Enable"]
pub type RXSIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - USART RX Start Frame Detection Enable"]
    #[inline(always)]
    pub fn sfde0(&self) -> SFDE0_R {
        SFDE0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USART RX Start Flag"]
    #[inline(always)]
    pub fn rxs0(&self) -> RXS0_R {
        RXS0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USART RX Start Interrupt Enable"]
    #[inline(always)]
    pub fn rxsie0(&self) -> RXSIE0_R {
        RXSIE0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - USART RX Start Frame Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sfde0(&mut self) -> SFDE0_W<UCSR0D_SPEC> {
        SFDE0_W::new(self, 5)
    }
    #[doc = "Bit 6 - USART RX Start Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxs0(&mut self) -> RXS0_W<UCSR0D_SPEC> {
        RXS0_W::new(self, 6)
    }
    #[doc = "Bit 7 - USART RX Start Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxsie0(&mut self) -> RXSIE0_W<UCSR0D_SPEC> {
        RXSIE0_W::new(self, 7)
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
#[doc = "USART Control and Status Register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr0d::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr0d::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSR0D_SPEC;
impl crate::RegisterSpec for UCSR0D_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucsr0d::R`](R) reader structure"]
impl crate::Readable for UCSR0D_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsr0d::W`](W) writer structure"]
impl crate::Writable for UCSR0D_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR0D to value 0"]
impl crate::Resettable for UCSR0D_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
