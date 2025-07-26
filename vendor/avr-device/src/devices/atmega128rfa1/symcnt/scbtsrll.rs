#[doc = "Register `SCBTSRLL` reader"]
pub type R = crate::R<SCBTSRLL_SPEC>;
#[doc = "Register `SCBTSRLL` writer"]
pub type W = crate::W<SCBTSRLL_SPEC>;
#[doc = "Field `SCBTSRLL` reader - Symbol Counter Beacon Timestamp Register LL-Byte"]
pub type SCBTSRLL_R = crate::FieldReader;
#[doc = "Field `SCBTSRLL` writer - Symbol Counter Beacon Timestamp Register LL-Byte"]
pub type SCBTSRLL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Beacon Timestamp Register LL-Byte"]
    #[inline(always)]
    pub fn scbtsrll(&self) -> SCBTSRLL_R {
        SCBTSRLL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Beacon Timestamp Register LL-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scbtsrll(&mut self) -> SCBTSRLL_W<SCBTSRLL_SPEC> {
        SCBTSRLL_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Beacon Timestamp Register LL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scbtsrll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scbtsrll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCBTSRLL_SPEC;
impl crate::RegisterSpec for SCBTSRLL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scbtsrll::R`](R) reader structure"]
impl crate::Readable for SCBTSRLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scbtsrll::W`](W) writer structure"]
impl crate::Writable for SCBTSRLL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCBTSRLL to value 0"]
impl crate::Resettable for SCBTSRLL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
