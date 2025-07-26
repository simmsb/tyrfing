#[doc = "Register `MCUSR` reader"]
pub type R = crate::R<MCUSR_SPEC>;
#[doc = "Register `MCUSR` writer"]
pub type W = crate::W<MCUSR_SPEC>;
#[doc = "Field `PORF` reader - Power-on reset flag"]
pub type PORF_R = crate::BitReader;
#[doc = "Field `PORF` writer - Power-on reset flag"]
pub type PORF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTRF` reader - External Reset Flag"]
pub type EXTRF_R = crate::BitReader;
#[doc = "Field `EXTRF` writer - External Reset Flag"]
pub type EXTRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BORF` reader - Brown-out Reset Flag"]
pub type BORF_R = crate::BitReader;
#[doc = "Field `BORF` writer - Brown-out Reset Flag"]
pub type BORF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDRF` reader - Watchdog Reset Flag"]
pub type WDRF_R = crate::BitReader;
#[doc = "Field `WDRF` writer - Watchdog Reset Flag"]
pub type WDRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power-on reset flag"]
    #[inline(always)]
    pub fn porf(&self) -> PORF_R {
        PORF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Reset Flag"]
    #[inline(always)]
    pub fn extrf(&self) -> EXTRF_R {
        EXTRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Brown-out Reset Flag"]
    #[inline(always)]
    pub fn borf(&self) -> BORF_R {
        BORF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog Reset Flag"]
    #[inline(always)]
    pub fn wdrf(&self) -> WDRF_R {
        WDRF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power-on reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn porf(&mut self) -> PORF_W<MCUSR_SPEC> {
        PORF_W::new(self, 0)
    }
    #[doc = "Bit 1 - External Reset Flag"]
    #[inline(always)]
    #[must_use]
    pub fn extrf(&mut self) -> EXTRF_W<MCUSR_SPEC> {
        EXTRF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Brown-out Reset Flag"]
    #[inline(always)]
    #[must_use]
    pub fn borf(&mut self) -> BORF_W<MCUSR_SPEC> {
        BORF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Watchdog Reset Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdrf(&mut self) -> WDRF_W<MCUSR_SPEC> {
        WDRF_W::new(self, 3)
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
#[doc = "MCU Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcusr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcusr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCUSR_SPEC;
impl crate::RegisterSpec for MCUSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mcusr::R`](R) reader structure"]
impl crate::Readable for MCUSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcusr::W`](W) writer structure"]
impl crate::Writable for MCUSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUSR to value 0"]
impl crate::Resettable for MCUSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
