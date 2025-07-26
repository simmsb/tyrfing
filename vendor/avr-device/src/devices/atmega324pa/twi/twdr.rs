#[doc = "Register `TWDR` reader"]
pub type R = crate::R<TWDR_SPEC>;
#[doc = "Register `TWDR` writer"]
pub type W = crate::W<TWDR_SPEC>;
#[doc = "Field `TWD` reader - TWI data bits"]
pub type TWD_R = crate::FieldReader;
#[doc = "Field `TWD` writer - TWI data bits"]
pub type TWD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TWI data bits"]
    #[inline(always)]
    pub fn twd(&self) -> TWD_R {
        TWD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - TWI data bits"]
    #[inline(always)]
    #[must_use]
    pub fn twd(&mut self) -> TWD_W<TWDR_SPEC> {
        TWD_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TWI Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWDR_SPEC;
impl crate::RegisterSpec for TWDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twdr::R`](R) reader structure"]
impl crate::Readable for TWDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twdr::W`](W) writer structure"]
impl crate::Writable for TWDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWDR to value 0"]
impl crate::Resettable for TWDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
