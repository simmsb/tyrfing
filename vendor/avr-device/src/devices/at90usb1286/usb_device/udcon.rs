#[doc = "Register `UDCON` reader"]
pub type R = crate::R<UDCON_SPEC>;
#[doc = "Register `UDCON` writer"]
pub type W = crate::W<UDCON_SPEC>;
#[doc = "Field `DETACH` reader - No Description."]
pub type DETACH_R = crate::BitReader;
#[doc = "Field `DETACH` writer - No Description."]
pub type DETACH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMWKUP` reader - No Description."]
pub type RMWKUP_R = crate::BitReader;
#[doc = "Field `RMWKUP` writer - No Description."]
pub type RMWKUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSM` reader - No Description."]
pub type LSM_R = crate::BitReader;
#[doc = "Field `LSM` writer - No Description."]
pub type LSM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn detach(&self) -> DETACH_R {
        DETACH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    pub fn rmwkup(&self) -> RMWKUP_R {
        RMWKUP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn detach(&mut self) -> DETACH_W<UDCON_SPEC> {
        DETACH_W::new(self, 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn rmwkup(&mut self) -> RMWKUP_W<UDCON_SPEC> {
        RMWKUP_W::new(self, 1)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn lsm(&mut self) -> LSM_W<UDCON_SPEC> {
        LSM_W::new(self, 2)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UDCON_SPEC;
impl crate::RegisterSpec for UDCON_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`udcon::R`](R) reader structure"]
impl crate::Readable for UDCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`udcon::W`](W) writer structure"]
impl crate::Writable for UDCON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDCON to value 0"]
impl crate::Resettable for UDCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
