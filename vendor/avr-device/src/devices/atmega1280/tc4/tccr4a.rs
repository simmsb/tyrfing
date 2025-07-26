#[doc = "Register `TCCR4A` reader"]
pub type R = crate::R<TCCR4A_SPEC>;
#[doc = "Register `TCCR4A` writer"]
pub type W = crate::W<TCCR4A_SPEC>;
#[doc = "Field `WGM4` reader - Waveform Generation Mode"]
pub type WGM4_R = crate::FieldReader;
#[doc = "Field `WGM4` writer - Waveform Generation Mode"]
pub type WGM4_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `COM4C` reader - Compare Output Mode 4C, bits"]
pub type COM4C_R = crate::FieldReader<COM4C_A>;
#[doc = "Compare Output Mode 4C, bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM4C_A {
    #[doc = "0: Normal port operation, OCix disconnected"]
    DISCONNECTED = 0,
    #[doc = "1: Toggle OCix on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE = 1,
    #[doc = "2: Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)"]
    MATCH_CLEAR = 2,
    #[doc = "3: Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)"]
    MATCH_SET = 3,
}
impl From<COM4C_A> for u8 {
    #[inline(always)]
    fn from(variant: COM4C_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COM4C_A {
    type Ux = u8;
}
impl COM4C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COM4C_A {
        match self.bits {
            0 => COM4C_A::DISCONNECTED,
            1 => COM4C_A::MATCH_TOGGLE,
            2 => COM4C_A::MATCH_CLEAR,
            3 => COM4C_A::MATCH_SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal port operation, OCix disconnected"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == COM4C_A::DISCONNECTED
    }
    #[doc = "Toggle OCix on Compare Match (Might depend on WGM)"]
    #[inline(always)]
    pub fn is_match_toggle(&self) -> bool {
        *self == COM4C_A::MATCH_TOGGLE
    }
    #[doc = "Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)"]
    #[inline(always)]
    pub fn is_match_clear(&self) -> bool {
        *self == COM4C_A::MATCH_CLEAR
    }
    #[doc = "Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)"]
    #[inline(always)]
    pub fn is_match_set(&self) -> bool {
        *self == COM4C_A::MATCH_SET
    }
}
#[doc = "Field `COM4C` writer - Compare Output Mode 4C, bits"]
pub type COM4C_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COM4C_A>;
impl<'a, REG> COM4C_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal port operation, OCix disconnected"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(COM4C_A::DISCONNECTED)
    }
    #[doc = "Toggle OCix on Compare Match (Might depend on WGM)"]
    #[inline(always)]
    pub fn match_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(COM4C_A::MATCH_TOGGLE)
    }
    #[doc = "Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)"]
    #[inline(always)]
    pub fn match_clear(self) -> &'a mut crate::W<REG> {
        self.variant(COM4C_A::MATCH_CLEAR)
    }
    #[doc = "Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)"]
    #[inline(always)]
    pub fn match_set(self) -> &'a mut crate::W<REG> {
        self.variant(COM4C_A::MATCH_SET)
    }
}
#[doc = "Field `COM4B` reader - Compare Output Mode 4B, bits"]
pub use COM4C_R as COM4B_R;
#[doc = "Field `COM4A` reader - Compare Output Mode 1A, bits"]
pub use COM4C_R as COM4A_R;
#[doc = "Field `COM4B` writer - Compare Output Mode 4B, bits"]
pub use COM4C_W as COM4B_W;
#[doc = "Field `COM4A` writer - Compare Output Mode 1A, bits"]
pub use COM4C_W as COM4A_W;
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm4(&self) -> WGM4_R {
        WGM4_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Compare Output Mode 4C, bits"]
    #[inline(always)]
    pub fn com4c(&self) -> COM4C_R {
        COM4C_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Compare Output Mode 4B, bits"]
    #[inline(always)]
    pub fn com4b(&self) -> COM4B_R {
        COM4B_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Compare Output Mode 1A, bits"]
    #[inline(always)]
    pub fn com4a(&self) -> COM4A_R {
        COM4A_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm4(&mut self) -> WGM4_W<TCCR4A_SPEC> {
        WGM4_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Compare Output Mode 4C, bits"]
    #[inline(always)]
    #[must_use]
    pub fn com4c(&mut self) -> COM4C_W<TCCR4A_SPEC> {
        COM4C_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Compare Output Mode 4B, bits"]
    #[inline(always)]
    #[must_use]
    pub fn com4b(&mut self) -> COM4B_W<TCCR4A_SPEC> {
        COM4B_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Compare Output Mode 1A, bits"]
    #[inline(always)]
    #[must_use]
    pub fn com4a(&mut self) -> COM4A_W<TCCR4A_SPEC> {
        COM4A_W::new(self, 6)
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
#[doc = "Timer/Counter4 Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr4a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr4a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR4A_SPEC;
impl crate::RegisterSpec for TCCR4A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr4a::R`](R) reader structure"]
impl crate::Readable for TCCR4A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr4a::W`](W) writer structure"]
impl crate::Writable for TCCR4A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR4A to value 0"]
impl crate::Resettable for TCCR4A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
