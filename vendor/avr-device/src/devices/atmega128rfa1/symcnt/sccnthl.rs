#[doc = "Register `SCCNTHL` reader"]
pub type R = crate::R<SCCNTHL_SPEC>;
#[doc = "Register `SCCNTHL` writer"]
pub type W = crate::W<SCCNTHL_SPEC>;
#[doc = "Field `SCCNTHL` reader - Symbol Counter Register HL-Byte"]
pub type SCCNTHL_R = crate::FieldReader;
#[doc = "Field `SCCNTHL` writer - Symbol Counter Register HL-Byte"]
pub type SCCNTHL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Register HL-Byte"]
    #[inline(always)]
    pub fn sccnthl(&self) -> SCCNTHL_R {
        SCCNTHL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Register HL-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn sccnthl(&mut self) -> SCCNTHL_W<SCCNTHL_SPEC> {
        SCCNTHL_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Register HL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sccnthl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccnthl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCCNTHL_SPEC;
impl crate::RegisterSpec for SCCNTHL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sccnthl::R`](R) reader structure"]
impl crate::Readable for SCCNTHL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sccnthl::W`](W) writer structure"]
impl crate::Writable for SCCNTHL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCCNTHL to value 0"]
impl crate::Resettable for SCCNTHL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
