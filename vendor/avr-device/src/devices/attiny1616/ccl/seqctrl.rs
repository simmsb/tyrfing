#[doc = "Register `SEQCTRL%s` reader"]
pub type R = crate::R<SEQCTRL_SPEC>;
#[doc = "Register `SEQCTRL%s` writer"]
pub type W = crate::W<SEQCTRL_SPEC>;
#[doc = "Field `SEQSEL` reader - Sequential Selection"]
pub type SEQSEL_R = crate::FieldReader<SEQSEL_A>;
#[doc = "Sequential Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEQSEL_A {
    #[doc = "0: Sequential logic disabled"]
    DISABLE = 0,
    #[doc = "1: D FlipFlop"]
    DFF = 1,
    #[doc = "2: JK FlipFlop"]
    JK = 2,
    #[doc = "3: D Latch"]
    LATCH = 3,
    #[doc = "4: RS Latch"]
    RS = 4,
}
impl From<SEQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEQSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SEQSEL_A {
    type Ux = u8;
}
impl SEQSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SEQSEL_A> {
        match self.bits {
            0 => Some(SEQSEL_A::DISABLE),
            1 => Some(SEQSEL_A::DFF),
            2 => Some(SEQSEL_A::JK),
            3 => Some(SEQSEL_A::LATCH),
            4 => Some(SEQSEL_A::RS),
            _ => None,
        }
    }
    #[doc = "Sequential logic disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SEQSEL_A::DISABLE
    }
    #[doc = "D FlipFlop"]
    #[inline(always)]
    pub fn is_dff(&self) -> bool {
        *self == SEQSEL_A::DFF
    }
    #[doc = "JK FlipFlop"]
    #[inline(always)]
    pub fn is_jk(&self) -> bool {
        *self == SEQSEL_A::JK
    }
    #[doc = "D Latch"]
    #[inline(always)]
    pub fn is_latch(&self) -> bool {
        *self == SEQSEL_A::LATCH
    }
    #[doc = "RS Latch"]
    #[inline(always)]
    pub fn is_rs(&self) -> bool {
        *self == SEQSEL_A::RS
    }
}
#[doc = "Field `SEQSEL` writer - Sequential Selection"]
pub type SEQSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SEQSEL_A>;
impl<'a, REG> SEQSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sequential logic disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SEQSEL_A::DISABLE)
    }
    #[doc = "D FlipFlop"]
    #[inline(always)]
    pub fn dff(self) -> &'a mut crate::W<REG> {
        self.variant(SEQSEL_A::DFF)
    }
    #[doc = "JK FlipFlop"]
    #[inline(always)]
    pub fn jk(self) -> &'a mut crate::W<REG> {
        self.variant(SEQSEL_A::JK)
    }
    #[doc = "D Latch"]
    #[inline(always)]
    pub fn latch(self) -> &'a mut crate::W<REG> {
        self.variant(SEQSEL_A::LATCH)
    }
    #[doc = "RS Latch"]
    #[inline(always)]
    pub fn rs(self) -> &'a mut crate::W<REG> {
        self.variant(SEQSEL_A::RS)
    }
}
impl R {
    #[doc = "Bits 0:2 - Sequential Selection"]
    #[inline(always)]
    pub fn seqsel(&self) -> SEQSEL_R {
        SEQSEL_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sequential Selection"]
    #[inline(always)]
    #[must_use]
    pub fn seqsel(&mut self) -> SEQSEL_W<SEQCTRL_SPEC> {
        SEQSEL_W::new(self, 0)
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
#[doc = "Sequential Control %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQCTRL_SPEC;
impl crate::RegisterSpec for SEQCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`seqctrl::R`](R) reader structure"]
impl crate::Readable for SEQCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seqctrl::W`](W) writer structure"]
impl crate::Writable for SEQCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQCTRL%s to value 0"]
impl crate::Resettable for SEQCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
