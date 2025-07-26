#[doc = "Register `SCCNTLL` reader"]
pub type R = crate::R<SCCNTLL_SPEC>;
#[doc = "Register `SCCNTLL` writer"]
pub type W = crate::W<SCCNTLL_SPEC>;
#[doc = "Field `SCCNTLL` reader - Symbol Counter Register LL-Byte"]
pub type SCCNTLL_R = crate::FieldReader;
#[doc = "Field `SCCNTLL` writer - Symbol Counter Register LL-Byte"]
pub type SCCNTLL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Register LL-Byte"]
    #[inline(always)]
    pub fn sccntll(&self) -> SCCNTLL_R {
        SCCNTLL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Register LL-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn sccntll(&mut self) -> SCCNTLL_W<SCCNTLL_SPEC> {
        SCCNTLL_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Register LL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sccntll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccntll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCCNTLL_SPEC;
impl crate::RegisterSpec for SCCNTLL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sccntll::R`](R) reader structure"]
impl crate::Readable for SCCNTLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sccntll::W`](W) writer structure"]
impl crate::Writable for SCCNTLL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCCNTLL to value 0"]
impl crate::Resettable for SCCNTLL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
