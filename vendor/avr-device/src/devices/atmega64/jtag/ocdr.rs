#[doc = "Register `OCDR` reader"]
pub type R = crate::R<OCDR_SPEC>;
#[doc = "Register `OCDR` writer"]
pub type W = crate::W<OCDR_SPEC>;
#[doc = "Field `OCDR` reader - On-Chip Debug Register Bits"]
pub type OCDR_R = crate::FieldReader;
#[doc = "Field `OCDR` writer - On-Chip Debug Register Bits"]
pub type OCDR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - On-Chip Debug Register Bits"]
    #[inline(always)]
    pub fn ocdr(&self) -> OCDR_R {
        OCDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - On-Chip Debug Register Bits"]
    #[inline(always)]
    #[must_use]
    pub fn ocdr(&mut self) -> OCDR_W<OCDR_SPEC> {
        OCDR_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "On-Chip Debug Related Register in I/O Memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCDR_SPEC;
impl crate::RegisterSpec for OCDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ocdr::R`](R) reader structure"]
impl crate::Readable for OCDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocdr::W`](W) writer structure"]
impl crate::Writable for OCDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCDR to value 0"]
impl crate::Resettable for OCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
