#[doc = "Register `SCTSRHH` reader"]
pub type R = crate::R<SCTSRHH_SPEC>;
#[doc = "Register `SCTSRHH` writer"]
pub type W = crate::W<SCTSRHH_SPEC>;
#[doc = "Field `SCTSRHH` reader - Symbol Counter Frame Timestamp Register HH-Byte"]
pub type SCTSRHH_R = crate::FieldReader;
#[doc = "Field `SCTSRHH` writer - Symbol Counter Frame Timestamp Register HH-Byte"]
pub type SCTSRHH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Frame Timestamp Register HH-Byte"]
    #[inline(always)]
    pub fn sctsrhh(&self) -> SCTSRHH_R {
        SCTSRHH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Frame Timestamp Register HH-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn sctsrhh(&mut self) -> SCTSRHH_W<SCTSRHH_SPEC> {
        SCTSRHH_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Frame Timestamp Register HH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctsrhh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctsrhh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCTSRHH_SPEC;
impl crate::RegisterSpec for SCTSRHH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sctsrhh::R`](R) reader structure"]
impl crate::Readable for SCTSRHH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sctsrhh::W`](W) writer structure"]
impl crate::Writable for SCTSRHH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTSRHH to value 0"]
impl crate::Resettable for SCTSRHH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
