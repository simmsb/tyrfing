#[doc = "Register `TCCR0B` reader"]
pub type R = crate::R<TCCR0B_SPEC>;
#[doc = "Register `TCCR0B` writer"]
pub type W = crate::W<TCCR0B_SPEC>;
#[doc = "Field `CS0` reader - Clock Select bits"]
pub type CS0_R = crate::FieldReader<CS0_A>;
#[doc = "Clock Select bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS0_A {
    #[doc = "0: No clock source (Timer/Counter stopped)"]
    NO_CLOCK = 0,
    #[doc = "1: Running, No Prescaling"]
    DIRECT = 1,
    #[doc = "2: Running, CLK/8"]
    PRESCALE_8 = 2,
    #[doc = "3: Running, CLK/32"]
    PRESCALE_32 = 3,
    #[doc = "4: Running, CLK/64"]
    PRESCALE_64 = 4,
    #[doc = "5: Running, CLK/128"]
    PRESCALE_128 = 5,
    #[doc = "6: Running, CLK/256"]
    PRESCALE_256 = 6,
    #[doc = "7: Running, CLK/1024"]
    PRESCALE_1024 = 7,
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
            0 => CS0_A::NO_CLOCK,
            1 => CS0_A::DIRECT,
            2 => CS0_A::PRESCALE_8,
            3 => CS0_A::PRESCALE_32,
            4 => CS0_A::PRESCALE_64,
            5 => CS0_A::PRESCALE_128,
            6 => CS0_A::PRESCALE_256,
            7 => CS0_A::PRESCALE_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == CS0_A::NO_CLOCK
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == CS0_A::DIRECT
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn is_prescale_8(&self) -> bool {
        *self == CS0_A::PRESCALE_8
    }
    #[doc = "Running, CLK/32"]
    #[inline(always)]
    pub fn is_prescale_32(&self) -> bool {
        *self == CS0_A::PRESCALE_32
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn is_prescale_64(&self) -> bool {
        *self == CS0_A::PRESCALE_64
    }
    #[doc = "Running, CLK/128"]
    #[inline(always)]
    pub fn is_prescale_128(&self) -> bool {
        *self == CS0_A::PRESCALE_128
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn is_prescale_256(&self) -> bool {
        *self == CS0_A::PRESCALE_256
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn is_prescale_1024(&self) -> bool {
        *self == CS0_A::PRESCALE_1024
    }
}
#[doc = "Field `CS0` writer - Clock Select bits"]
pub type CS0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, CS0_A>;
impl<'a, REG> CS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::NO_CLOCK)
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::DIRECT)
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn prescale_8(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::PRESCALE_8)
    }
    #[doc = "Running, CLK/32"]
    #[inline(always)]
    pub fn prescale_32(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::PRESCALE_32)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn prescale_64(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::PRESCALE_64)
    }
    #[doc = "Running, CLK/128"]
    #[inline(always)]
    pub fn prescale_128(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::PRESCALE_128)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn prescale_256(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::PRESCALE_256)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn prescale_1024(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::PRESCALE_1024)
    }
}
#[doc = "Field `FOC0A` reader - Force Output Compare A"]
pub type FOC0A_R = crate::BitReader;
#[doc = "Field `FOC0A` writer - Force Output Compare A"]
pub type FOC0A_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Clock Select bits"]
    #[inline(always)]
    pub fn cs0(&self) -> CS0_R {
        CS0_R::new(self.bits & 7)
    }
    #[doc = "Bit 7 - Force Output Compare A"]
    #[inline(always)]
    pub fn foc0a(&self) -> FOC0A_R {
        FOC0A_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn cs0(&mut self) -> CS0_W<TCCR0B_SPEC> {
        CS0_W::new(self, 0)
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
