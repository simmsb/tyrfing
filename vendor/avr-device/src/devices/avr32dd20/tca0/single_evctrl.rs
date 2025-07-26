#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<SINGLE_EVCTRL_SPEC>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<SINGLE_EVCTRL_SPEC>;
#[doc = "Field `CNTAEI` reader - Count on Event Input A"]
pub type CNTAEI_R = crate::BitReader;
#[doc = "Field `CNTAEI` writer - Count on Event Input A"]
pub type CNTAEI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVACTA` reader - Event Action A"]
pub type EVACTA_R = crate::FieldReader<EVACTA_A>;
#[doc = "Event Action A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVACTA_A {
    #[doc = "0: Count on positive edge event"]
    CNT_POSEDGE = 0,
    #[doc = "1: Count on any edge event"]
    CNT_ANYEDGE = 1,
    #[doc = "2: Count on prescaled clock while event line is 1."]
    CNT_HIGHLVL = 2,
    #[doc = "3: Count on prescaled clock. Event controls count direction. Up-count when event line is 0, down-count when event line is 1."]
    UPDOWN = 3,
}
impl From<EVACTA_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACTA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EVACTA_A {
    type Ux = u8;
}
impl EVACTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EVACTA_A> {
        match self.bits {
            0 => Some(EVACTA_A::CNT_POSEDGE),
            1 => Some(EVACTA_A::CNT_ANYEDGE),
            2 => Some(EVACTA_A::CNT_HIGHLVL),
            3 => Some(EVACTA_A::UPDOWN),
            _ => None,
        }
    }
    #[doc = "Count on positive edge event"]
    #[inline(always)]
    pub fn is_cnt_posedge(&self) -> bool {
        *self == EVACTA_A::CNT_POSEDGE
    }
    #[doc = "Count on any edge event"]
    #[inline(always)]
    pub fn is_cnt_anyedge(&self) -> bool {
        *self == EVACTA_A::CNT_ANYEDGE
    }
    #[doc = "Count on prescaled clock while event line is 1."]
    #[inline(always)]
    pub fn is_cnt_highlvl(&self) -> bool {
        *self == EVACTA_A::CNT_HIGHLVL
    }
    #[doc = "Count on prescaled clock. Event controls count direction. Up-count when event line is 0, down-count when event line is 1."]
    #[inline(always)]
    pub fn is_updown(&self) -> bool {
        *self == EVACTA_A::UPDOWN
    }
}
#[doc = "Field `EVACTA` writer - Event Action A"]
pub type EVACTA_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EVACTA_A>;
impl<'a, REG> EVACTA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Count on positive edge event"]
    #[inline(always)]
    pub fn cnt_posedge(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTA_A::CNT_POSEDGE)
    }
    #[doc = "Count on any edge event"]
    #[inline(always)]
    pub fn cnt_anyedge(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTA_A::CNT_ANYEDGE)
    }
    #[doc = "Count on prescaled clock while event line is 1."]
    #[inline(always)]
    pub fn cnt_highlvl(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTA_A::CNT_HIGHLVL)
    }
    #[doc = "Count on prescaled clock. Event controls count direction. Up-count when event line is 0, down-count when event line is 1."]
    #[inline(always)]
    pub fn updown(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTA_A::UPDOWN)
    }
}
#[doc = "Field `CNTBEI` reader - Count on Event Input B"]
pub type CNTBEI_R = crate::BitReader;
#[doc = "Field `CNTBEI` writer - Count on Event Input B"]
pub type CNTBEI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVACTB` reader - Event Action B"]
pub type EVACTB_R = crate::FieldReader<EVACTB_A>;
#[doc = "Event Action B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVACTB_A {
    #[doc = "0: No Action"]
    NONE = 0,
    #[doc = "3: Count on prescaled clock. Event controls count direction. Up-count when event line is 0, down-count when event line is 1."]
    UPDOWN = 3,
    #[doc = "4: Restart counter at positive edge event"]
    RESTART_POSEDGE = 4,
    #[doc = "5: Restart counter on any edge event"]
    RESTART_ANYEDGE = 5,
    #[doc = "6: Restart counter while event line is 1."]
    RESTART_HIGHLVL = 6,
}
impl From<EVACTB_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACTB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EVACTB_A {
    type Ux = u8;
}
impl EVACTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EVACTB_A> {
        match self.bits {
            0 => Some(EVACTB_A::NONE),
            3 => Some(EVACTB_A::UPDOWN),
            4 => Some(EVACTB_A::RESTART_POSEDGE),
            5 => Some(EVACTB_A::RESTART_ANYEDGE),
            6 => Some(EVACTB_A::RESTART_HIGHLVL),
            _ => None,
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == EVACTB_A::NONE
    }
    #[doc = "Count on prescaled clock. Event controls count direction. Up-count when event line is 0, down-count when event line is 1."]
    #[inline(always)]
    pub fn is_updown(&self) -> bool {
        *self == EVACTB_A::UPDOWN
    }
    #[doc = "Restart counter at positive edge event"]
    #[inline(always)]
    pub fn is_restart_posedge(&self) -> bool {
        *self == EVACTB_A::RESTART_POSEDGE
    }
    #[doc = "Restart counter on any edge event"]
    #[inline(always)]
    pub fn is_restart_anyedge(&self) -> bool {
        *self == EVACTB_A::RESTART_ANYEDGE
    }
    #[doc = "Restart counter while event line is 1."]
    #[inline(always)]
    pub fn is_restart_highlvl(&self) -> bool {
        *self == EVACTB_A::RESTART_HIGHLVL
    }
}
#[doc = "Field `EVACTB` writer - Event Action B"]
pub type EVACTB_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EVACTB_A>;
impl<'a, REG> EVACTB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTB_A::NONE)
    }
    #[doc = "Count on prescaled clock. Event controls count direction. Up-count when event line is 0, down-count when event line is 1."]
    #[inline(always)]
    pub fn updown(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTB_A::UPDOWN)
    }
    #[doc = "Restart counter at positive edge event"]
    #[inline(always)]
    pub fn restart_posedge(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTB_A::RESTART_POSEDGE)
    }
    #[doc = "Restart counter on any edge event"]
    #[inline(always)]
    pub fn restart_anyedge(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTB_A::RESTART_ANYEDGE)
    }
    #[doc = "Restart counter while event line is 1."]
    #[inline(always)]
    pub fn restart_highlvl(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTB_A::RESTART_HIGHLVL)
    }
}
impl R {
    #[doc = "Bit 0 - Count on Event Input A"]
    #[inline(always)]
    pub fn cntaei(&self) -> CNTAEI_R {
        CNTAEI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Event Action A"]
    #[inline(always)]
    pub fn evacta(&self) -> EVACTA_R {
        EVACTA_R::new((self.bits >> 1) & 7)
    }
    #[doc = "Bit 4 - Count on Event Input B"]
    #[inline(always)]
    pub fn cntbei(&self) -> CNTBEI_R {
        CNTBEI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Event Action B"]
    #[inline(always)]
    pub fn evactb(&self) -> EVACTB_R {
        EVACTB_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - Count on Event Input A"]
    #[inline(always)]
    #[must_use]
    pub fn cntaei(&mut self) -> CNTAEI_W<SINGLE_EVCTRL_SPEC> {
        CNTAEI_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - Event Action A"]
    #[inline(always)]
    #[must_use]
    pub fn evacta(&mut self) -> EVACTA_W<SINGLE_EVCTRL_SPEC> {
        EVACTA_W::new(self, 1)
    }
    #[doc = "Bit 4 - Count on Event Input B"]
    #[inline(always)]
    #[must_use]
    pub fn cntbei(&mut self) -> CNTBEI_W<SINGLE_EVCTRL_SPEC> {
        CNTBEI_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - Event Action B"]
    #[inline(always)]
    #[must_use]
    pub fn evactb(&mut self) -> EVACTB_W<SINGLE_EVCTRL_SPEC> {
        EVACTB_W::new(self, 5)
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
#[doc = "Event Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_evctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_evctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINGLE_EVCTRL_SPEC;
impl crate::RegisterSpec for SINGLE_EVCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`single_evctrl::R`](R) reader structure"]
impl crate::Readable for SINGLE_EVCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`single_evctrl::W`](W) writer structure"]
impl crate::Writable for SINGLE_EVCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for SINGLE_EVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
