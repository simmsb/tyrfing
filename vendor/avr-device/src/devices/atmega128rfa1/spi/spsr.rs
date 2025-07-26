#[doc = "Register `SPSR` reader"]
pub type R = crate::R<SPSR_SPEC>;
#[doc = "Register `SPSR` writer"]
pub type W = crate::W<SPSR_SPEC>;
#[doc = "Field `SPI2X` reader - Double SPI Speed Bit"]
pub type SPI2X_R = crate::BitReader;
#[doc = "Field `SPI2X` writer - Double SPI Speed Bit"]
pub type SPI2X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `WCOL` reader - Write Collision Flag"]
pub type WCOL_R = crate::BitReader;
#[doc = "Field `SPIF` reader - SPI Interrupt Flag"]
pub type SPIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Double SPI Speed Bit"]
    #[inline(always)]
    pub fn spi2x(&self) -> SPI2X_R {
        SPI2X_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 1) & 0x1f)
    }
    #[doc = "Bit 6 - Write Collision Flag"]
    #[inline(always)]
    pub fn wcol(&self) -> WCOL_R {
        WCOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Interrupt Flag"]
    #[inline(always)]
    pub fn spif(&self) -> SPIF_R {
        SPIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Double SPI Speed Bit"]
    #[inline(always)]
    #[must_use]
    pub fn spi2x(&mut self) -> SPI2X_W<SPSR_SPEC> {
        SPI2X_W::new(self, 0)
    }
    #[doc = "Bits 1:5 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<SPSR_SPEC> {
        RES_W::new(self, 1)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPSR_SPEC;
impl crate::RegisterSpec for SPSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spsr::R`](R) reader structure"]
impl crate::Readable for SPSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spsr::W`](W) writer structure"]
impl crate::Writable for SPSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPSR to value 0"]
impl crate::Resettable for SPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
