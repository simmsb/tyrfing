#[doc = "Register `HCMP2` reader"]
pub type R = crate::R<SPLIT_HCMP2_SPEC>;
#[doc = "Register `HCMP2` writer"]
pub type W = crate::W<SPLIT_HCMP2_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SPLIT_HCMP2_SPEC> {
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
#[doc = "High Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_hcmp2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_hcmp2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPLIT_HCMP2_SPEC;
impl crate::RegisterSpec for SPLIT_HCMP2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`split_hcmp2::R`](R) reader structure"]
impl crate::Readable for SPLIT_HCMP2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`split_hcmp2::W`](W) writer structure"]
impl crate::Writable for SPLIT_HCMP2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCMP2 to value 0"]
impl crate::Resettable for SPLIT_HCMP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
