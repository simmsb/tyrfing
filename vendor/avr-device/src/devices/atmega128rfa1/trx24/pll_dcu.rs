#[doc = "Register `PLL_DCU` reader"]
pub type R = crate::R<PLL_DCU_SPEC>;
#[doc = "Register `PLL_DCU` writer"]
pub type W = crate::W<PLL_DCU_SPEC>;
#[doc = "Field `PLL_DCU_START` reader - Start Delay Cell Calibration"]
pub type PLL_DCU_START_R = crate::BitReader;
#[doc = "Field `PLL_DCU_START` writer - Start Delay Cell Calibration"]
pub type PLL_DCU_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - Start Delay Cell Calibration"]
    #[inline(always)]
    pub fn pll_dcu_start(&self) -> PLL_DCU_START_R {
        PLL_DCU_START_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Start Delay Cell Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn pll_dcu_start(&mut self) -> PLL_DCU_START_W<PLL_DCU_SPEC> {
        PLL_DCU_START_W::new(self, 7)
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
#[doc = "Transceiver Delay Cell Calibration Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_dcu::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_dcu::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL_DCU_SPEC;
impl crate::RegisterSpec for PLL_DCU_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pll_dcu::R`](R) reader structure"]
impl crate::Readable for PLL_DCU_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll_dcu::W`](W) writer structure"]
impl crate::Writable for PLL_DCU_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL_DCU to value 0"]
impl crate::Resettable for PLL_DCU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
