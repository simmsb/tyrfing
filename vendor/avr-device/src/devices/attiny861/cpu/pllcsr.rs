#[doc = "Register `PLLCSR` reader"]
pub type R = crate::R<PLLCSR_SPEC>;
#[doc = "Register `PLLCSR` writer"]
pub type W = crate::W<PLLCSR_SPEC>;
#[doc = "Field `PLOCK` reader - PLL Lock detector"]
pub type PLOCK_R = crate::BitReader;
#[doc = "Field `PLOCK` writer - PLL Lock detector"]
pub type PLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLE` reader - PLL Enable"]
pub type PLLE_R = crate::BitReader;
#[doc = "Field `PLLE` writer - PLL Enable"]
pub type PLLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKE` reader - PCK Enable"]
pub type PCKE_R = crate::BitReader;
#[doc = "Field `PCKE` writer - PCK Enable"]
pub type PCKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSM` reader - Low speed mode"]
pub type LSM_R = crate::BitReader;
#[doc = "Field `LSM` writer - Low speed mode"]
pub type LSM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PLL Lock detector"]
    #[inline(always)]
    pub fn plock(&self) -> PLOCK_R {
        PLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL Enable"]
    #[inline(always)]
    pub fn plle(&self) -> PLLE_R {
        PLLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCK Enable"]
    #[inline(always)]
    pub fn pcke(&self) -> PCKE_R {
        PCKE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Low speed mode"]
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Lock detector"]
    #[inline(always)]
    #[must_use]
    pub fn plock(&mut self) -> PLOCK_W<PLLCSR_SPEC> {
        PLOCK_W::new(self, 0)
    }
    #[doc = "Bit 1 - PLL Enable"]
    #[inline(always)]
    #[must_use]
    pub fn plle(&mut self) -> PLLE_W<PLLCSR_SPEC> {
        PLLE_W::new(self, 1)
    }
    #[doc = "Bit 2 - PCK Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcke(&mut self) -> PCKE_W<PLLCSR_SPEC> {
        PCKE_W::new(self, 2)
    }
    #[doc = "Bit 7 - Low speed mode"]
    #[inline(always)]
    #[must_use]
    pub fn lsm(&mut self) -> LSM_W<PLLCSR_SPEC> {
        LSM_W::new(self, 7)
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
#[doc = "PLL Control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLCSR_SPEC;
impl crate::RegisterSpec for PLLCSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pllcsr::R`](R) reader structure"]
impl crate::Readable for PLLCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllcsr::W`](W) writer structure"]
impl crate::Writable for PLLCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCSR to value 0"]
impl crate::Resettable for PLLCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
