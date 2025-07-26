#[doc = "Register `SCTSRHL` reader"]
pub type R = crate::R<SCTSRHL_SPEC>;
#[doc = "Register `SCTSRHL` writer"]
pub type W = crate::W<SCTSRHL_SPEC>;
#[doc = "Field `SCTSRHL` reader - Symbol Counter Frame Timestamp Register HL-Byte"]
pub type SCTSRHL_R = crate::FieldReader;
#[doc = "Field `SCTSRHL` writer - Symbol Counter Frame Timestamp Register HL-Byte"]
pub type SCTSRHL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Frame Timestamp Register HL-Byte"]
    #[inline(always)]
    pub fn sctsrhl(&self) -> SCTSRHL_R {
        SCTSRHL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Frame Timestamp Register HL-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn sctsrhl(&mut self) -> SCTSRHL_W<SCTSRHL_SPEC> {
        SCTSRHL_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Frame Timestamp Register HL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctsrhl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctsrhl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCTSRHL_SPEC;
impl crate::RegisterSpec for SCTSRHL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sctsrhl::R`](R) reader structure"]
impl crate::Readable for SCTSRHL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sctsrhl::W`](W) writer structure"]
impl crate::Writable for SCTSRHL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTSRHL to value 0"]
impl crate::Resettable for SCTSRHL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
