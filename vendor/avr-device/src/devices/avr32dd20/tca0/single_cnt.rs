#[doc = "Register `CNT` reader"]
pub type R = crate::R<SINGLE_CNT_SPEC>;
#[doc = "Register `CNT` writer"]
pub type W = crate::W<SINGLE_CNT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SINGLE_CNT_SPEC> {
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
#[doc = "Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINGLE_CNT_SPEC;
impl crate::RegisterSpec for SINGLE_CNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`single_cnt::R`](R) reader structure"]
impl crate::Readable for SINGLE_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`single_cnt::W`](W) writer structure"]
impl crate::Writable for SINGLE_CNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for SINGLE_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
