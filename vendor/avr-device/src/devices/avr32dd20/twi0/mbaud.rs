#[doc = "Register `MBAUD` reader"]
pub type R = crate::R<MBAUD_SPEC>;
#[doc = "Register `MBAUD` writer"]
pub type W = crate::W<MBAUD_SPEC>;
#[doc = "Field `BAUD` reader - Baud Rate"]
pub type BAUD_R = crate::FieldReader;
#[doc = "Field `BAUD` writer - Baud Rate"]
pub type BAUD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Baud Rate"]
    #[inline(always)]
    pub fn baud(&self) -> BAUD_R {
        BAUD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Baud Rate"]
    #[inline(always)]
    #[must_use]
    pub fn baud(&mut self) -> BAUD_W<MBAUD_SPEC> {
        BAUD_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Host Baud Rate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mbaud::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mbaud::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MBAUD_SPEC;
impl crate::RegisterSpec for MBAUD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mbaud::R`](R) reader structure"]
impl crate::Readable for MBAUD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mbaud::W`](W) writer structure"]
impl crate::Writable for MBAUD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MBAUD to value 0"]
impl crate::Resettable for MBAUD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
