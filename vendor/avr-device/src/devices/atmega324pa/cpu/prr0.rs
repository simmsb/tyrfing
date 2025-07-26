#[doc = "Register `PRR0` reader"]
pub type R = crate::R<PRR0_SPEC>;
#[doc = "Register `PRR0` writer"]
pub type W = crate::W<PRR0_SPEC>;
#[doc = "Field `PRADC` reader - Power Reduction ADC"]
pub type PRADC_R = crate::BitReader;
#[doc = "Field `PRADC` writer - Power Reduction ADC"]
pub type PRADC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRUSART0` reader - Power Reduction USART0"]
pub type PRUSART0_R = crate::BitReader;
#[doc = "Field `PRUSART0` writer - Power Reduction USART0"]
pub type PRUSART0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSPI` reader - Power Reduction Serial Peripheral Interface"]
pub type PRSPI_R = crate::BitReader;
#[doc = "Field `PRSPI` writer - Power Reduction Serial Peripheral Interface"]
pub type PRSPI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTIM1` reader - Power Reduction Timer/Counter1"]
pub type PRTIM1_R = crate::BitReader;
#[doc = "Field `PRTIM1` writer - Power Reduction Timer/Counter1"]
pub type PRTIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRUSART1` reader - Power Reduction USART1"]
pub type PRUSART1_R = crate::BitReader;
#[doc = "Field `PRUSART1` writer - Power Reduction USART1"]
pub type PRUSART1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTIM0` reader - Power Reduction Timer/Counter0"]
pub type PRTIM0_R = crate::BitReader;
#[doc = "Field `PRTIM0` writer - Power Reduction Timer/Counter0"]
pub type PRTIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTIM2` reader - Power Reduction Timer/Counter2"]
pub type PRTIM2_R = crate::BitReader;
#[doc = "Field `PRTIM2` writer - Power Reduction Timer/Counter2"]
pub type PRTIM2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTWI` reader - Power Reduction TWI"]
pub type PRTWI_R = crate::BitReader;
#[doc = "Field `PRTWI` writer - Power Reduction TWI"]
pub type PRTWI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power Reduction ADC"]
    #[inline(always)]
    pub fn pradc(&self) -> PRADC_R {
        PRADC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Reduction USART0"]
    #[inline(always)]
    pub fn prusart0(&self) -> PRUSART0_R {
        PRUSART0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Reduction Serial Peripheral Interface"]
    #[inline(always)]
    pub fn prspi(&self) -> PRSPI_R {
        PRSPI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power Reduction Timer/Counter1"]
    #[inline(always)]
    pub fn prtim1(&self) -> PRTIM1_R {
        PRTIM1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power Reduction USART1"]
    #[inline(always)]
    pub fn prusart1(&self) -> PRUSART1_R {
        PRUSART1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Power Reduction Timer/Counter0"]
    #[inline(always)]
    pub fn prtim0(&self) -> PRTIM0_R {
        PRTIM0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Power Reduction Timer/Counter2"]
    #[inline(always)]
    pub fn prtim2(&self) -> PRTIM2_R {
        PRTIM2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power Reduction TWI"]
    #[inline(always)]
    pub fn prtwi(&self) -> PRTWI_R {
        PRTWI_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Reduction ADC"]
    #[inline(always)]
    #[must_use]
    pub fn pradc(&mut self) -> PRADC_W<PRR0_SPEC> {
        PRADC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Power Reduction USART0"]
    #[inline(always)]
    #[must_use]
    pub fn prusart0(&mut self) -> PRUSART0_W<PRR0_SPEC> {
        PRUSART0_W::new(self, 1)
    }
    #[doc = "Bit 2 - Power Reduction Serial Peripheral Interface"]
    #[inline(always)]
    #[must_use]
    pub fn prspi(&mut self) -> PRSPI_W<PRR0_SPEC> {
        PRSPI_W::new(self, 2)
    }
    #[doc = "Bit 3 - Power Reduction Timer/Counter1"]
    #[inline(always)]
    #[must_use]
    pub fn prtim1(&mut self) -> PRTIM1_W<PRR0_SPEC> {
        PRTIM1_W::new(self, 3)
    }
    #[doc = "Bit 4 - Power Reduction USART1"]
    #[inline(always)]
    #[must_use]
    pub fn prusart1(&mut self) -> PRUSART1_W<PRR0_SPEC> {
        PRUSART1_W::new(self, 4)
    }
    #[doc = "Bit 5 - Power Reduction Timer/Counter0"]
    #[inline(always)]
    #[must_use]
    pub fn prtim0(&mut self) -> PRTIM0_W<PRR0_SPEC> {
        PRTIM0_W::new(self, 5)
    }
    #[doc = "Bit 6 - Power Reduction Timer/Counter2"]
    #[inline(always)]
    #[must_use]
    pub fn prtim2(&mut self) -> PRTIM2_W<PRR0_SPEC> {
        PRTIM2_W::new(self, 6)
    }
    #[doc = "Bit 7 - Power Reduction TWI"]
    #[inline(always)]
    #[must_use]
    pub fn prtwi(&mut self) -> PRTWI_W<PRR0_SPEC> {
        PRTWI_W::new(self, 7)
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
#[doc = "Power Reduction Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRR0_SPEC;
impl crate::RegisterSpec for PRR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`prr0::R`](R) reader structure"]
impl crate::Readable for PRR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prr0::W`](W) writer structure"]
impl crate::Writable for PRR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRR0 to value 0"]
impl crate::Resettable for PRR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
