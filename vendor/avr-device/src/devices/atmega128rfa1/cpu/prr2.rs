#[doc = "Register `PRR2` reader"]
pub type R = crate::R<PRR2_SPEC>;
#[doc = "Register `PRR2` writer"]
pub type W = crate::W<PRR2_SPEC>;
#[doc = "Field `PRRAM0` reader - Power Reduction SRAM0"]
pub type PRRAM0_R = crate::BitReader;
#[doc = "Field `PRRAM0` writer - Power Reduction SRAM0"]
pub type PRRAM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRRAM1` reader - Power Reduction SRAM1"]
pub type PRRAM1_R = crate::BitReader;
#[doc = "Field `PRRAM1` writer - Power Reduction SRAM1"]
pub type PRRAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRRAM2` reader - Power Reduction SRAM2"]
pub type PRRAM2_R = crate::BitReader;
#[doc = "Field `PRRAM2` writer - Power Reduction SRAM2"]
pub type PRRAM2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRRAM3` reader - Power Reduction SRAM3"]
pub type PRRAM3_R = crate::BitReader;
#[doc = "Field `PRRAM3` writer - Power Reduction SRAM3"]
pub type PRRAM3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power Reduction SRAM0"]
    #[inline(always)]
    pub fn prram0(&self) -> PRRAM0_R {
        PRRAM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Reduction SRAM1"]
    #[inline(always)]
    pub fn prram1(&self) -> PRRAM1_R {
        PRRAM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Reduction SRAM2"]
    #[inline(always)]
    pub fn prram2(&self) -> PRRAM2_R {
        PRRAM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power Reduction SRAM3"]
    #[inline(always)]
    pub fn prram3(&self) -> PRRAM3_R {
        PRRAM3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Reduction SRAM0"]
    #[inline(always)]
    #[must_use]
    pub fn prram0(&mut self) -> PRRAM0_W<PRR2_SPEC> {
        PRRAM0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Power Reduction SRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn prram1(&mut self) -> PRRAM1_W<PRR2_SPEC> {
        PRRAM1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Power Reduction SRAM2"]
    #[inline(always)]
    #[must_use]
    pub fn prram2(&mut self) -> PRRAM2_W<PRR2_SPEC> {
        PRRAM2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Power Reduction SRAM3"]
    #[inline(always)]
    #[must_use]
    pub fn prram3(&mut self) -> PRRAM3_W<PRR2_SPEC> {
        PRRAM3_W::new(self, 3)
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
#[doc = "Power Reduction Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRR2_SPEC;
impl crate::RegisterSpec for PRR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`prr2::R`](R) reader structure"]
impl crate::Readable for PRR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prr2::W`](W) writer structure"]
impl crate::Writable for PRR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRR2 to value 0"]
impl crate::Resettable for PRR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
