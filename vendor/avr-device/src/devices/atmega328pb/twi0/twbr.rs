#[doc = "Register `TWBR` reader"]
pub type R = crate::R<TWBR_SPEC>;
#[doc = "Register `TWBR` writer"]
pub type W = crate::W<TWBR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TWBR_SPEC> {
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
#[doc = "TWI Bit Rate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWBR_SPEC;
impl crate::RegisterSpec for TWBR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twbr::R`](R) reader structure"]
impl crate::Readable for TWBR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twbr::W`](W) writer structure"]
impl crate::Writable for TWBR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWBR to value 0"]
impl crate::Resettable for TWBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
