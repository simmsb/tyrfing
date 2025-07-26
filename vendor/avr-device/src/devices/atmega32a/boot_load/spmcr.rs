#[doc = "Register `SPMCR` reader"]
pub type R = crate::R<SPMCR_SPEC>;
#[doc = "Register `SPMCR` writer"]
pub type W = crate::W<SPMCR_SPEC>;
#[doc = "Field `SPMEN` reader - Store Program Memory Enable"]
pub type SPMEN_R = crate::BitReader;
#[doc = "Field `SPMEN` writer - Store Program Memory Enable"]
pub type SPMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGERS` reader - Page Erase"]
pub type PGERS_R = crate::BitReader;
#[doc = "Field `PGERS` writer - Page Erase"]
pub type PGERS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGWRT` reader - Page Write"]
pub type PGWRT_R = crate::BitReader;
#[doc = "Field `PGWRT` writer - Page Write"]
pub type PGWRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLBSET` reader - Boot Lock Bit Set"]
pub type BLBSET_R = crate::BitReader;
#[doc = "Field `BLBSET` writer - Boot Lock Bit Set"]
pub type BLBSET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWWSRE` reader - Read While Write secion read enable"]
pub type RWWSRE_R = crate::BitReader;
#[doc = "Field `RWWSRE` writer - Read While Write secion read enable"]
pub type RWWSRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWWSB` reader - Read While Write Section Busy"]
pub type RWWSB_R = crate::BitReader;
#[doc = "Field `RWWSB` writer - Read While Write Section Busy"]
pub type RWWSB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPMIE` reader - SPM Interrupt Enable"]
pub type SPMIE_R = crate::BitReader;
#[doc = "Field `SPMIE` writer - SPM Interrupt Enable"]
pub type SPMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Store Program Memory Enable"]
    #[inline(always)]
    pub fn spmen(&self) -> SPMEN_R {
        SPMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page Erase"]
    #[inline(always)]
    pub fn pgers(&self) -> PGERS_R {
        PGERS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Page Write"]
    #[inline(always)]
    pub fn pgwrt(&self) -> PGWRT_R {
        PGWRT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Boot Lock Bit Set"]
    #[inline(always)]
    pub fn blbset(&self) -> BLBSET_R {
        BLBSET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read While Write secion read enable"]
    #[inline(always)]
    pub fn rwwsre(&self) -> RWWSRE_R {
        RWWSRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Read While Write Section Busy"]
    #[inline(always)]
    pub fn rwwsb(&self) -> RWWSB_R {
        RWWSB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPM Interrupt Enable"]
    #[inline(always)]
    pub fn spmie(&self) -> SPMIE_R {
        SPMIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Store Program Memory Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spmen(&mut self) -> SPMEN_W<SPMCR_SPEC> {
        SPMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Page Erase"]
    #[inline(always)]
    #[must_use]
    pub fn pgers(&mut self) -> PGERS_W<SPMCR_SPEC> {
        PGERS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Page Write"]
    #[inline(always)]
    #[must_use]
    pub fn pgwrt(&mut self) -> PGWRT_W<SPMCR_SPEC> {
        PGWRT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Boot Lock Bit Set"]
    #[inline(always)]
    #[must_use]
    pub fn blbset(&mut self) -> BLBSET_W<SPMCR_SPEC> {
        BLBSET_W::new(self, 3)
    }
    #[doc = "Bit 4 - Read While Write secion read enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwwsre(&mut self) -> RWWSRE_W<SPMCR_SPEC> {
        RWWSRE_W::new(self, 4)
    }
    #[doc = "Bit 6 - Read While Write Section Busy"]
    #[inline(always)]
    #[must_use]
    pub fn rwwsb(&mut self) -> RWWSB_W<SPMCR_SPEC> {
        RWWSB_W::new(self, 6)
    }
    #[doc = "Bit 7 - SPM Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spmie(&mut self) -> SPMIE_W<SPMCR_SPEC> {
        SPMIE_W::new(self, 7)
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
#[doc = "Store Program Memory Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPMCR_SPEC;
impl crate::RegisterSpec for SPMCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spmcr::R`](R) reader structure"]
impl crate::Readable for SPMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spmcr::W`](W) writer structure"]
impl crate::Writable for SPMCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPMCR to value 0"]
impl crate::Resettable for SPMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
