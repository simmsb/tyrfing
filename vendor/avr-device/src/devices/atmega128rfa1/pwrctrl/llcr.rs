#[doc = "Register `LLCR` reader"]
pub type R = crate::R<LLCR_SPEC>;
#[doc = "Register `LLCR` writer"]
pub type W = crate::W<LLCR_SPEC>;
#[doc = "Field `LLENCAL` reader - Enable Automatic Calibration"]
pub type LLENCAL_R = crate::BitReader;
#[doc = "Field `LLENCAL` writer - Enable Automatic Calibration"]
pub type LLENCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLSHORT` reader - Short Lower Calibration Circuit"]
pub type LLSHORT_R = crate::BitReader;
#[doc = "Field `LLSHORT` writer - Short Lower Calibration Circuit"]
pub type LLSHORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLTCO` reader - Temperature Coefficient of Current Source"]
pub type LLTCO_R = crate::BitReader;
#[doc = "Field `LLTCO` writer - Temperature Coefficient of Current Source"]
pub type LLTCO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLCAL` reader - Calibration Active"]
pub type LLCAL_R = crate::BitReader;
#[doc = "Field `LLCAL` writer - Calibration Active"]
pub type LLCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLCOMP` reader - Comparator Output"]
pub type LLCOMP_R = crate::BitReader;
#[doc = "Field `LLCOMP` writer - Comparator Output"]
pub type LLCOMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLDONE` reader - Calibration Done"]
pub type LLDONE_R = crate::BitReader;
#[doc = "Field `LLDONE` writer - Calibration Done"]
pub type LLDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enable Automatic Calibration"]
    #[inline(always)]
    pub fn llencal(&self) -> LLENCAL_R {
        LLENCAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Short Lower Calibration Circuit"]
    #[inline(always)]
    pub fn llshort(&self) -> LLSHORT_R {
        LLSHORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Temperature Coefficient of Current Source"]
    #[inline(always)]
    pub fn lltco(&self) -> LLTCO_R {
        LLTCO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Calibration Active"]
    #[inline(always)]
    pub fn llcal(&self) -> LLCAL_R {
        LLCAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparator Output"]
    #[inline(always)]
    pub fn llcomp(&self) -> LLCOMP_R {
        LLCOMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Calibration Done"]
    #[inline(always)]
    pub fn lldone(&self) -> LLDONE_R {
        LLDONE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Automatic Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn llencal(&mut self) -> LLENCAL_W<LLCR_SPEC> {
        LLENCAL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Short Lower Calibration Circuit"]
    #[inline(always)]
    #[must_use]
    pub fn llshort(&mut self) -> LLSHORT_W<LLCR_SPEC> {
        LLSHORT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Temperature Coefficient of Current Source"]
    #[inline(always)]
    #[must_use]
    pub fn lltco(&mut self) -> LLTCO_W<LLCR_SPEC> {
        LLTCO_W::new(self, 2)
    }
    #[doc = "Bit 3 - Calibration Active"]
    #[inline(always)]
    #[must_use]
    pub fn llcal(&mut self) -> LLCAL_W<LLCR_SPEC> {
        LLCAL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Comparator Output"]
    #[inline(always)]
    #[must_use]
    pub fn llcomp(&mut self) -> LLCOMP_W<LLCR_SPEC> {
        LLCOMP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Calibration Done"]
    #[inline(always)]
    #[must_use]
    pub fn lldone(&mut self) -> LLDONE_W<LLCR_SPEC> {
        LLDONE_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<LLCR_SPEC> {
        RES_W::new(self, 6)
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
#[doc = "Low Leakage Voltage Regulator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`llcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`llcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LLCR_SPEC;
impl crate::RegisterSpec for LLCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`llcr::R`](R) reader structure"]
impl crate::Readable for LLCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`llcr::W`](W) writer structure"]
impl crate::Writable for LLCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LLCR to value 0"]
impl crate::Resettable for LLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
