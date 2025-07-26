#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `WFB` writer - Wait For Break"]
pub type WFB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDF` reader - Break Detected Flag"]
pub type BDF_R = crate::BitReader;
#[doc = "Field `BDF` writer - Break Detected Flag"]
pub type BDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISFIF` reader - Inconsistent Sync Field Interrupt Flag"]
pub type ISFIF_R = crate::BitReader;
#[doc = "Field `ISFIF` writer - Inconsistent Sync Field Interrupt Flag"]
pub type ISFIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSIF` reader - Receive Start Interrupt"]
pub type RXSIF_R = crate::BitReader;
#[doc = "Field `DREIF` reader - Data Register Empty Flag"]
pub type DREIF_R = crate::BitReader;
#[doc = "Field `TXCIF` reader - Transmit Interrupt Flag"]
pub type TXCIF_R = crate::BitReader;
#[doc = "Field `TXCIF` writer - Transmit Interrupt Flag"]
pub type TXCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCIF` reader - Receive Complete Interrupt Flag"]
pub type RXCIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Break Detected Flag"]
    #[inline(always)]
    pub fn bdf(&self) -> BDF_R {
        BDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Inconsistent Sync Field Interrupt Flag"]
    #[inline(always)]
    pub fn isfif(&self) -> ISFIF_R {
        ISFIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Start Interrupt"]
    #[inline(always)]
    pub fn rxsif(&self) -> RXSIF_R {
        RXSIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Register Empty Flag"]
    #[inline(always)]
    pub fn dreif(&self) -> DREIF_R {
        DREIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Interrupt Flag"]
    #[inline(always)]
    pub fn txcif(&self) -> TXCIF_R {
        TXCIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Complete Interrupt Flag"]
    #[inline(always)]
    pub fn rxcif(&self) -> RXCIF_R {
        RXCIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wait For Break"]
    #[inline(always)]
    #[must_use]
    pub fn wfb(&mut self) -> WFB_W<STATUS_SPEC> {
        WFB_W::new(self, 0)
    }
    #[doc = "Bit 1 - Break Detected Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bdf(&mut self) -> BDF_W<STATUS_SPEC> {
        BDF_W::new(self, 1)
    }
    #[doc = "Bit 3 - Inconsistent Sync Field Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isfif(&mut self) -> ISFIF_W<STATUS_SPEC> {
        ISFIF_W::new(self, 3)
    }
    #[doc = "Bit 6 - Transmit Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txcif(&mut self) -> TXCIF_W<STATUS_SPEC> {
        TXCIF_W::new(self, 6)
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
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
