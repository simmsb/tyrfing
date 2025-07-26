#[doc = "Register `UBRR1` reader"]
pub type R = crate::R<UBRR1_SPEC>;
#[doc = "Register `UBRR1` writer"]
pub type W = crate::W<UBRR1_SPEC>;
#[doc = "Field `UBRR1` reader - USART Baud Rate bits"]
pub type UBRR1_R = crate::FieldReader<u16>;
#[doc = "Field `UBRR1` writer - USART Baud Rate bits"]
pub type UBRR1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - USART Baud Rate bits"]
    #[inline(always)]
    pub fn ubrr1(&self) -> UBRR1_R {
        UBRR1_R::new(self.bits & 0x0fff)
    }
}
impl W {
    #[doc = "Bits 0:11 - USART Baud Rate bits"]
    #[inline(always)]
    #[must_use]
    pub fn ubrr1(&mut self) -> UBRR1_W<UBRR1_SPEC> {
        UBRR1_W::new(self, 0)
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
#[doc = "USART Baud Rate Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ubrr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ubrr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UBRR1_SPEC;
impl crate::RegisterSpec for UBRR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ubrr1::R`](R) reader structure"]
impl crate::Readable for UBRR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ubrr1::W`](W) writer structure"]
impl crate::Writable for UBRR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UBRR1 to value 0"]
impl crate::Resettable for UBRR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
