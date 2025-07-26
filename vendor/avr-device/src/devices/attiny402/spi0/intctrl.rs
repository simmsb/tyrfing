#[doc = "Register `INTCTRL` reader"]
pub type R = crate::R<INTCTRL_SPEC>;
#[doc = "Register `INTCTRL` writer"]
pub type W = crate::W<INTCTRL_SPEC>;
#[doc = "Field `IE` reader - Interrupt Enable"]
pub type IE_R = crate::BitReader;
#[doc = "Field `IE` writer - Interrupt Enable"]
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSIE` reader - Client Select Trigger Interrupt Enable"]
pub type SSIE_R = crate::BitReader;
#[doc = "Field `SSIE` writer - Client Select Trigger Interrupt Enable"]
pub type SSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DREIE` reader - Data Register Empty Interrupt Enable"]
pub type DREIE_R = crate::BitReader;
#[doc = "Field `DREIE` writer - Data Register Empty Interrupt Enable"]
pub type DREIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCIE` reader - Transfer Complete Interrupt Enable"]
pub type TXCIE_R = crate::BitReader;
#[doc = "Field `TXCIE` writer - Transfer Complete Interrupt Enable"]
pub type TXCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCIE` reader - Receive Complete Interrupt Enable"]
pub type RXCIE_R = crate::BitReader;
#[doc = "Field `RXCIE` writer - Receive Complete Interrupt Enable"]
pub type RXCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Client Select Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn ssie(&self) -> SSIE_R {
        SSIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn dreie(&self) -> DREIE_R {
        DREIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn txcie(&self) -> TXCIE_R {
        TXCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Complete Interrupt Enable"]
    #[inline(always)]
    pub fn rxcie(&self) -> RXCIE_R {
        RXCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<INTCTRL_SPEC> {
        IE_W::new(self, 0)
    }
    #[doc = "Bit 4 - Client Select Trigger Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssie(&mut self) -> SSIE_W<INTCTRL_SPEC> {
        SSIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Data Register Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dreie(&mut self) -> DREIE_W<INTCTRL_SPEC> {
        DREIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txcie(&mut self) -> TXCIE_W<INTCTRL_SPEC> {
        TXCIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxcie(&mut self) -> RXCIE_W<INTCTRL_SPEC> {
        RXCIE_W::new(self, 7)
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
#[doc = "Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTCTRL_SPEC;
impl crate::RegisterSpec for INTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intctrl::R`](R) reader structure"]
impl crate::Readable for INTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intctrl::W`](W) writer structure"]
impl crate::Writable for INTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCTRL to value 0"]
impl crate::Resettable for INTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
