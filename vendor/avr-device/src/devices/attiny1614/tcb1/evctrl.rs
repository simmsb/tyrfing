#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EVCTRL_SPEC>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EVCTRL_SPEC>;
#[doc = "Field `CAPTEI` reader - Event Input Enable"]
pub type CAPTEI_R = crate::BitReader;
#[doc = "Field `CAPTEI` writer - Event Input Enable"]
pub type CAPTEI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE` reader - Event Edge"]
pub type EDGE_R = crate::BitReader;
#[doc = "Field `EDGE` writer - Event Edge"]
pub type EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILTER` reader - Input Capture Noise Cancellation Filter"]
pub type FILTER_R = crate::BitReader;
#[doc = "Field `FILTER` writer - Input Capture Noise Cancellation Filter"]
pub type FILTER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Event Input Enable"]
    #[inline(always)]
    pub fn captei(&self) -> CAPTEI_R {
        CAPTEI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Event Edge"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Input Capture Noise Cancellation Filter"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn captei(&mut self) -> CAPTEI_W<EVCTRL_SPEC> {
        CAPTEI_W::new(self, 0)
    }
    #[doc = "Bit 4 - Event Edge"]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EDGE_W<EVCTRL_SPEC> {
        EDGE_W::new(self, 4)
    }
    #[doc = "Bit 6 - Input Capture Noise Cancellation Filter"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FILTER_W<EVCTRL_SPEC> {
        FILTER_W::new(self, 6)
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
#[doc = "Event Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EVCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EVCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
