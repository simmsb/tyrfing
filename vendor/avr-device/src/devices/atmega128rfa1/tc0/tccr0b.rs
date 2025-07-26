#[doc = "Register `TCCR0B` reader"]
pub type R = crate::R<TCCR0B_SPEC>;
#[doc = "Register `TCCR0B` writer"]
pub type W = crate::W<TCCR0B_SPEC>;
#[doc = "Field `CS0` reader - Clock Select"]
pub type CS0_R = crate::FieldReader<CS0_A>;
#[doc = "Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS0_A {
    #[doc = "0: No clock source (Timer/Counter0 stopped)"]
    NO_CLOCK_SOURCE_TIMER_COUNTER0_STOPPED = 0,
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
    #[doc = "6: External clock source on T0 pin, clock on falling edge"]
    EXTERNAL_CLOCK_SOURCE_ON_T0_PIN_CLOCK_ON_FALLING_EDGE = 6,
    #[doc = "7: External clock source on T0 pin, clock on rising edge"]
    EXTERNAL_CLOCK_SOURCE_ON_T0_PIN_CLOCK_ON_RISING_EDGE = 7,
}
impl From<CS0_A> for u8 {
    #[inline(always)]
    fn from(variant: CS0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CS0_A {
    type Ux = u8;
}
impl CS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CS0_A {
        match self.bits {
            0 => CS0_A::NO_CLOCK_SOURCE_TIMER_COUNTER0_STOPPED,
            1 => CS0_A::CLK_IO_1_NO_PRESCALING,
            2 => CS0_A::CLK_IO_8_FROM_PRESCALER,
            3 => CS0_A::CLK_IO_64_FROM_PRESCALER,
            4 => CS0_A::CLK_IO_256_FROM_PRESCALER,
            5 => CS0_A::CLK_IO_1024_FROM_PRESCALER,
            6 => CS0_A::EXTERNAL_CLOCK_SOURCE_ON_T0_PIN_CLOCK_ON_FALLING_EDGE,
            7 => CS0_A::EXTERNAL_CLOCK_SOURCE_ON_T0_PIN_CLOCK_ON_RISING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock source (Timer/Counter0 stopped)"]
    #[inline(always)]
    pub fn is_no_clock_source_timer_counter0_stopped(&self) -> bool {
        *self == CS0_A::NO_CLOCK_SOURCE_TIMER_COUNTER0_STOPPED
    }
    #[doc = "clk_IO/1 (no prescaling)"]
    #[inline(always)]
    pub fn is_clk_io_1_no_prescaling(&self) -> bool {
        *self == CS0_A::CLK_IO_1_NO_PRESCALING
    }
    #[doc = "clk_IO/8 (from prescaler)"]
    #[inline(always)]
    pub fn is_clk_io_8_from_prescaler(&self) -> bool {
        *self == CS0_A::CLK_IO_8_FROM_PRESCALER
    }
    #[doc = "clk_IO/64 (from prescaler)"]
    #[inline(always)]
    pub fn is_clk_io_64_from_prescaler(&self) -> bool {
        *self == CS0_A::CLK_IO_64_FROM_PRESCALER
    }
    #[doc = "clk_IO/256 (from prescaler)"]
    #[inline(always)]
    pub fn is_clk_io_256_from_prescaler(&self) -> bool {
        *self == CS0_A::CLK_IO_256_FROM_PRESCALER
    }
    #[doc = "clk_IO/1024 (from prescaler)"]
    #[inline(always)]
    pub fn is_clk_io_1024_from_prescaler(&self) -> bool {
        *self == CS0_A::CLK_IO_1024_FROM_PRESCALER
    }
    #[doc = "External clock source on T0 pin, clock on falling edge"]
    #[inline(always)]
    pub fn is_external_clock_source_on_t0_pin_clock_on_falling_edge(&self) -> bool {
        *self == CS0_A::EXTERNAL_CLOCK_SOURCE_ON_T0_PIN_CLOCK_ON_FALLING_EDGE
    }
    #[doc = "External clock source on T0 pin, clock on rising edge"]
    #[inline(always)]
    pub fn is_external_clock_source_on_t0_pin_clock_on_rising_edge(&self) -> bool {
        *self == CS0_A::EXTERNAL_CLOCK_SOURCE_ON_T0_PIN_CLOCK_ON_RISING_EDGE
    }
}
#[doc = "Field `CS0` writer - Clock Select"]
pub type CS0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, CS0_A>;
impl<'a, REG> CS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock source (Timer/Counter0 stopped)"]
    #[inline(always)]
    pub fn no_clock_source_timer_counter0_stopped(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::NO_CLOCK_SOURCE_TIMER_COUNTER0_STOPPED)
    }
    #[doc = "clk_IO/1 (no prescaling)"]
    #[inline(always)]
    pub fn clk_io_1_no_prescaling(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::CLK_IO_1_NO_PRESCALING)
    }
    #[doc = "clk_IO/8 (from prescaler)"]
    #[inline(always)]
    pub fn clk_io_8_from_prescaler(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::CLK_IO_8_FROM_PRESCALER)
    }
    #[doc = "clk_IO/64 (from prescaler)"]
    #[inline(always)]
    pub fn clk_io_64_from_prescaler(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::CLK_IO_64_FROM_PRESCALER)
    }
    #[doc = "clk_IO/256 (from prescaler)"]
    #[inline(always)]
    pub fn clk_io_256_from_prescaler(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::CLK_IO_256_FROM_PRESCALER)
    }
    #[doc = "clk_IO/1024 (from prescaler)"]
    #[inline(always)]
    pub fn clk_io_1024_from_prescaler(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::CLK_IO_1024_FROM_PRESCALER)
    }
    #[doc = "External clock source on T0 pin, clock on falling edge"]
    #[inline(always)]
    pub fn external_clock_source_on_t0_pin_clock_on_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::EXTERNAL_CLOCK_SOURCE_ON_T0_PIN_CLOCK_ON_FALLING_EDGE)
    }
    #[doc = "External clock source on T0 pin, clock on rising edge"]
    #[inline(always)]
    pub fn external_clock_source_on_t0_pin_clock_on_rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::EXTERNAL_CLOCK_SOURCE_ON_T0_PIN_CLOCK_ON_RISING_EDGE)
    }
}
#[doc = "Field `WGM02` reader - No Description."]
pub type WGM02_R = crate::BitReader;
#[doc = "Field `WGM02` writer - No Description."]
pub type WGM02_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `FOC0B` reader - Force Output Compare B"]
pub type FOC0B_R = crate::BitReader;
#[doc = "Field `FOC0B` writer - Force Output Compare B"]
pub type FOC0B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOC0A` reader - Force Output Compare A"]
pub type FOC0A_R = crate::BitReader;
#[doc = "Field `FOC0A` writer - Force Output Compare A"]
pub type FOC0A_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    pub fn cs0(&self) -> CS0_R {
        CS0_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    pub fn wgm02(&self) -> WGM02_R {
        WGM02_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Force Output Compare B"]
    #[inline(always)]
    pub fn foc0b(&self) -> FOC0B_R {
        FOC0B_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Output Compare A"]
    #[inline(always)]
    pub fn foc0a(&self) -> FOC0A_R {
        FOC0A_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn cs0(&mut self) -> CS0_W<TCCR0B_SPEC> {
        CS0_W::new(self, 0)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn wgm02(&mut self) -> WGM02_W<TCCR0B_SPEC> {
        WGM02_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<TCCR0B_SPEC> {
        RES_W::new(self, 4)
    }
    #[doc = "Bit 6 - Force Output Compare B"]
    #[inline(always)]
    #[must_use]
    pub fn foc0b(&mut self) -> FOC0B_W<TCCR0B_SPEC> {
        FOC0B_W::new(self, 6)
    }
    #[doc = "Bit 7 - Force Output Compare A"]
    #[inline(always)]
    #[must_use]
    pub fn foc0a(&mut self) -> FOC0A_W<TCCR0B_SPEC> {
        FOC0A_W::new(self, 7)
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
#[doc = "Timer/Counter0 Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr0b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr0b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR0B_SPEC;
impl crate::RegisterSpec for TCCR0B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr0b::R`](R) reader structure"]
impl crate::Readable for TCCR0B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr0b::W`](W) writer structure"]
impl crate::Writable for TCCR0B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR0B to value 0"]
impl crate::Resettable for TCCR0B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
