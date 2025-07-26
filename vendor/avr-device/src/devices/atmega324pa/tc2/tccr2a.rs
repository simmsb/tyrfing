#[doc = "Register `TCCR2A` reader"]
pub type R = crate::R<TCCR2A_SPEC>;
#[doc = "Register `TCCR2A` writer"]
pub type W = crate::W<TCCR2A_SPEC>;
#[doc = "Field `WGM2` reader - Waveform Genration Mode"]
pub type WGM2_R = crate::FieldReader<WGM2_A>;
#[doc = "Waveform Genration Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WGM2_A {
    #[doc = "0: Normal, Top: `0xff`, Update: *Immediate*, Flag: *MAX*"]
    NORMAL_TOP = 0,
    #[doc = "1: Phase Correct PWM, Top: `0xff`, Update: *TOP*, Flag: *BOTTOM*"]
    PWM_PHASE = 1,
    #[doc = "2: CTC, Top: *OCRA*, Update: *Immediate*, Flag: *MAX*"]
    CTC = 2,
    #[doc = "3: Fast PWM, Top: `0xff`, Update: *TOP*, Flag: *MAX*"]
    PWM_FAST = 3,
}
impl From<WGM2_A> for u8 {
    #[inline(always)]
    fn from(variant: WGM2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WGM2_A {
    type Ux = u8;
}
impl WGM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WGM2_A {
        match self.bits {
            0 => WGM2_A::NORMAL_TOP,
            1 => WGM2_A::PWM_PHASE,
            2 => WGM2_A::CTC,
            3 => WGM2_A::PWM_FAST,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal, Top: `0xff`, Update: *Immediate*, Flag: *MAX*"]
    #[inline(always)]
    pub fn is_normal_top(&self) -> bool {
        *self == WGM2_A::NORMAL_TOP
    }
    #[doc = "Phase Correct PWM, Top: `0xff`, Update: *TOP*, Flag: *BOTTOM*"]
    #[inline(always)]
    pub fn is_pwm_phase(&self) -> bool {
        *self == WGM2_A::PWM_PHASE
    }
    #[doc = "CTC, Top: *OCRA*, Update: *Immediate*, Flag: *MAX*"]
    #[inline(always)]
    pub fn is_ctc(&self) -> bool {
        *self == WGM2_A::CTC
    }
    #[doc = "Fast PWM, Top: `0xff`, Update: *TOP*, Flag: *MAX*"]
    #[inline(always)]
    pub fn is_pwm_fast(&self) -> bool {
        *self == WGM2_A::PWM_FAST
    }
}
#[doc = "Field `WGM2` writer - Waveform Genration Mode"]
pub type WGM2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WGM2_A>;
impl<'a, REG> WGM2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal, Top: `0xff`, Update: *Immediate*, Flag: *MAX*"]
    #[inline(always)]
    pub fn normal_top(self) -> &'a mut crate::W<REG> {
        self.variant(WGM2_A::NORMAL_TOP)
    }
    #[doc = "Phase Correct PWM, Top: `0xff`, Update: *TOP*, Flag: *BOTTOM*"]
    #[inline(always)]
    pub fn pwm_phase(self) -> &'a mut crate::W<REG> {
        self.variant(WGM2_A::PWM_PHASE)
    }
    #[doc = "CTC, Top: *OCRA*, Update: *Immediate*, Flag: *MAX*"]
    #[inline(always)]
    pub fn ctc(self) -> &'a mut crate::W<REG> {
        self.variant(WGM2_A::CTC)
    }
    #[doc = "Fast PWM, Top: `0xff`, Update: *TOP*, Flag: *MAX*"]
    #[inline(always)]
    pub fn pwm_fast(self) -> &'a mut crate::W<REG> {
        self.variant(WGM2_A::PWM_FAST)
    }
}
#[doc = "Field `COM2B` reader - Compare Output B Mode"]
pub type COM2B_R = crate::FieldReader<COM2B_A>;
#[doc = "Compare Output B Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM2B_A {
    #[doc = "0: Normal port operation, OCix disconnected"]
    DISCONNECTED = 0,
    #[doc = "1: Toggle OCix on Compare Match (Might depend on WGM)"]
    MATCH_TOGGLE = 1,
    #[doc = "2: Clear OCix on Compare Match (If PWM is enabled, OCix is set at TOP)"]
    MATCH_CLEAR = 2,
    #[doc = "3: Set OCix on Compare Match (If PWM is enabled, OCix is cleared at TOP)"]
    MATCH_SET = 3,
}
impl From<COM2B_A> for u8 {
    #[inline(always)]
    fn from(variant: COM2B_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COM2B_A {
    type Ux = u8;
}
impl COM2B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COM2B_A {
        match self.bits {
            0 => COM2B_A::DISCONNECTED,
            1 => COM2B_A::MATCH_TOGGLE,
            2 => COM2B_A::MATCH_CLEAR,
            3 => COM2B_A::MATCH_SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal port operation, OCix disconnected"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == COM2B_A::DISCONNECTED
    }
    #[doc = "Toggle OCix on Compare Match (Might depend on WGM)"]
    #[inline(always)]
    pub fn is_match_toggle(&self) -> bool {
        *self == COM2B_A::MATCH_TOGGLE
    }
    #[doc = "Clear OCix on Compare Match (If PWM is enabled, OCix is set at TOP)"]
    #[inline(always)]
    pub fn is_match_clear(&self) -> bool {
        *self == COM2B_A::MATCH_CLEAR
    }
    #[doc = "Set OCix on Compare Match (If PWM is enabled, OCix is cleared at TOP)"]
    #[inline(always)]
    pub fn is_match_set(&self) -> bool {
        *self == COM2B_A::MATCH_SET
    }
}
#[doc = "Field `COM2B` writer - Compare Output B Mode"]
pub type COM2B_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COM2B_A>;
impl<'a, REG> COM2B_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal port operation, OCix disconnected"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(COM2B_A::DISCONNECTED)
    }
    #[doc = "Toggle OCix on Compare Match (Might depend on WGM)"]
    #[inline(always)]
    pub fn match_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(COM2B_A::MATCH_TOGGLE)
    }
    #[doc = "Clear OCix on Compare Match (If PWM is enabled, OCix is set at TOP)"]
    #[inline(always)]
    pub fn match_clear(self) -> &'a mut crate::W<REG> {
        self.variant(COM2B_A::MATCH_CLEAR)
    }
    #[doc = "Set OCix on Compare Match (If PWM is enabled, OCix is cleared at TOP)"]
    #[inline(always)]
    pub fn match_set(self) -> &'a mut crate::W<REG> {
        self.variant(COM2B_A::MATCH_SET)
    }
}
#[doc = "Field `COM2A` reader - Compare Output A Mode"]
pub use COM2B_R as COM2A_R;
#[doc = "Field `COM2A` writer - Compare Output A Mode"]
pub use COM2B_W as COM2A_W;
impl R {
    #[doc = "Bits 0:1 - Waveform Genration Mode"]
    #[inline(always)]
    pub fn wgm2(&self) -> WGM2_R {
        WGM2_R::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - Compare Output B Mode"]
    #[inline(always)]
    pub fn com2b(&self) -> COM2B_R {
        COM2B_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Compare Output A Mode"]
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
    #[doc = "Bits 4:5 - Compare Output B Mode"]
    #[inline(always)]
    #[must_use]
    pub fn com2b(&mut self) -> COM2B_W<TCCR2A_SPEC> {
        COM2B_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Compare Output A Mode"]
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
