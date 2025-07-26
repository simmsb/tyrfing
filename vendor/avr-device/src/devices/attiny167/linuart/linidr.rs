#[doc = "Register `LINIDR` reader"]
pub type R = crate::R<LINIDR_SPEC>;
#[doc = "Register `LINIDR` writer"]
pub type W = crate::W<LINIDR_SPEC>;
#[doc = "Field `LID` reader - Identifier bit 5 or Data Length bits"]
pub type LID_R = crate::FieldReader;
#[doc = "Field `LID` writer - Identifier bit 5 or Data Length bits"]
pub type LID_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
#[doc = "Field `LP` reader - Parity bits"]
pub type LP_R = crate::FieldReader;
#[doc = "Field `LP` writer - Parity bits"]
pub type LP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - Identifier bit 5 or Data Length bits"]
    #[inline(always)]
    pub fn lid(&self) -> LID_R {
        LID_R::new(self.bits & 0x3f)
    }
    #[doc = "Bits 6:7 - Parity bits"]
    #[inline(always)]
    pub fn lp(&self) -> LP_R {
        LP_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:5 - Identifier bit 5 or Data Length bits"]
    #[inline(always)]
    #[must_use]
    pub fn lid(&mut self) -> LID_W<LINIDR_SPEC> {
        LID_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - Parity bits"]
    #[inline(always)]
    #[must_use]
    pub fn lp(&mut self) -> LP_W<LINIDR_SPEC> {
        LP_W::new(self, 6)
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
#[doc = "LIN Identifier Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINIDR_SPEC;
impl crate::RegisterSpec for LINIDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`linidr::R`](R) reader structure"]
impl crate::Readable for LINIDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`linidr::W`](W) writer structure"]
impl crate::Writable for LINIDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINIDR to value 0"]
impl crate::Resettable for LINIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
