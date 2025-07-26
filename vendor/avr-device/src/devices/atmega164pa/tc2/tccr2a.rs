#[doc = "Register `TCCR2A` reader"]
pub type R = crate::R<TCCR2A_SPEC>;
#[doc = "Register `TCCR2A` writer"]
pub type W = crate::W<TCCR2A_SPEC>;
#[doc = "Field `WGM2` reader - Waveform Genration Mode"]
pub type WGM2_R = crate::FieldReader;
#[doc = "Field `WGM2` writer - Waveform Genration Mode"]
pub type WGM2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `COM2B` reader - Compare Output Mode bits"]
pub type COM2B_R = crate::FieldReader;
#[doc = "Field `COM2B` writer - Compare Output Mode bits"]
pub type COM2B_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `COM2A` reader - Compare Output Mode bits"]
pub type COM2A_R = crate::FieldReader;
#[doc = "Field `COM2A` writer - Compare Output Mode bits"]
pub type COM2A_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Waveform Genration Mode"]
    #[inline(always)]
    pub fn wgm2(&self) -> WGM2_R {
        WGM2_R::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - Compare Output Mode bits"]
    #[inline(always)]
    pub fn com2b(&self) -> COM2B_R {
        COM2B_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Compare Output Mode bits"]
    #[inline(always)]
    pub fn com2a(&self) -> COM2A_R {
        COM2A_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Genration Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm2(&mut self) -> WGM2_W<TCCR2A_SPEC> {
        WGM2_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Compare Output Mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn com2b(&mut self) -> COM2B_W<TCCR2A_SPEC> {
        COM2B_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Compare Output Mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn com2a(&mut self) -> COM2A_W<TCCR2A_SPEC> {
        COM2A_W::new(self, 6)
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
#[doc = "Timer/Counter2 Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr2a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr2a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR2A_SPEC;
impl crate::RegisterSpec for TCCR2A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr2a::R`](R) reader structure"]
impl crate::Readable for TCCR2A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr2a::W`](W) writer structure"]
impl crate::Writable for TCCR2A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR2A to value 0"]
impl crate::Resettable for TCCR2A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
