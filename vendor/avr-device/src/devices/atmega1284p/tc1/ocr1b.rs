#[doc = "Register `OCR1B` reader"]
pub type R = crate::R<OCR1B_SPEC>;
#[doc = "Register `OCR1B` writer"]
pub type W = crate::W<OCR1B_SPEC>;
#[doc = "Field `OCR1B` reader - Timer/Counter1 Output Compare B bits"]
pub type OCR1B_R = crate::FieldReader<u16>;
#[doc = "Field `OCR1B` writer - Timer/Counter1 Output Compare B bits"]
pub type OCR1B_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer/Counter1 Output Compare B bits"]
    #[inline(always)]
    pub fn ocr1b(&self) -> OCR1B_R {
        OCR1B_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer/Counter1 Output Compare B bits"]
    #[inline(always)]
    #[must_use]
    pub fn ocr1b(&mut self) -> OCR1B_W<OCR1B_SPEC> {
        OCR1B_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer/Counter1 Output Compare Register B Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr1b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr1b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCR1B_SPEC;
impl crate::RegisterSpec for OCR1B_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ocr1b::R`](R) reader structure"]
impl crate::Readable for OCR1B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocr1b::W`](W) writer structure"]
impl crate::Writable for OCR1B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCR1B to value 0"]
impl crate::Resettable for OCR1B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
