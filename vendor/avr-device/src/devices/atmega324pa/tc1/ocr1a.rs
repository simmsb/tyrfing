#[doc = "Register `OCR1A` reader"]
pub type R = crate::R<OCR1A_SPEC>;
#[doc = "Register `OCR1A` writer"]
pub type W = crate::W<OCR1A_SPEC>;
#[doc = "Field `OCR1A` reader - Timer/Counter1 Output Compare A bits"]
pub type OCR1A_R = crate::FieldReader<u16>;
#[doc = "Field `OCR1A` writer - Timer/Counter1 Output Compare A bits"]
pub type OCR1A_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer/Counter1 Output Compare A bits"]
    #[inline(always)]
    pub fn ocr1a(&self) -> OCR1A_R {
        OCR1A_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer/Counter1 Output Compare A bits"]
    #[inline(always)]
    #[must_use]
    pub fn ocr1a(&mut self) -> OCR1A_W<OCR1A_SPEC> {
        OCR1A_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer/Counter1 Output Compare Register A Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr1a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr1a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCR1A_SPEC;
impl crate::RegisterSpec for OCR1A_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ocr1a::R`](R) reader structure"]
impl crate::Readable for OCR1A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocr1a::W`](W) writer structure"]
impl crate::Writable for OCR1A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCR1A to value 0"]
impl crate::Resettable for OCR1A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
