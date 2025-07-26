#[doc = "Register `MCUCR` reader"]
pub type R = crate::R<MCUCR_SPEC>;
#[doc = "Register `MCUCR` writer"]
pub type W = crate::W<MCUCR_SPEC>;
#[doc = "Field `PUD` reader - Pull-up Disable"]
pub type PUD_R = crate::BitReader;
#[doc = "Field `PUD` writer - Pull-up Disable"]
pub type PUD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODSE` reader - BOD Sleep Enable"]
pub type BODSE_R = crate::BitReader;
#[doc = "Field `BODSE` writer - BOD Sleep Enable"]
pub type BODSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODS` reader - BOD Sleep"]
pub type BODS_R = crate::BitReader;
#[doc = "Field `BODS` writer - BOD Sleep"]
pub type BODS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Pull-up Disable"]
    #[inline(always)]
    pub fn pud(&self) -> PUD_R {
        PUD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BOD Sleep Enable"]
    #[inline(always)]
    pub fn bodse(&self) -> BODSE_R {
        BODSE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BOD Sleep"]
    #[inline(always)]
    pub fn bods(&self) -> BODS_R {
        BODS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Pull-up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pud(&mut self) -> PUD_W<MCUCR_SPEC> {
        PUD_W::new(self, 4)
    }
    #[doc = "Bit 5 - BOD Sleep Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bodse(&mut self) -> BODSE_W<MCUCR_SPEC> {
        BODSE_W::new(self, 5)
    }
    #[doc = "Bit 6 - BOD Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn bods(&mut self) -> BODS_W<MCUCR_SPEC> {
        BODS_W::new(self, 6)
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
#[doc = "MCU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCUCR_SPEC;
impl crate::RegisterSpec for MCUCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mcucr::R`](R) reader structure"]
impl crate::Readable for MCUCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcucr::W`](W) writer structure"]
impl crate::Writable for MCUCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUCR to value 0"]
impl crate::Resettable for MCUCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
