#[doc = "Register `OCR4B` reader"]
pub type R = crate::R<OCR4B_SPEC>;
#[doc = "Register `OCR4B` writer"]
pub type W = crate::W<OCR4B_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<OCR4B_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer/Counter4 Output Compare Register B Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr4b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr4b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCR4B_SPEC;
impl crate::RegisterSpec for OCR4B_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ocr4b::R`](R) reader structure"]
impl crate::Readable for OCR4B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocr4b::W`](W) writer structure"]
impl crate::Writable for OCR4B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCR4B to value 0"]
impl crate::Resettable for OCR4B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
