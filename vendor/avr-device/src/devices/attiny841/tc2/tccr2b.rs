#[doc = "Register `TCCR2B` reader"]
pub type R = crate::R<TCCR2B_SPEC>;
#[doc = "Register `TCCR2B` writer"]
pub type W = crate::W<TCCR2B_SPEC>;
#[doc = "Field `CS2` reader - Clock Select bits"]
pub type CS2_R = crate::FieldReader<CS2_A>;
#[doc = "Clock Select bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS2_A {
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
impl From<CS2_A> for u8 {
    #[inline(always)]
    fn from(variant: CS2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CS2_A {
    type Ux = u8;
}
impl CS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CS2_A {
        match self.bits {
            0 => CS2_A::NO_CLOCK,
            1 => CS2_A::DIRECT,
            2 => CS2_A::PRESCALE_8,
            3 => CS2_A::PRESCALE_64,
            4 => CS2_A::PRESCALE_256,
            5 => CS2_A::PRESCALE_1024,
            6 => CS2_A::EXT_FALLING,
            7 => CS2_A::EXT_RISING,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == CS2_A::NO_CLOCK
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == CS2_A::DIRECT
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn is_prescale_8(&self) -> bool {
        *self == CS2_A::PRESCALE_8
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn is_prescale_64(&self) -> bool {
        *self == CS2_A::PRESCALE_64
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn is_prescale_256(&self) -> bool {
        *self == CS2_A::PRESCALE_256
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn is_prescale_1024(&self) -> bool {
        *self == CS2_A::PRESCALE_1024
    }
    #[doc = "Running, ExtClk Tx Falling Edge"]
    #[inline(always)]
    pub fn is_ext_falling(&self) -> bool {
        *self == CS2_A::EXT_FALLING
    }
    #[doc = "Running, ExtClk Tx Rising Edge"]
    #[inline(always)]
    pub fn is_ext_rising(&self) -> bool {
        *self == CS2_A::EXT_RISING
    }
}
#[doc = "Field `CS2` writer - Clock Select bits"]
pub type CS2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, CS2_A>;
impl<'a, REG> CS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(CS2_A::NO_CLOCK)
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut crate::W<REG> {
        self.variant(CS2_A::DIRECT)
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn prescale_8(self) -> &'a mut crate::W<REG> {
        self.variant(CS2_A::PRESCALE_8)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn prescale_64(self) -> &'a mut crate::W<REG> {
        self.variant(CS2_A::PRESCALE_64)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn prescale_256(self) -> &'a mut crate::W<REG> {
        self.variant(CS2_A::PRESCALE_256)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn prescale_1024(self) -> &'a mut crate::W<REG> {
        self.variant(CS2_A::PRESCALE_1024)
    }
    #[doc = "Running, ExtClk Tx Falling Edge"]
    #[inline(always)]
    pub fn ext_falling(self) -> &'a mut crate::W<REG> {
        self.variant(CS2_A::EXT_FALLING)
    }
    #[doc = "Running, ExtClk Tx Rising Edge"]
    #[inline(always)]
    pub fn ext_rising(self) -> &'a mut crate::W<REG> {
        self.variant(CS2_A::EXT_RISING)
    }
}
#[doc = "Field `WGM2` reader - Waveform Generation Mode Bits"]
pub type WGM2_R = crate::FieldReader;
#[doc = "Field `WGM2` writer - Waveform Generation Mode Bits"]
pub type WGM2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `ICES2` reader - Input Capture 2 Edge Select"]
pub type ICES2_R = crate::BitReader;
#[doc = "Field `ICES2` writer - Input Capture 2 Edge Select"]
pub type ICES2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICNC2` reader - Input Capture 2 Noise Canceler"]
pub type ICNC2_R = crate::BitReader;
#[doc = "Field `ICNC2` writer - Input Capture 2 Noise Canceler"]
pub type ICNC2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Clock Select bits"]
    #[inline(always)]
    pub fn cs2(&self) -> CS2_R {
        CS2_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode Bits"]
    #[inline(always)]
    pub fn wgm2(&self) -> WGM2_R {
        WGM2_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 6 - Input Capture 2 Edge Select"]
    #[inline(always)]
    pub fn ices2(&self) -> ICES2_R {
        ICES2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input Capture 2 Noise Canceler"]
    #[inline(always)]
    pub fn icnc2(&self) -> ICNC2_R {
        ICNC2_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn cs2(&mut self) -> CS2_W<TCCR2B_SPEC> {
        CS2_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode Bits"]
    #[inline(always)]
    #[must_use]
    pub fn wgm2(&mut self) -> WGM2_W<TCCR2B_SPEC> {
        WGM2_W::new(self, 3)
    }
    #[doc = "Bit 6 - Input Capture 2 Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn ices2(&mut self) -> ICES2_W<TCCR2B_SPEC> {
        ICES2_W::new(self, 6)
    }
    #[doc = "Bit 7 - Input Capture 2 Noise Canceler"]
    #[inline(always)]
    #[must_use]
    pub fn icnc2(&mut self) -> ICNC2_W<TCCR2B_SPEC> {
        ICNC2_W::new(self, 7)
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
#[doc = "Timer/Counter2 Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr2b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr2b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR2B_SPEC;
impl crate::RegisterSpec for TCCR2B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr2b::R`](R) reader structure"]
impl crate::Readable for TCCR2B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr2b::W`](W) writer structure"]
impl crate::Writable for TCCR2B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR2B to value 0"]
impl crate::Resettable for TCCR2B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
