#[doc = "Register `LINBRRL` reader"]
pub type R = crate::R<LINBRRL_SPEC>;
#[doc = "Register `LINBRRL` writer"]
pub type W = crate::W<LINBRRL_SPEC>;
#[doc = "Field `LDIV` reader - No Description."]
pub type LDIV_R = crate::FieldReader;
#[doc = "Field `LDIV` writer - No Description."]
pub type LDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - No Description."]
    #[inline(always)]
    pub fn ldiv(&self) -> LDIV_R {
        LDIV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn ldiv(&mut self) -> LDIV_W<LINBRRL_SPEC> {
        LDIV_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LIN Baud Rate Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linbrrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linbrrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINBRRL_SPEC;
impl crate::RegisterSpec for LINBRRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`linbrrl::R`](R) reader structure"]
impl crate::Readable for LINBRRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`linbrrl::W`](W) writer structure"]
impl crate::Writable for LINBRRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINBRRL to value 0"]
impl crate::Resettable for LINBRRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
