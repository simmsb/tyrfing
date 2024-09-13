#[doc = "Register `INPUTCTRLA` reader"]
pub type R = crate::R<INPUTCTRLA_SPEC>;
#[doc = "Register `INPUTCTRLA` writer"]
pub type W = crate::W<INPUTCTRLA_SPEC>;
#[doc = "Field `INPUTMODE` reader - Input mode"]
pub type INPUTMODE_R = crate::FieldReader<INPUTMODE_A>;
#[doc = "Input mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUTMODE_A {
    #[doc = "0: Input has no actions"]
    NONE = 0,
    #[doc = "1: Stop output, jump to opposite compare cycle and wait"]
    JMPWAIT = 1,
    #[doc = "2: Stop output, execute opposite compare cycle and wait"]
    EXECWAIT = 2,
    #[doc = "3: stop output, execute opposite compare cycle while fault active"]
    EXECFAULT = 3,
    #[doc = "4: Stop all outputs, maintain frequency"]
    FREQ = 4,
    #[doc = "5: Stop all outputs, execute dead time while fault active"]
    EXECDT = 5,
    #[doc = "6: Stop all outputs, jump to next compare cycle and wait"]
    WAIT = 6,
    #[doc = "7: Stop all outputs, wait for software action"]
    WAITSW = 7,
    #[doc = "8: Stop output on edge, jump to next compare cycle"]
    EDGETRIG = 8,
    #[doc = "9: Stop output on edge, maintain frequency"]
    EDGETRIGFREQ = 9,
    #[doc = "10: Stop output at level, maintain frequency"]
    LVLTRIGFREQ = 10,
}
impl From<INPUTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUTMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INPUTMODE_A {
    type Ux = u8;
}
impl INPUTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INPUTMODE_A> {
        match self.bits {
            0 => Some(INPUTMODE_A::NONE),
            1 => Some(INPUTMODE_A::JMPWAIT),
            2 => Some(INPUTMODE_A::EXECWAIT),
            3 => Some(INPUTMODE_A::EXECFAULT),
            4 => Some(INPUTMODE_A::FREQ),
            5 => Some(INPUTMODE_A::EXECDT),
            6 => Some(INPUTMODE_A::WAIT),
            7 => Some(INPUTMODE_A::WAITSW),
            8 => Some(INPUTMODE_A::EDGETRIG),
            9 => Some(INPUTMODE_A::EDGETRIGFREQ),
            10 => Some(INPUTMODE_A::LVLTRIGFREQ),
            _ => None,
        }
    }
    #[doc = "Input has no actions"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == INPUTMODE_A::NONE
    }
    #[doc = "Stop output, jump to opposite compare cycle and wait"]
    #[inline(always)]
    pub fn is_jmpwait(&self) -> bool {
        *self == INPUTMODE_A::JMPWAIT
    }
    #[doc = "Stop output, execute opposite compare cycle and wait"]
    #[inline(always)]
    pub fn is_execwait(&self) -> bool {
        *self == INPUTMODE_A::EXECWAIT
    }
    #[doc = "stop output, execute opposite compare cycle while fault active"]
    #[inline(always)]
    pub fn is_execfault(&self) -> bool {
        *self == INPUTMODE_A::EXECFAULT
    }
    #[doc = "Stop all outputs, maintain frequency"]
    #[inline(always)]
    pub fn is_freq(&self) -> bool {
        *self == INPUTMODE_A::FREQ
    }
    #[doc = "Stop all outputs, execute dead time while fault active"]
    #[inline(always)]
    pub fn is_execdt(&self) -> bool {
        *self == INPUTMODE_A::EXECDT
    }
    #[doc = "Stop all outputs, jump to next compare cycle and wait"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == INPUTMODE_A::WAIT
    }
    #[doc = "Stop all outputs, wait for software action"]
    #[inline(always)]
    pub fn is_waitsw(&self) -> bool {
        *self == INPUTMODE_A::WAITSW
    }
    #[doc = "Stop output on edge, jump to next compare cycle"]
    #[inline(always)]
    pub fn is_edgetrig(&self) -> bool {
        *self == INPUTMODE_A::EDGETRIG
    }
    #[doc = "Stop output on edge, maintain frequency"]
    #[inline(always)]
    pub fn is_edgetrigfreq(&self) -> bool {
        *self == INPUTMODE_A::EDGETRIGFREQ
    }
    #[doc = "Stop output at level, maintain frequency"]
    #[inline(always)]
    pub fn is_lvltrigfreq(&self) -> bool {
        *self == INPUTMODE_A::LVLTRIGFREQ
    }
}
#[doc = "Field `INPUTMODE` writer - Input mode"]
pub type INPUTMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, INPUTMODE_A>;
impl<'a, REG> INPUTMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input has no actions"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTMODE_A::NONE)
    }
    #[doc = "Stop output, jump to opposite compare cycle and wait"]
    #[inline(always)]
    pub fn jmpwait(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTMODE_A::JMPWAIT)
    }
    #[doc = "Stop output, execute opposite compare cycle and wait"]
    #[inline(always)]
    pub fn execwait(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTMODE_A::EXECWAIT)
    }
    #[doc = "stop output, execute opposite compare cycle while fault active"]
    #[inline(always)]
    pub fn execfault(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTMODE_A::EXECFAULT)
    }
    #[doc = "Stop all outputs, maintain frequency"]
    #[inline(always)]
    pub fn freq(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTMODE_A::FREQ)
    }
    #[doc = "Stop all outputs, execute dead time while fault active"]
    #[inline(always)]
    pub fn execdt(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTMODE_A::EXECDT)
    }
    #[doc = "Stop all outputs, jump to next compare cycle and wait"]
    #[inline(always)]
    pub fn wait(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTMODE_A::WAIT)
    }
    #[doc = "Stop all outputs, wait for software action"]
    #[inline(always)]
    pub fn waitsw(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTMODE_A::WAITSW)
    }
    #[doc = "Stop output on edge, jump to next compare cycle"]
    #[inline(always)]
    pub fn edgetrig(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTMODE_A::EDGETRIG)
    }
    #[doc = "Stop output on edge, maintain frequency"]
    #[inline(always)]
    pub fn edgetrigfreq(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTMODE_A::EDGETRIGFREQ)
    }
    #[doc = "Stop output at level, maintain frequency"]
    #[inline(always)]
    pub fn lvltrigfreq(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTMODE_A::LVLTRIGFREQ)
    }
}
impl R {
    #[doc = "Bits 0:3 - Input mode"]
    #[inline(always)]
    pub fn inputmode(&self) -> INPUTMODE_R {
        INPUTMODE_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input mode"]
    #[inline(always)]
    #[must_use]
    pub fn inputmode(&mut self) -> INPUTMODE_W<INPUTCTRLA_SPEC> {
        INPUTMODE_W::new(self, 0)
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
#[doc = "Input Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inputctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inputctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INPUTCTRLA_SPEC;
impl crate::RegisterSpec for INPUTCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`inputctrla::R`](R) reader structure"]
impl crate::Readable for INPUTCTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inputctrla::W`](W) writer structure"]
impl crate::Writable for INPUTCTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INPUTCTRLA to value 0"]
impl crate::Resettable for INPUTCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
