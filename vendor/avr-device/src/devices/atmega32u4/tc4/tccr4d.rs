#[doc = "Register `TCCR4D` reader"]
pub type R = crate::R<TCCR4D_SPEC>;
#[doc = "Register `TCCR4D` writer"]
pub type W = crate::W<TCCR4D_SPEC>;
#[doc = "Field `WGM4` reader - Waveform Generation Mode bits"]
pub type WGM4_R = crate::FieldReader<WGM4_A>;
#[doc = "Waveform Generation Mode bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WGM4_A {
    #[doc = "0: Fast PWM, Update: *TOP*, Flag: *TOP*"]
    PWM_FAST = 0,
    #[doc = "1: Phase and Frequency Correct PWM, Update: *BOTTOM*, Flag: *BOTTOM*"]
    PWM_CORRECT = 1,
    #[doc = "2: PWM6 / Single-slope, Update: *TOP*, Flag: *TOP*"]
    PWM_SINGLE_SLOPE = 2,
    #[doc = "3: PWM6 / Dual-slope, Update: *BOTTOM*, Flag: *BOTTOM*"]
    PWM_DUAL_SLOPE = 3,
}
impl From<WGM4_A> for u8 {
    #[inline(always)]
    fn from(variant: WGM4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WGM4_A {
    type Ux = u8;
}
impl WGM4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WGM4_A {
        match self.bits {
            0 => WGM4_A::PWM_FAST,
            1 => WGM4_A::PWM_CORRECT,
            2 => WGM4_A::PWM_SINGLE_SLOPE,
            3 => WGM4_A::PWM_DUAL_SLOPE,
            _ => unreachable!(),
        }
    }
    #[doc = "Fast PWM, Update: *TOP*, Flag: *TOP*"]
    #[inline(always)]
    pub fn is_pwm_fast(&self) -> bool {
        *self == WGM4_A::PWM_FAST
    }
    #[doc = "Phase and Frequency Correct PWM, Update: *BOTTOM*, Flag: *BOTTOM*"]
    #[inline(always)]
    pub fn is_pwm_correct(&self) -> bool {
        *self == WGM4_A::PWM_CORRECT
    }
    #[doc = "PWM6 / Single-slope, Update: *TOP*, Flag: *TOP*"]
    #[inline(always)]
    pub fn is_pwm_single_slope(&self) -> bool {
        *self == WGM4_A::PWM_SINGLE_SLOPE
    }
    #[doc = "PWM6 / Dual-slope, Update: *BOTTOM*, Flag: *BOTTOM*"]
    #[inline(always)]
    pub fn is_pwm_dual_slope(&self) -> bool {
        *self == WGM4_A::PWM_DUAL_SLOPE
    }
}
#[doc = "Field `WGM4` writer - Waveform Generation Mode bits"]
pub type WGM4_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WGM4_A>;
impl<'a, REG> WGM4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Fast PWM, Update: *TOP*, Flag: *TOP*"]
    #[inline(always)]
    pub fn pwm_fast(self) -> &'a mut crate::W<REG> {
        self.variant(WGM4_A::PWM_FAST)
    }
    #[doc = "Phase and Frequency Correct PWM, Update: *BOTTOM*, Flag: *BOTTOM*"]
    #[inline(always)]
    pub fn pwm_correct(self) -> &'a mut crate::W<REG> {
        self.variant(WGM4_A::PWM_CORRECT)
    }
    #[doc = "PWM6 / Single-slope, Update: *TOP*, Flag: *TOP*"]
    #[inline(always)]
    pub fn pwm_single_slope(self) -> &'a mut crate::W<REG> {
        self.variant(WGM4_A::PWM_SINGLE_SLOPE)
    }
    #[doc = "PWM6 / Dual-slope, Update: *BOTTOM*, Flag: *BOTTOM*"]
    #[inline(always)]
    pub fn pwm_dual_slope(self) -> &'a mut crate::W<REG> {
        self.variant(WGM4_A::PWM_DUAL_SLOPE)
    }
}
#[doc = "Field `FPF4` reader - Fault Protection Interrupt Flag"]
pub type FPF4_R = crate::BitReader;
#[doc = "Field `FPF4` writer - Fault Protection Interrupt Flag"]
pub type FPF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPAC4` reader - Fault Protection Analog Comparator Enable"]
pub type FPAC4_R = crate::BitReader;
#[doc = "Field `FPAC4` writer - Fault Protection Analog Comparator Enable"]
pub type FPAC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPES4` reader - Fault Protection Edge Select"]
pub type FPES4_R = crate::BitReader;
#[doc = "Field `FPES4` writer - Fault Protection Edge Select"]
pub type FPES4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPNC4` reader - Fault Protection Noise Canceler"]
pub type FPNC4_R = crate::BitReader;
#[doc = "Field `FPNC4` writer - Fault Protection Noise Canceler"]
pub type FPNC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPEN4` reader - Fault Protection Mode Enable"]
pub type FPEN4_R = crate::BitReader;
#[doc = "Field `FPEN4` writer - Fault Protection Mode Enable"]
pub type FPEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIE4` reader - Fault Protection Interrupt Enable"]
pub type FPIE4_R = crate::BitReader;
#[doc = "Field `FPIE4` writer - Fault Protection Interrupt Enable"]
pub type FPIE4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode bits"]
    #[inline(always)]
    pub fn wgm4(&self) -> WGM4_R {
        WGM4_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Fault Protection Interrupt Flag"]
    #[inline(always)]
    pub fn fpf4(&self) -> FPF4_R {
        FPF4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault Protection Analog Comparator Enable"]
    #[inline(always)]
    pub fn fpac4(&self) -> FPAC4_R {
        FPAC4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault Protection Edge Select"]
    #[inline(always)]
    pub fn fpes4(&self) -> FPES4_R {
        FPES4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fault Protection Noise Canceler"]
    #[inline(always)]
    pub fn fpnc4(&self) -> FPNC4_R {
        FPNC4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fault Protection Mode Enable"]
    #[inline(always)]
    pub fn fpen4(&self) -> FPEN4_R {
        FPEN4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fault Protection Interrupt Enable"]
    #[inline(always)]
    pub fn fpie4(&self) -> FPIE4_R {
        FPIE4_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Generation Mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn wgm4(&mut self) -> WGM4_W<TCCR4D_SPEC> {
        WGM4_W::new(self, 0)
    }
    #[doc = "Bit 2 - Fault Protection Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpf4(&mut self) -> FPF4_W<TCCR4D_SPEC> {
        FPF4_W::new(self, 2)
    }
    #[doc = "Bit 3 - Fault Protection Analog Comparator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpac4(&mut self) -> FPAC4_W<TCCR4D_SPEC> {
        FPAC4_W::new(self, 3)
    }
    #[doc = "Bit 4 - Fault Protection Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn fpes4(&mut self) -> FPES4_W<TCCR4D_SPEC> {
        FPES4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Fault Protection Noise Canceler"]
    #[inline(always)]
    #[must_use]
    pub fn fpnc4(&mut self) -> FPNC4_W<TCCR4D_SPEC> {
        FPNC4_W::new(self, 5)
    }
    #[doc = "Bit 6 - Fault Protection Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpen4(&mut self) -> FPEN4_W<TCCR4D_SPEC> {
        FPEN4_W::new(self, 6)
    }
    #[doc = "Bit 7 - Fault Protection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpie4(&mut self) -> FPIE4_W<TCCR4D_SPEC> {
        FPIE4_W::new(self, 7)
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
#[doc = "Timer/Counter 4 Control Register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr4d::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr4d::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR4D_SPEC;
impl crate::RegisterSpec for TCCR4D_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr4d::R`](R) reader structure"]
impl crate::Readable for TCCR4D_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr4d::W`](W) writer structure"]
impl crate::Writable for TCCR4D_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR4D to value 0"]
impl crate::Resettable for TCCR4D_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
