#[doc = "Register `CALIB` reader"]
pub type R = crate::R<CALIB_SPEC>;
#[doc = "Register `CALIB` writer"]
pub type W = crate::W<CALIB_SPEC>;
#[doc = "Field `ERROR` reader - Error Correction Value"]
pub type ERROR_R = crate::FieldReader;
#[doc = "Field `ERROR` writer - Error Correction Value"]
pub type ERROR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Field `SIGN` reader - Error Correction Sign Bit"]
pub type SIGN_R = crate::BitReader;
#[doc = "Field `SIGN` writer - Error Correction Sign Bit"]
pub type SIGN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Error Correction Value"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Error Correction Sign Bit"]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Error Correction Value"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<CALIB_SPEC> {
        ERROR_W::new(self, 0)
    }
    #[doc = "Bit 7 - Error Correction Sign Bit"]
    #[inline(always)]
    #[must_use]
    pub fn sign(&mut self) -> SIGN_W<CALIB_SPEC> {
        SIGN_W::new(self, 7)
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
#[doc = "Calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calib::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calib::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALIB_SPEC;
impl crate::RegisterSpec for CALIB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`calib::R`](R) reader structure"]
impl crate::Readable for CALIB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`calib::W`](W) writer structure"]
impl crate::Writable for CALIB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CALIB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
