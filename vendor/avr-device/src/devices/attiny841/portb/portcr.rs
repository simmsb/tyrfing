#[doc = "Register `PORTCR` reader"]
pub type R = crate::R<PORTCR_SPEC>;
#[doc = "Register `PORTCR` writer"]
pub type W = crate::W<PORTCR_SPEC>;
#[doc = "Field `BBMB` reader - Break-Before-Make Mode Enable"]
pub type BBMB_R = crate::BitReader;
#[doc = "Field `BBMB` writer - Break-Before-Make Mode Enable"]
pub type BBMB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Break-Before-Make Mode Enable"]
    #[inline(always)]
    pub fn bbmb(&self) -> BBMB_R {
        BBMB_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Break-Before-Make Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bbmb(&mut self) -> BBMB_W<PORTCR_SPEC> {
        BBMB_W::new(self, 1)
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
#[doc = "Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
