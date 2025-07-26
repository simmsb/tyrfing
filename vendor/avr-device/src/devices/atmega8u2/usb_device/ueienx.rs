#[doc = "Register `UEIENX` reader"]
pub type R = crate::R<UEIENX_SPEC>;
#[doc = "Register `UEIENX` writer"]
pub type W = crate::W<UEIENX_SPEC>;
#[doc = "Field `TXINE` reader - Transmitter Ready Interrupt Enable Flag"]
pub type TXINE_R = crate::BitReader;
#[doc = "Field `TXINE` writer - Transmitter Ready Interrupt Enable Flag"]
pub type TXINE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLEDE` reader - Stalled Interrupt Enable Flag"]
pub type STALLEDE_R = crate::BitReader;
#[doc = "Field `STALLEDE` writer - Stalled Interrupt Enable Flag"]
pub type STALLEDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTE` reader - Received OUT Data Interrupt Enable Flag"]
pub type RXOUTE_R = crate::BitReader;
#[doc = "Field `RXOUTE` writer - Received OUT Data Interrupt Enable Flag"]
pub type RXOUTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTPE` reader - Received SETUP Interrupt Enable Flag"]
pub type RXSTPE_R = crate::BitReader;
#[doc = "Field `RXSTPE` writer - Received SETUP Interrupt Enable Flag"]
pub type RXSTPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKOUTE` reader - NAK OUT Interrupt Enable Bit"]
pub type NAKOUTE_R = crate::BitReader;
#[doc = "Field `NAKOUTE` writer - NAK OUT Interrupt Enable Bit"]
pub type NAKOUTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINE` reader - NAK IN Interrupt Enable Bit"]
pub type NAKINE_R = crate::BitReader;
#[doc = "Field `NAKINE` writer - NAK IN Interrupt Enable Bit"]
pub type NAKINE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLERRE` reader - Flow Error Interrupt Enable Flag"]
pub type FLERRE_R = crate::BitReader;
#[doc = "Field `FLERRE` writer - Flow Error Interrupt Enable Flag"]
pub type FLERRE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmitter Ready Interrupt Enable Flag"]
    #[inline(always)]
    pub fn txine(&self) -> TXINE_R {
        TXINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stalled Interrupt Enable Flag"]
    #[inline(always)]
    pub fn stallede(&self) -> STALLEDE_R {
        STALLEDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Received OUT Data Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rxoute(&self) -> RXOUTE_R {
        RXOUTE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Received SETUP Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rxstpe(&self) -> RXSTPE_R {
        RXSTPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK OUT Interrupt Enable Bit"]
    #[inline(always)]
    pub fn nakoute(&self) -> NAKOUTE_R {
        NAKOUTE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - NAK IN Interrupt Enable Bit"]
    #[inline(always)]
    pub fn nakine(&self) -> NAKINE_R {
        NAKINE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Flow Error Interrupt Enable Flag"]
    #[inline(always)]
    pub fn flerre(&self) -> FLERRE_R {
        FLERRE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter Ready Interrupt Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txine(&mut self) -> TXINE_W<UEIENX_SPEC> {
        TXINE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Stalled Interrupt Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn stallede(&mut self) -> STALLEDE_W<UEIENX_SPEC> {
        STALLEDE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Received OUT Data Interrupt Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxoute(&mut self) -> RXOUTE_W<UEIENX_SPEC> {
        RXOUTE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Received SETUP Interrupt Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpe(&mut self) -> RXSTPE_W<UEIENX_SPEC> {
        RXSTPE_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAK OUT Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn nakoute(&mut self) -> NAKOUTE_W<UEIENX_SPEC> {
        NAKOUTE_W::new(self, 4)
    }
    #[doc = "Bit 6 - NAK IN Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn nakine(&mut self) -> NAKINE_W<UEIENX_SPEC> {
        NAKINE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Flow Error Interrupt Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn flerre(&mut self) -> FLERRE_W<UEIENX_SPEC> {
        FLERRE_W::new(self, 7)
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
#[doc = "USB Endpoint Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ueienx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ueienx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEIENX_SPEC;
impl crate::RegisterSpec for UEIENX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ueienx::R`](R) reader structure"]
impl crate::Readable for UEIENX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ueienx::W`](W) writer structure"]
impl crate::Writable for UEIENX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEIENX to value 0"]
impl crate::Resettable for UEIENX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
