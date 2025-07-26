#[doc = "Register `BODCR` reader"]
pub type R = crate::R<BODCR_SPEC>;
#[doc = "Register `BODCR` writer"]
pub type W = crate::W<BODCR_SPEC>;
#[doc = "Field `BPDSE` reader - No Description."]
pub type BPDSE_R = crate::BitReader;
#[doc = "Field `BPDSE` writer - No Description."]
pub type BPDSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BPDS` reader - No Description."]
pub type BPDS_R = crate::BitReader;
#[doc = "Field `BPDS` writer - No Description."]
pub type BPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn bpdse(&self) -> BPDSE_R {
        BPDSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    pub fn bpds(&self) -> BPDS_R {
        BPDS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn bpdse(&mut self) -> BPDSE_W<BODCR_SPEC> {
        BPDSE_W::new(self, 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn bpds(&mut self) -> BPDS_W<BODCR_SPEC> {
        BPDS_W::new(self, 1)
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
#[doc = "BOD control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bodcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bodcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BODCR_SPEC;
impl crate::RegisterSpec for BODCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bodcr::R`](R) reader structure"]
impl crate::Readable for BODCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bodcr::W`](W) writer structure"]
impl crate::Writable for BODCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BODCR to value 0"]
impl crate::Resettable for BODCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
