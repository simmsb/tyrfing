#[doc = "Register `SCBTSRHL` reader"]
pub type R = crate::R<SCBTSRHL_SPEC>;
#[doc = "Register `SCBTSRHL` writer"]
pub type W = crate::W<SCBTSRHL_SPEC>;
#[doc = "Field `SCBTSRHL` reader - Symbol Counter Beacon Timestamp Register HL-Byte"]
pub type SCBTSRHL_R = crate::FieldReader;
#[doc = "Field `SCBTSRHL` writer - Symbol Counter Beacon Timestamp Register HL-Byte"]
pub type SCBTSRHL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Beacon Timestamp Register HL-Byte"]
    #[inline(always)]
    pub fn scbtsrhl(&self) -> SCBTSRHL_R {
        SCBTSRHL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Beacon Timestamp Register HL-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scbtsrhl(&mut self) -> SCBTSRHL_W<SCBTSRHL_SPEC> {
        SCBTSRHL_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Beacon Timestamp Register HL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scbtsrhl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scbtsrhl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCBTSRHL_SPEC;
impl crate::RegisterSpec for SCBTSRHL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scbtsrhl::R`](R) reader structure"]
impl crate::Readable for SCBTSRHL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scbtsrhl::W`](W) writer structure"]
impl crate::Writable for SCBTSRHL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCBTSRHL to value 0"]
impl crate::Resettable for SCBTSRHL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
