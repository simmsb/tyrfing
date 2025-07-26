#[doc = "Register `TXPLCTRL` reader"]
pub type R = crate::R<TXPLCTRL_SPEC>;
#[doc = "Register `TXPLCTRL` writer"]
pub type W = crate::W<TXPLCTRL_SPEC>;
#[doc = "Field `TXPL` reader - Transmit pulse length"]
pub type TXPL_R = crate::FieldReader;
#[doc = "Field `TXPL` writer - Transmit pulse length"]
pub type TXPL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit pulse length"]
    #[inline(always)]
    pub fn txpl(&self) -> TXPL_R {
        TXPL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit pulse length"]
    #[inline(always)]
    #[must_use]
    pub fn txpl(&mut self) -> TXPL_W<TXPLCTRL_SPEC> {
        TXPL_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "IRCOM Transmitter Pulse Length Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txplctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txplctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXPLCTRL_SPEC;
impl crate::RegisterSpec for TXPLCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txplctrl::R`](R) reader structure"]
impl crate::Readable for TXPLCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txplctrl::W`](W) writer structure"]
impl crate::Writable for TXPLCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXPLCTRL to value 0"]
impl crate::Resettable for TXPLCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
