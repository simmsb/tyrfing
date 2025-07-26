#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `DAC0REFEN` reader - DAC0/AC0 reference enable"]
pub type DAC0REFEN_R = crate::BitReader;
#[doc = "Field `DAC0REFEN` writer - DAC0/AC0 reference enable"]
pub type DAC0REFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0REFEN` reader - ADC0 reference enable"]
pub type ADC0REFEN_R = crate::BitReader;
#[doc = "Field `ADC0REFEN` writer - ADC0 reference enable"]
pub type ADC0REFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1REFEN` reader - DAC1/AC1 reference enable"]
pub type DAC1REFEN_R = crate::BitReader;
#[doc = "Field `DAC1REFEN` writer - DAC1/AC1 reference enable"]
pub type DAC1REFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1REFEN` reader - ADC1 reference enable"]
pub type ADC1REFEN_R = crate::BitReader;
#[doc = "Field `ADC1REFEN` writer - ADC1 reference enable"]
pub type ADC1REFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC2REFEN` reader - DAC2/AC2 reference enable"]
pub type DAC2REFEN_R = crate::BitReader;
#[doc = "Field `DAC2REFEN` writer - DAC2/AC2 reference enable"]
pub type DAC2REFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DAC0/AC0 reference enable"]
    #[inline(always)]
    pub fn dac0refen(&self) -> DAC0REFEN_R {
        DAC0REFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC0 reference enable"]
    #[inline(always)]
    pub fn adc0refen(&self) -> ADC0REFEN_R {
        ADC0REFEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - DAC1/AC1 reference enable"]
    #[inline(always)]
    pub fn dac1refen(&self) -> DAC1REFEN_R {
        DAC1REFEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC1 reference enable"]
    #[inline(always)]
    pub fn adc1refen(&self) -> ADC1REFEN_R {
        ADC1REFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DAC2/AC2 reference enable"]
    #[inline(always)]
    pub fn dac2refen(&self) -> DAC2REFEN_R {
        DAC2REFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC0/AC0 reference enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac0refen(&mut self) -> DAC0REFEN_W<CTRLB_SPEC> {
        DAC0REFEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC0 reference enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0refen(&mut self) -> ADC0REFEN_W<CTRLB_SPEC> {
        ADC0REFEN_W::new(self, 1)
    }
    #[doc = "Bit 3 - DAC1/AC1 reference enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac1refen(&mut self) -> DAC1REFEN_W<CTRLB_SPEC> {
        DAC1REFEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC1 reference enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1refen(&mut self) -> ADC1REFEN_W<CTRLB_SPEC> {
        ADC1REFEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - DAC2/AC2 reference enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac2refen(&mut self) -> DAC2REFEN_W<CTRLB_SPEC> {
        DAC2REFEN_W::new(self, 5)
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
#[doc = "Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
