#[doc = "Register `PLL_CF` reader"]
pub type R = crate::R<PLL_CF_SPEC>;
#[doc = "Register `PLL_CF` writer"]
pub type W = crate::W<PLL_CF_SPEC>;
#[doc = "Field `PLL_CF_START` reader - Start Center Frequency Calibration"]
pub type PLL_CF_START_R = crate::BitReader;
#[doc = "Field `PLL_CF_START` writer - Start Center Frequency Calibration"]
pub type PLL_CF_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - Start Center Frequency Calibration"]
    #[inline(always)]
    pub fn pll_cf_start(&self) -> PLL_CF_START_R {
        PLL_CF_START_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Start Center Frequency Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn pll_cf_start(&mut self) -> PLL_CF_START_W<PLL_CF_SPEC> {
        PLL_CF_START_W::new(self, 7)
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
#[doc = "Transceiver Center Frequency Calibration Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_cf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_cf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL_CF_SPEC;
impl crate::RegisterSpec for PLL_CF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pll_cf::R`](R) reader structure"]
impl crate::Readable for PLL_CF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll_cf::W`](W) writer structure"]
impl crate::Writable for PLL_CF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL_CF to value 0"]
impl crate::Resettable for PLL_CF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
