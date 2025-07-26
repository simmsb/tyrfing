#[doc = "Register `PCMSK0` reader"]
pub type R = crate::R<PCMSK0_SPEC>;
#[doc = "Register `PCMSK0` writer"]
pub type W = crate::W<PCMSK0_SPEC>;
#[doc = "Field `PCINT` reader - Pin Change Enable bits"]
pub type PCINT_R = crate::FieldReader;
#[doc = "Field `PCINT` writer - Pin Change Enable bits"]
pub type PCINT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Pin Change Enable bits"]
    #[inline(always)]
    pub fn pcint(&self) -> PCINT_R {
        PCINT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pin Change Enable bits"]
    #[inline(always)]
    #[must_use]
    pub fn pcint(&mut self) -> PCINT_W<PCMSK0_SPEC> {
        PCINT_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Pin Change Mask Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmsk0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmsk0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCMSK0_SPEC;
impl crate::RegisterSpec for PCMSK0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pcmsk0::R`](R) reader structure"]
impl crate::Readable for PCMSK0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcmsk0::W`](W) writer structure"]
impl crate::Writable for PCMSK0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCMSK0 to value 0"]
impl crate::Resettable for PCMSK0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
