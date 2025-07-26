#[doc = "Register `LUT0CTRLA` reader"]
pub type R = crate::R<LUT0CTRLA_SPEC>;
#[doc = "Register `LUT0CTRLA` writer"]
pub type W = crate::W<LUT0CTRLA_SPEC>;
#[doc = "Field `ENABLE` reader - LUT Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - LUT Enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSRC` reader - Clock Source Selection"]
pub type CLKSRC_R = crate::FieldReader<CLKSRC_A>;
#[doc = "Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSRC_A {
    #[doc = "0: CLK_PER is clocking the LUT"]
    CLKPER = 0,
    #[doc = "1: IN\\[2\\]
is clocking the LUT"]
    IN2 = 1,
    #[doc = "4: 20MHz oscillator before prescaler is clocking the LUT"]
    OSC20M = 4,
    #[doc = "5: 32kHz oscillator is clocking the LUT"]
    OSCULP32K = 5,
    #[doc = "6: 1kHz (32kHz oscillator after DIV32) is clocking the LUT"]
    OSCULP1K = 6,
}
impl From<CLKSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKSRC_A {
    type Ux = u8;
}
impl CLKSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKSRC_A> {
        match self.bits {
            0 => Some(CLKSRC_A::CLKPER),
            1 => Some(CLKSRC_A::IN2),
            4 => Some(CLKSRC_A::OSC20M),
            5 => Some(CLKSRC_A::OSCULP32K),
            6 => Some(CLKSRC_A::OSCULP1K),
            _ => None,
        }
    }
    #[doc = "CLK_PER is clocking the LUT"]
    #[inline(always)]
    pub fn is_clkper(&self) -> bool {
        *self == CLKSRC_A::CLKPER
    }
    #[doc = "IN\\[2\\]
is clocking the LUT"]
    #[inline(always)]
    pub fn is_in2(&self) -> bool {
        *self == CLKSRC_A::IN2
    }
    #[doc = "20MHz oscillator before prescaler is clocking the LUT"]
    #[inline(always)]
    pub fn is_osc20m(&self) -> bool {
        *self == CLKSRC_A::OSC20M
    }
    #[doc = "32kHz oscillator is clocking the LUT"]
    #[inline(always)]
    pub fn is_osculp32k(&self) -> bool {
        *self == CLKSRC_A::OSCULP32K
    }
    #[doc = "1kHz (32kHz oscillator after DIV32) is clocking the LUT"]
    #[inline(always)]
    pub fn is_osculp1k(&self) -> bool {
        *self == CLKSRC_A::OSCULP1K
    }
}
#[doc = "Field `CLKSRC` writer - Clock Source Selection"]
pub type CLKSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CLKSRC_A>;
impl<'a, REG> CLKSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_PER is clocking the LUT"]
    #[inline(always)]
    pub fn clkper(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSRC_A::CLKPER)
    }
    #[doc = "IN\\[2\\]
