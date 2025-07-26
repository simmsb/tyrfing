#[doc = "Register `XFDCSR` reader"]
pub type R = crate::R<XFDCSR_SPEC>;
#[doc = "Register `XFDCSR` writer"]
pub type W = crate::W<XFDCSR_SPEC>;
#[doc = "Field `XFDIE` reader - Failure Detection Interrupt Enable"]
pub type XFDIE_R = crate::BitReader;
#[doc = "Field `XFDIE` writer - Failure Detection Interrupt Enable"]
pub type XFDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFDIF` reader - Failure Detection Interrupt Flag"]
pub type XFDIF_R = crate::BitReader;
#[doc = "Field `XFDIF` writer - Failure Detection Interrupt Flag"]
pub type XFDIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Failure Detection Interrupt Enable"]
    #[inline(always)]
    pub fn xfdie(&self) -> XFDIE_R {
        XFDIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Failure Detection Interrupt Flag"]
    #[inline(always)]
    pub fn xfdif(&self) -> XFDIF_R {
        XFDIF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Failure Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xfdie(&mut self) -> XFDIE_W<XFDCSR_SPEC> {
        XFDIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Failure Detection Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn xfdif(&mut self) -> XFDIF_W<XFDCSR_SPEC> {
        XFDIF_W::new(self, 1)
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
#[doc = "XOSC Failure Detection Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xfdcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xfdcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XFDCSR_SPEC;
impl crate::RegisterSpec for XFDCSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xfdcsr::R`](R) reader structure"]
impl crate::Readable for XFDCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xfdcsr::W`](W) writer structure"]
impl crate::Writable for XFDCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XFDCSR to value 0"]
impl crate::Resettable for XFDCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
