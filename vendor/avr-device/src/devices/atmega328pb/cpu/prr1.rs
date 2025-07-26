#[doc = "Register `PRR1` reader"]
pub type R = crate::R<PRR1_SPEC>;
#[doc = "Register `PRR1` writer"]
pub type W = crate::W<PRR1_SPEC>;
#[doc = "Field `PRTIM3` reader - Power Reduction Timer/Counter3"]
pub type PRTIM3_R = crate::BitReader;
#[doc = "Field `PRTIM3` writer - Power Reduction Timer/Counter3"]
pub type PRTIM3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSPI1` reader - Power Reduction Serial Peripheral Interface 1"]
pub type PRSPI1_R = crate::BitReader;
#[doc = "Field `PRSPI1` writer - Power Reduction Serial Peripheral Interface 1"]
pub type PRSPI1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTIM4` reader - Power Reduction Timer/Counter4"]
pub type PRTIM4_R = crate::BitReader;
#[doc = "Field `PRTIM4` writer - Power Reduction Timer/Counter4"]
pub type PRTIM4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRPTC` reader - Power Reduction Peripheral Touch Controller"]
pub type PRPTC_R = crate::BitReader;
#[doc = "Field `PRPTC` writer - Power Reduction Peripheral Touch Controller"]
pub type PRPTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTWI1` reader - Power Reduction TWI1"]
pub type PRTWI1_R = crate::BitReader;
#[doc = "Field `PRTWI1` writer - Power Reduction TWI1"]
pub type PRTWI1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power Reduction Timer/Counter3"]
    #[inline(always)]
    pub fn prtim3(&self) -> PRTIM3_R {
        PRTIM3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Power Reduction Serial Peripheral Interface 1"]
    #[inline(always)]
    pub fn prspi1(&self) -> PRSPI1_R {
        PRSPI1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power Reduction Timer/Counter4"]
    #[inline(always)]
    pub fn prtim4(&self) -> PRTIM4_R {
        PRTIM4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power Reduction Peripheral Touch Controller"]
    #[inline(always)]
    pub fn prptc(&self) -> PRPTC_R {
        PRPTC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Power Reduction TWI1"]
    #[inline(always)]
    pub fn prtwi1(&self) -> PRTWI1_R {
        PRTWI1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Reduction Timer/Counter3"]
    #[inline(always)]
    #[must_use]
    pub fn prtim3(&mut self) -> PRTIM3_W<PRR1_SPEC> {
        PRTIM3_W::new(self, 0)
    }
    #[doc = "Bit 2 - Power Reduction Serial Peripheral Interface 1"]
    #[inline(always)]
    #[must_use]
    pub fn prspi1(&mut self) -> PRSPI1_W<PRR1_SPEC> {
        PRSPI1_W::new(self, 2)
    }
    #[doc = "Bit 3 - Power Reduction Timer/Counter4"]
    #[inline(always)]
    #[must_use]
    pub fn prtim4(&mut self) -> PRTIM4_W<PRR1_SPEC> {
        PRTIM4_W::new(self, 3)
    }
    #[doc = "Bit 4 - Power Reduction Peripheral Touch Controller"]
    #[inline(always)]
    #[must_use]
    pub fn prptc(&mut self) -> PRPTC_W<PRR1_SPEC> {
        PRPTC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Power Reduction TWI1"]
    #[inline(always)]
    #[must_use]
    pub fn prtwi1(&mut self) -> PRTWI1_W<PRR1_SPEC> {
        PRTWI1_W::new(self, 5)
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
#[doc = "Power Reduction Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRR1_SPEC;
impl crate::RegisterSpec for PRR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`prr1::R`](R) reader structure"]
impl crate::Readable for PRR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prr1::W`](W) writer structure"]
impl crate::Writable for PRR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRR1 to value 0"]
impl crate::Resettable for PRR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
