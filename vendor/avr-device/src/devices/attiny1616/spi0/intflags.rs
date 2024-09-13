#[doc = "Register `INTFLAGS` reader"]
pub type R = crate::R<INTFLAGS_SPEC>;
#[doc = "Register `INTFLAGS` writer"]
pub type W = crate::W<INTFLAGS_SPEC>;
#[doc = "Field `BUFOVF` reader - Buffer Overflow"]
pub type BUFOVF_R = crate::BitReader;
#[doc = "Field `BUFOVF` writer - Buffer Overflow"]
pub type BUFOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSIF` reader - Client Select Trigger Interrupt Flag"]
pub type SSIF_R = crate::BitReader;
#[doc = "Field `SSIF` writer - Client Select Trigger Interrupt Flag"]
pub type SSIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DREIF` reader - Data Register Empty Interrupt Flag"]
pub type DREIF_R = crate::BitReader;
#[doc = "Field `DREIF` writer - Data Register Empty Interrupt Flag"]
pub type DREIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCIF` reader - Transfer Complete Interrupt Flag"]
pub type TXCIF_R = crate::BitReader;
#[doc = "Field `TXCIF` writer - Transfer Complete Interrupt Flag"]
pub type TXCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRCOL` reader - Write Collision"]
pub type WRCOL_R = crate::BitReader;
#[doc = "Field `WRCOL` writer - Write Collision"]
pub type WRCOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IF` reader - Interrupt Flag"]
pub type IF_R = crate::BitReader;
#[doc = "Field `IF` writer - Interrupt Flag"]
pub type IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCIF` reader - Receive Complete Interrupt Flag"]
pub type RXCIF_R = crate::BitReader;
#[doc = "Field `RXCIF` writer - Receive Complete Interrupt Flag"]
pub type RXCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Buffer Overflow"]
    #[inline(always)]
    pub fn bufovf(&self) -> BUFOVF_R {
        BUFOVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Client Select Trigger Interrupt Flag"]
    #[inline(always)]
    pub fn ssif(&self) -> SSIF_R {
        SSIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Register Empty Interrupt Flag"]
    #[inline(always)]
    pub fn dreif(&self) -> DREIF_R {
        DREIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Complete Interrupt Flag"]
    #[inline(always)]
    pub fn txcif(&self) -> TXCIF_R {
        TXCIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 6 - Write Collision"]
    #[inline(always)]
    pub fn wrcol(&self) -> WRCOL_R {
        WRCOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Flag"]
    #[inline(always)]
    pub fn if_(&self) -> IF_R {
        IF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Complete Interrupt Flag"]
    #[inline(always)]
    pub fn rxcif(&self) -> RXCIF_R {
        RXCIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn bufovf(&mut self) -> BUFOVF_W<INTFLAGS_SPEC> {
        BUFOVF_W::new(self, 0)
    }
    #[doc = "Bit 4 - Client Select Trigger Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ssif(&mut self) -> SSIF_W<INTFLAGS_SPEC> {
        SSIF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Data Register Empty Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dreif(&mut self) -> DREIF_W<INTFLAGS_SPEC> {
        DREIF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer Complete Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txcif(&mut self) -> TXCIF_W<INTFLAGS_SPEC> {
        TXCIF_W::new(self, 6)
    }
    #[doc = "Bit 6 - Write Collision"]
    #[inline(always)]
    #[must_use]
    pub fn wrcol(&mut self) -> WRCOL_W<INTFLAGS_SPEC> {
        WRCOL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn if_(&mut self) -> IF_W<INTFLAGS_SPEC> {
        IF_W::new(self, 7)
    }
    #[doc = "Bit 7 - Receive Complete Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxcif(&mut self) -> RXCIF_W<INTFLAGS_SPEC> {
        RXCIF_W::new(self, 7)
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
#[doc = "Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflags::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflags::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFLAGS_SPEC;
impl crate::RegisterSpec for INTFLAGS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intflags::R`](R) reader structure"]
impl crate::Readable for INTFLAGS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intflags::W`](W) writer structure"]
impl crate::Writable for INTFLAGS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAGS to value 0"]
impl crate::Resettable for INTFLAGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
