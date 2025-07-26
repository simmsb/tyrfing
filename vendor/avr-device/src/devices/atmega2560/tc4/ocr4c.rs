#[doc = "Register `OCR4C` reader"]
pub type R = crate::R<OCR4C_SPEC>;
#[doc = "Register `OCR4C` writer"]
pub type W = crate::W<OCR4C_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<OCR4C_SPEC> {
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
#[doc = "Timer/Counter4 Output Compare Register B Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr4c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr4c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCR4C_SPEC;
impl crate::RegisterSpec for OCR4C_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ocr4c::R`](R) reader structure"]
impl crate::Readable for OCR4C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocr4c::W`](W) writer structure"]
impl crate::Writable for OCR4C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCR4C to value 0"]
impl crate::Resettable for OCR4C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
