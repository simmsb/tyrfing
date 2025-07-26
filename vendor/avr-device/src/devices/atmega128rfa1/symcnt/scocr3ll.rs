#[doc = "Register `SCOCR3LL` reader"]
pub type R = crate::R<SCOCR3LL_SPEC>;
#[doc = "Register `SCOCR3LL` writer"]
pub type W = crate::W<SCOCR3LL_SPEC>;
#[doc = "Field `SCOCR3LL` reader - Symbol Counter Output Compare Register 3 LL-Byte"]
pub type SCOCR3LL_R = crate::FieldReader;
#[doc = "Field `SCOCR3LL` writer - Symbol Counter Output Compare Register 3 LL-Byte"]
pub type SCOCR3LL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 3 LL-Byte"]
    #[inline(always)]
    pub fn scocr3ll(&self) -> SCOCR3LL_R {
        SCOCR3LL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 3 LL-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scocr3ll(&mut self) -> SCOCR3LL_W<SCOCR3LL_SPEC> {
        SCOCR3LL_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Output Compare Register 3 LL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr3ll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr3ll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCOCR3LL_SPEC;
impl crate::RegisterSpec for SCOCR3LL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scocr3ll::R`](R) reader structure"]
impl crate::Readable for SCOCR3LL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scocr3ll::W`](W) writer structure"]
impl crate::Writable for SCOCR3LL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCOCR3LL to value 0"]
impl crate::Resettable for SCOCR3LL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
