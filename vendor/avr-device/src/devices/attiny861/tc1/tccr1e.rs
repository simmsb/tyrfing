#[doc = "Register `TCCR1E` reader"]
pub type R = crate::R<TCCR1E_SPEC>;
#[doc = "Register `TCCR1E` writer"]
pub type W = crate::W<TCCR1E_SPEC>;
#[doc = "Field `OC1OE` reader - Ouput Compare Override Enable Bits"]
pub type OC1OE_R = crate::FieldReader;
#[doc = "Field `OC1OE` writer - Ouput Compare Override Enable Bits"]
pub type OC1OE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Ouput Compare Override Enable Bits"]
    #[inline(always)]
    pub fn oc1oe(&self) -> OC1OE_R {
        OC1OE_R::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Ouput Compare Override Enable Bits"]
    #[inline(always)]
    #[must_use]
    pub fn oc1oe(&mut self) -> OC1OE_W<TCCR1E_SPEC> {
        OC1OE_W::new(self, 0)
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
#[doc = "Timer/Counter1 Control Register E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr1e::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr1e::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR1E_SPEC;
impl crate::RegisterSpec for TCCR1E_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr1e::R`](R) reader structure"]
impl crate::Readable for TCCR1E_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr1e::W`](W) writer structure"]
impl crate::Writable for TCCR1E_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR1E to value 0"]
impl crate::Resettable for TCCR1E_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
