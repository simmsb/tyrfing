#[doc = "Register `DACREF` reader"]
pub type R = crate::R<DACREF_SPEC>;
#[doc = "Register `DACREF` writer"]
pub type W = crate::W<DACREF_SPEC>;
#[doc = "Field `DATA` reader - DAC voltage reference"]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `DATA` writer - DAC voltage reference"]
pub type DATA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC voltage reference"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC voltage reference"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<DACREF_SPEC> {
        DATA_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Referance scale control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacref::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacref::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DACREF_SPEC;
impl crate::RegisterSpec for DACREF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dacref::R`](R) reader structure"]
impl crate::Readable for DACREF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dacref::W`](W) writer structure"]
impl crate::Writable for DACREF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACREF to value 0"]
impl crate::Resettable for DACREF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
