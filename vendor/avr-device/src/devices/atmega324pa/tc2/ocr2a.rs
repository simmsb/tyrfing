#[doc = "Register `OCR2A` reader"]
pub type R = crate::R<OCR2A_SPEC>;
#[doc = "Register `OCR2A` writer"]
pub type W = crate::W<OCR2A_SPEC>;
#[doc = "Field `OCR2A` reader - Timer/Counter0 Output Compare A bits"]
pub type OCR2A_R = crate::FieldReader;
#[doc = "Field `OCR2A` writer - Timer/Counter0 Output Compare A bits"]
pub type OCR2A_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timer/Counter0 Output Compare A bits"]
    #[inline(always)]
    pub fn ocr2a(&self) -> OCR2A_R {
        OCR2A_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer/Counter0 Output Compare A bits"]
    #[inline(always)]
    #[must_use]
    pub fn ocr2a(&mut self) -> OCR2A_W<OCR2A_SPEC> {
        OCR2A_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer/Counter2 Output Compare Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr2a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr2a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCR2A_SPEC;
impl crate::RegisterSpec for OCR2A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ocr2a::R`](R) reader structure"]
impl crate::Readable for OCR2A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocr2a::W`](W) writer structure"]
impl crate::Writable for OCR2A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCR2A to value 0"]
impl crate::Resettable for OCR2A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
