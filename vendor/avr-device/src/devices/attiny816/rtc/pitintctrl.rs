#[doc = "Register `PITINTCTRL` reader"]
pub type R = crate::R<PITINTCTRL_SPEC>;
#[doc = "Register `PITINTCTRL` writer"]
pub type W = crate::W<PITINTCTRL_SPEC>;
#[doc = "Field `PI` reader - Periodic Interrupt"]
pub type PI_R = crate::BitReader;
#[doc = "Field `PI` writer - Periodic Interrupt"]
pub type PI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Periodic Interrupt"]
    #[inline(always)]
    pub fn pi(&self) -> PI_R {
        PI_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Periodic Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pi(&mut self) -> PI_W<PITINTCTRL_SPEC> {
        PI_W::new(self, 0)
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
#[doc = "PIT Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pitintctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pitintctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PITINTCTRL_SPEC;
impl crate::RegisterSpec for PITINTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pitintctrl::R`](R) reader structure"]
impl crate::Readable for PITINTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pitintctrl::W`](W) writer structure"]
impl crate::Writable for PITINTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PITINTCTRL to value 0"]
impl crate::Resettable for PITINTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
