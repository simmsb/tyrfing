#[doc = "Register `INTCTRL` reader"]
pub type R = crate::R<INTCTRL_SPEC>;
#[doc = "Register `INTCTRL` writer"]
pub type W = crate::W<INTCTRL_SPEC>;
#[doc = "Field `OVF` reader - Overflow interrupt enable"]
pub type OVF_R = crate::BitReader;
#[doc = "Field `OVF` writer - Overflow interrupt enable"]
pub type OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGA` reader - Trigger A interrupt enable"]
pub type TRIGA_R = crate::BitReader;
#[doc = "Field `TRIGA` writer - Trigger A interrupt enable"]
pub type TRIGA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGB` reader - Trigger B interrupt enable"]
pub type TRIGB_R = crate::BitReader;
#[doc = "Field `TRIGB` writer - Trigger B interrupt enable"]
pub type TRIGB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Trigger A interrupt enable"]
    #[inline(always)]
    pub fn triga(&self) -> TRIGA_R {
        TRIGA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trigger B interrupt enable"]
    #[inline(always)]
    pub fn trigb(&self) -> TRIGB_R {
        TRIGB_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<INTCTRL_SPEC> {
        OVF_W::new(self, 0)
    }
    #[doc = "Bit 2 - Trigger A interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn triga(&mut self) -> TRIGA_W<INTCTRL_SPEC> {
        TRIGA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Trigger B interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn trigb(&mut self) -> TRIGB_W<INTCTRL_SPEC> {
        TRIGB_W::new(self, 3)
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
#[doc = "Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTCTRL_SPEC;
impl crate::RegisterSpec for INTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intctrl::R`](R) reader structure"]
impl crate::Readable for INTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intctrl::W`](W) writer structure"]
impl crate::Writable for INTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCTRL to value 0"]
impl crate::Resettable for INTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
