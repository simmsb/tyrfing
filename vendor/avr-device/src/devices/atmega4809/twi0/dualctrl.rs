#[doc = "Register `DUALCTRL` reader"]
pub type R = crate::R<DUALCTRL_SPEC>;
#[doc = "Register `DUALCTRL` writer"]
pub type W = crate::W<DUALCTRL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DUALCTRL_SPEC> {
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
#[doc = "Dual Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dualctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dualctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DUALCTRL_SPEC;
impl crate::RegisterSpec for DUALCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dualctrl::R`](R) reader structure"]
impl crate::Readable for DUALCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dualctrl::W`](W) writer structure"]
impl crate::Writable for DUALCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DUALCTRL to value 0"]
impl crate::Resettable for DUALCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
