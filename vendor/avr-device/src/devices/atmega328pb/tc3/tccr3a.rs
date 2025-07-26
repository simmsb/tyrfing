#[doc = "Register `TCCR3A` reader"]
pub type R = crate::R<TCCR3A_SPEC>;
#[doc = "Register `TCCR3A` writer"]
pub type W = crate::W<TCCR3A_SPEC>;
#[doc = "Field `WGM3` reader - Waveform Genration Mode"]
pub type WGM3_R = crate::FieldReader;
#[doc = "Field `WGM3` writer - Waveform Genration Mode"]
pub type WGM3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `COM3B` reader - Compare Output Mode bits"]
pub type COM3B_R = crate::FieldReader<COM3B_A>;
#[doc = "Compare Output Mode bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM3B_A {
    #[doc = "0: Normal port operation, OCix disconnected"]
    DISCONNECTED = 0,
    #[doc = "1: Toggle OCix on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE = 1,
    #[doc = "2: Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)"]
    MATCH_CLEAR = 2,
    #[doc = "3: Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)"]
    MATCH_SET = 3,
}
impl From<COM3B_A> for u8 {
    #[inline(always)]
    fn from(variant: COM3B_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COM3B_A {
    type Ux = u8;
}
impl COM3B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COM3B_A {
        match self.bits {
            0 => COM3B_A::DISCONNECTED,
            1 => COM3B_A::MATCH_TOGGLE,
            2 => COM3B_A::MATCH_CLEAR,
            3 => COM3B_A::MATCH_SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal port operation, OCix disconnected"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == COM3B_A::DISCONNECTED
    }
    #[doc = "Toggle OCix on Compare Match (Might depend on WGM)"]
    #[inline(always)]
    pub fn is_match_toggle(&self) -> bool {
        *self == COM3B_A::MATCH_TOGGLE
    }
    #[doc = "Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)"]
    #[inline(always)]
    pub fn is_match_clear(&self) -> bool {
        *self == COM3B_A::MATCH_CLEAR
    }
    #[doc = "Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)"]
    #[inline(always)]
    pub fn is_match_set(&self) -> bool {
        *self == COM3B_A::MATCH_SET
    }
}
#[doc = "Field `COM3B` writer - Compare Output Mode bits"]
pub type COM3B_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COM3B_A>;
impl<'a, REG> COM3B_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal port operation, OCix disconnected"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(COM3B_A::DISCONNECTED)
    }
    #[doc = "Toggle OCix on Compare Match (Might depend on WGM)"]
    #[inline(always)]
    pub fn match_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(COM3B_A::MATCH_TOGGLE)
    }
    #[doc = "Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)"]
    #[inline(always)]
    pub fn match_clear(self) -> &'a mut crate::W<REG> {
        self.variant(COM3B_A::MATCH_CLEAR)
    }
    #[doc = "Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)"]
    #[inline(always)]
    pub fn match_set(self) -> &'a mut crate::W<REG> {
        self.variant(COM3B_A::MATCH_SET)
    }
}
#[doc = "Field `COM3A` reader - Compare Output Mode bits"]
pub use COM3B_R as COM3A_R;
#[doc = "Field `COM3A` writer - Compare Output Mode bits"]
pub use COM3B_W as COM3A_W;
impl R {
    #[doc = "Bits 0:1 - Waveform Genration Mode"]
    #[inline(always)]
    pub fn wgm3(&self) -> WGM3_R {
        WGM3_R::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - Compare Output Mode bits"]
    #[inline(always)]
    pub fn com3b(&self) -> COM3B_R {
        COM3B_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Compare Output Mode bits"]
    #[inline(always)]
    pub fn com3a(&self) -> COM3A_R {
        COM3A_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Genration Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm3(&mut self) -> WGM3_W<TCCR3A_SPEC> {
        WGM3_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Compare Output Mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn com3b(&mut self) -> COM3B_W<TCCR3A_SPEC> {
        COM3B_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Compare Output Mode bits"]
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
