#[doc = "Register `UEBCLX` reader"]
pub type R = crate::R<UEBCLX_SPEC>;
#[doc = "Register `UEBCLX` writer"]
pub type W = crate::W<UEBCLX_SPEC>;
#[doc = "Field `BYCT` reader - Byte Count bits"]
pub type BYCT_R = crate::FieldReader;
#[doc = "Field `BYCT` writer - Byte Count bits"]
pub type BYCT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Byte Count bits"]
    #[inline(always)]
    pub fn byct(&self) -> BYCT_R {
        BYCT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Byte Count bits"]
    #[inline(always)]
    #[must_use]
    pub fn byct(&mut self) -> BYCT_W<UEBCLX_SPEC> {
        BYCT_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB Endpoint Byte Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uebclx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uebclx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEBCLX_SPEC;
impl crate::RegisterSpec for UEBCLX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uebclx::R`](R) reader structure"]
impl crate::Readable for UEBCLX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uebclx::W`](W) writer structure"]
impl crate::Writable for UEBCLX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEBCLX to value 0"]
impl crate::Resettable for UEBCLX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
