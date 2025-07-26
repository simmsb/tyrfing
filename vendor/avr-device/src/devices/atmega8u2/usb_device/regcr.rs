#[doc = "Register `REGCR` reader"]
pub type R = crate::R<REGCR_SPEC>;
#[doc = "Register `REGCR` writer"]
pub type W = crate::W<REGCR_SPEC>;
#[doc = "Field `REGDIS` reader - Regulator Disable"]
pub type REGDIS_R = crate::BitReader;
#[doc = "Field `REGDIS` writer - Regulator Disable"]
pub type REGDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Regulator Disable"]
    #[inline(always)]
    pub fn regdis(&self) -> REGDIS_R {
        REGDIS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Regulator Disable"]
    #[inline(always)]
    #[must_use]
    pub fn regdis(&mut self) -> REGDIS_W<REGCR_SPEC> {
        REGDIS_W::new(self, 0)
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
#[doc = "Regulator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGCR_SPEC;
impl crate::RegisterSpec for REGCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`regcr::R`](R) reader structure"]
impl crate::Readable for REGCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`regcr::W`](W) writer structure"]
impl crate::Writable for REGCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGCR to value 0"]
impl crate::Resettable for REGCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
