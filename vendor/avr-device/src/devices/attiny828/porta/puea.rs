#[doc = "Register `PUEA` reader"]
pub type R = crate::R<PUEA_SPEC>;
#[doc = "Register `PUEA` writer"]
pub type W = crate::W<PUEA_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PUEA_SPEC> {
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
#[doc = "Pull-up Enable Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`puea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`puea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUEA_SPEC;
impl crate::RegisterSpec for PUEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`puea::R`](R) reader structure"]
impl crate::Readable for PUEA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`puea::W`](W) writer structure"]
impl crate::Writable for PUEA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUEA to value 0"]
impl crate::Resettable for PUEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
