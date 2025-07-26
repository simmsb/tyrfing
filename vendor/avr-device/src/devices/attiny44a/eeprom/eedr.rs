#[doc = "Register `EEDR` reader"]
pub type R = crate::R<EEDR_SPEC>;
#[doc = "Register `EEDR` writer"]
pub type W = crate::W<EEDR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<EEDR_SPEC> {
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
#[doc = "EEPROM Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eedr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eedr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEDR_SPEC;
impl crate::RegisterSpec for EEDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eedr::R`](R) reader structure"]
impl crate::Readable for EEDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eedr::W`](W) writer structure"]
impl crate::Writable for EEDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEDR to value 0"]
impl crate::Resettable for EEDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
