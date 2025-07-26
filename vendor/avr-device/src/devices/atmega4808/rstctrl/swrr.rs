#[doc = "Register `SWRR` reader"]
pub type R = crate::R<SWRR_SPEC>;
#[doc = "Register `SWRR` writer"]
pub type W = crate::W<SWRR_SPEC>;
#[doc = "Field `SWRE` reader - Software reset enable"]
pub type SWRE_R = crate::BitReader;
#[doc = "Field `SWRE` writer - Software reset enable"]
pub type SWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn swre(&self) -> SWRE_R {
        SWRE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    #[must_use]
    pub fn swre(&mut self) -> SWRE_W<SWRR_SPEC> {
        SWRE_W::new(self, 0)
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
#[doc = "Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWRR_SPEC;
impl crate::RegisterSpec for SWRR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`swrr::R`](R) reader structure"]
impl crate::Readable for SWRR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swrr::W`](W) writer structure"]
impl crate::Writable for SWRR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWRR to value 0"]
impl crate::Resettable for SWRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
