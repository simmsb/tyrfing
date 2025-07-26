#[doc = "Register `OCR0B` reader"]
pub type R = crate::R<OCR0B_SPEC>;
#[doc = "Register `OCR0B` writer"]
pub type W = crate::W<OCR0B_SPEC>;
#[doc = "Field `OCR0B` reader - Timer/Counter0 Output Compare B bits"]
pub type OCR0B_R = crate::FieldReader;
#[doc = "Field `OCR0B` writer - Timer/Counter0 Output Compare B bits"]
pub type OCR0B_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timer/Counter0 Output Compare B bits"]
    #[inline(always)]
    pub fn ocr0b(&self) -> OCR0B_R {
        OCR0B_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer/Counter0 Output Compare B bits"]
    #[inline(always)]
    #[must_use]
    pub fn ocr0b(&mut self) -> OCR0B_W<OCR0B_SPEC> {
        OCR0B_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer/Counter0 Output Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr0b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr0b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCR0B_SPEC;
impl crate::RegisterSpec for OCR0B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ocr0b::R`](R) reader structure"]
impl crate::Readable for OCR0B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocr0b::W`](W) writer structure"]
impl crate::Writable for OCR0B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCR0B to value 0"]
impl crate::Resettable for OCR0B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
