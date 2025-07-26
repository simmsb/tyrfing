#[doc = "Register `SFIOR` reader"]
pub type R = crate::R<SFIOR_SPEC>;
#[doc = "Register `SFIOR` writer"]
pub type W = crate::W<SFIOR_SPEC>;
#[doc = "Field `PSR0` reader - Prescaler Reset Timer/Counter0"]
pub type PSR0_R = crate::BitReader;
#[doc = "Field `PSR0` writer - Prescaler Reset Timer/Counter0"]
pub type PSR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSM` reader - Timer/Counter Synchronization Mode"]
pub type TSM_R = crate::BitReader;
#[doc = "Field `TSM` writer - Timer/Counter Synchronization Mode"]
pub type TSM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Prescaler Reset Timer/Counter0"]
    #[inline(always)]
    pub fn psr0(&self) -> PSR0_R {
        PSR0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer/Counter Synchronization Mode"]
    #[inline(always)]
    pub fn tsm(&self) -> TSM_R {
        TSM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Prescaler Reset Timer/Counter0"]
    #[inline(always)]
    #[must_use]
    pub fn psr0(&mut self) -> PSR0_W<SFIOR_SPEC> {
        PSR0_W::new(self, 1)
    }
    #[doc = "Bit 7 - Timer/Counter Synchronization Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tsm(&mut self) -> TSM_W<SFIOR_SPEC> {
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
#[doc = "Special Function IO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfior::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfior::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFIOR_SPEC;
impl crate::RegisterSpec for SFIOR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sfior::R`](R) reader structure"]
impl crate::Readable for SFIOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sfior::W`](W) writer structure"]
impl crate::Writable for SFIOR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFIOR to value 0"]
impl crate::Resettable for SFIOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
