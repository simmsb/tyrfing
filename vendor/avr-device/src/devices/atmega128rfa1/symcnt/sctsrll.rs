#[doc = "Register `SCTSRLL` reader"]
pub type R = crate::R<SCTSRLL_SPEC>;
#[doc = "Register `SCTSRLL` writer"]
pub type W = crate::W<SCTSRLL_SPEC>;
#[doc = "Field `SCTSRLL` reader - Symbol Counter Frame Timestamp Register LL-Byte"]
pub type SCTSRLL_R = crate::FieldReader;
#[doc = "Field `SCTSRLL` writer - Symbol Counter Frame Timestamp Register LL-Byte"]
pub type SCTSRLL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Frame Timestamp Register LL-Byte"]
    #[inline(always)]
    pub fn sctsrll(&self) -> SCTSRLL_R {
        SCTSRLL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Frame Timestamp Register LL-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn sctsrll(&mut self) -> SCTSRLL_W<SCTSRLL_SPEC> {
        SCTSRLL_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Frame Timestamp Register LL-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctsrll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctsrll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCTSRLL_SPEC;
impl crate::RegisterSpec for SCTSRLL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sctsrll::R`](R) reader structure"]
impl crate::Readable for SCTSRLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sctsrll::W`](W) writer structure"]
impl crate::Writable for SCTSRLL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTSRLL to value 0"]
impl crate::Resettable for SCTSRLL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
