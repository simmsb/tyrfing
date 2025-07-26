#[doc = "Register `SCBTSRLH` reader"]
pub type R = crate::R<SCBTSRLH_SPEC>;
#[doc = "Register `SCBTSRLH` writer"]
pub type W = crate::W<SCBTSRLH_SPEC>;
#[doc = "Field `SCBTSRLH` reader - Symbol Counter Beacon Timestamp Register LH-Byte"]
pub type SCBTSRLH_R = crate::FieldReader;
#[doc = "Field `SCBTSRLH` writer - Symbol Counter Beacon Timestamp Register LH-Byte"]
pub type SCBTSRLH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Beacon Timestamp Register LH-Byte"]
    #[inline(always)]
    pub fn scbtsrlh(&self) -> SCBTSRLH_R {
        SCBTSRLH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Beacon Timestamp Register LH-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scbtsrlh(&mut self) -> SCBTSRLH_W<SCBTSRLH_SPEC> {
        SCBTSRLH_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Beacon Timestamp Register LH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scbtsrlh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scbtsrlh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCBTSRLH_SPEC;
impl crate::RegisterSpec for SCBTSRLH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scbtsrlh::R`](R) reader structure"]
impl crate::Readable for SCBTSRLH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scbtsrlh::W`](W) writer structure"]
impl crate::Writable for SCBTSRLH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCBTSRLH to value 0"]
impl crate::Resettable for SCBTSRLH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
