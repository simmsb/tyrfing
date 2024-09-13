#[doc = "Register `FAULTCTRL` reader"]
pub type R = crate::R<FAULTCTRL_SPEC>;
#[doc = "Register `FAULTCTRL` writer"]
pub type W = crate::W<FAULTCTRL_SPEC>;
#[doc = "Field `CMPA` reader - Compare A value"]
pub type CMPA_R = crate::BitReader;
#[doc = "Field `CMPA` writer - Compare A value"]
pub type CMPA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPB` reader - Compare B value"]
pub type CMPB_R = crate::BitReader;
#[doc = "Field `CMPB` writer - Compare B value"]
pub type CMPB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPC` reader - Compare C value"]
pub type CMPC_R = crate::BitReader;
#[doc = "Field `CMPC` writer - Compare C value"]
pub type CMPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPD` reader - Compare D vaule"]
pub type CMPD_R = crate::BitReader;
#[doc = "Field `CMPD` writer - Compare D vaule"]
pub type CMPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPAEN` reader - Compare A enable"]
pub type CMPAEN_R = crate::BitReader;
#[doc = "Field `CMPAEN` writer - Compare A enable"]
pub type CMPAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPBEN` reader - Compare B enable"]
pub type CMPBEN_R = crate::BitReader;
#[doc = "Field `CMPBEN` writer - Compare B enable"]
pub type CMPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPCEN` reader - Compare C enable"]
pub type CMPCEN_R = crate::BitReader;
#[doc = "Field `CMPCEN` writer - Compare C enable"]
pub type CMPCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPDEN` reader - Compare D enable"]
pub type CMPDEN_R = crate::BitReader;
#[doc = "Field `CMPDEN` writer - Compare D enable"]
pub type CMPDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Compare A value"]
    #[inline(always)]
    pub fn cmpa(&self) -> CMPA_R {
        CMPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare B value"]
    #[inline(always)]
    pub fn cmpb(&self) -> CMPB_R {
        CMPB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare C value"]
    #[inline(always)]
    pub fn cmpc(&self) -> CMPC_R {
        CMPC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare D vaule"]
    #[inline(always)]
    pub fn cmpd(&self) -> CMPD_R {
        CMPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare A enable"]
    #[inline(always)]
    pub fn cmpaen(&self) -> CMPAEN_R {
        CMPAEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare B enable"]
    #[inline(always)]
    pub fn cmpben(&self) -> CMPBEN_R {
        CMPBEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare C enable"]
    #[inline(always)]
    pub fn cmpcen(&self) -> CMPCEN_R {
        CMPCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare D enable"]
    #[inline(always)]
    pub fn cmpden(&self) -> CMPDEN_R {
        CMPDEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare A value"]
    #[inline(always)]
    #[must_use]
    pub fn cmpa(&mut self) -> CMPA_W<FAULTCTRL_SPEC> {
        CMPA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Compare B value"]
    #[inline(always)]
    #[must_use]
    pub fn cmpb(&mut self) -> CMPB_W<FAULTCTRL_SPEC> {
        CMPB_W::new(self, 1)
    }
    #[doc = "Bit 2 - Compare C value"]
    #[inline(always)]
    #[must_use]
    pub fn cmpc(&mut self) -> CMPC_W<FAULTCTRL_SPEC> {
        CMPC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Compare D vaule"]
    #[inline(always)]
    #[must_use]
    pub fn cmpd(&mut self) -> CMPD_W<FAULTCTRL_SPEC> {
        CMPD_W::new(self, 3)
    }
    #[doc = "Bit 4 - Compare A enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpaen(&mut self) -> CMPAEN_W<FAULTCTRL_SPEC> {
        CMPAEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Compare B enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpben(&mut self) -> CMPBEN_W<FAULTCTRL_SPEC> {
        CMPBEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Compare C enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcen(&mut self) -> CMPCEN_W<FAULTCTRL_SPEC> {
        CMPCEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Compare D enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpden(&mut self) -> CMPDEN_W<FAULTCTRL_SPEC> {
        CMPDEN_W::new(self, 7)
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
#[doc = "Fault Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`faultctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`faultctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FAULTCTRL_SPEC;
impl crate::RegisterSpec for FAULTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`faultctrl::R`](R) reader structure"]
impl crate::Readable for FAULTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`faultctrl::W`](W) writer structure"]
impl crate::Writable for FAULTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FAULTCTRL to value 0"]
impl crate::Resettable for FAULTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
