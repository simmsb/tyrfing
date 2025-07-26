#[doc = "Register `TCD0CFG` reader"]
pub type R = crate::R<TCD0CFG_SPEC>;
#[doc = "Field `CMPA` reader - Compare A Default Output Value"]
pub type CMPA_R = crate::BitReader;
#[doc = "Field `CMPB` reader - Compare B Default Output Value"]
pub type CMPB_R = crate::BitReader;
#[doc = "Field `CMPC` reader - Compare C Default Output Value"]
pub type CMPC_R = crate::BitReader;
#[doc = "Field `CMPD` reader - Compare D Default Output Value"]
pub type CMPD_R = crate::BitReader;
#[doc = "Field `CMPAEN` reader - Compare A Output Enable"]
pub type CMPAEN_R = crate::BitReader;
#[doc = "Field `CMPBEN` reader - Compare B Output Enable"]
pub type CMPBEN_R = crate::BitReader;
#[doc = "Field `CMPCEN` reader - Compare C Output Enable"]
pub type CMPCEN_R = crate::BitReader;
#[doc = "Field `CMPDEN` reader - Compare D Output Enable"]
pub type CMPDEN_R = crate::BitReader;
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
#[doc = "TCD0 Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcd0cfg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCD0CFG_SPEC;
impl crate::RegisterSpec for TCD0CFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tcd0cfg::R`](R) reader structure"]
impl crate::Readable for TCD0CFG_SPEC {}
#[doc = "`reset()` method sets TCD0CFG to value 0"]
impl crate::Resettable for TCD0CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
