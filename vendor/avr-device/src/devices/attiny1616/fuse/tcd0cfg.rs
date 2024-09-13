#[doc = "Register `TCD0CFG` reader"]
pub type R = crate::R<TCD0CFG_SPEC>;
#[doc = "Register `TCD0CFG` writer"]
pub type W = crate::W<TCD0CFG_SPEC>;
#[doc = "Field `CMPA` reader - Compare A Default Output Value"]
pub type CMPA_R = crate::BitReader;
#[doc = "Field `CMPA` writer - Compare A Default Output Value"]
pub type CMPA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPB` reader - Compare B Default Output Value"]
pub type CMPB_R = crate::BitReader;
#[doc = "Field `CMPB` writer - Compare B Default Output Value"]
pub type CMPB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPC` reader - Compare C Default Output Value"]
pub type CMPC_R = crate::BitReader;
#[doc = "Field `CMPC` writer - Compare C Default Output Value"]
pub type CMPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPD` reader - Compare D Default Output Value"]
pub type CMPD_R = crate::BitReader;
#[doc = "Field `CMPD` writer - Compare D Default Output Value"]
pub type CMPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPAEN` reader - Compare A Output Enable"]
pub type CMPAEN_R = crate::BitReader;
#[doc = "Field `CMPAEN` writer - Compare A Output Enable"]
pub type CMPAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPBEN` reader - Compare B Output Enable"]
pub type CMPBEN_R = crate::BitReader;
#[doc = "Field `CMPBEN` writer - Compare B Output Enable"]
pub type CMPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPCEN` reader - Compare C Output Enable"]
pub type CMPCEN_R = crate::BitReader;
#[doc = "Field `CMPCEN` writer - Compare C Output Enable"]
pub type CMPCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPDEN` reader - Compare D Output Enable"]
pub type CMPDEN_R = crate::BitReader;
#[doc = "Field `CMPDEN` writer - Compare D Output Enable"]
pub type CMPDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Compare A Default Output Value"]
    #[inline(always)]
    pub fn cmpa(&self) -> CMPA_R {
        CMPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare B Default Output Value"]
    #[inline(always)]
    pub fn cmpb(&self) -> CMPB_R {
        CMPB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare C Default Output Value"]
    #[inline(always)]
    pub fn cmpc(&self) -> CMPC_R {
        CMPC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare D Default Output Value"]
    #[inline(always)]
    pub fn cmpd(&self) -> CMPD_R {
        CMPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare A Output Enable"]
    #[inline(always)]
    pub fn cmpaen(&self) -> CMPAEN_R {
        CMPAEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare B Output Enable"]
    #[inline(always)]
    pub fn cmpben(&self) -> CMPBEN_R {
        CMPBEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare C Output Enable"]
    #[inline(always)]
    pub fn cmpcen(&self) -> CMPCEN_R {
        CMPCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare D Output Enable"]
    #[inline(always)]
    pub fn cmpden(&self) -> CMPDEN_R {
        CMPDEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare A Default Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmpa(&mut self) -> CMPA_W<TCD0CFG_SPEC> {
        CMPA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Compare B Default Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmpb(&mut self) -> CMPB_W<TCD0CFG_SPEC> {
        CMPB_W::new(self, 1)
    }
    #[doc = "Bit 2 - Compare C Default Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmpc(&mut self) -> CMPC_W<TCD0CFG_SPEC> {
        CMPC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Compare D Default Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmpd(&mut self) -> CMPD_W<TCD0CFG_SPEC> {
        CMPD_W::new(self, 3)
    }
    #[doc = "Bit 4 - Compare A Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpaen(&mut self) -> CMPAEN_W<TCD0CFG_SPEC> {
        CMPAEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Compare B Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpben(&mut self) -> CMPBEN_W<TCD0CFG_SPEC> {
        CMPBEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Compare C Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcen(&mut self) -> CMPCEN_W<TCD0CFG_SPEC> {
        CMPCEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Compare D Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpden(&mut self) -> CMPDEN_W<TCD0CFG_SPEC> {
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
#[doc = "TCD0 Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcd0cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcd0cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCD0CFG_SPEC;
impl crate::RegisterSpec for TCD0CFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tcd0cfg::R`](R) reader structure"]
impl crate::Readable for TCD0CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcd0cfg::W`](W) writer structure"]
impl crate::Writable for TCD0CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCD0CFG to value 0"]
impl crate::Resettable for TCD0CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
