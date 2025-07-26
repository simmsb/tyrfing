#[doc = "Register `CLKSEL1` reader"]
pub type R = crate::R<CLKSEL1_SPEC>;
#[doc = "Register `CLKSEL1` writer"]
pub type W = crate::W<CLKSEL1_SPEC>;
#[doc = "Field `EXCKSEL` reader - No Description."]
pub type EXCKSEL_R = crate::FieldReader;
#[doc = "Field `EXCKSEL` writer - No Description."]
pub type EXCKSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `RCCKSEL` reader - No Description."]
pub type RCCKSEL_R = crate::FieldReader;
#[doc = "Field `RCCKSEL` writer - No Description."]
pub type RCCKSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    pub fn excksel(&self) -> EXCKSEL_R {
        EXCKSEL_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - No Description."]
    #[inline(always)]
    pub fn rccksel(&self) -> RCCKSEL_R {
        RCCKSEL_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn excksel(&mut self) -> EXCKSEL_W<CLKSEL1_SPEC> {
        EXCKSEL_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn rccksel(&mut self) -> RCCKSEL_W<CLKSEL1_SPEC> {
        RCCKSEL_W::new(self, 4)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKSEL1_SPEC;
impl crate::RegisterSpec for CLKSEL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clksel1::R`](R) reader structure"]
impl crate::Readable for CLKSEL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clksel1::W`](W) writer structure"]
impl crate::Writable for CLKSEL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKSEL1 to value 0"]
impl crate::Resettable for CLKSEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
