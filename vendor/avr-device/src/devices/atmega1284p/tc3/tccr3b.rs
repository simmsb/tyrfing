#[doc = "Register `TCCR3B` reader"]
pub type R = crate::R<TCCR3B_SPEC>;
#[doc = "Register `TCCR3B` writer"]
pub type W = crate::W<TCCR3B_SPEC>;
#[doc = "Field `CS3` reader - Clock Select3 bits"]
pub type CS3_R = crate::FieldReader<CS3_A>;
#[doc = "Clock Select3 bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS3_A {
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
            0 => CS3_A::NO_CLOCK,
            1 => CS3_A::DIRECT,
            2 => CS3_A::PRESCALE_8,
            3 => CS3_A::PRESCALE_64,
            4 => CS3_A::PRESCALE_256,
            5 => CS3_A::PRESCALE_1024,
            6 => CS3_A::EXT_FALLING,
            7 => CS3_A::EXT_RISING,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == CS3_A::NO_CLOCK
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == CS3_A::DIRECT
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn is_prescale_8(&self) -> bool {
        *self == CS3_A::PRESCALE_8
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn is_prescale_64(&self) -> bool {
        *self == CS3_A::PRESCALE_64
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn is_prescale_256(&self) -> bool {
        *self == CS3_A::PRESCALE_256
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn is_prescale_1024(&self) -> bool {
        *self == CS3_A::PRESCALE_1024
    }
    #[doc = "Running, ExtClk Tx Falling Edge"]
    #[inline(always)]
    pub fn is_ext_falling(&self) -> bool {
        *self == CS3_A::EXT_FALLING
    }
    #[doc = "Running, ExtClk Tx Rising Edge"]
    #[inline(always)]
    pub fn is_ext_rising(&self) -> bool {
        *self == CS3_A::EXT_RISING
    }
}
#[doc = "Field `CS3` writer - Clock Select3 bits"]
pub type CS3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, CS3_A>;
impl<'a, REG> CS3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(CS3_A::NO_CLOCK)
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut crate::W<REG> {
        self.variant(CS3_A::DIRECT)
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn prescale_8(self) -> &'a mut crate::W<REG> {
        self.variant(CS3_A::PRESCALE_8)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn prescale_64(self) -> &'a mut crate::W<REG> {
        self.variant(CS3_A::PRESCALE_64)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn prescale_256(self) -> &'a mut crate::W<REG> {
        self.variant(CS3_A::PRESCALE_256)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn prescale_1024(self) -> &'a mut crate::W<REG> {
        self.variant(CS3_A::PRESCALE_1024)
    }
    #[doc = "Running, ExtClk Tx Falling Edge"]
    #[inline(always)]
    pub fn ext_falling(self) -> &'a mut crate::W<REG> {
        self.variant(CS3_A::EXT_FALLING)
    }
    #[doc = "Running, ExtClk Tx Rising Edge"]
    #[inline(always)]
    pub fn ext_rising(self) -> &'a mut crate::W<REG> {
        self.variant(CS3_A::EXT_RISING)
    }
}
#[doc = "Field `WGM3` reader - Waveform Generation Mode Bits"]
pub type WGM3_R = crate::FieldReader;
#[doc = "Field `WGM3` writer - Waveform Generation Mode Bits"]
pub type WGM3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `ICES3` reader - Input Capture 3 Edge Select"]
pub type ICES3_R = crate::BitReader;
#[doc = "Field `ICES3` writer - Input Capture 3 Edge Select"]
pub type ICES3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICNC3` reader - Input Capture 3 Noise Canceler"]
pub type ICNC3_R = crate::BitReader;
#[doc = "Field `ICNC3` writer - Input Capture 3 Noise Canceler"]
pub type ICNC3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Clock Select3 bits"]
    #[inline(always)]
    pub fn cs3(&self) -> CS3_R {
        CS3_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode Bits"]
    #[inline(always)]
    pub fn wgm3(&self) -> WGM3_R {
        WGM3_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 6 - Input Capture 3 Edge Select"]
    #[inline(always)]
    pub fn ices3(&self) -> ICES3_R {
        ICES3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input Capture 3 Noise Canceler"]
    #[inline(always)]
    pub fn icnc3(&self) -> ICNC3_R {
        ICNC3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select3 bits"]
    #[inline(always)]
    #[must_use]
    pub fn cs3(&mut self) -> CS3_W<TCCR3B_SPEC> {
        CS3_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode Bits"]
    #[inline(always)]
    #[must_use]
    pub fn wgm3(&mut self) -> WGM3_W<TCCR3B_SPEC> {
        WGM3_W::new(self, 3)
    }
    #[doc = "Bit 6 - Input Capture 3 Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn ices3(&mut self) -> ICES3_W<TCCR3B_SPEC> {
        ICES3_W::new(self, 6)
    }
    #[doc = "Bit 7 - Input Capture 3 Noise Canceler"]
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
