#[doc = "Register `SCOCR3HL` reader"]
pub type R = crate::R<SCOCR3HL_SPEC>;
#[doc = "Register `SCOCR3HL` writer"]
pub type W = crate::W<SCOCR3HL_SPEC>;
#[doc = "Field `SCOCR3HL` reader - Symbol Counter Output Compare Register 3 HL-Byte"]
pub type SCOCR3HL_R = crate::FieldReader;
#[doc = "Field `SCOCR3HL` writer - Symbol Counter Output Compare Register 3 HL-Byte"]
pub type SCOCR3HL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 3 HL-Byte"]
    #[inline(always)]
    pub fn scocr3hl(&self) -> SCOCR3HL_R {
        SCOCR3HL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 3 HL-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scocr3hl(&mut self) -> SCOCR3HL_W<SCOCR3HL_SPEC> {
        SCOCR3HL_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Output Compare Register 3 HL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr3hl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr3hl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCOCR3HL_SPEC;
impl crate::RegisterSpec for SCOCR3HL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scocr3hl::R`](R) reader structure"]
impl crate::Readable for SCOCR3HL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scocr3hl::W`](W) writer structure"]
impl crate::Writable for SCOCR3HL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCOCR3HL to value 0"]
impl crate::Resettable for SCOCR3HL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
