#[doc = "Register `PORTCR` reader"]
pub type R = crate::R<PORTCR_SPEC>;
#[doc = "Register `PORTCR` writer"]
pub type W = crate::W<PORTCR_SPEC>;
#[doc = "Field `PUDA` reader - No Description."]
pub type PUDA_R = crate::BitReader;
#[doc = "Field `PUDA` writer - No Description."]
pub type PUDA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUDB` reader - No Description."]
pub type PUDB_R = crate::BitReader;
#[doc = "Field `PUDB` writer - No Description."]
pub type PUDB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUDC` reader - No Description."]
pub type PUDC_R = crate::BitReader;
#[doc = "Field `PUDC` writer - No Description."]
pub type PUDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUDD` reader - No Description."]
pub type PUDD_R = crate::BitReader;
#[doc = "Field `PUDD` writer - No Description."]
pub type PUDD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBMA` reader - No Description."]
pub type BBMA_R = crate::BitReader;
#[doc = "Field `BBMA` writer - No Description."]
pub type BBMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBMB` reader - No Description."]
pub type BBMB_R = crate::BitReader;
#[doc = "Field `BBMB` writer - No Description."]
pub type BBMB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBMC` reader - No Description."]
pub type BBMC_R = crate::BitReader;
#[doc = "Field `BBMC` writer - No Description."]
pub type BBMC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBMD` reader - No Description."]
pub type BBMD_R = crate::BitReader;
#[doc = "Field `BBMD` writer - No Description."]
pub type BBMD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn puda(&self) -> PUDA_R {
        PUDA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    pub fn pudb(&self) -> PUDB_R {
        PUDB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    pub fn pudc(&self) -> PUDC_R {
        PUDC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    pub fn pudd(&self) -> PUDD_R {
        PUDD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    pub fn bbma(&self) -> BBMA_R {
        BBMA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    pub fn bbmb(&self) -> BBMB_R {
        BBMB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    pub fn bbmc(&self) -> BBMC_R {
        BBMC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    pub fn bbmd(&self) -> BBMD_R {
        BBMD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn puda(&mut self) -> PUDA_W<PORTCR_SPEC> {
        PUDA_W::new(self, 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn pudb(&mut self) -> PUDB_W<PORTCR_SPEC> {
        PUDB_W::new(self, 1)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn pudc(&mut self) -> PUDC_W<PORTCR_SPEC> {
        PUDC_W::new(self, 2)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn pudd(&mut self) -> PUDD_W<PORTCR_SPEC> {
        PUDD_W::new(self, 3)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn bbma(&mut self) -> BBMA_W<PORTCR_SPEC> {
        BBMA_W::new(self, 4)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn bbmb(&mut self) -> BBMB_W<PORTCR_SPEC> {
        BBMB_W::new(self, 5)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn bbmc(&mut self) -> BBMC_W<PORTCR_SPEC> {
        BBMC_W::new(self, 6)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn bbmd(&mut self) -> BBMD_W<PORTCR_SPEC> {
        BBMD_W::new(self, 7)
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
#[doc = "Port Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PORTCR_SPEC;
impl crate::RegisterSpec for PORTCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`portcr::R`](R) reader structure"]
impl crate::Readable for PORTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`portcr::W`](W) writer structure"]
impl crate::Writable for PORTCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORTCR to value 0"]
impl crate::Resettable for PORTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
