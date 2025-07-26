#[doc = "Register `CMPBSET` reader"]
pub type R = crate::R<CMPBSET_SPEC>;
#[doc = "Register `CMPBSET` writer"]
pub type W = crate::W<CMPBSET_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CMPBSET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Compare B Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpbset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpbset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPBSET_SPEC;
impl crate::RegisterSpec for CMPBSET_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cmpbset::R`](R) reader structure"]
impl crate::Readable for CMPBSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpbset::W`](W) writer structure"]
impl crate::Writable for CMPBSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPBSET to value 0"]
impl crate::Resettable for CMPBSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
