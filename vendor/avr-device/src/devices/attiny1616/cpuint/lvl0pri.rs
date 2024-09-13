#[doc = "Register `LVL0PRI` reader"]
pub type R = crate::R<LVL0PRI_SPEC>;
#[doc = "Register `LVL0PRI` writer"]
pub type W = crate::W<LVL0PRI_SPEC>;
#[doc = "Field `LVL0PRI` reader - Interrupt Level Priority"]
pub type LVL0PRI_R = crate::FieldReader;
#[doc = "Field `LVL0PRI` writer - Interrupt Level Priority"]
pub type LVL0PRI_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt Level Priority"]
    #[inline(always)]
    pub fn lvl0pri(&self) -> LVL0PRI_R {
        LVL0PRI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt Level Priority"]
    #[inline(always)]
    #[must_use]
    pub fn lvl0pri(&mut self) -> LVL0PRI_W<LVL0PRI_SPEC> {
        LVL0PRI_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Level 0 Priority\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvl0pri::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvl0pri::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LVL0PRI_SPEC;
impl crate::RegisterSpec for LVL0PRI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvl0pri::R`](R) reader structure"]
impl crate::Readable for LVL0PRI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lvl0pri::W`](W) writer structure"]
impl crate::Writable for LVL0PRI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVL0PRI to value 0"]
impl crate::Resettable for LVL0PRI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
