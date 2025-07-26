#[doc = "Register `CLKSELR` reader"]
pub type R = crate::R<CLKSELR_SPEC>;
#[doc = "Register `CLKSELR` writer"]
pub type W = crate::W<CLKSELR_SPEC>;
#[doc = "Field `CSEL` reader - Clock Source Select bit 3 - CKSEL3 fuse substitution"]
pub type CSEL_R = crate::FieldReader;
#[doc = "Field `CSEL` writer - Clock Source Select bit 3 - CKSEL3 fuse substitution"]
pub type CSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `CSUT` reader - Clock Start-up Time bit 1 - SUT1 fuse substitution"]
pub type CSUT_R = crate::FieldReader;
#[doc = "Field `CSUT` writer - Clock Start-up Time bit 1 - SUT1 fuse substitution"]
pub type CSUT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `COUT` reader - Clock Out - CKOUT fuse substitution"]
pub type COUT_R = crate::BitReader;
#[doc = "Field `COUT` writer - Clock Out - CKOUT fuse substitution"]
pub type COUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Clock Source Select bit 3 - CKSEL3 fuse substitution"]
    #[inline(always)]
    pub fn csel(&self) -> CSEL_R {
        CSEL_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - Clock Start-up Time bit 1 - SUT1 fuse substitution"]
    #[inline(always)]
    pub fn csut(&self) -> CSUT_R {
        CSUT_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Clock Out - CKOUT fuse substitution"]
    #[inline(always)]
    pub fn cout(&self) -> COUT_R {
        COUT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Source Select bit 3 - CKSEL3 fuse substitution"]
    #[inline(always)]
    #[must_use]
    pub fn csel(&mut self) -> CSEL_W<CLKSELR_SPEC> {
        CSEL_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Clock Start-up Time bit 1 - SUT1 fuse substitution"]
    #[inline(always)]
    #[must_use]
    pub fn csut(&mut self) -> CSUT_W<CLKSELR_SPEC> {
        CSUT_W::new(self, 4)
    }
    #[doc = "Bit 6 - Clock Out - CKOUT fuse substitution"]
    #[inline(always)]
    #[must_use]
    pub fn cout(&mut self) -> COUT_W<CLKSELR_SPEC> {
        COUT_W::new(self, 6)
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
#[doc = "Clock Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKSELR_SPEC;
impl crate::RegisterSpec for CLKSELR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clkselr::R`](R) reader structure"]
impl crate::Readable for CLKSELR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkselr::W`](W) writer structure"]
impl crate::Writable for CLKSELR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKSELR to value 0"]
impl crate::Resettable for CLKSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
