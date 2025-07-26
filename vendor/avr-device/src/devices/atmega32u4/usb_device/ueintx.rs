#[doc = "Register `UEINTX` reader"]
pub type R = crate::R<UEINTX_SPEC>;
#[doc = "Register `UEINTX` writer"]
pub type W = crate::W<UEINTX_SPEC>;
#[doc = "Field `TXINI` reader - No Description."]
pub type TXINI_R = crate::BitReader;
#[doc = "Field `TXINI` writer - No Description."]
pub type TXINI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLEDI` reader - No Description."]
pub type STALLEDI_R = crate::BitReader;
#[doc = "Field `STALLEDI` writer - No Description."]
pub type STALLEDI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTI` reader - No Description."]
pub type RXOUTI_R = crate::BitReader;
#[doc = "Field `RXOUTI` writer - No Description."]
pub type RXOUTI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTPI` reader - No Description."]
pub type RXSTPI_R = crate::BitReader;
#[doc = "Field `RXSTPI` writer - No Description."]
pub type RXSTPI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKOUTI` reader - No Description."]
pub type NAKOUTI_R = crate::BitReader;
#[doc = "Field `NAKOUTI` writer - No Description."]
pub type NAKOUTI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWAL` reader - No Description."]
pub type RWAL_R = crate::BitReader;
#[doc = "Field `RWAL` writer - No Description."]
pub type RWAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINI` reader - No Description."]
pub type NAKINI_R = crate::BitReader;
#[doc = "Field `NAKINI` writer - No Description."]
pub type NAKINI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOCON` reader - No Description."]
pub type FIFOCON_R = crate::BitReader;
#[doc = "Field `FIFOCON` writer - No Description."]
pub type FIFOCON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn txini(&self) -> TXINI_R {
        TXINI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    pub fn stalledi(&self) -> STALLEDI_R {
        STALLEDI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    pub fn rxouti(&self) -> RXOUTI_R {
        RXOUTI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    pub fn rxstpi(&self) -> RXSTPI_R {
        RXSTPI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    pub fn nakouti(&self) -> NAKOUTI_R {
        NAKOUTI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    pub fn rwal(&self) -> RWAL_R {
        RWAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    pub fn nakini(&self) -> NAKINI_R {
        NAKINI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    pub fn fifocon(&self) -> FIFOCON_R {
        FIFOCON_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn txini(&mut self) -> TXINI_W<UEINTX_SPEC> {
        TXINI_W::new(self, 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn stalledi(&mut self) -> STALLEDI_W<UEINTX_SPEC> {
        STALLEDI_W::new(self, 1)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn rxouti(&mut self) -> RXOUTI_W<UEINTX_SPEC> {
        RXOUTI_W::new(self, 2)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn rxstpi(&mut self) -> RXSTPI_W<UEINTX_SPEC> {
        RXSTPI_W::new(self, 3)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn nakouti(&mut self) -> NAKOUTI_W<UEINTX_SPEC> {
        NAKOUTI_W::new(self, 4)
    }
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn rwal(&mut self) -> RWAL_W<UEINTX_SPEC> {
        RWAL_W::new(self, 5)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn nakini(&mut self) -> NAKINI_W<UEINTX_SPEC> {
        NAKINI_W::new(self, 6)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn fifocon(&mut self) -> FIFOCON_W<UEINTX_SPEC> {
        FIFOCON_W::new(self, 7)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ueintx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ueintx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEINTX_SPEC;
impl crate::RegisterSpec for UEINTX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ueintx::R`](R) reader structure"]
impl crate::Readable for UEINTX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ueintx::W`](W) writer structure"]
impl crate::Writable for UEINTX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEINTX to value 0"]
impl crate::Resettable for UEINTX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
