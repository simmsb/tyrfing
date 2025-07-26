#[doc = "Register `TCCR1D` reader"]
pub type R = crate::R<TCCR1D_SPEC>;
#[doc = "Register `TCCR1D` writer"]
pub type W = crate::W<TCCR1D_SPEC>;
#[doc = "Field `WGM1` reader - Waveform Generation Mode Bit"]
pub type WGM1_R = crate::FieldReader<WGM1_A>;
#[doc = "Waveform Generation Mode Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WGM1_A {
    #[doc = "0: Fast PWM, Update: *TOP*, Flag: *TOP*"]
    PWM_FAST = 0,
    #[doc = "1: Phase and Frequency Correct PWM, Update: *BOTTOM*, Flag: *BOTTOM*"]
    PWM_CORRECT = 1,
    #[doc = "2: PWM6 / Single-slope, Update: *TOP*, Flag: *TOP*"]
    PWM_SINGLE_SLOPE = 2,
    #[doc = "3: PWM6 / Dual-slope, Update: *BOTTOM*, Flag: *BOTTOM*"]
    PWM_DUAL_SLOPE = 3,
}
impl From<WGM1_A> for u8 {
    #[inline(always)]
    fn from(variant: WGM1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WGM1_A {
    type Ux = u8;
}
impl WGM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WGM1_A {
        match self.bits {
            0 => WGM1_A::PWM_FAST,
            1 => WGM1_A::PWM_CORRECT,
            2 => WGM1_A::PWM_SINGLE_SLOPE,
            3 => WGM1_A::PWM_DUAL_SLOPE,
            _ => unreachable!(),
        }
    }
    #[doc = "Fast PWM, Update: *TOP*, Flag: *TOP*"]
    #[inline(always)]
    pub fn is_pwm_fast(&self) -> bool {
        *self == WGM1_A::PWM_FAST
    }
    #[doc = "Phase and Frequency Correct PWM, Update: *BOTTOM*, Flag: *BOTTOM*"]
    #[inline(always)]
    pub fn is_pwm_correct(&self) -> bool {
        *self == WGM1_A::PWM_CORRECT
    }
    #[doc = "PWM6 / Single-slope, Update: *TOP*, Flag: *TOP*"]
    #[inline(always)]
    pub fn is_pwm_single_slope(&self) -> bool {
        *self == WGM1_A::PWM_SINGLE_SLOPE
    }
    #[doc = "PWM6 / Dual-slope, Update: *BOTTOM*, Flag: *BOTTOM*"]
    #[inline(always)]
    pub fn is_pwm_dual_slope(&self) -> bool {
        *self == WGM1_A::PWM_DUAL_SLOPE
    }
}
#[doc = "Field `WGM1` writer - Waveform Generation Mode Bit"]
pub type WGM1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WGM1_A>;
impl<'a, REG> WGM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Fast PWM, Update: *TOP*, Flag: *TOP*"]
    #[inline(always)]
    pub fn pwm_fast(self) -> &'a mut crate::W<REG> {
        self.variant(WGM1_A::PWM_FAST)
    }
    #[doc = "Phase and Frequency Correct PWM, Update: *BOTTOM*, Flag: *BOTTOM*"]
    #[inline(always)]
    pub fn pwm_correct(self) -> &'a mut crate::W<REG> {
        self.variant(WGM1_A::PWM_CORRECT)
    }
    #[doc = "PWM6 / Single-slope, Update: *TOP*, Flag: *TOP*"]
    #[inline(always)]
    pub fn pwm_single_slope(self) -> &'a mut crate::W<REG> {
        self.variant(WGM1_A::PWM_SINGLE_SLOPE)
    }
    #[doc = "PWM6 / Dual-slope, Update: *BOTTOM*, Flag: *BOTTOM*"]
    #[inline(always)]
    pub fn pwm_dual_slope(self) -> &'a mut crate::W<REG> {
        self.variant(WGM1_A::PWM_DUAL_SLOPE)
    }
}
#[doc = "Field `FPF1` reader - Fault Protection Interrupt Flag"]
pub type FPF1_R = crate::BitReader;
#[doc = "Field `FPF1` writer - Fault Protection Interrupt Flag"]
pub type FPF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPAC1` reader - Fault Protection Analog Comparator Enable"]
pub type FPAC1_R = crate::BitReader;
#[doc = "Field `FPAC1` writer - Fault Protection Analog Comparator Enable"]
pub type FPAC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPES1` reader - Fault Protection Edge Select"]
pub type FPES1_R = crate::BitReader;
#[doc = "Field `FPES1` writer - Fault Protection Edge Select"]
pub type FPES1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPNC1` reader - Fault Protection Noise Canceler"]
pub type FPNC1_R = crate::BitReader;
#[doc = "Field `FPNC1` writer - Fault Protection Noise Canceler"]
pub type FPNC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPEN1` reader - Fault Protection Mode Enable"]
pub type FPEN1_R = crate::BitReader;
#[doc = "Field `FPEN1` writer - Fault Protection Mode Enable"]
pub type FPEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIE1` reader - Fault Protection Interrupt Enable"]
pub type FPIE1_R = crate::BitReader;
#[doc = "Field `FPIE1` writer - Fault Protection Interrupt Enable"]
pub type FPIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode Bit"]
    #[inline(always)]
    pub fn wgm1(&self) -> WGM1_R {
        WGM1_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Fault Protection Interrupt Flag"]
    #[inline(always)]
    pub fn fpf1(&self) -> FPF1_R {
        FPF1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault Protection Analog Comparator Enable"]
    #[inline(always)]
    pub fn fpac1(&self) -> FPAC1_R {
        FPAC1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault Protection Edge Select"]
    #[inline(always)]
    pub fn fpes1(&self) -> FPES1_R {
        FPES1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fault Protection Noise Canceler"]
    #[inline(always)]
    pub fn fpnc1(&self) -> FPNC1_R {
        FPNC1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fault Protection Mode Enable"]
    #[inline(always)]
    pub fn fpen1(&self) -> FPEN1_R {
        FPEN1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fault Protection Interrupt Enable"]
    #[inline(always)]
    pub fn fpie1(&self) -> FPIE1_R {
        FPIE1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Generation Mode Bit"]
    #[inline(always)]
    #[must_use]
    pub fn wgm1(&mut self) -> WGM1_W<TCCR1D_SPEC> {
        WGM1_W::new(self, 0)
    }
    #[doc = "Bit 2 - Fault Protection Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpf1(&mut self) -> FPF1_W<TCCR1D_SPEC> {
        FPF1_W::new(self, 2)
    }
    #[doc = "Bit 3 - Fault Protection Analog Comparator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpac1(&mut self) -> FPAC1_W<TCCR1D_SPEC> {
        FPAC1_W::new(self, 3)
    }
    #[doc = "Bit 4 - Fault Protection Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn fpes1(&mut self) -> FPES1_W<TCCR1D_SPEC> {
        FPES1_W::new(self, 4)
    }
    #[doc = "Bit 5 - Fault Protection Noise Canceler"]
    #[inline(always)]
    #[must_use]
    pub fn fpnc1(&mut self) -> FPNC1_W<TCCR1D_SPEC> {
        FPNC1_W::new(self, 5)
    }
    #[doc = "Bit 6 - Fault Protection Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpen1(&mut self) -> FPEN1_W<TCCR1D_SPEC> {
        FPEN1_W::new(self, 6)
    }
    #[doc = "Bit 7 - Fault Protection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpie1(&mut self) -> FPIE1_W<TCCR1D_SPEC> {
        FPIE1_W::new(self, 7)
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
#[doc = "Timer/Counter Control Register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr1d::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr1d::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR1D_SPEC;
impl crate::RegisterSpec for TCCR1D_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr1d::R`](R) reader structure"]
impl crate::Readable for TCCR1D_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr1d::W`](W) writer structure"]
impl crate::Writable for TCCR1D_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR1D to value 0"]
impl crate::Resettable for TCCR1D_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
