#[doc = "Register `TCNT1` reader"]
pub type R = crate::R<TCNT1_SPEC>;
#[doc = "Register `TCNT1` writer"]
pub type W = crate::W<TCNT1_SPEC>;
#[doc = "Field `TCNT1` reader - Timer/Counter1 bits"]
pub type TCNT1_R = crate::FieldReader<u16>;
#[doc = "Field `TCNT1` writer - Timer/Counter1 bits"]
pub type TCNT1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer/Counter1 bits"]
    #[inline(always)]
    pub fn tcnt1(&self) -> TCNT1_R {
        TCNT1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer/Counter1 bits"]
    #[inline(always)]
    #[must_use]
    pub fn tcnt1(&mut self) -> TCNT1_W<TCNT1_SPEC> {
        TCNT1_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer/Counter1 Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcnt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcnt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCNT1_SPEC;
impl crate::RegisterSpec for TCNT1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tcnt1::R`](R) reader structure"]
impl crate::Readable for TCNT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcnt1::W`](W) writer structure"]
impl crate::Writable for TCNT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCNT1 to value 0"]
impl crate::Resettable for TCNT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
