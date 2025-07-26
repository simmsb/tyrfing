#[doc = "Register `OCR3A` reader"]
pub type R = crate::R<OCR3A_SPEC>;
#[doc = "Register `OCR3A` writer"]
pub type W = crate::W<OCR3A_SPEC>;
#[doc = "Field `OCR3A` reader - Timer/Counter3 Output Compare A bits"]
pub type OCR3A_R = crate::FieldReader<u16>;
#[doc = "Field `OCR3A` writer - Timer/Counter3 Output Compare A bits"]
pub type OCR3A_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer/Counter3 Output Compare A bits"]
    #[inline(always)]
    pub fn ocr3a(&self) -> OCR3A_R {
        OCR3A_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer/Counter3 Output Compare A bits"]
    #[inline(always)]
    #[must_use]
    pub fn ocr3a(&mut self) -> OCR3A_W<OCR3A_SPEC> {
        OCR3A_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer/Counter3 Output Compare Register A Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr3a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr3a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCR3A_SPEC;
impl crate::RegisterSpec for OCR3A_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ocr3a::R`](R) reader structure"]
impl crate::Readable for OCR3A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocr3a::W`](W) writer structure"]
impl crate::Writable for OCR3A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCR3A to value 0"]
impl crate::Resettable for OCR3A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
