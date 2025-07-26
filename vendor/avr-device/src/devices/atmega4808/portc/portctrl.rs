#[doc = "Register `PORTCTRL` reader"]
pub type R = crate::R<PORTCTRL_SPEC>;
#[doc = "Register `PORTCTRL` writer"]
pub type W = crate::W<PORTCTRL_SPEC>;
#[doc = "Field `SRL` reader - Slew Rate Limit Enable"]
pub type SRL_R = crate::BitReader;
#[doc = "Field `SRL` writer - Slew Rate Limit Enable"]
pub type SRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Slew Rate Limit Enable"]
    #[inline(always)]
    pub fn srl(&self) -> SRL_R {
        SRL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slew Rate Limit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn srl(&mut self) -> SRL_W<PORTCTRL_SPEC> {
        SRL_W::new(self, 0)
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
#[doc = "Port Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PORTCTRL_SPEC;
impl crate::RegisterSpec for PORTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`portctrl::R`](R) reader structure"]
impl crate::Readable for PORTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`portctrl::W`](W) writer structure"]
impl crate::Writable for PORTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORTCTRL to value 0"]
impl crate::Resettable for PORTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
