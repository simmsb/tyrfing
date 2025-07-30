#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `LVL0RR` reader - Round-robin Scheduling Enable"]
pub type LVL0RR_R = crate::BitReader<LVL0RR_A>;
#[doc = "Round-robin Scheduling Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVL0RR_A {
    #[doc = "0: Priority is fixed for priority level 0 interrupt requests: The lowest interrupt vector address has the highest priority."]
    FIXED = 0,
    #[doc = "1: The round robin priority scheme is enabled for priority level 0 interrupt requests"]
    ROUNDROBIN = 1,
}
impl From<LVL0RR_A> for bool {
    #[inline(always)]
    fn from(variant: LVL0RR_A) -> Self {
        variant as u8 != 0
    }
}
impl LVL0RR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LVL0RR_A {
        match self.bits {
            false => LVL0RR_A::FIXED,
            true => LVL0RR_A::ROUNDROBIN,
        }
    }
    #[doc = "Priority is fixed for priority level 0 interrupt requests: The lowest interrupt vector address has the highest priority."]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == LVL0RR_A::FIXED
    }
    #[doc = "The round robin priority scheme is enabled for priority level 0 interrupt requests"]
    #[inline(always)]
    pub fn is_roundrobin(&self) -> bool {
        *self == LVL0RR_A::ROUNDROBIN
    }
}
#[doc = "Field `LVL0RR` writer - Round-robin Scheduling Enable"]
pub type LVL0RR_W<'a, REG> = crate::BitWriter<'a, REG, LVL0RR_A>;
impl<'a, REG> LVL0RR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Priority is fixed for priority level 0 interrupt requests: The lowest interrupt vector address has the highest priority."]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut crate::W<REG> {
        self.variant(LVL0RR_A::FIXED)
    }
    #[doc = "The round robin priority scheme is enabled for priority level 0 interrupt requests"]
    #[inline(always)]
    pub fn roundrobin(self) -> &'a mut crate::W<REG> {
        self.variant(LVL0RR_A::ROUNDROBIN)
    }
}
#[doc = "Field `CVT` reader - Compact Vector Table"]
pub type CVT_R = crate::BitReader<CVT_A>;
#[doc = "Compact Vector Table\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CVT_A {
    #[doc = "0: Compact Vector Table function is disabled"]
    NORMAL = 0,
    #[doc = "1: Compact Vector Table function is enabled"]
    COMPACT = 1,
}
impl From<CVT_A> for bool {
    #[inline(always)]
    fn from(variant: CVT_A) -> Self {
        variant as u8 != 0
    }
}
impl CVT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CVT_A {
        match self.bits {
            false => CVT_A::NORMAL,
            true => CVT_A::COMPACT,
        }
    }
    #[doc = "Compact Vector Table function is disabled"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CVT_A::NORMAL
    }
    #[doc = "Compact Vector Table function is enabled"]
    #[inline(always)]
    pub fn is_compact(&self) -> bool {
        *self == CVT_A::COMPACT
    }
}
#[doc = "Field `CVT` writer - Compact Vector Table"]
pub type CVT_W<'a, REG> = crate::BitWriter<'a, REG, CVT_A>;
impl<'a, REG> CVT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compact Vector Table function is disabled"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(CVT_A::NORMAL)
    }
    #[doc = "Compact Vector Table function is enabled"]
    #[inline(always)]
    pub fn compact(self) -> &'a mut crate::W<REG> {
        self.variant(CVT_A::COMPACT)
    }
}
#[doc = "Field `IVSEL` reader - Interrupt Vector Select"]
pub type IVSEL_R = crate::BitReader<IVSEL_A>;
#[doc = "Interrupt Vector Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IVSEL_A {
    #[doc = "0: Interrupt vectors are placed after the BOOT section of the Flash"]
    AFTERBOOT = 0,
    #[doc = "1: Interrupt vectors are placed at the start of the BOOT section of the Flash"]
    INBOOT = 1,
}
impl From<IVSEL_A> for bool {
    #[inline(always)]
    fn from(variant: IVSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl IVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IVSEL_A {
        match self.bits {
            false => IVSEL_A::AFTERBOOT,
            true => IVSEL_A::INBOOT,
        }
    }
    #[doc = "Interrupt vectors are placed after the BOOT section of the Flash"]
    #[inline(always)]
    pub fn is_afterboot(&self) -> bool {
        *self == IVSEL_A::AFTERBOOT
    }
    #[doc = "Interrupt vectors are placed at the start of the BOOT section of the Flash"]
    #[inline(always)]
    pub fn is_inboot(&self) -> bool {
        *self == IVSEL_A::INBOOT
    }
}
#[doc = "Field `IVSEL` writer - Interrupt Vector Select"]
pub type IVSEL_W<'a, REG> = crate::BitWriter<'a, REG, IVSEL_A>;
impl<'a, REG> IVSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt vectors are placed after the BOOT section of the Flash"]
    #[inline(always)]
    pub fn afterboot(self) -> &'a mut crate::W<REG> {
        self.variant(IVSEL_A::AFTERBOOT)
    }
    #[doc = "Interrupt vectors are placed at the start of the BOOT section of the Flash"]
    #[inline(always)]
    pub fn inboot(self) -> &'a mut crate::W<REG> {
        self.variant(IVSEL_A::INBOOT)
    }
}
impl R {
    #[doc = "Bit 0 - Round-robin Scheduling Enable"]
    #[inline(always)]
    pub fn lvl0rr(&self) -> LVL0RR_R {
        LVL0RR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Compact Vector Table"]
    #[inline(always)]
    pub fn cvt(&self) -> CVT_R {
        CVT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Vector Select"]
    #[inline(always)]
    pub fn ivsel(&self) -> IVSEL_R {
        IVSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Round-robin Scheduling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvl0rr(&mut self) -> LVL0RR_W<CTRLA_SPEC> {
        LVL0RR_W::new(self, 0)
    }
    #[doc = "Bit 5 - Compact Vector Table"]
    #[inline(always)]
    #[must_use]
    pub fn cvt(&mut self) -> CVT_W<CTRLA_SPEC> {
        CVT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt Vector Select"]
    #[inline(always)]
    #[must_use]
    pub fn ivsel(&mut self) -> IVSEL_W<CTRLA_SPEC> {
        IVSEL_W::new(self, 6)
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
