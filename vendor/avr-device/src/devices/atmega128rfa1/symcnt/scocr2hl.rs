#[doc = "Register `SCOCR2HL` reader"]
pub type R = crate::R<SCOCR2HL_SPEC>;
#[doc = "Register `SCOCR2HL` writer"]
pub type W = crate::W<SCOCR2HL_SPEC>;
#[doc = "Field `SCOCR2HL` reader - Symbol Counter Output Compare Register 2 HL-Byte"]
pub type SCOCR2HL_R = crate::FieldReader;
#[doc = "Field `SCOCR2HL` writer - Symbol Counter Output Compare Register 2 HL-Byte"]
pub type SCOCR2HL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 2 HL-Byte"]
    #[inline(always)]
    pub fn scocr2hl(&self) -> SCOCR2HL_R {
        SCOCR2HL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 2 HL-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scocr2hl(&mut self) -> SCOCR2HL_W<SCOCR2HL_SPEC> {
        SCOCR2HL_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Output Compare Register 2 HL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr2hl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr2hl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCOCR2HL_SPEC;
impl crate::RegisterSpec for SCOCR2HL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scocr2hl::R`](R) reader structure"]
impl crate::Readable for SCOCR2HL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scocr2hl::W`](W) writer structure"]
impl crate::Writable for SCOCR2HL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCOCR2HL to value 0"]
impl crate::Resettable for SCOCR2HL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
