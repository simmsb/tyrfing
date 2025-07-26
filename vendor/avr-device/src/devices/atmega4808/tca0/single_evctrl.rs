#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<SINGLE_EVCTRL_SPEC>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<SINGLE_EVCTRL_SPEC>;
#[doc = "Field `CNTEI` reader - Count on Event Input"]
pub type CNTEI_R = crate::BitReader;
#[doc = "Field `CNTEI` writer - Count on Event Input"]
pub type CNTEI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVACT` reader - Event Action"]
pub type EVACT_R = crate::FieldReader<EVACT_A>;
#[doc = "Event Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVACT_A {
    #[doc = "0: Count on positive edge event"]
    POSEDGE = 0,
    #[doc = "1: Count on any edge event"]
    ANYEDGE = 1,
    #[doc = "2: Count on prescaled clock while event line is 1."]
    HIGHLVL = 2,
    #[doc = "3: Count on prescaled clock. Event controls count direction. Up-count when event line is 0, down-count when event line is 1."]
    UPDOWN = 3,
}
impl From<EVACT_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EVACT_A {
    type Ux = u8;
}
impl EVACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EVACT_A {
        match self.bits {
            0 => EVACT_A::POSEDGE,
            1 => EVACT_A::ANYEDGE,
            2 => EVACT_A::HIGHLVL,
            3 => EVACT_A::UPDOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Count on positive edge event"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == EVACT_A::POSEDGE
    }
    #[doc = "Count on any edge event"]
    #[inline(always)]
    pub fn is_anyedge(&self) -> bool {
        *self == EVACT_A::ANYEDGE
    }
    #[doc = "Count on prescaled clock while event line is 1."]
    #[inline(always)]
    pub fn is_highlvl(&self) -> bool {
        *self == EVACT_A::HIGHLVL
    }
    #[doc = "Count on prescaled clock. Event controls count direction. Up-count when event line is 0, down-count when event line is 1."]
    #[inline(always)]
    pub fn is_updown(&self) -> bool {
        *self == EVACT_A::UPDOWN
    }
}
#[doc = "Field `EVACT` writer - Event Action"]
pub type EVACT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EVACT_A>;
impl<'a, REG> EVACT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Count on positive edge event"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut crate::W<REG> {
        self.variant(EVACT_A::POSEDGE)
    }
    #[doc = "Count on any edge event"]
    #[inline(always)]
    pub fn anyedge(self) -> &'a mut crate::W<REG> {
        self.variant(EVACT_A::ANYEDGE)
    }
    #[doc = "Count on prescaled clock while event line is 1."]
    #[inline(always)]
    pub fn highlvl(self) -> &'a mut crate::W<REG> {
        self.variant(EVACT_A::HIGHLVL)
    }
    #[doc = "Count on prescaled clock. Event controls count direction. Up-count when event line is 0, down-count when event line is 1."]
    #[inline(always)]
    pub fn updown(self) -> &'a mut crate::W<REG> {
        self.variant(EVACT_A::UPDOWN)
    }
}
impl R {
    #[doc = "Bit 0 - Count on Event Input"]
    #[inline(always)]
    pub fn cntei(&self) -> CNTEI_R {
        CNTEI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Event Action"]
    #[inline(always)]
    pub fn evact(&self) -> EVACT_R {
        EVACT_R::new((self.bits >> 1) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Count on Event Input"]
    #[inline(always)]
    #[must_use]
    pub fn cntei(&mut self) -> CNTEI_W<SINGLE_EVCTRL_SPEC> {
        CNTEI_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Event Action"]
    #[inline(always)]
    #[must_use]
    pub fn evact(&mut self) -> EVACT_W<SINGLE_EVCTRL_SPEC> {
        EVACT_W::new(self, 1)
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
