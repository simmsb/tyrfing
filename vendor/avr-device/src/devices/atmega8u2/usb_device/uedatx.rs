#[doc = "Register `UEDATX` reader"]
pub type R = crate::R<UEDATX_SPEC>;
#[doc = "Register `UEDATX` writer"]
pub type W = crate::W<UEDATX_SPEC>;
#[doc = "Field `DAT` reader - Data bits"]
pub type DAT_R = crate::FieldReader;
#[doc = "Field `DAT` writer - Data bits"]
pub type DAT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data bits"]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data bits"]
    #[inline(always)]
    #[must_use]
    pub fn dat(&mut self) -> DAT_W<UEDATX_SPEC> {
        DAT_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB Data Endpoint\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uedatx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uedatx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEDATX_SPEC;
impl crate::RegisterSpec for UEDATX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uedatx::R`](R) reader structure"]
impl crate::Readable for UEDATX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uedatx::W`](W) writer structure"]
impl crate::Writable for UEDATX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEDATX to value 0"]
impl crate::Resettable for UEDATX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
