#[doc = "Register `TCCR1B` reader"]
pub type R = crate::R<TCCR1B_SPEC>;
#[doc = "Register `TCCR1B` writer"]
pub type W = crate::W<TCCR1B_SPEC>;
#[doc = "Field `CS1` reader - Clock Select"]
pub type CS1_R = crate::FieldReader<CS1_A>;
#[doc = "Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS1_A {
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
impl From<CS1_A> for u8 {
    #[inline(always)]
    fn from(variant: CS1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CS1_A {
    type Ux = u8;
}
impl CS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CS1_A {
        match self.bits {
            0 => CS1_A::NO_CLOCK_SOURCE_TIMER_COUNTER_STOPPED,
            1 => CS1_A::CLK_IO_1_NO_PRESCALING,
            2 => CS1_A::CLK_IO_8_FROM_PRESCALER,
            3 => CS1_A::CLK_IO_64_FROM_PRESCALER,
            4 => CS1_A::CLK_IO_256_FROM_PRESCALER,
            5 => CS1_A::CLK_IO_1024_FROM_PRESCALER,
            6 => CS1_A::EXTERNAL_CLOCK_SOURCE_ON_TN_PIN_CLOCK_ON_FALLING_EDGE,
            7 => CS1_A::EXTERNAL_CLOCK_SOURCE_ON_TN_PIN_CLOCK_ON_RISING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn is_no_clock_source_timer_counter_stopped(&self) -> bool {
        *self == CS1_A::NO_CLOCK_SOURCE_TIMER_COUNTER_STOPPED
    }
    #[doc = "clk_IO/1 (no prescaling)"]
    #[inline(always)]
    pub fn is_clk_io_1_no_prescaling(&self) -> bool {
        *self == CS1_A::CLK_IO_1_NO_PRESCALING
    }
    #[doc = "clk_IO/8 (from prescaler)"]
    #[inline(always)]
    pub fn is_clk_io_8_from_prescaler(&self) -> bool {
        *self == CS1_A::CLK_IO_8_FROM_PRESCALER
    }
    #[doc = "clk_IO/64 (from prescaler)"]
    #[inline(always)]
    pub fn is_clk_io_64_from_prescaler(&self) -> bool {
        *self == CS1_A::CLK_IO_64_FROM_PRESCALER
    }
    #[doc = "clk_IO/256 (from prescaler)"]
    #[inline(always)]
    pub fn is_clk_io_256_from_prescaler(&self) -> bool {
        *self == CS1_A::CLK_IO_256_FROM_PRESCALER
    }
    #[doc = "clk_IO/1024 (from prescaler)"]
    #[inline(always)]
    pub fn is_clk_io_1024_from_prescaler(&self) -> bool {
        *self == CS1_A::CLK_IO_1024_FROM_PRESCALER
    }
    #[doc = "External clock source on Tn pin, clock on falling edge"]
    #[inline(always)]
    pub fn is_external_clock_source_on_tn_pin_clock_on_falling_edge(&self) -> bool {
        *self == CS1_A::EXTERNAL_CLOCK_SOURCE_ON_TN_PIN_CLOCK_ON_FALLING_EDGE
    }
    #[doc = "External clock source on Tn pin, clock on rising edge"]
    #[inline(always)]
    pub fn is_external_clock_source_on_tn_pin_clock_on_rising_edge(&self) -> bool {
        *self == CS1_A::EXTERNAL_CLOCK_SOURCE_ON_TN_PIN_CLOCK_ON_RISING_EDGE
    }
}
#[doc = "Field `CS1` writer - Clock Select"]
pub type CS1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, CS1_A>;
impl<'a, REG> CS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn no_clock_source_timer_counter_stopped(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::NO_CLOCK_SOURCE_TIMER_COUNTER_STOPPED)
    }
    #[doc = "clk_IO/1 (no prescaling)"]
    #[inline(always)]
    pub fn clk_io_1_no_prescaling(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::CLK_IO_1_NO_PRESCALING)
    }
    #[doc = "clk_IO/8 (from prescaler)"]
    #[inline(always)]
    pub fn clk_io_8_from_prescaler(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::CLK_IO_8_FROM_PRESCALER)
    }
    #[doc = "clk_IO/64 (from prescaler)"]
    #[inline(always)]
    pub fn clk_io_64_from_prescaler(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::CLK_IO_64_FROM_PRESCALER)
    }
    #[doc = "clk_IO/256 (from prescaler)"]
    #[inline(always)]
    pub fn clk_io_256_from_prescaler(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::CLK_IO_256_FROM_PRESCALER)
    }
    #[doc = "clk_IO/1024 (from prescaler)"]
    #[inline(always)]
    pub fn clk_io_1024_from_prescaler(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::CLK_IO_1024_FROM_PRESCALER)
    }
    #[doc = "External clock source on Tn pin, clock on falling edge"]
    #[inline(always)]
    pub fn external_clock_source_on_tn_pin_clock_on_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::EXTERNAL_CLOCK_SOURCE_ON_TN_PIN_CLOCK_ON_FALLING_EDGE)
    }
    #[doc = "External clock source on Tn pin, clock on rising edge"]
    #[inline(always)]
    pub fn external_clock_source_on_tn_pin_clock_on_rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::EXTERNAL_CLOCK_SOURCE_ON_TN_PIN_CLOCK_ON_RISING_EDGE)
    }
}
#[doc = "Field `WGM1` reader - Waveform Generation Mode"]
pub type WGM1_R = crate::FieldReader<WGM1_A>;
#[doc = "Waveform Generation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WGM1_A {
    #[doc = "0: Normal mode of operation"]
    NORMAL_MODE_OF_OPERATION = 0,
    #[doc = "1: PWM, phase correct, 8-bit"]
    PWM_PHASE_CORRECT_8_BIT = 1,
    #[doc = "2: PWM, phase correct, 9-bit"]
    PWM_PHASE_CORRECT_9_BIT = 2,
    #[doc = "3: PWM, phase correct, 10-bit"]
    PWM_PHASE_CORRECT_10_BIT = 3,
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
            0 => WGM1_A::NORMAL_MODE_OF_OPERATION,
            1 => WGM1_A::PWM_PHASE_CORRECT_8_BIT,
            2 => WGM1_A::PWM_PHASE_CORRECT_9_BIT,
            3 => WGM1_A::PWM_PHASE_CORRECT_10_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal mode of operation"]
    #[inline(always)]
    pub fn is_normal_mode_of_operation(&self) -> bool {
        *self == WGM1_A::NORMAL_MODE_OF_OPERATION
    }
    #[doc = "PWM, phase correct, 8-bit"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_8_bit(&self) -> bool {
        *self == WGM1_A::PWM_PHASE_CORRECT_8_BIT
    }
    #[doc = "PWM, phase correct, 9-bit"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_9_bit(&self) -> bool {
        *self == WGM1_A::PWM_PHASE_CORRECT_9_BIT
    }
    #[doc = "PWM, phase correct, 10-bit"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_10_bit(&self) -> bool {
        *self == WGM1_A::PWM_PHASE_CORRECT_10_BIT
    }
}
#[doc = "Field `WGM1` writer - Waveform Generation Mode"]
pub type WGM1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WGM1_A>;
impl<'a, REG> WGM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode of operation"]
    #[inline(always)]
    pub fn normal_mode_of_operation(self) -> &'a mut crate::W<REG> {
        self.variant(WGM1_A::NORMAL_MODE_OF_OPERATION)
    }
    #[doc = "PWM, phase correct, 8-bit"]
    #[inline(always)]
    pub fn pwm_phase_correct_8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(WGM1_A::PWM_PHASE_CORRECT_8_BIT)
    }
    #[doc = "PWM, phase correct, 9-bit"]
    #[inline(always)]
    pub fn pwm_phase_correct_9_bit(self) -> &'a mut crate::W<REG> {
        self.variant(WGM1_A::PWM_PHASE_CORRECT_9_BIT)
    }
    #[doc = "PWM, phase correct, 10-bit"]
    #[inline(always)]
    pub fn pwm_phase_correct_10_bit(self) -> &'a mut crate::W<REG> {
        self.variant(WGM1_A::PWM_PHASE_CORRECT_10_BIT)
    }
}
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::BitReader;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICES1` reader - Input Capture 1 Edge Select"]
pub type ICES1_R = crate::BitReader;
#[doc = "Field `ICES1` writer - Input Capture 1 Edge Select"]
pub type ICES1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICNC1` reader - Input Capture 1 Noise Canceller"]
pub type ICNC1_R = crate::BitReader;
#[doc = "Field `ICNC1` writer - Input Capture 1 Noise Canceller"]
pub type ICNC1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    pub fn cs1(&self) -> CS1_R {
        CS1_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm1(&self) -> WGM1_R {
        WGM1_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input Capture 1 Edge Select"]
    #[inline(always)]
    pub fn ices1(&self) -> ICES1_R {
        ICES1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input Capture 1 Noise Canceller"]
    #[inline(always)]
    pub fn icnc1(&self) -> ICNC1_R {
        ICNC1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn cs1(&mut self) -> CS1_W<TCCR1B_SPEC> {
        CS1_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm1(&mut self) -> WGM1_W<TCCR1B_SPEC> {
        WGM1_W::new(self, 3)
    }
    #[doc = "Bit 5 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<TCCR1B_SPEC> {
        RES_W::new(self, 5)
    }
    #[doc = "Bit 6 - Input Capture 1 Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn ices1(&mut self) -> ICES1_W<TCCR1B_SPEC> {
        ICES1_W::new(self, 6)
    }
    #[doc = "Bit 7 - Input Capture 1 Noise Canceller"]
    #[inline(always)]
    #[must_use]
    pub fn icnc1(&mut self) -> ICNC1_W<TCCR1B_SPEC> {
        ICNC1_W::new(self, 7)
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
#[doc = "Timer/Counter1 Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr1b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr1b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR1B_SPEC;
impl crate::RegisterSpec for TCCR1B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr1b::R`](R) reader structure"]
impl crate::Readable for TCCR1B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr1b::W`](W) writer structure"]
impl crate::Writable for TCCR1B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR1B to value 0"]
impl crate::Resettable for TCCR1B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
