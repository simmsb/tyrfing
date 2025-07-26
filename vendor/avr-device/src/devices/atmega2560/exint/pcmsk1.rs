#[doc = "Register `PCMSK1` reader"]
pub type R = crate::R<PCMSK1_SPEC>;
#[doc = "Register `PCMSK1` writer"]
pub type W = crate::W<PCMSK1_SPEC>;
#[doc = "Field `PCINT` reader - Pin Change Mask interrupt"]
pub type PCINT_R = crate::FieldReader;
#[doc = "Field `PCINT` writer - Pin Change Mask interrupt"]
pub type PCINT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Pin Change Mask interrupt"]
    #[inline(always)]
    pub fn pcint(&self) -> PCINT_R {
        PCINT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pin Change Mask interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pcint(&mut self) -> PCINT_W<PCMSK1_SPEC> {
        PCINT_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Pin Change Mask Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmsk1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmsk1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCMSK1_SPEC;
impl crate::RegisterSpec for PCMSK1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pcmsk1::R`](R) reader structure"]
impl crate::Readable for PCMSK1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcmsk1::W`](W) writer structure"]
impl crate::Writable for PCMSK1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCMSK1 to value 0"]
impl crate::Resettable for PCMSK1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
