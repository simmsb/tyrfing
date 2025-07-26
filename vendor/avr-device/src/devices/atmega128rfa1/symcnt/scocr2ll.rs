#[doc = "Register `SCOCR2LL` reader"]
pub type R = crate::R<SCOCR2LL_SPEC>;
#[doc = "Register `SCOCR2LL` writer"]
pub type W = crate::W<SCOCR2LL_SPEC>;
#[doc = "Field `SCOCR2LL` reader - Symbol Counter Output Compare Register 2 LL-Byte"]
pub type SCOCR2LL_R = crate::FieldReader;
#[doc = "Field `SCOCR2LL` writer - Symbol Counter Output Compare Register 2 LL-Byte"]
pub type SCOCR2LL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 2 LL-Byte"]
    #[inline(always)]
    pub fn scocr2ll(&self) -> SCOCR2LL_R {
        SCOCR2LL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 2 LL-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scocr2ll(&mut self) -> SCOCR2LL_W<SCOCR2LL_SPEC> {
        SCOCR2LL_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Output Compare Register 2 LL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr2ll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr2ll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCOCR2LL_SPEC;
impl crate::RegisterSpec for SCOCR2LL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scocr2ll::R`](R) reader structure"]
impl crate::Readable for SCOCR2LL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scocr2ll::W`](W) writer structure"]
impl crate::Writable for SCOCR2LL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCOCR2LL to value 0"]
impl crate::Resettable for SCOCR2LL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
