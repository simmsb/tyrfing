#[doc = "Register `ICR1` reader"]
pub type R = crate::R<ICR1_SPEC>;
#[doc = "Register `ICR1` writer"]
pub type W = crate::W<ICR1_SPEC>;
#[doc = "Field `ICR1` reader - Timer/Counter1 Input Capture bits"]
pub type ICR1_R = crate::FieldReader<u16>;
#[doc = "Field `ICR1` writer - Timer/Counter1 Input Capture bits"]
pub type ICR1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer/Counter1 Input Capture bits"]
    #[inline(always)]
    pub fn icr1(&self) -> ICR1_R {
        ICR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer/Counter1 Input Capture bits"]
    #[inline(always)]
    #[must_use]
    pub fn icr1(&mut self) -> ICR1_W<ICR1_SPEC> {
        ICR1_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer/Counter1 Input Capture Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR1_SPEC;
impl crate::RegisterSpec for ICR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`icr1::R`](R) reader structure"]
impl crate::Readable for ICR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icr1::W`](W) writer structure"]
impl crate::Writable for ICR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR1 to value 0"]
impl crate::Resettable for ICR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
