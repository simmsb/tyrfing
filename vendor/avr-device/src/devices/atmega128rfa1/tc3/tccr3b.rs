#[doc = "Register `TCCR3B` reader"]
pub type R = crate::R<TCCR3B_SPEC>;
#[doc = "Register `TCCR3B` writer"]
pub type W = crate::W<TCCR3B_SPEC>;
#[doc = "Field `CS3` reader - Clock Select"]
pub type CS3_R = crate::FieldReader<CS3_A>;
#[doc = "Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS3_A {
    #[doc = "0: No clock source (Timer/Counter stopped)"]
    NO_CLOCK_SOURCE_TIMER_COUNTER_STOPPED = 0,
    #[doc = "1: clk_IO/1 (no prescaling)"]
    CLK_IO_1_NO_PRESCALING = 1,
    #[doc = "2: clk_IO/8 (from prescaler)"]
    CLK_IO_8_FROM_PRESCALER = 2,
    #[doc = "3: clk_IO/64 (from prescaler)"]
    CLK_IO_64_FROM_PRESCALER = 3,
    #[doc = "4: clk_IO/256 (from prescaler)"]
    CLK_IO_256_FROM_PRESCALER = 4,
    #[doc = "5: clk_IO/1024 (from prescaler)"]
    CLK_IO_1024_FROM_PRESCALER = 5,
    #[doc = "6: External clock source on Tn pin, clock on falling edge"]
    EXTERNAL_CLOCK_SOURCE_ON_TN_PIN_CLOCK_ON_FALLING_EDGE = 6,
    #[doc = "7: External clock source on Tn pin, clock on rising edge"]
    EXTERNAL_CLOCK_SOURCE_ON_TN_PIN_CLOCK_ON_RISING_EDGE = 7,
}
impl From<CS3_A> for u8 {
    #[inline(always)]
    fn from(variant: CS3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CS3_A {
    type Ux = u8;
}
impl CS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CS3_A {
        match self.bits {
            0 => CS3_A::NO_CLOCK_SOURCE_TIMER_COUNTER_STOPPED,
            1 => CS3_A::CLK_IO_1_NO_PRESCALING,
            2 => CS3_A::CLK_IO_8_FROM_PRESCALER,
            3 => CS3_A::CLK_IO_64_FROM_PRESCALER,
            4 => CS3_A::CLK_IO_256_FROM_PRESCALER,
            5 => CS3_A::CLK_IO_1024_FROM_PRESCALER,
            6 => CS3_A::EXTERNAL_CLOCK_SOURCE_ON_TN_PIN_CLOCK_ON_FALLING_EDGE,
            7 => CS3_A::EXTERNAL_CLOCK_SOURCE_ON_TN_PIN_CLOCK_ON_RISING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn is_no_clock_source_timer_counter_stopped(&self) -> bool {
        *self == CS3_A::NO_CLOCK_SOURCE_TIMER_COUNTER_STOPPED
    }
    #[doc = "clk_IO/1 (no prescaling)"]
    #[inline(always)]
    pub fn is_clk_io_1_no_prescaling(&self) -> bool {
        *self == CS3_A::CLK_IO_1_NO_PRESCALING
    }
    #[doc = "clk_IO/8 (from prescaler)"]
    #[inline(always)]
    pub fn is_clk_io_8_from_prescaler(&self) -> bool {
        *self == CS3_A::CLK_IO_8_FROM_PRESCALER
    }
    #[doc = "clk_IO/64 (from prescaler)"]
    #[inline(always)]
    pub fn is_clk_io_64_from_prescaler(&self) -> bool {
        *self == CS3_A::CLK_IO_64_FROM_PRESCALER
    }
    #[doc = "clk_IO/256 (from prescaler)"]
    #[inline(always)]
    pub fn is_clk_io_256_from_prescaler(&self) -> bool {
        *self == CS3_A::CLK_IO_256_FROM_PRESCALER
    }
    #[doc = "clk_IO/1024 (from prescaler)"]
    #[inline(always)]
    pub fn is_clk_io_1024_from_prescaler(&self) -> bool {
        *self == CS3_A::CLK_IO_1024_FROM_PRESCALER
    }
    #[doc = "External clock source on Tn pin, clock on falling edge"]
    #[inline(always)]
    pub fn is_external_clock_source_on_tn_pin_clock_on_falling_edge(&self) -> bool {
        *self == CS3_A::EXTERNAL_CLOCK_SOURCE_ON_TN_PIN_CLOCK_ON_FALLING_EDGE
    }
    #[doc = "External clock source on Tn pin, clock on rising edge"]
    #[inline(always)]
    pub fn is_external_clock_source_on_tn_pin_clock_on_rising_edge(&self) -> bool {
        *self == CS3_A::EXTERNAL_CLOCK_SOURCE_ON_TN_PIN_CLOCK_ON_RISING_EDGE
    }
}
#[doc = "Field `CS3` writer - Clock Select"]
pub type CS3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, CS3_A>;
impl<'a, REG> CS3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn no_clock_source_timer_counter_stopped(self) -> &'a mut crate::W<REG> {
        self.variant(CS3_A::NO_CLOCK_SOURCE_TIMER_COUNTER_STOPPED)
    }
    #[doc = "clk_IO/1 (no prescaling)"]
    #[inline(always)]
    pub fn clk_io_1_no_prescaling(self) -> &'a mut crate::W<REG> {
        self.variant(CS3_A::CLK_IO_1_NO_PRESCALING)
    }
    #[doc = "clk_IO/8 (from prescaler)"]
    #[inline(always)]
    pub fn clk_io_8_from_prescaler(self) -> &'a mut crate::W<REG> {
        self.variant(CS3_A::CLK_IO_8_FROM_PRESCALER)
    }
    #[doc = "clk_IO/64 (from prescaler)"]
    #[inline(always)]
    pub fn clk_io_64_from_prescaler(self) -> &'a mut crate::W<REG> {
        self.variant(CS3_A::CLK_IO_64_FROM_PRESCALER)
    }
    #[doc = "clk_IO/256 (from prescaler)"]
    #[inline(always)]
    pub fn clk_io_256_from_prescaler(self) -> &'a mut crate::W<REG> {
        self.variant(CS3_A::CLK_IO_256_FROM_PRESCALER)
    }
    #[doc = "clk_IO/1024 (from prescaler)"]
    #[inline(always)]
    pub fn clk_io_1024_from_prescaler(self) -> &'a mut crate::W<REG> {
        self.variant(CS3_A::CLK_IO_1024_FROM_PRESCALER)
    }
    #[doc = "External clock source on Tn pin, clock on falling edge"]
    #[inline(always)]
    pub fn external_clock_source_on_tn_pin_clock_on_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CS3_A::EXTERNAL_CLOCK_SOURCE_ON_TN_PIN_CLOCK_ON_FALLING_EDGE)
    }
    #[doc = "External clock source on Tn pin, clock on rising edge"]
    #[inline(always)]
    pub fn external_clock_source_on_tn_pin_clock_on_rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CS3_A::EXTERNAL_CLOCK_SOURCE_ON_TN_PIN_CLOCK_ON_RISING_EDGE)
    }
}
#[doc = "Field `WGM3` reader - Waveform Generation Mode"]
pub type WGM3_R = crate::FieldReader<WGM3_A>;
#[doc = "Waveform Generation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WGM3_A {
    #[doc = "0: Normal mode of operation"]
    NORMAL_MODE_OF_OPERATION = 0,
    #[doc = "1: PWM, phase correct, 8-bit"]
    PWM_PHASE_CORRECT_8_BIT = 1,
    #[doc = "2: PWM, phase correct, 9-bit"]
    PWM_PHASE_CORRECT_9_BIT = 2,
    #[doc = "3: PWM, phase correct, 10-bit"]
    PWM_PHASE_CORRECT_10_BIT = 3,
}
impl From<WGM3_A> for u8 {
    #[inline(always)]
    fn from(variant: WGM3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WGM3_A {
    type Ux = u8;
}
impl WGM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WGM3_A {
        match self.bits {
            0 => WGM3_A::NORMAL_MODE_OF_OPERATION,
            1 => WGM3_A::PWM_PHASE_CORRECT_8_BIT,
            2 => WGM3_A::PWM_PHASE_CORRECT_9_BIT,
            3 => WGM3_A::PWM_PHASE_CORRECT_10_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal mode of operation"]
    #[inline(always)]
    pub fn is_normal_mode_of_operation(&self) -> bool {
        *self == WGM3_A::NORMAL_MODE_OF_OPERATION
    }
    #[doc = "PWM, phase correct, 8-bit"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_8_bit(&self) -> bool {
        *self == WGM3_A::PWM_PHASE_CORRECT_8_BIT
    }
    #[doc = "PWM, phase correct, 9-bit"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_9_bit(&self) -> bool {
        *self == WGM3_A::PWM_PHASE_CORRECT_9_BIT
    }
    #[doc = "PWM, phase correct, 10-bit"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_10_bit(&self) -> bool {
        *self == WGM3_A::PWM_PHASE_CORRECT_10_BIT
    }
}
#[doc = "Field `WGM3` writer - Waveform Generation Mode"]
pub type WGM3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WGM3_A>;
impl<'a, REG> WGM3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode of operation"]
    #[inline(always)]
    pub fn normal_mode_of_operation(self) -> &'a mut crate::W<REG> {
        self.variant(WGM3_A::NORMAL_MODE_OF_OPERATION)
    }
    #[doc = "PWM, phase correct, 8-bit"]
    #[inline(always)]
    pub fn pwm_phase_correct_8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(WGM3_A::PWM_PHASE_CORRECT_8_BIT)
    }
    #[doc = "PWM, phase correct, 9-bit"]
    #[inline(always)]
    pub fn pwm_phase_correct_9_bit(self) -> &'a mut crate::W<REG> {
        self.variant(WGM3_A::PWM_PHASE_CORRECT_9_BIT)
    }
    #[doc = "PWM, phase correct, 10-bit"]
    #[inline(always)]
    pub fn pwm_phase_correct_10_bit(self) -> &'a mut crate::W<REG> {
        self.variant(WGM3_A::PWM_PHASE_CORRECT_10_BIT)
    }
}
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::BitReader;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICES3` reader - Input Capture 3 Edge Select"]
pub type ICES3_R = crate::BitReader;
#[doc = "Field `ICES3` writer - Input Capture 3 Edge Select"]
pub type ICES3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICNC3` reader - Input Capture 3 Noise Canceller"]
pub type ICNC3_R = crate::BitReader;
#[doc = "Field `ICNC3` writer - Input Capture 3 Noise Canceller"]
pub type ICNC3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    pub fn cs3(&self) -> CS3_R {
        CS3_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm3(&self) -> WGM3_R {
        WGM3_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input Capture 3 Edge Select"]
    #[inline(always)]
    pub fn ices3(&self) -> ICES3_R {
        ICES3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input Capture 3 Noise Canceller"]
    #[inline(always)]
    pub fn icnc3(&self) -> ICNC3_R {
        ICNC3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn cs3(&mut self) -> CS3_W<TCCR3B_SPEC> {
        CS3_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm3(&mut self) -> WGM3_W<TCCR3B_SPEC> {
        WGM3_W::new(self, 3)
    }
    #[doc = "Bit 5 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<TCCR3B_SPEC> {
        RES_W::new(self, 5)
    }
    #[doc = "Bit 6 - Input Capture 3 Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn ices3(&mut self) -> ICES3_W<TCCR3B_SPEC> {
        ICES3_W::new(self, 6)
    }
    #[doc = "Bit 7 - Input Capture 3 Noise Canceller"]
    #[inline(always)]
    #[must_use]
    pub fn icnc3(&mut self) -> ICNC3_W<TCCR3B_SPEC> {
        ICNC3_W::new(self, 7)
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
#[doc = "Timer/Counter3 Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr3b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr3b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR3B_SPEC;
impl crate::RegisterSpec for TCCR3B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr3b::R`](R) reader structure"]
impl crate::Readable for TCCR3B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr3b::W`](W) writer structure"]
impl crate::Writable for TCCR3B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR3B to value 0"]
impl crate::Resettable for TCCR3B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
