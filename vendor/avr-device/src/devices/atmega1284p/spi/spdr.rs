#[doc = "Register `SPDR` reader"]
pub type R = crate::R<SPDR_SPEC>;
#[doc = "Register `SPDR` writer"]
pub type W = crate::W<SPDR_SPEC>;
#[doc = "Field `SPD` reader - SPI Data bits"]
pub type SPD_R = crate::FieldReader;
#[doc = "Field `SPD` writer - SPI Data bits"]
pub type SPD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SPI Data bits"]
    #[inline(always)]
    pub fn spd(&self) -> SPD_R {
        SPD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI Data bits"]
    #[inline(always)]
    #[must_use]
    pub fn spd(&mut self) -> SPD_W<SPDR_SPEC> {
        SPD_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPDR_SPEC;
impl crate::RegisterSpec for SPDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spdr::R`](R) reader structure"]
impl crate::Readable for SPDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spdr::W`](W) writer structure"]
impl crate::Writable for SPDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPDR to value 0"]
impl crate::Resettable for SPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
