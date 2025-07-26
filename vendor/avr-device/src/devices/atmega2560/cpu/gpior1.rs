#[doc = "Register `GPIOR1` reader"]
pub type R = crate::R<GPIOR1_SPEC>;
#[doc = "Register `GPIOR1` writer"]
pub type W = crate::W<GPIOR1_SPEC>;
#[doc = "Field `GPIOR` reader - General Purpose IO Register 1 bis"]
pub type GPIOR_R = crate::FieldReader;
#[doc = "Field `GPIOR` writer - General Purpose IO Register 1 bis"]
pub type GPIOR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - General Purpose IO Register 1 bis"]
    #[inline(always)]
    pub fn gpior(&self) -> GPIOR_R {
        GPIOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - General Purpose IO Register 1 bis"]
    #[inline(always)]
    #[must_use]
    pub fn gpior(&mut self) -> GPIOR_W<GPIOR1_SPEC> {
        GPIOR_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "General Purpose IO Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpior1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpior1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOR1_SPEC;
impl crate::RegisterSpec for GPIOR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gpior1::R`](R) reader structure"]
impl crate::Readable for GPIOR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpior1::W`](W) writer structure"]
impl crate::Writable for GPIOR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOR1 to value 0"]
impl crate::Resettable for GPIOR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
