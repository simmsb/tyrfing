#[doc = "Register `GPIOR2` reader"]
pub type R = crate::R<GPIOR2_SPEC>;
#[doc = "Register `GPIOR2` writer"]
pub type W = crate::W<GPIOR2_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<GPIOR2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "General Purpose IO Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpior2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpior2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOR2_SPEC;
impl crate::RegisterSpec for GPIOR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gpior2::R`](R) reader structure"]
impl crate::Readable for GPIOR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpior2::W`](W) writer structure"]
impl crate::Writable for GPIOR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOR2 to value 0"]
impl crate::Resettable for GPIOR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
