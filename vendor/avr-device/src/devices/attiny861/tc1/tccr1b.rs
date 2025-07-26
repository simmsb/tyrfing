#[doc = "Register `TCCR1B` reader"]
pub type R = crate::R<TCCR1B_SPEC>;
#[doc = "Register `TCCR1B` writer"]
pub type W = crate::W<TCCR1B_SPEC>;
#[doc = "Field `CS1` reader - Clock Select Bits"]
pub type CS1_R = crate::FieldReader<CS1_A>;
#[doc = "Clock Select Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS1_A {
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
            0 => CS1_A::NO_CLOCK,
            1 => CS1_A::DIRECT,
            2 => CS1_A::PRESCALE_2,
            3 => CS1_A::PRESCALE_4,
            4 => CS1_A::PRESCALE_8,
            5 => CS1_A::PRESCALE_16,
            6 => CS1_A::PRESCALE_32,
            7 => CS1_A::PRESCALE_64,
            8 => CS1_A::PRESCALE_128,
            9 => CS1_A::PRESCALE_256,
            10 => CS1_A::PRESCALE_512,
            11 => CS1_A::PRESCALE_1024,
            12 => CS1_A::PRESCALE_2048,
            13 => CS1_A::PRESCALE_4096,
            14 => CS1_A::PRESCALE_8192,
            15 => CS1_A::PRESCALE_16384,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == CS1_A::NO_CLOCK
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == CS1_A::DIRECT
    }
    #[doc = "Running, CLK/2"]
    #[inline(always)]
    pub fn is_prescale_2(&self) -> bool {
        *self == CS1_A::PRESCALE_2
    }
    #[doc = "Running, CLK/4"]
    #[inline(always)]
    pub fn is_prescale_4(&self) -> bool {
        *self == CS1_A::PRESCALE_4
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn is_prescale_8(&self) -> bool {
        *self == CS1_A::PRESCALE_8
    }
    #[doc = "Running, CLK/16"]
    #[inline(always)]
    pub fn is_prescale_16(&self) -> bool {
        *self == CS1_A::PRESCALE_16
    }
    #[doc = "Running, CLK/32"]
    #[inline(always)]
    pub fn is_prescale_32(&self) -> bool {
        *self == CS1_A::PRESCALE_32
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn is_prescale_64(&self) -> bool {
        *self == CS1_A::PRESCALE_64
    }
    #[doc = "Running, CLK/128"]
    #[inline(always)]
    pub fn is_prescale_128(&self) -> bool {
        *self == CS1_A::PRESCALE_128
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn is_prescale_256(&self) -> bool {
        *self == CS1_A::PRESCALE_256
    }
    #[doc = "Running, CLK/512"]
    #[inline(always)]
    pub fn is_prescale_512(&self) -> bool {
        *self == CS1_A::PRESCALE_512
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn is_prescale_1024(&self) -> bool {
        *self == CS1_A::PRESCALE_1024
    }
    #[doc = "Running, CLK/2048"]
    #[inline(always)]
    pub fn is_prescale_2048(&self) -> bool {
        *self == CS1_A::PRESCALE_2048
    }
    #[doc = "Running, CLK/4096"]
    #[inline(always)]
    pub fn is_prescale_4096(&self) -> bool {
        *self == CS1_A::PRESCALE_4096
    }
    #[doc = "Running, CLK/8192"]
    #[inline(always)]
    pub fn is_prescale_8192(&self) -> bool {
        *self == CS1_A::PRESCALE_8192
    }
    #[doc = "Running, CLK/16384"]
    #[inline(always)]
    pub fn is_prescale_16384(&self) -> bool {
        *self == CS1_A::PRESCALE_16384
    }
}
#[doc = "Field `CS1` writer - Clock Select Bits"]
pub type CS1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, CS1_A>;
impl<'a, REG> CS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::NO_CLOCK)
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::DIRECT)
    }
    #[doc = "Running, CLK/2"]
    #[inline(always)]
    pub fn prescale_2(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::PRESCALE_2)
    }
    #[doc = "Running, CLK/4"]
    #[inline(always)]
    pub fn prescale_4(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::PRESCALE_4)
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn prescale_8(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::PRESCALE_8)
    }
    #[doc = "Running, CLK/16"]
    #[inline(always)]
    pub fn prescale_16(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::PRESCALE_16)
    }
    #[doc = "Running, CLK/32"]
    #[inline(always)]
    pub fn prescale_32(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::PRESCALE_32)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn prescale_64(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::PRESCALE_64)
    }
    #[doc = "Running, CLK/128"]
    #[inline(always)]
    pub fn prescale_128(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::PRESCALE_128)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn prescale_256(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::PRESCALE_256)
    }
    #[doc = "Running, CLK/512"]
    #[inline(always)]
    pub fn prescale_512(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::PRESCALE_512)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn prescale_1024(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::PRESCALE_1024)
    }
    #[doc = "Running, CLK/2048"]
    #[inline(always)]
    pub fn prescale_2048(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::PRESCALE_2048)
    }
    #[doc = "Running, CLK/4096"]
    #[inline(always)]
    pub fn prescale_4096(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::PRESCALE_4096)
    }
    #[doc = "Running, CLK/8192"]
    #[inline(always)]
    pub fn prescale_8192(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::PRESCALE_8192)
    }
    #[doc = "Running, CLK/16384"]
    #[inline(always)]
    pub fn prescale_16384(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::PRESCALE_16384)
    }
}
#[doc = "Field `DTPS1` reader - Dead Time Prescaler"]
pub type DTPS1_R = crate::FieldReader<DTPS1_A>;
#[doc = "Dead Time Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTPS1_A {
    #[doc = "0: 1x (no division)"]
    X1 = 0,
    #[doc = "1: 2x"]
    X2 = 1,
    #[doc = "2: 4x"]
    X4 = 2,
    #[doc = "3: 8x"]
    X8 = 3,
}
impl From<DTPS1_A> for u8 {
    #[inline(always)]
    fn from(variant: DTPS1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTPS1_A {
    type Ux = u8;
}
impl DTPS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTPS1_A {
        match self.bits {
            0 => DTPS1_A::X1,
            1 => DTPS1_A::X2,
            2 => DTPS1_A::X4,
            3 => DTPS1_A::X8,
            _ => unreachable!(),
        }
    }
    #[doc = "1x (no division)"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == DTPS1_A::X1
    }
    #[doc = "2x"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == DTPS1_A::X2
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == DTPS1_A::X4
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == DTPS1_A::X8
    }
}
#[doc = "Field `DTPS1` writer - Dead Time Prescaler"]
pub type DTPS1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DTPS1_A>;
impl<'a, REG> DTPS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1x (no division)"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut crate::W<REG> {
        self.variant(DTPS1_A::X1)
    }
    #[doc = "2x"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut crate::W<REG> {
        self.variant(DTPS1_A::X2)
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut crate::W<REG> {
        self.variant(DTPS1_A::X4)
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut crate::W<REG> {
        self.variant(DTPS1_A::X8)
    }
}
#[doc = "Field `PSR1` reader - Timer/Counter 1 Prescaler reset"]
pub type PSR1_R = crate::BitReader;
#[doc = "Field `PSR1` writer - Timer/Counter 1 Prescaler reset"]
pub type PSR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1X` reader - PWM Inversion Mode"]
pub type PWM1X_R = crate::BitReader;
#[doc = "Field `PWM1X` writer - PWM Inversion Mode"]
pub type PWM1X_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Clock Select Bits"]
    #[inline(always)]
    pub fn cs1(&self) -> CS1_R {
        CS1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - Dead Time Prescaler"]
    #[inline(always)]
    pub fn dtps1(&self) -> DTPS1_R {
        DTPS1_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Timer/Counter 1 Prescaler reset"]
    #[inline(always)]
    pub fn psr1(&self) -> PSR1_R {
        PSR1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM Inversion Mode"]
    #[inline(always)]
    pub fn pwm1x(&self) -> PWM1X_R {
        PWM1X_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Select Bits"]
    #[inline(always)]
    #[must_use]
    pub fn cs1(&mut self) -> CS1_W<TCCR1B_SPEC> {
        CS1_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Dead Time Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn dtps1(&mut self) -> DTPS1_W<TCCR1B_SPEC> {
        DTPS1_W::new(self, 4)
    }
    #[doc = "Bit 6 - Timer/Counter 1 Prescaler reset"]
    #[inline(always)]
    #[must_use]
    pub fn psr1(&mut self) -> PSR1_W<TCCR1B_SPEC> {
        PSR1_W::new(self, 6)
    }
    #[doc = "Bit 7 - PWM Inversion Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1x(&mut self) -> PWM1X_W<TCCR1B_SPEC> {
        PWM1X_W::new(self, 7)
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
#[doc = "Timer/Counter Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr1b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr1b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
