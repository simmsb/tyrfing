#[doc = "Register `TCCR0A` reader"]
pub type R = crate::R<TCCR0A_SPEC>;
#[doc = "Register `TCCR0A` writer"]
pub type W = crate::W<TCCR0A_SPEC>;
#[doc = "Field `WGM0` reader - Waveform Genration Mode bits"]
pub type WGM0_R = crate::FieldReader;
#[doc = "Field `WGM0` writer - Waveform Genration Mode bits"]
pub type WGM0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `COM0A` reader - Compare Output Mode bits"]
pub type COM0A_R = crate::FieldReader;
#[doc = "Field `COM0A` writer - Compare Output Mode bits"]
pub type COM0A_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Waveform Genration Mode bits"]
    #[inline(always)]
    pub fn wgm0(&self) -> WGM0_R {
        WGM0_R::new(self.bits & 3)
    }
    #[doc = "Bits 6:7 - Compare Output Mode bits"]
    #[inline(always)]
    pub fn com0a(&self) -> COM0A_R {
        COM0A_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Genration Mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn wgm0(&mut self) -> WGM0_W<TCCR0A_SPEC> {
        WGM0_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - Compare Output Mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn com0a(&mut self) -> COM0A_W<TCCR0A_SPEC> {
        COM0A_W::new(self, 6)
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
#[doc = "Timer/Counter0 Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr0a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr0a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR0A_SPEC;
impl crate::RegisterSpec for TCCR0A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr0a::R`](R) reader structure"]
impl crate::Readable for TCCR0A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr0a::W`](W) writer structure"]
impl crate::Writable for TCCR0A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR0A to value 0"]
impl crate::Resettable for TCCR0A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