is clocking the LUT"]
    #[inline(always)]
    pub fn in2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSRC_A::IN2)
    }
    #[doc = "20MHz oscillator before prescaler is clocking the LUT"]
    #[inline(always)]
    pub fn osc20m(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSRC_A::OSC20M)
    }
    #[doc = "32kHz oscillator is clocking the LUT"]
    #[inline(always)]
    pub fn osculp32k(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSRC_A::OSCULP32K)
    }
    #[doc = "1kHz (32kHz oscillator after DIV32) is clocking the LUT"]
    #[inline(always)]
    pub fn osculp1k(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSRC_A::OSCULP1K)
    }
}
#[doc = "Field `FILTSEL` reader - Filter Selection"]
pub type FILTSEL_R = crate::FieldReader<FILTSEL_A>;
#[doc = "Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FILTSEL_A {
    #[doc = "0: Filter disabled"]
    DISABLE = 0,
    #[doc = "1: Synchronizer enabled"]
    SYNCH = 1,
    #[doc = "2: Filter enabled"]
    FILTER = 2,
}
impl From<FILTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FILTSEL_A {
    type Ux = u8;
}
impl FILTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FILTSEL_A> {
        match self.bits {
            0 => Some(FILTSEL_A::DISABLE),
            1 => Some(FILTSEL_A::SYNCH),
            2 => Some(FILTSEL_A::FILTER),
            _ => None,
        }
    }
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FILTSEL_A::DISABLE
    }
    #[doc = "Synchronizer enabled"]
    #[inline(always)]
    pub fn is_synch(&self) -> bool {
        *self == FILTSEL_A::SYNCH
    }
    #[doc = "Filter enabled"]
    #[inline(always)]
    pub fn is_filter(&self) -> bool {
        *self == FILTSEL_A::FILTER
    }
}
#[doc = "Field `FILTSEL` writer - Filter Selection"]
pub type FILTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FILTSEL_A>;
impl<'a, REG> FILTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FILTSEL_A::DISABLE)
    }
    #[doc = "Synchronizer enabled"]
    #[inline(always)]
    pub fn synch(self) -> &'a mut crate::W<REG> {
        self.variant(FILTSEL_A::SYNCH)
    }
    #[doc = "Filter enabled"]
    #[inline(always)]
    pub fn filter(self) -> &'a mut crate::W<REG> {
        self.variant(FILTSEL_A::FILTER)
    }
}
#[doc = "Field `OUTEN` reader - Output Enable"]
pub type OUTEN_R = crate::BitReader;
#[doc = "Field `OUTEN` writer - Output Enable"]
pub type OUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGEDET` reader - Edge Detection Enable"]
pub type EDGEDET_R = crate::BitReader<EDGEDET_A>;
#[doc = "Edge Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGEDET_A {
    #[doc = "0: Edge detector is disabled"]
    DIS = 0,
    #[doc = "1: Edge detector is enabled"]
    EN = 1,
}
impl From<EDGEDET_A> for bool {
    #[inline(always)]
    fn from(variant: EDGEDET_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGEDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EDGEDET_A {
        match self.bits {
            false => EDGEDET_A::DIS,
            true => EDGEDET_A::EN,
        }
    }
    #[doc = "Edge detector is disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EDGEDET_A::DIS
    }
    #[doc = "Edge detector is enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EDGEDET_A::EN
    }
}
#[doc = "Field `EDGEDET` writer - Edge Detection Enable"]
pub type EDGEDET_W<'a, REG> = crate::BitWriter<'a, REG, EDGEDET_A>;
impl<'a, REG> EDGEDET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Edge detector is disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(EDGEDET_A::DIS)
    }
    #[doc = "Edge detector is enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(EDGEDET_A::EN)
    }
}
impl R {
    #[doc = "Bit 0 - LUT Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Clock Source Selection"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new((self.bits >> 1) & 7)
    }
    #[doc = "Bits 4:5 - Filter Selection"]
    #[inline(always)]
    pub fn filtsel(&self) -> FILTSEL_R {
        FILTSEL_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Output Enable"]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Edge Detection Enable"]
    #[inline(always)]
    pub fn edgedet(&self) -> EDGEDET_R {
        EDGEDET_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LUT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<LUT0CTRLA_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn clksrc(&mut self) -> CLKSRC_W<LUT0CTRLA_SPEC> {
        CLKSRC_W::new(self, 1)
    }
    #[doc = "Bits 4:5 - Filter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn filtsel(&mut self) -> FILTSEL_W<LUT0CTRLA_SPEC> {
        FILTSEL_W::new(self, 4)
    }
    #[doc = "Bit 6 - Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn outen(&mut self) -> OUTEN_W<LUT0CTRLA_SPEC> {
        OUTEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Edge Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn edgedet(&mut self) -> EDGEDET_W<LUT0CTRLA_SPEC> {
        EDGEDET_W::new(self, 7)
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
#[doc = "LUT Control 0 A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut0ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut0ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LUT0CTRLA_SPEC;
impl crate::RegisterSpec for LUT0CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lut0ctrla::R`](R) reader structure"]
impl crate::Readable for LUT0CTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lut0ctrla::W`](W) writer structure"]
impl crate::Writable for LUT0CTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LUT0CTRLA to value 0"]
impl crate::Resettable for LUT0CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
