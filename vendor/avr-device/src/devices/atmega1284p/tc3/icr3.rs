#[doc = "Register `ICR3` reader"]
pub type R = crate::R<ICR3_SPEC>;
#[doc = "Register `ICR3` writer"]
pub type W = crate::W<ICR3_SPEC>;
#[doc = "Field `ICR3` reader - Timer/Counter3 Input Capture bits"]
pub type ICR3_R = crate::FieldReader<u16>;
#[doc = "Field `ICR3` writer - Timer/Counter3 Input Capture bits"]
pub type ICR3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer/Counter3 Input Capture bits"]
    #[inline(always)]
    pub fn icr3(&self) -> ICR3_R {
        ICR3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer/Counter3 Input Capture bits"]
    #[inline(always)]
    #[must_use]
    pub fn icr3(&mut self) -> ICR3_W<ICR3_SPEC> {
        ICR3_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer/Counter3 Input Capture Register Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR3_SPEC;
impl crate::RegisterSpec for ICR3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`icr3::R`](R) reader structure"]
impl crate::Readable for ICR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icr3::W`](W) writer structure"]
impl crate::Writable for ICR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR3 to value 0"]
impl crate::Resettable for ICR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
