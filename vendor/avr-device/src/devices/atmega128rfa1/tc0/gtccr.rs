#[doc = "Register `GTCCR` reader"]
pub type R = crate::R<GTCCR_SPEC>;
#[doc = "Register `GTCCR` writer"]
pub type W = crate::W<GTCCR_SPEC>;
#[doc = "Field `PSRSYNC` reader - Prescaler Reset for Synchronous Timer/Counters"]
pub type PSRSYNC_R = crate::BitReader;
#[doc = "Field `PSRSYNC` writer - Prescaler Reset for Synchronous Timer/Counters"]
pub type PSRSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRASY` reader - Prescaler Reset Timer/Counter2"]
pub type PSRASY_R = crate::BitReader;
#[doc = "Field `PSRASY` writer - Prescaler Reset Timer/Counter2"]
pub type PSRASY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `TSM` reader - Timer/Counter Synchronization Mode"]
pub type TSM_R = crate::BitReader;
#[doc = "Field `TSM` writer - Timer/Counter Synchronization Mode"]
pub type TSM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Prescaler Reset for Synchronous Timer/Counters"]
    #[inline(always)]
    pub fn psrsync(&self) -> PSRSYNC_R {
        PSRSYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Prescaler Reset Timer/Counter2"]
    #[inline(always)]
    pub fn psrasy(&self) -> PSRASY_R {
        PSRASY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 2) & 0x1f)
    }
    #[doc = "Bit 7 - Timer/Counter Synchronization Mode"]
    #[inline(always)]
    pub fn tsm(&self) -> TSM_R {
        TSM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Prescaler Reset for Synchronous Timer/Counters"]
    #[inline(always)]
    #[must_use]
    pub fn psrsync(&mut self) -> PSRSYNC_W<GTCCR_SPEC> {
        PSRSYNC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Prescaler Reset Timer/Counter2"]
    #[inline(always)]
    #[must_use]
    pub fn psrasy(&mut self) -> PSRASY_W<GTCCR_SPEC> {
        PSRASY_W::new(self, 1)
    }
    #[doc = "Bits 2:6 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<GTCCR_SPEC> {
        RES_W::new(self, 2)
    }
    #[doc = "Bit 7 - Timer/Counter Synchronization Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tsm(&mut self) -> TSM_W<GTCCR_SPEC> {
        TSM_W::new(self, 7)
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
#[doc = "General Timer/Counter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTCCR_SPEC;
impl crate::RegisterSpec for GTCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gtccr::R`](R) reader structure"]
impl crate::Readable for GTCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtccr::W`](W) writer structure"]
impl crate::Writable for GTCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCCR to value 0"]
impl crate::Resettable for GTCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
