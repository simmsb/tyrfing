#[doc = "Register `SPSR0` reader"]
pub type R = crate::R<SPSR0_SPEC>;
#[doc = "Register `SPSR0` writer"]
pub type W = crate::W<SPSR0_SPEC>;
#[doc = "Field `SPI2X0` reader - Double SPI Speed Bit"]
pub type SPI2X0_R = crate::BitReader;
#[doc = "Field `SPI2X0` writer - Double SPI Speed Bit"]
pub type SPI2X0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCOL0` reader - Write Collision Flag"]
pub type WCOL0_R = crate::BitReader;
#[doc = "Field `WCOL0` writer - Write Collision Flag"]
pub type WCOL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIF0` reader - SPI Interrupt Flag"]
pub type SPIF0_R = crate::BitReader;
#[doc = "Field `SPIF0` writer - SPI Interrupt Flag"]
pub type SPIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Double SPI Speed Bit"]
    #[inline(always)]
    pub fn spi2x0(&self) -> SPI2X0_R {
        SPI2X0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - Write Collision Flag"]
    #[inline(always)]
    pub fn wcol0(&self) -> WCOL0_R {
        WCOL0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Interrupt Flag"]
    #[inline(always)]
    pub fn spif0(&self) -> SPIF0_R {
        SPIF0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Double SPI Speed Bit"]
    #[inline(always)]
    #[must_use]
    pub fn spi2x0(&mut self) -> SPI2X0_W<SPSR0_SPEC> {
        SPI2X0_W::new(self, 0)
    }
    #[doc = "Bit 6 - Write Collision Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wcol0(&mut self) -> WCOL0_W<SPSR0_SPEC> {
        WCOL0_W::new(self, 6)
    }
    #[doc = "Bit 7 - SPI Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn spif0(&mut self) -> SPIF0_W<SPSR0_SPEC> {
        SPIF0_W::new(self, 7)
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
#[doc = "SPI Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spsr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spsr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPSR0_SPEC;
impl crate::RegisterSpec for SPSR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spsr0::R`](R) reader structure"]
impl crate::Readable for SPSR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spsr0::W`](W) writer structure"]
impl crate::Writable for SPSR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPSR0 to value 0"]
impl crate::Resettable for SPSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
