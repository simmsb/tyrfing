#[doc = "Register `RCCTRL` reader"]
pub type R = crate::R<RCCTRL_SPEC>;
#[doc = "Register `RCCTRL` writer"]
pub type W = crate::W<RCCTRL_SPEC>;
#[doc = "Field `RCFREQ` reader - No Description."]
pub type RCFREQ_R = crate::BitReader;
#[doc = "Field `RCFREQ` writer - No Description."]
pub type RCFREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn rcfreq(&self) -> RCFREQ_R {
        RCFREQ_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn rcfreq(&mut self) -> RCFREQ_W<RCCTRL_SPEC> {
        RCFREQ_W::new(self, 0)
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
#[doc = "Oscillator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCCTRL_SPEC;
impl crate::RegisterSpec for RCCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rcctrl::R`](R) reader structure"]
impl crate::Readable for RCCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcctrl::W`](W) writer structure"]
impl crate::Writable for RCCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCCTRL to value 0"]
impl crate::Resettable for RCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
