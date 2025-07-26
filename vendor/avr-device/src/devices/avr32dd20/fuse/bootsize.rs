#[doc = "Register `BOOTSIZE` reader"]
pub type R = crate::R<BOOTSIZE_SPEC>;
#[doc = "Register `BOOTSIZE` writer"]
pub type W = crate::W<BOOTSIZE_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<BOOTSIZE_SPEC> {
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
#[doc = "Boot Section Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bootsize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bootsize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTSIZE_SPEC;
impl crate::RegisterSpec for BOOTSIZE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bootsize::R`](R) reader structure"]
impl crate::Readable for BOOTSIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bootsize::W`](W) writer structure"]
impl crate::Writable for BOOTSIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOOTSIZE to value 0"]
impl crate::Resettable for BOOTSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
