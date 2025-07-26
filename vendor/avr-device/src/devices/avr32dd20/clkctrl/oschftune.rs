#[doc = "Register `OSCHFTUNE` reader"]
pub type R = crate::R<OSCHFTUNE_SPEC>;
#[doc = "Register `OSCHFTUNE` writer"]
pub type W = crate::W<OSCHFTUNE_SPEC>;
#[doc = "Field `TUNE` reader - Tune"]
pub type TUNE_R = crate::FieldReader;
#[doc = "Field `TUNE` writer - Tune"]
pub type TUNE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Tune"]
    #[inline(always)]
    pub fn tune(&self) -> TUNE_R {
        TUNE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Tune"]
    #[inline(always)]
    #[must_use]
    pub fn tune(&mut self) -> TUNE_W<OSCHFTUNE_SPEC> {
        TUNE_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OSCHF Tune\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oschftune::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oschftune::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCHFTUNE_SPEC;
impl crate::RegisterSpec for OSCHFTUNE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`oschftune::R`](R) reader structure"]
impl crate::Readable for OSCHFTUNE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oschftune::W`](W) writer structure"]
impl crate::Writable for OSCHFTUNE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCHFTUNE to value 0"]
impl crate::Resettable for OSCHFTUNE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
