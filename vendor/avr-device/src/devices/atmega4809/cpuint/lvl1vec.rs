#[doc = "Register `LVL1VEC` reader"]
pub type R = crate::R<LVL1VEC_SPEC>;
#[doc = "Register `LVL1VEC` writer"]
pub type W = crate::W<LVL1VEC_SPEC>;
#[doc = "Field `LVL1VEC` reader - Interrupt Vector with High Priority"]
pub type LVL1VEC_R = crate::FieldReader;
#[doc = "Field `LVL1VEC` writer - Interrupt Vector with High Priority"]
pub type LVL1VEC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt Vector with High Priority"]
    #[inline(always)]
    pub fn lvl1vec(&self) -> LVL1VEC_R {
        LVL1VEC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt Vector with High Priority"]
    #[inline(always)]
    #[must_use]
    pub fn lvl1vec(&mut self) -> LVL1VEC_W<LVL1VEC_SPEC> {
        LVL1VEC_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Level 1 Priority Vector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvl1vec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvl1vec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LVL1VEC_SPEC;
impl crate::RegisterSpec for LVL1VEC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvl1vec::R`](R) reader structure"]
impl crate::Readable for LVL1VEC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lvl1vec::W`](W) writer structure"]
impl crate::Writable for LVL1VEC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVL1VEC to value 0"]
impl crate::Resettable for LVL1VEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
