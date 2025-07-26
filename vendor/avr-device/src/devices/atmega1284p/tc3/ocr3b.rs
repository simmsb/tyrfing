#[doc = "Register `OCR3B` reader"]
pub type R = crate::R<OCR3B_SPEC>;
#[doc = "Register `OCR3B` writer"]
pub type W = crate::W<OCR3B_SPEC>;
#[doc = "Field `OCR3B` reader - Timer/Counter3 Output Compare B bits"]
pub type OCR3B_R = crate::FieldReader<u16>;
#[doc = "Field `OCR3B` writer - Timer/Counter3 Output Compare B bits"]
pub type OCR3B_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer/Counter3 Output Compare B bits"]
    #[inline(always)]
    pub fn ocr3b(&self) -> OCR3B_R {
        OCR3B_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer/Counter3 Output Compare B bits"]
    #[inline(always)]
    #[must_use]
    pub fn ocr3b(&mut self) -> OCR3B_W<OCR3B_SPEC> {
        OCR3B_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer/Counter3 Output Compare Register B Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr3b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr3b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCR3B_SPEC;
impl crate::RegisterSpec for OCR3B_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ocr3b::R`](R) reader structure"]
impl crate::Readable for OCR3B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocr3b::W`](W) writer structure"]
impl crate::Writable for OCR3B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCR3B to value 0"]
impl crate::Resettable for OCR3B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
