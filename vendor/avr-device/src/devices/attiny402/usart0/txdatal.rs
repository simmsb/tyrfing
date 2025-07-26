#[doc = "Register `TXDATAL` reader"]
pub type R = crate::R<TXDATAL_SPEC>;
#[doc = "Register `TXDATAL` writer"]
pub type W = crate::W<TXDATAL_SPEC>;
#[doc = "Field `DATA` reader - Transmit Data Register"]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `DATA` writer - Transmit Data Register"]
pub type DATA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Data Register"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Data Register"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<TXDATAL_SPEC> {
        DATA_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit Data Low Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdatal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdatal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDATAL_SPEC;
impl crate::RegisterSpec for TXDATAL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txdatal::R`](R) reader structure"]
impl crate::Readable for TXDATAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txdatal::W`](W) writer structure"]
impl crate::Writable for TXDATAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDATAL to value 0"]
impl crate::Resettable for TXDATAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
