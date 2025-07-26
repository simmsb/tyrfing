#[doc = "Register `SCOCR1LH` reader"]
pub type R = crate::R<SCOCR1LH_SPEC>;
#[doc = "Register `SCOCR1LH` writer"]
pub type W = crate::W<SCOCR1LH_SPEC>;
#[doc = "Field `SCOCR1LH` reader - Symbol Counter Output Compare Register 1 LH-Byte"]
pub type SCOCR1LH_R = crate::FieldReader;
#[doc = "Field `SCOCR1LH` writer - Symbol Counter Output Compare Register 1 LH-Byte"]
pub type SCOCR1LH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 1 LH-Byte"]
    #[inline(always)]
    pub fn scocr1lh(&self) -> SCOCR1LH_R {
        SCOCR1LH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 1 LH-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scocr1lh(&mut self) -> SCOCR1LH_W<SCOCR1LH_SPEC> {
        SCOCR1LH_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Output Compare Register 1 LH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr1lh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr1lh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCOCR1LH_SPEC;
impl crate::RegisterSpec for SCOCR1LH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scocr1lh::R`](R) reader structure"]
impl crate::Readable for SCOCR1LH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scocr1lh::W`](W) writer structure"]
impl crate::Writable for SCOCR1LH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCOCR1LH to value 0"]
impl crate::Resettable for SCOCR1LH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
