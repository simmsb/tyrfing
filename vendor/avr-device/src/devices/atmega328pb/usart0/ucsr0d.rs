#[doc = "Register `UCSR0D` reader"]
pub type R = crate::R<UCSR0D_SPEC>;
#[doc = "Register `UCSR0D` writer"]
pub type W = crate::W<UCSR0D_SPEC>;
#[doc = "Field `SFDE` reader - Start frame detection enable"]
pub type SFDE_R = crate::BitReader;
#[doc = "Field `SFDE` writer - Start frame detection enable"]
pub type SFDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXS` reader - USART RX Start"]
pub type RXS_R = crate::BitReader;
#[doc = "Field `RXS` writer - USART RX Start"]
pub type RXS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSIE` reader - USART RX Start Interrupt Enable"]
pub type RXSIE_R = crate::BitReader;
#[doc = "Field `RXSIE` writer - USART RX Start Interrupt Enable"]
pub type RXSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Start frame detection enable"]
    #[inline(always)]
    pub fn sfde(&self) -> SFDE_R {
        SFDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USART RX Start"]
    #[inline(always)]
    pub fn rxs(&self) -> RXS_R {
        RXS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USART RX Start Interrupt Enable"]
    #[inline(always)]
    pub fn rxsie(&self) -> RXSIE_R {
        RXSIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Start frame detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn sfde(&mut self) -> SFDE_W<UCSR0D_SPEC> {
        SFDE_W::new(self, 5)
    }
    #[doc = "Bit 6 - USART RX Start"]
    #[inline(always)]
    #[must_use]
    pub fn rxs(&mut self) -> RXS_W<UCSR0D_SPEC> {
        RXS_W::new(self, 6)
    }
    #[doc = "Bit 7 - USART RX Start Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxsie(&mut self) -> RXSIE_W<UCSR0D_SPEC> {
        RXSIE_W::new(self, 7)
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
