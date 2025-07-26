#[doc = "Register `SCOCR1LL` reader"]
pub type R = crate::R<SCOCR1LL_SPEC>;
#[doc = "Register `SCOCR1LL` writer"]
pub type W = crate::W<SCOCR1LL_SPEC>;
#[doc = "Field `SCOCR1LL` reader - Symbol Counter Output Compare Register 1 LL-Byte"]
pub type SCOCR1LL_R = crate::FieldReader;
#[doc = "Field `SCOCR1LL` writer - Symbol Counter Output Compare Register 1 LL-Byte"]
pub type SCOCR1LL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 1 LL-Byte"]
    #[inline(always)]
    pub fn scocr1ll(&self) -> SCOCR1LL_R {
        SCOCR1LL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 1 LL-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scocr1ll(&mut self) -> SCOCR1LL_W<SCOCR1LL_SPEC> {
        SCOCR1LL_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Output Compare Register 1 LL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr1ll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr1ll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCOCR1LL_SPEC;
impl crate::RegisterSpec for SCOCR1LL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scocr1ll::R`](R) reader structure"]
impl crate::Readable for SCOCR1LL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scocr1ll::W`](W) writer structure"]
impl crate::Writable for SCOCR1LL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCOCR1LL to value 0"]
impl crate::Resettable for SCOCR1LL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
