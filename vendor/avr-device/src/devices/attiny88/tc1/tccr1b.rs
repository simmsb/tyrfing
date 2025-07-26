#[doc = "Register `TCCR1B` reader"]
pub type R = crate::R<TCCR1B_SPEC>;
#[doc = "Register `TCCR1B` writer"]
pub type W = crate::W<TCCR1B_SPEC>;
#[doc = "Field `CS1` reader - Prescaler source of Timer/Counter 1"]
pub type CS1_R = crate::FieldReader<CS1_A>;
#[doc = "Prescaler source of Timer/Counter 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS1_A {
    #[doc = "0: No clock source (Timer/Counter stopped)"]
    NO_CLOCK = 0,
    #[doc = "1: Running, No Prescaling"]
    DIRECT = 1,
    #[doc = "2: Running, CLK/8"]
    PRESCALE_8 = 2,
    #[doc = "3: Running, CLK/64"]
    PRESCALE_64 = 3,
    #[doc = "4: Running, CLK/256"]
    PRESCALE_256 = 4,
    #[doc = "5: Running, CLK/1024"]
    PRESCALE_1024 = 5,
    #[doc = "6: Running, ExtClk Tx Falling Edge"]
    EXT_FALLING = 6,
    #[doc = "7: Running, ExtClk Tx Rising Edge"]
    EXT_RISING = 7,
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
            2 => CS1_A::PRESCALE_8,
            3 => CS1_A::PRESCALE_64,
            4 => CS1_A::PRESCALE_256,
            5 => CS1_A::PRESCALE_1024,
            6 => CS1_A::EXT_FALLING,
            7 => CS1_A::EXT_RISING,
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
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn is_prescale_8(&self) -> bool {
        *self == CS1_A::PRESCALE_8
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn is_prescale_64(&self) -> bool {
        *self == CS1_A::PRESCALE_64
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn is_prescale_256(&self) -> bool {
        *self == CS1_A::PRESCALE_256
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn is_prescale_1024(&self) -> bool {
        *self == CS1_A::PRESCALE_1024
    }
    #[doc = "Running, ExtClk Tx Falling Edge"]
    #[inline(always)]
    pub fn is_ext_falling(&self) -> bool {
        *self == CS1_A::EXT_FALLING
    }
    #[doc = "Running, ExtClk Tx Rising Edge"]
    #[inline(always)]
    pub fn is_ext_rising(&self) -> bool {
        *self == CS1_A::EXT_RISING
    }
}
#[doc = "Field `CS1` writer - Prescaler source of Timer/Counter 1"]
pub type CS1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, CS1_A>;
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
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn prescale_8(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::PRESCALE_8)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn prescale_64(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::PRESCALE_64)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn prescale_256(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::PRESCALE_256)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn prescale_1024(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::PRESCALE_1024)
    }
    #[doc = "Running, ExtClk Tx Falling Edge"]
    #[inline(always)]
    pub fn ext_falling(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::EXT_FALLING)
    }
    #[doc = "Running, ExtClk Tx Rising Edge"]
    #[inline(always)]
    pub fn ext_rising(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::EXT_RISING)
    }
}
#[doc = "Field `WGM1` reader - Waveform Generation Mode"]
pub type WGM1_R = crate::FieldReader;
#[doc = "Field `WGM1` writer - Waveform Generation Mode"]
pub type WGM1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `ICES1` reader - Input Capture 1 Edge Select"]
pub type ICES1_R = crate::BitReader;
#[doc = "Field `ICES1` writer - Input Capture 1 Edge Select"]
pub type ICES1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICNC1` reader - Input Capture 1 Noise Canceler"]
pub type ICNC1_R = crate::BitReader;
#[doc = "Field `ICNC1` writer - Input Capture 1 Noise Canceler"]
pub type ICNC1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Prescaler source of Timer/Counter 1"]
    #[inline(always)]
    pub fn cs1(&self) -> CS1_R {
        CS1_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm1(&self) -> WGM1_R {
        WGM1_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 6 - Input Capture 1 Edge Select"]
    #[inline(always)]
    pub fn ices1(&self) -> ICES1_R {
        ICES1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input Capture 1 Noise Canceler"]
    #[inline(always)]
    pub fn icnc1(&self) -> ICNC1_R {
        ICNC1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescaler source of Timer/Counter 1"]
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
    #[doc = "Bit 6 - Input Capture 1 Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn ices1(&mut self) -> ICES1_W<TCCR1B_SPEC> {
        ICES1_W::new(self, 6)
    }
    #[doc = "Bit 7 - Input Capture 1 Noise Canceler"]
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
