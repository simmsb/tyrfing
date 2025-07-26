#[doc = "Register `UBRR0` reader"]
pub type R = crate::R<UBRR0_SPEC>;
#[doc = "Register `UBRR0` writer"]
pub type W = crate::W<UBRR0_SPEC>;
#[doc = "Field `UBRR0` reader - USART Baud Rate Register"]
pub type UBRR0_R = crate::FieldReader<u16>;
#[doc = "Field `UBRR0` writer - USART Baud Rate Register"]
pub type UBRR0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - USART Baud Rate Register"]
    #[inline(always)]
    pub fn ubrr0(&self) -> UBRR0_R {
        UBRR0_R::new(self.bits & 0x0fff)
    }
}
impl W {
    #[doc = "Bits 0:11 - USART Baud Rate Register"]
    #[inline(always)]
    #[must_use]
    pub fn ubrr0(&mut self) -> UBRR0_W<UBRR0_SPEC> {
        UBRR0_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USART Baud Rate Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ubrr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ubrr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UBRR0_SPEC;
impl crate::RegisterSpec for UBRR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ubrr0::R`](R) reader structure"]
impl crate::Readable for UBRR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ubrr0::W`](W) writer structure"]
impl crate::Writable for UBRR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UBRR0 to value 0"]
impl crate::Resettable for UBRR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
