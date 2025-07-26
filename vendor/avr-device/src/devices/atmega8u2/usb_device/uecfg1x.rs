#[doc = "Register `UECFG1X` reader"]
pub type R = crate::R<UECFG1X_SPEC>;
#[doc = "Register `UECFG1X` writer"]
pub type W = crate::W<UECFG1X_SPEC>;
#[doc = "Field `ALLOC` reader - Endpoint Allocation Bit"]
pub type ALLOC_R = crate::BitReader;
#[doc = "Field `ALLOC` writer - Endpoint Allocation Bit"]
pub type ALLOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPBK` reader - Endpoint Bank Bits"]
pub type EPBK_R = crate::FieldReader;
#[doc = "Field `EPBK` writer - Endpoint Bank Bits"]
pub type EPBK_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `EPSIZE` reader - Endpoint Size Bits"]
pub type EPSIZE_R = crate::FieldReader;
#[doc = "Field `EPSIZE` writer - Endpoint Size Bits"]
pub type EPSIZE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
impl R {
    #[doc = "Bit 1 - Endpoint Allocation Bit"]
    #[inline(always)]
    pub fn alloc(&self) -> ALLOC_R {
        ALLOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Endpoint Bank Bits"]
    #[inline(always)]
    pub fn epbk(&self) -> EPBK_R {
        EPBK_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:6 - Endpoint Size Bits"]
    #[inline(always)]
    pub fn epsize(&self) -> EPSIZE_R {
        EPSIZE_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bit 1 - Endpoint Allocation Bit"]
    #[inline(always)]
    #[must_use]
    pub fn alloc(&mut self) -> ALLOC_W<UECFG1X_SPEC> {
        ALLOC_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Endpoint Bank Bits"]
    #[inline(always)]
    #[must_use]
    pub fn epbk(&mut self) -> EPBK_W<UECFG1X_SPEC> {
        EPBK_W::new(self, 2)
    }
    #[doc = "Bits 4:6 - Endpoint Size Bits"]
    #[inline(always)]
    #[must_use]
    pub fn epsize(&mut self) -> EPSIZE_W<UECFG1X_SPEC> {
        EPSIZE_W::new(self, 4)
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
#[doc = "USB Endpoint Configuration 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uecfg1x::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uecfg1x::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UECFG1X_SPEC;
impl crate::RegisterSpec for UECFG1X_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uecfg1x::R`](R) reader structure"]
impl crate::Readable for UECFG1X_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uecfg1x::W`](W) writer structure"]
impl crate::Writable for UECFG1X_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UECFG1X to value 0"]
impl crate::Resettable for UECFG1X_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
