#[doc = "Register `USERROW3` reader"]
pub type R = crate::R<USERROW3_SPEC>;
#[doc = "Register `USERROW3` writer"]
pub type W = crate::W<USERROW3_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<USERROW3_SPEC> {
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
#[doc = "User Row Byte 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userrow3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userrow3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USERROW3_SPEC;
impl crate::RegisterSpec for USERROW3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`userrow3::R`](R) reader structure"]
impl crate::Readable for USERROW3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`userrow3::W`](W) writer structure"]
impl crate::Writable for USERROW3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USERROW3 to value 0"]
impl crate::Resettable for USERROW3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
