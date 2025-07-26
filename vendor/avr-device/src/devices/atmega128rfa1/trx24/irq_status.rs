#[doc = "Register `IRQ_STATUS` reader"]
pub type R = crate::R<IRQ_STATUS_SPEC>;
#[doc = "Register `IRQ_STATUS` writer"]
pub type W = crate::W<IRQ_STATUS_SPEC>;
#[doc = "Field `PLL_LOCK` reader - PLL Lock Interrupt Status"]
pub type PLL_LOCK_R = crate::BitReader;
#[doc = "Field `PLL_LOCK` writer - PLL Lock Interrupt Status"]
pub type PLL_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_UNLOCK` reader - PLL Unlock Interrupt Status"]
pub type PLL_UNLOCK_R = crate::BitReader;
#[doc = "Field `PLL_UNLOCK` writer - PLL Unlock Interrupt Status"]
pub type PLL_UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_START` reader - RX_START Interrupt Status"]
pub type RX_START_R = crate::BitReader;
#[doc = "Field `RX_START` writer - RX_START Interrupt Status"]
pub type RX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_END` reader - RX_END Interrupt Status"]
pub type RX_END_R = crate::BitReader;
#[doc = "Field `RX_END` writer - RX_END Interrupt Status"]
pub type RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCA_ED_DONE` reader - End of ED Measurement Interrupt Status"]
pub type CCA_ED_DONE_R = crate::BitReader;
#[doc = "Field `CCA_ED_DONE` writer - End of ED Measurement Interrupt Status"]
pub type CCA_ED_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMI` reader - Address Match Interrupt Status"]
pub type AMI_R = crate::BitReader;
#[doc = "Field `AMI` writer - Address Match Interrupt Status"]
pub type AMI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_END` reader - TX_END Interrupt Status"]
pub type TX_END_R = crate::BitReader;
#[doc = "Field `TX_END` writer - TX_END Interrupt Status"]
pub type TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWAKE` reader - Awake Interrupt Status"]
pub type AWAKE_R = crate::BitReader;
#[doc = "Field `AWAKE` writer - Awake Interrupt Status"]
pub type AWAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PLL Lock Interrupt Status"]
    #[inline(always)]
    pub fn pll_lock(&self) -> PLL_LOCK_R {
        PLL_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL Unlock Interrupt Status"]
    #[inline(always)]
    pub fn pll_unlock(&self) -> PLL_UNLOCK_R {
        PLL_UNLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX_START Interrupt Status"]
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX_END Interrupt Status"]
    #[inline(always)]
    pub fn rx_end(&self) -> RX_END_R {
        RX_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of ED Measurement Interrupt Status"]
    #[inline(always)]
    pub fn cca_ed_done(&self) -> CCA_ED_DONE_R {
        CCA_ED_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Address Match Interrupt Status"]
    #[inline(always)]
    pub fn ami(&self) -> AMI_R {
        AMI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX_END Interrupt Status"]
    #[inline(always)]
    pub fn tx_end(&self) -> TX_END_R {
        TX_END_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Awake Interrupt Status"]
    #[inline(always)]
    pub fn awake(&self) -> AWAKE_R {
        AWAKE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Lock Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pll_lock(&mut self) -> PLL_LOCK_W<IRQ_STATUS_SPEC> {
        PLL_LOCK_W::new(self, 0)
    }
    #[doc = "Bit 1 - PLL Unlock Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pll_unlock(&mut self) -> PLL_UNLOCK_W<IRQ_STATUS_SPEC> {
        PLL_UNLOCK_W::new(self, 1)
    }
    #[doc = "Bit 2 - RX_START Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn rx_start(&mut self) -> RX_START_W<IRQ_STATUS_SPEC> {
        RX_START_W::new(self, 2)
    }
    #[doc = "Bit 3 - RX_END Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn rx_end(&mut self) -> RX_END_W<IRQ_STATUS_SPEC> {
        RX_END_W::new(self, 3)
    }
    #[doc = "Bit 4 - End of ED Measurement Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn cca_ed_done(&mut self) -> CCA_ED_DONE_W<IRQ_STATUS_SPEC> {
        CCA_ED_DONE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Address Match Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ami(&mut self) -> AMI_W<IRQ_STATUS_SPEC> {
        AMI_W::new(self, 5)
    }
    #[doc = "Bit 6 - TX_END Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn tx_end(&mut self) -> TX_END_W<IRQ_STATUS_SPEC> {
        TX_END_W::new(self, 6)
    }
    #[doc = "Bit 7 - Awake Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn awake(&mut self) -> AWAKE_W<IRQ_STATUS_SPEC> {
        AWAKE_W::new(self, 7)
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
#[doc = "Transceiver Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ_STATUS_SPEC;
impl crate::RegisterSpec for IRQ_STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`irq_status::R`](R) reader structure"]
impl crate::Readable for IRQ_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irq_status::W`](W) writer structure"]
impl crate::Writable for IRQ_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQ_STATUS to value 0"]
impl crate::Resettable for IRQ_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
