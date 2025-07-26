#[doc = "Register `SPH` reader"]
pub type R = crate::R<SPH_SPEC>;
#[doc = "Register `SPH` writer"]
pub type W = crate::W<SPH_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SPH_SPEC> {
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
#[doc = "Stack Pointer High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sph::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sph::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPH_SPEC;
impl crate::RegisterSpec for SPH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sph::R`](R) reader structure"]
impl crate::Readable for SPH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sph::W`](W) writer structure"]
impl crate::Writable for SPH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPH to value 0"]
impl crate::Resettable for SPH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
