#[doc = "Register `ADMUXA` reader"]
pub type R = crate::R<ADMUXA_SPEC>;
#[doc = "Register `ADMUXA` writer"]
pub type W = crate::W<ADMUXA_SPEC>;
#[doc = "Field `MUX` reader - Analog Channel and Gain Selection Bits"]
pub type MUX_R = crate::FieldReader;
#[doc = "Field `MUX` writer - Analog Channel and Gain Selection Bits"]
pub type MUX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Analog Channel and Gain Selection Bits"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Analog Channel and Gain Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mux(&mut self) -> MUX_W<ADMUXA_SPEC> {
        MUX_W::new(self, 0)
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
#[doc = "The ADC multiplexer Selection Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`admuxa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`admuxa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADMUXA_SPEC;
impl crate::RegisterSpec for ADMUXA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`admuxa::R`](R) reader structure"]
impl crate::Readable for ADMUXA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`admuxa::W`](W) writer structure"]
impl crate::Writable for ADMUXA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADMUXA to value 0"]
impl crate::Resettable for ADMUXA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
