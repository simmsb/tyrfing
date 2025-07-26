#[doc = "Register `SPDR0` reader"]
pub type R = crate::R<SPDR0_SPEC>;
#[doc = "Register `SPDR0` writer"]
pub type W = crate::W<SPDR0_SPEC>;
#[doc = "Field `SPDR0` reader - SPI Data bits"]
pub type SPDR0_R = crate::FieldReader;
#[doc = "Field `SPDR0` writer - SPI Data bits"]
pub type SPDR0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SPI Data bits"]
    #[inline(always)]
    pub fn spdr0(&self) -> SPDR0_R {
        SPDR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI Data bits"]
    #[inline(always)]
    #[must_use]
    pub fn spdr0(&mut self) -> SPDR0_W<SPDR0_SPEC> {
        SPDR0_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPDR0_SPEC;
impl crate::RegisterSpec for SPDR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spdr0::R`](R) reader structure"]
impl crate::Readable for SPDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spdr0::W`](W) writer structure"]
impl crate::Writable for SPDR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPDR0 to value 0"]
impl crate::Resettable for SPDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
