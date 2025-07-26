#[doc = "Register `RSTFR` reader"]
pub type R = crate::R<RSTFR_SPEC>;
#[doc = "Register `RSTFR` writer"]
pub type W = crate::W<RSTFR_SPEC>;
#[doc = "Field `PORF` reader - Power on Reset flag"]
pub type PORF_R = crate::BitReader;
#[doc = "Field `PORF` writer - Power on Reset flag"]
pub type PORF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BORF` reader - Brown out detector Reset flag"]
pub type BORF_R = crate::BitReader;
#[doc = "Field `BORF` writer - Brown out detector Reset flag"]
pub type BORF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTRF` reader - External Reset flag"]
pub type EXTRF_R = crate::BitReader;
#[doc = "Field `EXTRF` writer - External Reset flag"]
pub type EXTRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDRF` reader - Watch dog Reset flag"]
pub type WDRF_R = crate::BitReader;
#[doc = "Field `WDRF` writer - Watch dog Reset flag"]
pub type WDRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRF` reader - Software Reset flag"]
pub type SWRF_R = crate::BitReader;
#[doc = "Field `SWRF` writer - Software Reset flag"]
pub type SWRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDIRF` reader - UPDI Reset flag"]
pub type UPDIRF_R = crate::BitReader;
#[doc = "Field `UPDIRF` writer - UPDI Reset flag"]
pub type UPDIRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power on Reset flag"]
    #[inline(always)]
    pub fn porf(&self) -> PORF_R {
        PORF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Brown out detector Reset flag"]
    #[inline(always)]
    pub fn borf(&self) -> BORF_R {
        BORF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Reset flag"]
    #[inline(always)]
    pub fn extrf(&self) -> EXTRF_R {
        EXTRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watch dog Reset flag"]
    #[inline(always)]
    pub fn wdrf(&self) -> WDRF_R {
        WDRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software Reset flag"]
    #[inline(always)]
    pub fn swrf(&self) -> SWRF_R {
        SWRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UPDI Reset flag"]
    #[inline(always)]
    pub fn updirf(&self) -> UPDIRF_R {
        UPDIRF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power on Reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn porf(&mut self) -> PORF_W<RSTFR_SPEC> {
        PORF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Brown out detector Reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn borf(&mut self) -> BORF_W<RSTFR_SPEC> {
        BORF_W::new(self, 1)
    }
    #[doc = "Bit 2 - External Reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn extrf(&mut self) -> EXTRF_W<RSTFR_SPEC> {
        EXTRF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Watch dog Reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdrf(&mut self) -> WDRF_W<RSTFR_SPEC> {
        WDRF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Software Reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn swrf(&mut self) -> SWRF_W<RSTFR_SPEC> {
        SWRF_W::new(self, 4)
    }
    #[doc = "Bit 5 - UPDI Reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn updirf(&mut self) -> UPDIRF_W<RSTFR_SPEC> {
        UPDIRF_W::new(self, 5)
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
#[doc = "Reset Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSTFR_SPEC;
impl crate::RegisterSpec for RSTFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rstfr::R`](R) reader structure"]
impl crate::Readable for RSTFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rstfr::W`](W) writer structure"]
impl crate::Writable for RSTFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTFR to value 0"]
impl crate::Resettable for RSTFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
