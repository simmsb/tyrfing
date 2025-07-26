#[doc = "Register `CCA_THRES` reader"]
pub type R = crate::R<CCA_THRES_SPEC>;
#[doc = "Register `CCA_THRES` writer"]
pub type W = crate::W<CCA_THRES_SPEC>;
#[doc = "Field `CCA_ED_THRES` reader - ED Threshold Level for CCA Measurement"]
pub type CCA_ED_THRES_R = crate::FieldReader;
#[doc = "Field `CCA_ED_THRES` writer - ED Threshold Level for CCA Measurement"]
pub type CCA_ED_THRES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `CCA_CS_THRES` reader - CS Threshold Level for CCA Measurement"]
pub type CCA_CS_THRES_R = crate::FieldReader;
#[doc = "Field `CCA_CS_THRES` writer - CS Threshold Level for CCA Measurement"]
pub type CCA_CS_THRES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ED Threshold Level for CCA Measurement"]
    #[inline(always)]
    pub fn cca_ed_thres(&self) -> CCA_ED_THRES_R {
        CCA_ED_THRES_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - CS Threshold Level for CCA Measurement"]
    #[inline(always)]
    pub fn cca_cs_thres(&self) -> CCA_CS_THRES_R {
        CCA_CS_THRES_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - ED Threshold Level for CCA Measurement"]
    #[inline(always)]
    #[must_use]
    pub fn cca_ed_thres(&mut self) -> CCA_ED_THRES_W<CCA_THRES_SPEC> {
        CCA_ED_THRES_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - CS Threshold Level for CCA Measurement"]
    #[inline(always)]
    #[must_use]
    pub fn cca_cs_thres(&mut self) -> CCA_CS_THRES_W<CCA_THRES_SPEC> {
        CCA_CS_THRES_W::new(self, 4)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transceiver CCA Threshold Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cca_thres::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cca_thres::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCA_THRES_SPEC;
impl crate::RegisterSpec for CCA_THRES_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cca_thres::R`](R) reader structure"]
impl crate::Readable for CCA_THRES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cca_thres::W`](W) writer structure"]
impl crate::Writable for CCA_THRES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCA_THRES to value 0"]
impl crate::Resettable for CCA_THRES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
