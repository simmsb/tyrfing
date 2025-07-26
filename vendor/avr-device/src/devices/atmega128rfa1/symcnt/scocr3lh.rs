#[doc = "Register `SCOCR3LH` reader"]
pub type R = crate::R<SCOCR3LH_SPEC>;
#[doc = "Register `SCOCR3LH` writer"]
pub type W = crate::W<SCOCR3LH_SPEC>;
#[doc = "Field `SCOCR3LH` reader - Symbol Counter Output Compare Register 3 LH-Byte"]
pub type SCOCR3LH_R = crate::FieldReader;
#[doc = "Field `SCOCR3LH` writer - Symbol Counter Output Compare Register 3 LH-Byte"]
pub type SCOCR3LH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 3 LH-Byte"]
    #[inline(always)]
    pub fn scocr3lh(&self) -> SCOCR3LH_R {
        SCOCR3LH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 3 LH-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scocr3lh(&mut self) -> SCOCR3LH_W<SCOCR3LH_SPEC> {
        SCOCR3LH_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Output Compare Register 3 LH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr3lh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr3lh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCOCR3LH_SPEC;
impl crate::RegisterSpec for SCOCR3LH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scocr3lh::R`](R) reader structure"]
impl crate::Readable for SCOCR3LH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scocr3lh::W`](W) writer structure"]
impl crate::Writable for SCOCR3LH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCOCR3LH to value 0"]
impl crate::Resettable for SCOCR3LH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
