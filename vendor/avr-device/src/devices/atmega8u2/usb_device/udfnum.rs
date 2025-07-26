#[doc = "Register `UDFNUM` reader"]
pub type R = crate::R<UDFNUM_SPEC>;
#[doc = "Register `UDFNUM` writer"]
pub type W = crate::W<UDFNUM_SPEC>;
#[doc = "Field `FNUM` reader - Frame Number Upper Flag"]
pub type FNUM_R = crate::FieldReader<u16>;
#[doc = "Field `FNUM` writer - Frame Number Upper Flag"]
pub type FNUM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Frame Number Upper Flag"]
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new(self.bits & 0x07ff)
    }
}
impl W {
    #[doc = "Bits 0:10 - Frame Number Upper Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fnum(&mut self) -> FNUM_W<UDFNUM_SPEC> {
        FNUM_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB Device Frame Number High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udfnum::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udfnum::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UDFNUM_SPEC;
impl crate::RegisterSpec for UDFNUM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`udfnum::R`](R) reader structure"]
impl crate::Readable for UDFNUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`udfnum::W`](W) writer structure"]
impl crate::Writable for UDFNUM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDFNUM to value 0"]
impl crate::Resettable for UDFNUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
