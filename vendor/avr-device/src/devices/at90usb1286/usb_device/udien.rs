#[doc = "Register `UDIEN` reader"]
pub type R = crate::R<UDIEN_SPEC>;
#[doc = "Register `UDIEN` writer"]
pub type W = crate::W<UDIEN_SPEC>;
#[doc = "Field `SUSPE` reader - No Description."]
pub type SUSPE_R = crate::BitReader;
#[doc = "Field `SUSPE` writer - No Description."]
pub type SUSPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFE` reader - No Description."]
pub type SOFE_R = crate::BitReader;
#[doc = "Field `SOFE` writer - No Description."]
pub type SOFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EORSTE` reader - No Description."]
pub type EORSTE_R = crate::BitReader;
#[doc = "Field `EORSTE` writer - No Description."]
pub type EORSTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPE` reader - No Description."]
pub type WAKEUPE_R = crate::BitReader;
#[doc = "Field `WAKEUPE` writer - No Description."]
pub type WAKEUPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EORSME` reader - No Description."]
pub type EORSME_R = crate::BitReader;
#[doc = "Field `EORSME` writer - No Description."]
pub type EORSME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPRSME` reader - No Description."]
pub type UPRSME_R = crate::BitReader;
#[doc = "Field `UPRSME` writer - No Description."]
pub type UPRSME_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn suspe(&self) -> SUSPE_R {
        SUSPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    pub fn sofe(&self) -> SOFE_R {
        SOFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    pub fn eorste(&self) -> EORSTE_R {
        EORSTE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    pub fn wakeupe(&self) -> WAKEUPE_R {
        WAKEUPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    pub fn eorsme(&self) -> EORSME_R {
        EORSME_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    pub fn uprsme(&self) -> UPRSME_R {
        UPRSME_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn suspe(&mut self) -> SUSPE_W<UDIEN_SPEC> {
        SUSPE_W::new(self, 0)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn sofe(&mut self) -> SOFE_W<UDIEN_SPEC> {
        SOFE_W::new(self, 2)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn eorste(&mut self) -> EORSTE_W<UDIEN_SPEC> {
        EORSTE_W::new(self, 3)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn wakeupe(&mut self) -> WAKEUPE_W<UDIEN_SPEC> {
        WAKEUPE_W::new(self, 4)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn eorsme(&mut self) -> EORSME_W<UDIEN_SPEC> {
        EORSME_W::new(self, 5)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn uprsme(&mut self) -> UPRSME_W<UDIEN_SPEC> {
        UPRSME_W::new(self, 6)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UDIEN_SPEC;
impl crate::RegisterSpec for UDIEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`udien::R`](R) reader structure"]
impl crate::Readable for UDIEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`udien::W`](W) writer structure"]
impl crate::Writable for UDIEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDIEN to value 0"]
impl crate::Resettable for UDIEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
