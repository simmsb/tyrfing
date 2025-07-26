#[doc = "Register `PAN_ID_1` reader"]
pub type R = crate::R<PAN_ID_1_SPEC>;
#[doc = "Register `PAN_ID_1` writer"]
pub type W = crate::W<PAN_ID_1_SPEC>;
#[doc = "Field `PAN_ID_` reader - MAC Personal Area Network ID"]
pub type PAN_ID__R = crate::FieldReader;
#[doc = "Field `PAN_ID_` writer - MAC Personal Area Network ID"]
pub type PAN_ID__W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - MAC Personal Area Network ID"]
    #[inline(always)]
    pub fn pan_id_(&self) -> PAN_ID__R {
        PAN_ID__R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - MAC Personal Area Network ID"]
    #[inline(always)]
    #[must_use]
    pub fn pan_id_(&mut self) -> PAN_ID__W<PAN_ID_1_SPEC> {
        PAN_ID__W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transceiver Personal Area Network ID Register (High Byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pan_id_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pan_id_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAN_ID_1_SPEC;
impl crate::RegisterSpec for PAN_ID_1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pan_id_1::R`](R) reader structure"]
impl crate::Readable for PAN_ID_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pan_id_1::W`](W) writer structure"]
impl crate::Writable for PAN_ID_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAN_ID_1 to value 0"]
impl crate::Resettable for PAN_ID_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
