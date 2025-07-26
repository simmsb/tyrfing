#[doc = "Register `CLKCR` reader"]
pub type R = crate::R<CLKCR_SPEC>;
#[doc = "Register `CLKCR` writer"]
pub type W = crate::W<CLKCR_SPEC>;
#[doc = "Field `CKSEL` reader - Clock Select Bits"]
pub type CKSEL_R = crate::FieldReader;
#[doc = "Field `CKSEL` writer - Clock Select Bits"]
pub type CKSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `SUT` reader - Start-up Time"]
pub type SUT_R = crate::BitReader;
#[doc = "Field `SUT` writer - Start-up Time"]
pub type SUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKOUTC` reader - Clock Output (Copy). Active low."]
pub type CKOUTC_R = crate::BitReader;
#[doc = "Field `CKOUTC` writer - Clock Output (Copy). Active low."]
pub type CKOUTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTR` reader - Clock Switch Trigger"]
pub type CSTR_R = crate::BitReader;
#[doc = "Field `CSTR` writer - Clock Switch Trigger"]
pub type CSTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCRDY` reader - Oscillator Ready"]
pub type OSCRDY_R = crate::BitReader;
#[doc = "Field `OSCRDY` writer - Oscillator Ready"]
pub type OSCRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Clock Select Bits"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Start-up Time"]
    #[inline(always)]
    pub fn sut(&self) -> SUT_R {
        SUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Output (Copy). Active low."]
    #[inline(always)]
    pub fn ckoutc(&self) -> CKOUTC_R {
        CKOUTC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock Switch Trigger"]
    #[inline(always)]
    pub fn cstr(&self) -> CSTR_R {
        CSTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Oscillator Ready"]
    #[inline(always)]
    pub fn oscrdy(&self) -> OSCRDY_R {
        OSCRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Select Bits"]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CKSEL_W<CLKCR_SPEC> {
        CKSEL_W::new(self, 0)
    }
    #[doc = "Bit 4 - Start-up Time"]
    #[inline(always)]
    #[must_use]
    pub fn sut(&mut self) -> SUT_W<CLKCR_SPEC> {
        SUT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clock Output (Copy). Active low."]
    #[inline(always)]
    #[must_use]
    pub fn ckoutc(&mut self) -> CKOUTC_W<CLKCR_SPEC> {
        CKOUTC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clock Switch Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn cstr(&mut self) -> CSTR_W<CLKCR_SPEC> {
        CSTR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Oscillator Ready"]
    #[inline(always)]
    #[must_use]
    pub fn oscrdy(&mut self) -> OSCRDY_W<CLKCR_SPEC> {
        OSCRDY_W::new(self, 7)
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
#[doc = "Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKCR_SPEC;
impl crate::RegisterSpec for CLKCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clkcr::R`](R) reader structure"]
impl crate::Readable for CLKCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkcr::W`](W) writer structure"]
impl crate::Writable for CLKCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKCR to value 0"]
impl crate::Resettable for CLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
