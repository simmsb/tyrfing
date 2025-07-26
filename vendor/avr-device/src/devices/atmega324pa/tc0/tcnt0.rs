#[doc = "Register `TCNT0` reader"]
pub type R = crate::R<TCNT0_SPEC>;
#[doc = "Register `TCNT0` writer"]
pub type W = crate::W<TCNT0_SPEC>;
#[doc = "Field `TCNT0` reader - Timer/Counter0 bits"]
pub type TCNT0_R = crate::FieldReader;
#[doc = "Field `TCNT0` writer - Timer/Counter0 bits"]
pub type TCNT0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timer/Counter0 bits"]
    #[inline(always)]
    pub fn tcnt0(&self) -> TCNT0_R {
        TCNT0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer/Counter0 bits"]
    #[inline(always)]
    #[must_use]
    pub fn tcnt0(&mut self) -> TCNT0_W<TCNT0_SPEC> {
        TCNT0_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer/Counter0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcnt0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcnt0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCNT0_SPEC;
impl crate::RegisterSpec for TCNT0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tcnt0::R`](R) reader structure"]
impl crate::Readable for TCNT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcnt0::W`](W) writer structure"]
impl crate::Writable for TCNT0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCNT0 to value 0"]
impl crate::Resettable for TCNT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
