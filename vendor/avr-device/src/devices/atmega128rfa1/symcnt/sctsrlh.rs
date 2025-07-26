#[doc = "Register `SCTSRLH` reader"]
pub type R = crate::R<SCTSRLH_SPEC>;
#[doc = "Register `SCTSRLH` writer"]
pub type W = crate::W<SCTSRLH_SPEC>;
#[doc = "Field `SCTSRLH` reader - Symbol Counter Frame Timestamp Register LH-Byte"]
pub type SCTSRLH_R = crate::FieldReader;
#[doc = "Field `SCTSRLH` writer - Symbol Counter Frame Timestamp Register LH-Byte"]
pub type SCTSRLH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Frame Timestamp Register LH-Byte"]
    #[inline(always)]
    pub fn sctsrlh(&self) -> SCTSRLH_R {
        SCTSRLH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Frame Timestamp Register LH-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn sctsrlh(&mut self) -> SCTSRLH_W<SCTSRLH_SPEC> {
        SCTSRLH_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Frame Timestamp Register LH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctsrlh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctsrlh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCTSRLH_SPEC;
impl crate::RegisterSpec for SCTSRLH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sctsrlh::R`](R) reader structure"]
impl crate::Readable for SCTSRLH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sctsrlh::W`](W) writer structure"]
impl crate::Writable for SCTSRLH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTSRLH to value 0"]
impl crate::Resettable for SCTSRLH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
