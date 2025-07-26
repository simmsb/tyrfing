#[doc = "Register `PITDBGCTRL` reader"]
pub type R = crate::R<PITDBGCTRL_SPEC>;
#[doc = "Register `PITDBGCTRL` writer"]
pub type W = crate::W<PITDBGCTRL_SPEC>;
#[doc = "Field `DBGRUN` reader - Run in debug"]
pub type DBGRUN_R = crate::BitReader;
#[doc = "Field `DBGRUN` writer - Run in debug"]
pub type DBGRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Run in debug"]
    #[inline(always)]
    pub fn dbgrun(&self) -> DBGRUN_R {
        DBGRUN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Run in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbgrun(&mut self) -> DBGRUN_W<PITDBGCTRL_SPEC> {
        DBGRUN_W::new(self, 0)
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
#[doc = "PIT Debug control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pitdbgctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pitdbgctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PITDBGCTRL_SPEC;
impl crate::RegisterSpec for PITDBGCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pitdbgctrl::R`](R) reader structure"]
impl crate::Readable for PITDBGCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pitdbgctrl::W`](W) writer structure"]
impl crate::Writable for PITDBGCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PITDBGCTRL to value 0"]
impl crate::Resettable for PITDBGCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
