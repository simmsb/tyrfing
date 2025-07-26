#[doc = "Register `TST_RX_LENGTH` reader"]
pub type R = crate::R<TST_RX_LENGTH_SPEC>;
#[doc = "Register `TST_RX_LENGTH` writer"]
pub type W = crate::W<TST_RX_LENGTH_SPEC>;
#[doc = "Field `RX_LENGTH` reader - Received Frame Length"]
pub type RX_LENGTH_R = crate::FieldReader;
#[doc = "Field `RX_LENGTH` writer - Received Frame Length"]
pub type RX_LENGTH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Received Frame Length"]
    #[inline(always)]
    pub fn rx_length(&self) -> RX_LENGTH_R {
        RX_LENGTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Received Frame Length"]
    #[inline(always)]
    #[must_use]
    pub fn rx_length(&mut self) -> RX_LENGTH_W<TST_RX_LENGTH_SPEC> {
        RX_LENGTH_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transceiver Received Frame Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tst_rx_length::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tst_rx_length::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TST_RX_LENGTH_SPEC;
impl crate::RegisterSpec for TST_RX_LENGTH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tst_rx_length::R`](R) reader structure"]
impl crate::Readable for TST_RX_LENGTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tst_rx_length::W`](W) writer structure"]
impl crate::Writable for TST_RX_LENGTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TST_RX_LENGTH to value 0"]
impl crate::Resettable for TST_RX_LENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
