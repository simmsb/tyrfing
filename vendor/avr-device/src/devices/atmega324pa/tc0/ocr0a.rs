#[doc = "Register `OCR0A` reader"]
pub type R = crate::R<OCR0A_SPEC>;
#[doc = "Register `OCR0A` writer"]
pub type W = crate::W<OCR0A_SPEC>;
#[doc = "Field `OCR0A` reader - Timer/Counter0 Output Compare A bits"]
pub type OCR0A_R = crate::FieldReader;
#[doc = "Field `OCR0A` writer - Timer/Counter0 Output Compare A bits"]
pub type OCR0A_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timer/Counter0 Output Compare A bits"]
    #[inline(always)]
    pub fn ocr0a(&self) -> OCR0A_R {
        OCR0A_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer/Counter0 Output Compare A bits"]
    #[inline(always)]
    #[must_use]
    pub fn ocr0a(&mut self) -> OCR0A_W<OCR0A_SPEC> {
        OCR0A_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer/Counter0 Output Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr0a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr0a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCR0A_SPEC;
impl crate::RegisterSpec for OCR0A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ocr0a::R`](R) reader structure"]
impl crate::Readable for OCR0A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocr0a::W`](W) writer structure"]
impl crate::Writable for OCR0A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCR0A to value 0"]
impl crate::Resettable for OCR0A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
