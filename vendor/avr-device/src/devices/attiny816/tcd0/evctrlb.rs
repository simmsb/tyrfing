#[doc = "Register `EVCTRLB` reader"]
pub type R = crate::R<EVCTRLB_SPEC>;
#[doc = "Register `EVCTRLB` writer"]
pub type W = crate::W<EVCTRLB_SPEC>;
#[doc = "Field `TRIGEI` reader - Trigger event enable"]
pub type TRIGEI_R = crate::BitReader;
#[doc = "Field `TRIGEI` writer - Trigger event enable"]
pub type TRIGEI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTION` reader - event action"]
pub type ACTION_R = crate::BitReader<ACTION_A>;
#[doc = "event action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTION_A {
    #[doc = "0: Event trigger a fault"]
    FAULT = 0,
    #[doc = "1: Event trigger a fault and capture"]
    CAPTURE = 1,
}
impl From<ACTION_A> for bool {
    #[inline(always)]
    fn from(variant: ACTION_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACTION_A {
        match self.bits {
            false => ACTION_A::FAULT,
            true => ACTION_A::CAPTURE,
        }
    }
    #[doc = "Event trigger a fault"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == ACTION_A::FAULT
    }
    #[doc = "Event trigger a fault and capture"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == ACTION_A::CAPTURE
    }
}
#[doc = "Field `ACTION` writer - event action"]
pub type ACTION_W<'a, REG> = crate::BitWriter<'a, REG, ACTION_A>;
impl<'a, REG> ACTION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event trigger a fault"]
    #[inline(always)]
    pub fn fault(self) -> &'a mut crate::W<REG> {
        self.variant(ACTION_A::FAULT)
    }
    #[doc = "Event trigger a fault and capture"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(ACTION_A::CAPTURE)
    }
}
#[doc = "Field `EDGE` reader - edge select"]
pub type EDGE_R = crate::BitReader<EDGE_A>;
#[doc = "edge select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE_A {
    #[doc = "0: The falling edge or low level of event generates retrigger or fault action"]
    FALL_LOW = 0,
    #[doc = "1: The rising edge or high level of event generates retrigger or fault action"]
    RISE_HIGH = 1,
}
impl From<EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EDGE_A {
        match self.bits {
            false => EDGE_A::FALL_LOW,
            true => EDGE_A::RISE_HIGH,
        }
    }
    #[doc = "The falling edge or low level of event generates retrigger or fault action"]
    #[inline(always)]
    pub fn is_fall_low(&self) -> bool {
        *self == EDGE_A::FALL_LOW
    }
    #[doc = "The rising edge or high level of event generates retrigger or fault action"]
    #[inline(always)]
    pub fn is_rise_high(&self) -> bool {
        *self == EDGE_A::RISE_HIGH
    }
}
#[doc = "Field `EDGE` writer - edge select"]
pub type EDGE_W<'a, REG> = crate::BitWriter<'a, REG, EDGE_A>;
impl<'a, REG> EDGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The falling edge or low level of event generates retrigger or fault action"]
    #[inline(always)]
    pub fn fall_low(self) -> &'a mut crate::W<REG> {
        self.variant(EDGE_A::FALL_LOW)
    }
    #[doc = "The rising edge or high level of event generates retrigger or fault action"]
    #[inline(always)]
    pub fn rise_high(self) -> &'a mut crate::W<REG> {
        self.variant(EDGE_A::RISE_HIGH)
    }
}
#[doc = "Field `CFG` reader - event config"]
pub type CFG_R = crate::FieldReader<CFG_A>;
#[doc = "event config\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFG_A {
    #[doc = "0: Neither Filter nor Asynchronous Event is enabled"]
    NEITHER = 0,
    #[doc = "1: Input Capture Noise Cancellation Filter enabled"]
    FILTERON = 1,
    #[doc = "2: Asynchronous Event output qualification enabled"]
    ASYNCON = 2,
}
impl From<CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CFG_A {
    type Ux = u8;
}
impl CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CFG_A> {
        match self.bits {
            0 => Some(CFG_A::NEITHER),
            1 => Some(CFG_A::FILTERON),
            2 => Some(CFG_A::ASYNCON),
            _ => None,
        }
    }
    #[doc = "Neither Filter nor Asynchronous Event is enabled"]
    #[inline(always)]
    pub fn is_neither(&self) -> bool {
        *self == CFG_A::NEITHER
    }
    #[doc = "Input Capture Noise Cancellation Filter enabled"]
    #[inline(always)]
    pub fn is_filteron(&self) -> bool {
        *self == CFG_A::FILTERON
    }
    #[doc = "Asynchronous Event output qualification enabled"]
    #[inline(always)]
    pub fn is_asyncon(&self) -> bool {
        *self == CFG_A::ASYNCON
    }
}
#[doc = "Field `CFG` writer - event config"]
pub type CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CFG_A>;
impl<'a, REG> CFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Neither Filter nor Asynchronous Event is enabled"]
    #[inline(always)]
    pub fn neither(self) -> &'a mut crate::W<REG> {
        self.variant(CFG_A::NEITHER)
    }
    #[doc = "Input Capture Noise Cancellation Filter enabled"]
    #[inline(always)]
    pub fn filteron(self) -> &'a mut crate::W<REG> {
        self.variant(CFG_A::FILTERON)
    }
    #[doc = "Asynchronous Event output qualification enabled"]
    #[inline(always)]
    pub fn asyncon(self) -> &'a mut crate::W<REG> {
        self.variant(CFG_A::ASYNCON)
    }
}
impl R {
    #[doc = "Bit 0 - Trigger event enable"]
    #[inline(always)]
    pub fn trigei(&self) -> TRIGEI_R {
        TRIGEI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - event action"]
    #[inline(always)]
    pub fn action(&self) -> ACTION_R {
        ACTION_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - edge select"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - event config"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger event enable"]
    #[inline(always)]
    #[must_use]
    pub fn trigei(&mut self) -> TRIGEI_W<EVCTRLB_SPEC> {
        TRIGEI_W::new(self, 0)
    }
    #[doc = "Bit 2 - event action"]
    #[inline(always)]
    #[must_use]
    pub fn action(&mut self) -> ACTION_W<EVCTRLB_SPEC> {
        ACTION_W::new(self, 2)
    }
    #[doc = "Bit 4 - edge select"]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EDGE_W<EVCTRLB_SPEC> {
        EDGE_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - event config"]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CFG_W<EVCTRLB_SPEC> {
        CFG_W::new(self, 6)
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
#[doc = "EVCTRLB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVCTRLB_SPEC;
impl crate::RegisterSpec for EVCTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`evctrlb::R`](R) reader structure"]
impl crate::Readable for EVCTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evctrlb::W`](W) writer structure"]
impl crate::Writable for EVCTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVCTRLB to value 0"]
impl crate::Resettable for EVCTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
