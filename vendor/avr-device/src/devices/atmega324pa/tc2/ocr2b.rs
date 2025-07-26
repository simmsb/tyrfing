#[doc = "Register `OCR2B` reader"]
pub type R = crate::R<OCR2B_SPEC>;
#[doc = "Register `OCR2B` writer"]
pub type W = crate::W<OCR2B_SPEC>;
#[doc = "Field `OCR2B` reader - Timer/Counter2 Output Compare B bits"]
pub type OCR2B_R = crate::FieldReader;
#[doc = "Field `OCR2B` writer - Timer/Counter2 Output Compare B bits"]
pub type OCR2B_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timer/Counter2 Output Compare B bits"]
    #[inline(always)]
    pub fn ocr2b(&self) -> OCR2B_R {
        OCR2B_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer/Counter2 Output Compare B bits"]
    #[inline(always)]
    #[must_use]
    pub fn ocr2b(&mut self) -> OCR2B_W<OCR2B_SPEC> {
        OCR2B_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer/Counter2 Output Compare Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr2b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr2b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCR2B_SPEC;
impl crate::RegisterSpec for OCR2B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ocr2b::R`](R) reader structure"]
impl crate::Readable for OCR2B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocr2b::W`](W) writer structure"]
impl crate::Writable for OCR2B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCR2B to value 0"]
impl crate::Resettable for OCR2B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
