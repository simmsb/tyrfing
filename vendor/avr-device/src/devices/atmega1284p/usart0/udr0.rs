#[doc = "Register `UDR0` reader"]
pub type R = crate::R<UDR0_SPEC>;
#[doc = "Register `UDR0` writer"]
pub type W = crate::W<UDR0_SPEC>;
#[doc = "Field `UDR0` reader - USART I/O Data bits"]
pub type UDR0_R = crate::FieldReader;
#[doc = "Field `UDR0` writer - USART I/O Data bits"]
pub type UDR0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - USART I/O Data bits"]
    #[inline(always)]
    pub fn udr0(&self) -> UDR0_R {
        UDR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USART I/O Data bits"]
    #[inline(always)]
    #[must_use]
    pub fn udr0(&mut self) -> UDR0_W<UDR0_SPEC> {
        UDR0_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USART I/O Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UDR0_SPEC;
impl crate::RegisterSpec for UDR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`udr0::R`](R) reader structure"]
impl crate::Readable for UDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`udr0::W`](W) writer structure"]
impl crate::Writable for UDR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDR0 to value 0"]
impl crate::Resettable for UDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
