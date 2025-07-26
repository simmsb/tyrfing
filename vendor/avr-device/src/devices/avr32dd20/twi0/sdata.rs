#[doc = "Register `SDATA` reader"]
pub type R = crate::R<SDATA_SPEC>;
#[doc = "Register `SDATA` writer"]
pub type W = crate::W<SDATA_SPEC>;
#[doc = "Field `DATA` reader - Data"]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `DATA` writer - Data"]
pub type DATA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<SDATA_SPEC> {
        DATA_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Client Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDATA_SPEC;
impl crate::RegisterSpec for SDATA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sdata::R`](R) reader structure"]
impl crate::Readable for SDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdata::W`](W) writer structure"]
impl crate::Writable for SDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDATA to value 0"]
impl crate::Resettable for SDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
