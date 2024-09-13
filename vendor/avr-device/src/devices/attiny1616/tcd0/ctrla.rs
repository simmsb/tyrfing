#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCPRES` reader - Syncronization prescaler"]
pub type SYNCPRES_R = crate::FieldReader<SYNCPRES_A>;
#[doc = "Syncronization prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCPRES_A {
    #[doc = "0: Selevted clock source divided by 1"]
    DIV1 = 0,
    #[doc = "1: Selevted clock source divided by 2"]
    DIV2 = 1,
    #[doc = "2: Selevted clock source divided by 4"]
    DIV4 = 2,
    #[doc = "3: Selevted clock source divided by 8"]
    DIV8 = 3,
}
impl From<SYNCPRES_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCPRES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNCPRES_A {
    type Ux = u8;
}
impl SYNCPRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCPRES_A {
        match self.bits {
            0 => SYNCPRES_A::DIV1,
            1 => SYNCPRES_A::DIV2,
            2 => SYNCPRES_A::DIV4,
            3 => SYNCPRES_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Selevted clock source divided by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == SYNCPRES_A::DIV1
    }
    #[doc = "Selevted clock source divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == SYNCPRES_A::DIV2
    }
    #[doc = "Selevted clock source divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == SYNCPRES_A::DIV4
    }
    #[doc = "Selevted clock source divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == SYNCPRES_A::DIV8
    }
}
#[doc = "Field `SYNCPRES` writer - Syncronization prescaler"]
pub type SYNCPRES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SYNCPRES_A>;
impl<'a, REG> SYNCPRES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selevted clock source divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCPRES_A::DIV1)
    }
    #[doc = "Selevted clock source divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCPRES_A::DIV2)
    }
    #[doc = "Selevted clock source divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCPRES_A::DIV4)
    }
    #[doc = "Selevted clock source divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCPRES_A::DIV8)
    }
}
#[doc = "Field `CNTPRES` reader - counter prescaler"]
pub type CNTPRES_R = crate::FieldReader<CNTPRES_A>;
#[doc = "counter prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNTPRES_A {
    #[doc = "0: Sync clock divided by 1"]
    DIV1 = 0,
    #[doc = "1: Sync clock divided by 4"]
    DIV4 = 1,
    #[doc = "2: Sync clock divided by 32"]
    DIV32 = 2,
}
impl From<CNTPRES_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTPRES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CNTPRES_A {
    type Ux = u8;
}
impl CNTPRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CNTPRES_A> {
        match self.bits {
            0 => Some(CNTPRES_A::DIV1),
            1 => Some(CNTPRES_A::DIV4),
            2 => Some(CNTPRES_A::DIV32),
            _ => None,
        }
    }
    #[doc = "Sync clock divided by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CNTPRES_A::DIV1
    }
    #[doc = "Sync clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CNTPRES_A::DIV4
    }
    #[doc = "Sync clock divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CNTPRES_A::DIV32
    }
}
#[doc = "Field `CNTPRES` writer - counter prescaler"]
pub type CNTPRES_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CNTPRES_A>;
impl<'a, REG> CNTPRES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sync clock divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRES_A::DIV1)
    }
    #[doc = "Sync clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRES_A::DIV4)
    }
    #[doc = "Sync clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRES_A::DIV32)
    }
}
#[doc = "Field `CLKSEL` reader - clock select"]
pub type CLKSEL_R = crate::FieldReader<CLKSEL_A>;
#[doc = "clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: 20 MHz oscillator"]
    _20MHZ = 0,
    #[doc = "2: External clock"]
    EXTCLK = 2,
    #[doc = "3: System clock"]
    SYSCLK = 3,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKSEL_A {
    type Ux = u8;
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::_20MHZ),
            2 => Some(CLKSEL_A::EXTCLK),
            3 => Some(CLKSEL_A::SYSCLK),
            _ => None,
        }
    }
    #[doc = "20 MHz oscillator"]
    #[inline(always)]
    pub fn is_20mhz(&self) -> bool {
        *self == CLKSEL_A::_20MHZ
    }
    #[doc = "External clock"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == CLKSEL_A::EXTCLK
    }
    #[doc = "System clock"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == CLKSEL_A::SYSCLK
    }
}
#[doc = "Field `CLKSEL` writer - clock select"]
pub type CLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CLKSEL_A>;
impl<'a, REG> CLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "20 MHz oscillator"]
    #[inline(always)]
    pub fn _20mhz(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::_20MHZ)
    }
    #[doc = "External clock"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::EXTCLK)
    }
    #[doc = "System clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::SYSCLK)
    }
}
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Syncronization prescaler"]
    #[inline(always)]
    pub fn syncpres(&self) -> SYNCPRES_R {
        SYNCPRES_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bits 3:4 - counter prescaler"]
    #[inline(always)]
    pub fn cntpres(&self) -> CNTPRES_R {
        CNTPRES_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bits 5:6 - clock select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits >> 5) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRLA_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Syncronization prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn syncpres(&mut self) -> SYNCPRES_W<CTRLA_SPEC> {
        SYNCPRES_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - counter prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn cntpres(&mut self) -> CNTPRES_W<CTRLA_SPEC> {
        CNTPRES_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - clock select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<CTRLA_SPEC> {
        CLKSEL_W::new(self, 5)
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
#[doc = "Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
