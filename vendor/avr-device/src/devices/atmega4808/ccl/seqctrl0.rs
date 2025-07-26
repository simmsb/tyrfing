#[doc = "Register `SEQCTRL0` reader"]
pub type R = crate::R<SEQCTRL0_SPEC>;
#[doc = "Register `SEQCTRL0` writer"]
pub type W = crate::W<SEQCTRL0_SPEC>;
#[doc = "Field `SEQSEL0` reader - Sequential Selection"]
pub type SEQSEL0_R = crate::FieldReader<SEQSEL0_A>;
#[doc = "Sequential Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEQSEL0_A {
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
impl From<SEQSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: SEQSEL0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SEQSEL0_A {
    type Ux = u8;
}
impl SEQSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SEQSEL0_A> {
        match self.bits {
            0 => Some(SEQSEL0_A::DISABLE),
            1 => Some(SEQSEL0_A::DFF),
            2 => Some(SEQSEL0_A::JK),
            3 => Some(SEQSEL0_A::LATCH),
            4 => Some(SEQSEL0_A::RS),
            _ => None,
        }
    }
    #[doc = "Sequential logic disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SEQSEL0_A::DISABLE
    }
    #[doc = "D FlipFlop"]
    #[inline(always)]
    pub fn is_dff(&self) -> bool {
        *self == SEQSEL0_A::DFF
    }
    #[doc = "JK FlipFlop"]
    #[inline(always)]
    pub fn is_jk(&self) -> bool {
        *self == SEQSEL0_A::JK
    }
    #[doc = "D Latch"]
    #[inline(always)]
    pub fn is_latch(&self) -> bool {
        *self == SEQSEL0_A::LATCH
    }
    #[doc = "RS Latch"]
    #[inline(always)]
    pub fn is_rs(&self) -> bool {
        *self == SEQSEL0_A::RS
    }
}
#[doc = "Field `SEQSEL0` writer - Sequential Selection"]
pub type SEQSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SEQSEL0_A>;
impl<'a, REG> SEQSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sequential logic disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SEQSEL0_A::DISABLE)
    }
    #[doc = "D FlipFlop"]
    #[inline(always)]
    pub fn dff(self) -> &'a mut crate::W<REG> {
        self.variant(SEQSEL0_A::DFF)
    }
    #[doc = "JK FlipFlop"]
    #[inline(always)]
    pub fn jk(self) -> &'a mut crate::W<REG> {
        self.variant(SEQSEL0_A::JK)
    }
    #[doc = "D Latch"]
    #[inline(always)]
    pub fn latch(self) -> &'a mut crate::W<REG> {
        self.variant(SEQSEL0_A::LATCH)
    }
    #[doc = "RS Latch"]
    #[inline(always)]
    pub fn rs(self) -> &'a mut crate::W<REG> {
        self.variant(SEQSEL0_A::RS)
    }
}
impl R {
    #[doc = "Bits 0:2 - Sequential Selection"]
    #[inline(always)]
    pub fn seqsel0(&self) -> SEQSEL0_R {
        SEQSEL0_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sequential Selection"]
    #[inline(always)]
    #[must_use]
    pub fn seqsel0(&mut self) -> SEQSEL0_W<SEQCTRL0_SPEC> {
        SEQSEL0_W::new(self, 0)
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
#[doc = "Sequential Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQCTRL0_SPEC;
impl crate::RegisterSpec for SEQCTRL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`seqctrl0::R`](R) reader structure"]
impl crate::Readable for SEQCTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seqctrl0::W`](W) writer structure"]
impl crate::Writable for SEQCTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQCTRL0 to value 0"]
impl crate::Resettable for SEQCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
