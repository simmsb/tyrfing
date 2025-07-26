#[doc = "Register `TCNT2` reader"]
pub type R = crate::R<TCNT2_SPEC>;
#[doc = "Register `TCNT2` writer"]
pub type W = crate::W<TCNT2_SPEC>;
#[doc = "Field `TCNT2` reader - Timer/Counter2 bits"]
pub type TCNT2_R = crate::FieldReader;
#[doc = "Field `TCNT2` writer - Timer/Counter2 bits"]
pub type TCNT2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timer/Counter2 bits"]
    #[inline(always)]
    pub fn tcnt2(&self) -> TCNT2_R {
        TCNT2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer/Counter2 bits"]
    #[inline(always)]
    #[must_use]
    pub fn tcnt2(&mut self) -> TCNT2_W<TCNT2_SPEC> {
        TCNT2_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer/Counter2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcnt2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcnt2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCNT2_SPEC;
impl crate::RegisterSpec for TCNT2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tcnt2::R`](R) reader structure"]
impl crate::Readable for TCNT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcnt2::W`](W) writer structure"]
impl crate::Writable for TCNT2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCNT2 to value 0"]
impl crate::Resettable for TCNT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
