#[doc = "Register `TCNT3` reader"]
pub type R = crate::R<TCNT3_SPEC>;
#[doc = "Register `TCNT3` writer"]
pub type W = crate::W<TCNT3_SPEC>;
#[doc = "Field `TCNT3` reader - Timer/Counter3 bits"]
pub type TCNT3_R = crate::FieldReader<u16>;
#[doc = "Field `TCNT3` writer - Timer/Counter3 bits"]
pub type TCNT3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer/Counter3 bits"]
    #[inline(always)]
    pub fn tcnt3(&self) -> TCNT3_R {
        TCNT3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer/Counter3 bits"]
    #[inline(always)]
    #[must_use]
    pub fn tcnt3(&mut self) -> TCNT3_W<TCNT3_SPEC> {
        TCNT3_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer/Counter3 Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcnt3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcnt3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCNT3_SPEC;
impl crate::RegisterSpec for TCNT3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tcnt3::R`](R) reader structure"]
impl crate::Readable for TCNT3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcnt3::W`](W) writer structure"]
impl crate::Writable for TCNT3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCNT3 to value 0"]
impl crate::Resettable for TCNT3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
