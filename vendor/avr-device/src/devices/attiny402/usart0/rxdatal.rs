#[doc = "Register `RXDATAL` reader"]
pub type R = crate::R<RXDATAL_SPEC>;
#[doc = "Field `DATA` reader - RX Data"]
pub type DATA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RX Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Receive Data Low Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdatal::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDATAL_SPEC;
impl crate::RegisterSpec for RXDATAL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxdatal::R`](R) reader structure"]
impl crate::Readable for RXDATAL_SPEC {}
#[doc = "`reset()` method sets RXDATAL to value 0"]
impl crate::Resettable for RXDATAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
