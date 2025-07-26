#[doc = "Register `UHWCON` reader"]
pub type R = crate::R<UHWCON_SPEC>;
#[doc = "Register `UHWCON` writer"]
pub type W = crate::W<UHWCON_SPEC>;
#[doc = "Field `UVREGE` reader - No Description."]
pub type UVREGE_R = crate::BitReader;
#[doc = "Field `UVREGE` writer - No Description."]
pub type UVREGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UVCONE` reader - No Description."]
pub type UVCONE_R = crate::BitReader;
#[doc = "Field `UVCONE` writer - No Description."]
pub type UVCONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UIDE` reader - No Description."]
pub type UIDE_R = crate::BitReader;
#[doc = "Field `UIDE` writer - No Description."]
pub type UIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UIMOD` reader - No Description."]
pub type UIMOD_R = crate::BitReader;
#[doc = "Field `UIMOD` writer - No Description."]
pub type UIMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn uvrege(&self) -> UVREGE_R {
        UVREGE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    pub fn uvcone(&self) -> UVCONE_R {
        UVCONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    pub fn uide(&self) -> UIDE_R {
        UIDE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    pub fn uimod(&self) -> UIMOD_R {
        UIMOD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn uvrege(&mut self) -> UVREGE_W<UHWCON_SPEC> {
        UVREGE_W::new(self, 0)
    }
    #[doc = "Bit 4 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn uvcone(&mut self) -> UVCONE_W<UHWCON_SPEC> {
        UVCONE_W::new(self, 4)
    }
    #[doc = "Bit 6 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn uide(&mut self) -> UIDE_W<UHWCON_SPEC> {
        UIDE_W::new(self, 6)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn uimod(&mut self) -> UIMOD_W<UHWCON_SPEC> {
        UIMOD_W::new(self, 7)
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
#[doc = "USB Hardware Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhwcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhwcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UHWCON_SPEC;
impl crate::RegisterSpec for UHWCON_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uhwcon::R`](R) reader structure"]
impl crate::Readable for UHWCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uhwcon::W`](W) writer structure"]
impl crate::Writable for UHWCON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UHWCON to value 0"]
impl crate::Resettable for UHWCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
