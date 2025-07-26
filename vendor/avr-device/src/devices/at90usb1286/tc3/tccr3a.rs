#[doc = "Register `TCCR3A` reader"]
pub type R = crate::R<TCCR3A_SPEC>;
#[doc = "Register `TCCR3A` writer"]
pub type W = crate::W<TCCR3A_SPEC>;
#[doc = "Field `WGM3` reader - Waveform Generation Mode"]
pub type WGM3_R = crate::FieldReader;
#[doc = "Field `WGM3` writer - Waveform Generation Mode"]
pub type WGM3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `COM3C` reader - Compare Output Mode 3C, bits"]
pub type COM3C_R = crate::FieldReader;
#[doc = "Field `COM3C` writer - Compare Output Mode 3C, bits"]
pub type COM3C_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `COM3B` reader - Compare Output Mode 3B, bits"]
pub type COM3B_R = crate::FieldReader;
#[doc = "Field `COM3B` writer - Compare Output Mode 3B, bits"]
pub type COM3B_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `COM3A` reader - Compare Output Mode 1A, bits"]
pub type COM3A_R = crate::FieldReader;
#[doc = "Field `COM3A` writer - Compare Output Mode 1A, bits"]
pub type COM3A_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm3(&self) -> WGM3_R {
        WGM3_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Compare Output Mode 3C, bits"]
    #[inline(always)]
    pub fn com3c(&self) -> COM3C_R {
        COM3C_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Compare Output Mode 3B, bits"]
    #[inline(always)]
    pub fn com3b(&self) -> COM3B_R {
        COM3B_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Compare Output Mode 1A, bits"]
    #[inline(always)]
    pub fn com3a(&self) -> COM3A_R {
        COM3A_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm3(&mut self) -> WGM3_W<TCCR3A_SPEC> {
        WGM3_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Compare Output Mode 3C, bits"]
    #[inline(always)]
    #[must_use]
    pub fn com3c(&mut self) -> COM3C_W<TCCR3A_SPEC> {
        COM3C_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Compare Output Mode 3B, bits"]
    #[inline(always)]
    #[must_use]
    pub fn com3b(&mut self) -> COM3B_W<TCCR3A_SPEC> {
        COM3B_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Compare Output Mode 1A, bits"]
    #[inline(always)]
    #[must_use]
    pub fn com3a(&mut self) -> COM3A_W<TCCR3A_SPEC> {
        COM3A_W::new(self, 6)
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
#[doc = "Timer/Counter3 Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr3a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr3a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR3A_SPEC;
impl crate::RegisterSpec for TCCR3A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr3a::R`](R) reader structure"]
impl crate::Readable for TCCR3A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr3a::W`](W) writer structure"]
impl crate::Writable for TCCR3A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR3A to value 0"]
impl crate::Resettable for TCCR3A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
