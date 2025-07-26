#[doc = "Register `PRR` reader"]
pub type R = crate::R<PRR_SPEC>;
#[doc = "Register `PRR` writer"]
pub type W = crate::W<PRR_SPEC>;
#[doc = "Field `PRADC` reader - Power Reduction ADC"]
pub type PRADC_R = crate::BitReader;
#[doc = "Field `PRADC` writer - Power Reduction ADC"]
pub type PRADC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTIM0` reader - Power Reduction Timer/Counter0"]
pub type PRTIM0_R = crate::BitReader;
#[doc = "Field `PRTIM0` writer - Power Reduction Timer/Counter0"]
pub type PRTIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power Reduction ADC"]
    #[inline(always)]
    pub fn pradc(&self) -> PRADC_R {
        PRADC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Reduction Timer/Counter0"]
    #[inline(always)]
    pub fn prtim0(&self) -> PRTIM0_R {
        PRTIM0_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Reduction ADC"]
    #[inline(always)]
    #[must_use]
    pub fn pradc(&mut self) -> PRADC_W<PRR_SPEC> {
        PRADC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Power Reduction Timer/Counter0"]
    #[inline(always)]
    #[must_use]
    pub fn prtim0(&mut self) -> PRTIM0_W<PRR_SPEC> {
        PRTIM0_W::new(self, 1)
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
#[doc = "Power Reduction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRR_SPEC;
impl crate::RegisterSpec for PRR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`prr::R`](R) reader structure"]
impl crate::Readable for PRR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prr::W`](W) writer structure"]
impl crate::Writable for PRR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRR to value 0"]
impl crate::Resettable for PRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
