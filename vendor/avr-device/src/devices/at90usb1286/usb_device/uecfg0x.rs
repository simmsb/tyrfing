#[doc = "Register `UECFG0X` reader"]
pub type R = crate::R<UECFG0X_SPEC>;
#[doc = "Register `UECFG0X` writer"]
pub type W = crate::W<UECFG0X_SPEC>;
#[doc = "Field `EPDIR` reader - No Description."]
pub type EPDIR_R = crate::BitReader;
#[doc = "Field `EPDIR` writer - No Description."]
pub type EPDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTYPE` reader - No Description."]
pub type EPTYPE_R = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - No Description."]
pub type EPTYPE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 6:7 - No Description."]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EPDIR_W<UECFG0X_SPEC> {
        EPDIR_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<UECFG0X_SPEC> {
        EPTYPE_W::new(self, 6)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uecfg0x::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uecfg0x::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UECFG0X_SPEC;
impl crate::RegisterSpec for UECFG0X_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uecfg0x::R`](R) reader structure"]
impl crate::Readable for UECFG0X_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uecfg0x::W`](W) writer structure"]
impl crate::Writable for UECFG0X_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UECFG0X to value 0"]
impl crate::Resettable for UECFG0X_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
