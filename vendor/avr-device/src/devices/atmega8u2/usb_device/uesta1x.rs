#[doc = "Register `UESTA1X` reader"]
pub type R = crate::R<UESTA1X_SPEC>;
#[doc = "Register `UESTA1X` writer"]
pub type W = crate::W<UESTA1X_SPEC>;
#[doc = "Field `CURRBK` reader - Current Bank"]
pub type CURRBK_R = crate::FieldReader;
#[doc = "Field `CURRBK` writer - Current Bank"]
pub type CURRBK_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `CTRLDIR` reader - Control Direction"]
pub type CTRLDIR_R = crate::BitReader;
#[doc = "Field `CTRLDIR` writer - Control Direction"]
pub type CTRLDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Current Bank"]
    #[inline(always)]
    pub fn currbk(&self) -> CURRBK_R {
        CURRBK_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Control Direction"]
    #[inline(always)]
    pub fn ctrldir(&self) -> CTRLDIR_R {
        CTRLDIR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Current Bank"]
    #[inline(always)]
    #[must_use]
    pub fn currbk(&mut self) -> CURRBK_W<UESTA1X_SPEC> {
        CURRBK_W::new(self, 0)
    }
    #[doc = "Bit 2 - Control Direction"]
    #[inline(always)]
    #[must_use]
    pub fn ctrldir(&mut self) -> CTRLDIR_W<UESTA1X_SPEC> {
        CTRLDIR_W::new(self, 2)
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
#[doc = "USB Endpoint Status 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uesta1x::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uesta1x::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UESTA1X_SPEC;
impl crate::RegisterSpec for UESTA1X_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uesta1x::R`](R) reader structure"]
impl crate::Readable for UESTA1X_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uesta1x::W`](W) writer structure"]
impl crate::Writable for UESTA1X_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UESTA1X to value 0"]
impl crate::Resettable for UESTA1X_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
