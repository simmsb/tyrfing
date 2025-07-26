#[doc = "Register `TCCR4B` reader"]
pub type R = crate::R<TCCR4B_SPEC>;
#[doc = "Register `TCCR4B` writer"]
pub type W = crate::W<TCCR4B_SPEC>;
#[doc = "Field `CS4` reader - Clock Select Bits"]
pub type CS4_R = crate::FieldReader<CS4_A>;
#[doc = "Clock Select Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS4_A {
    #[doc = "0: No clock source (Timer/Counter stopped)"]
    NO_CLOCK = 0,
    #[doc = "1: Running, No Prescaling"]
    DIRECT = 1,
    #[doc = "2: Running, CLK/2"]
    PRESCALE_2 = 2,
    #[doc = "3: Running, CLK/4"]
    PRESCALE_4 = 3,
    #[doc = "4: Running, CLK/8"]
    PRESCALE_8 = 4,
    #[doc = "5: Running, CLK/16"]
    PRESCALE_16 = 5,
    #[doc = "6: Running, CLK/32"]
    PRESCALE_32 = 6,
    #[doc = "7: Running, CLK/64"]
    PRESCALE_64 = 7,
    #[doc = "8: Running, CLK/128"]
    PRESCALE_128 = 8,
    #[doc = "9: Running, CLK/256"]
    PRESCALE_256 = 9,
    #[doc = "10: Running, CLK/512"]
    PRESCALE_512 = 10,
    #[doc = "11: Running, CLK/1024"]
    PRESCALE_1024 = 11,
    #[doc = "12: Running, CLK/2048"]
    PRESCALE_2048 = 12,
    #[doc = "13: Running, CLK/4096"]
    PRESCALE_4096 = 13,
    #[doc = "14: Running, CLK/8192"]
    PRESCALE_8192 = 14,
    #[doc = "15: Running, CLK/16384"]
    PRESCALE_16384 = 15,
}
impl From<CS4_A> for u8 {
    #[inline(always)]
    fn from(variant: CS4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CS4_A {
    type Ux = u8;
}
impl CS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CS4_A {
        match self.bits {
            0 => CS4_A::NO_CLOCK,
            1 => CS4_A::DIRECT,
            2 => CS4_A::PRESCALE_2,
            3 => CS4_A::PRESCALE_4,
            4 => CS4_A::PRESCALE_8,
            5 => CS4_A::PRESCALE_16,
            6 => CS4_A::PRESCALE_32,
            7 => CS4_A::PRESCALE_64,
            8 => CS4_A::PRESCALE_128,
            9 => CS4_A::PRESCALE_256,
            10 => CS4_A::PRESCALE_512,
            11 => CS4_A::PRESCALE_1024,
            12 => CS4_A::PRESCALE_2048,
            13 => CS4_A::PRESCALE_4096,
            14 => CS4_A::PRESCALE_8192,
            15 => CS4_A::PRESCALE_16384,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == CS4_A::NO_CLOCK
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == CS4_A::DIRECT
    }
    #[doc = "Running, CLK/2"]
    #[inline(always)]
    pub fn is_prescale_2(&self) -> bool {
        *self == CS4_A::PRESCALE_2
    }
    #[doc = "Running, CLK/4"]
    #[inline(always)]
    pub fn is_prescale_4(&self) -> bool {
        *self == CS4_A::PRESCALE_4
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn is_prescale_8(&self) -> bool {
        *self == CS4_A::PRESCALE_8
    }
    #[doc = "Running, CLK/16"]
    #[inline(always)]
    pub fn is_prescale_16(&self) -> bool {
        *self == CS4_A::PRESCALE_16
    }
    #[doc = "Running, CLK/32"]
    #[inline(always)]
    pub fn is_prescale_32(&self) -> bool {
        *self == CS4_A::PRESCALE_32
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn is_prescale_64(&self) -> bool {
        *self == CS4_A::PRESCALE_64
    }
    #[doc = "Running, CLK/128"]
    #[inline(always)]
    pub fn is_prescale_128(&self) -> bool {
        *self == CS4_A::PRESCALE_128
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn is_prescale_256(&self) -> bool {
        *self == CS4_A::PRESCALE_256
    }
    #[doc = "Running, CLK/512"]
    #[inline(always)]
    pub fn is_prescale_512(&self) -> bool {
        *self == CS4_A::PRESCALE_512
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn is_prescale_1024(&self) -> bool {
        *self == CS4_A::PRESCALE_1024
    }
    #[doc = "Running, CLK/2048"]
    #[inline(always)]
    pub fn is_prescale_2048(&self) -> bool {
        *self == CS4_A::PRESCALE_2048
    }
    #[doc = "Running, CLK/4096"]
    #[inline(always)]
    pub fn is_prescale_4096(&self) -> bool {
        *self == CS4_A::PRESCALE_4096
    }
    #[doc = "Running, CLK/8192"]
    #[inline(always)]
    pub fn is_prescale_8192(&self) -> bool {
        *self == CS4_A::PRESCALE_8192
    }
    #[doc = "Running, CLK/16384"]
    #[inline(always)]
    pub fn is_prescale_16384(&self) -> bool {
        *self == CS4_A::PRESCALE_16384
    }
}
#[doc = "Field `CS4` writer - Clock Select Bits"]
pub type CS4_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, CS4_A>;
impl<'a, REG> CS4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(CS4_A::NO_CLOCK)
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut crate::W<REG> {
        self.variant(CS4_A::DIRECT)
    }
    #[doc = "Running, CLK/2"]
    #[inline(always)]
    pub fn prescale_2(self) -> &'a mut crate::W<REG> {
        self.variant(CS4_A::PRESCALE_2)
    }
    #[doc = "Running, CLK/4"]
    #[inline(always)]
    pub fn prescale_4(self) -> &'a mut crate::W<REG> {
        self.variant(CS4_A::PRESCALE_4)
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn prescale_8(self) -> &'a mut crate::W<REG> {
        self.variant(CS4_A::PRESCALE_8)
    }
    #[doc = "Running, CLK/16"]
    #[inline(always)]
    pub fn prescale_16(self) -> &'a mut crate::W<REG> {
        self.variant(CS4_A::PRESCALE_16)
    }
    #[doc = "Running, CLK/32"]
    #[inline(always)]
    pub fn prescale_32(self) -> &'a mut crate::W<REG> {
        self.variant(CS4_A::PRESCALE_32)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn prescale_64(self) -> &'a mut crate::W<REG> {
        self.variant(CS4_A::PRESCALE_64)
    }
    #[doc = "Running, CLK/128"]
    #[inline(always)]
    pub fn prescale_128(self) -> &'a mut crate::W<REG> {
        self.variant(CS4_A::PRESCALE_128)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn prescale_256(self) -> &'a mut crate::W<REG> {
        self.variant(CS4_A::PRESCALE_256)
    }
    #[doc = "Running, CLK/512"]
    #[inline(always)]
    pub fn prescale_512(self) -> &'a mut crate::W<REG> {
        self.variant(CS4_A::PRESCALE_512)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn prescale_1024(self) -> &'a mut crate::W<REG> {
        self.variant(CS4_A::PRESCALE_1024)
    }
    #[doc = "Running, CLK/2048"]
    #[inline(always)]
    pub fn prescale_2048(self) -> &'a mut crate::W<REG> {
        self.variant(CS4_A::PRESCALE_2048)
    }
    #[doc = "Running, CLK/4096"]
    #[inline(always)]
    pub fn prescale_4096(self) -> &'a mut crate::W<REG> {
        self.variant(CS4_A::PRESCALE_4096)
    }
    #[doc = "Running, CLK/8192"]
    #[inline(always)]
    pub fn prescale_8192(self) -> &'a mut crate::W<REG> {
        self.variant(CS4_A::PRESCALE_8192)
    }
    #[doc = "Running, CLK/16384"]
    #[inline(always)]
    pub fn prescale_16384(self) -> &'a mut crate::W<REG> {
        self.variant(CS4_A::PRESCALE_16384)
    }
}
#[doc = "Field `DTPS4` reader - Dead Time Prescaler Bits"]
pub type DTPS4_R = crate::FieldReader<DTPS4_A>;
#[doc = "Dead Time Prescaler Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTPS4_A {
    #[doc = "0: 1x (no division)"]
    X1 = 0,
    #[doc = "1: 2x"]
    X2 = 1,
    #[doc = "2: 4x"]
    X4 = 2,
    #[doc = "3: 8x"]
    X8 = 3,
}
impl From<DTPS4_A> for u8 {
    #[inline(always)]
    fn from(variant: DTPS4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTPS4_A {
    type Ux = u8;
}
impl DTPS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTPS4_A {
        match self.bits {
            0 => DTPS4_A::X1,
            1 => DTPS4_A::X2,
            2 => DTPS4_A::X4,
            3 => DTPS4_A::X8,
            _ => unreachable!(),
        }
    }
    #[doc = "1x (no division)"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == DTPS4_A::X1
    }
    #[doc = "2x"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == DTPS4_A::X2
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == DTPS4_A::X4
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == DTPS4_A::X8
    }
}
#[doc = "Field `DTPS4` writer - Dead Time Prescaler Bits"]
pub type DTPS4_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DTPS4_A>;
impl<'a, REG> DTPS4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1x (no division)"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut crate::W<REG> {
        self.variant(DTPS4_A::X1)
    }
    #[doc = "2x"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut crate::W<REG> {
        self.variant(DTPS4_A::X2)
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut crate::W<REG> {
        self.variant(DTPS4_A::X4)
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut crate::W<REG> {
        self.variant(DTPS4_A::X8)
    }
}
#[doc = "Field `PSR4` reader - Prescaler Reset Timer/Counter 4"]
pub type PSR4_R = crate::BitReader;
#[doc = "Field `PSR4` writer - Prescaler Reset Timer/Counter 4"]
pub type PSR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM4X` reader - PWM Inversion Mode"]
pub type PWM4X_R = crate::BitReader;
#[doc = "Field `PWM4X` writer - PWM Inversion Mode"]
pub type PWM4X_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Clock Select Bits"]
    #[inline(always)]
    pub fn cs4(&self) -> CS4_R {
        CS4_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - Dead Time Prescaler Bits"]
    #[inline(always)]
    pub fn dtps4(&self) -> DTPS4_R {
        DTPS4_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Prescaler Reset Timer/Counter 4"]
    #[inline(always)]
    pub fn psr4(&self) -> PSR4_R {
        PSR4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM Inversion Mode"]
    #[inline(always)]
    pub fn pwm4x(&self) -> PWM4X_R {
        PWM4X_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Select Bits"]
    #[inline(always)]
    #[must_use]
    pub fn cs4(&mut self) -> CS4_W<TCCR4B_SPEC> {
        CS4_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Dead Time Prescaler Bits"]
    #[inline(always)]
    #[must_use]
    pub fn dtps4(&mut self) -> DTPS4_W<TCCR4B_SPEC> {
        DTPS4_W::new(self, 4)
    }
    #[doc = "Bit 6 - Prescaler Reset Timer/Counter 4"]
    #[inline(always)]
    #[must_use]
    pub fn psr4(&mut self) -> PSR4_W<TCCR4B_SPEC> {
        PSR4_W::new(self, 6)
    }
    #[doc = "Bit 7 - PWM Inversion Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm4x(&mut self) -> PWM4X_W<TCCR4B_SPEC> {
        PWM4X_W::new(self, 7)
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
#[doc = "Timer/Counter4 Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr4b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr4b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR4B_SPEC;
impl crate::RegisterSpec for TCCR4B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr4b::R`](R) reader structure"]
impl crate::Readable for TCCR4B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr4b::W`](W) writer structure"]
impl crate::Writable for TCCR4B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR4B to value 0"]
impl crate::Resettable for TCCR4B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
