#[doc = "Register `CTRLE` reader"]
pub type R = crate::R<CTRLE_SPEC>;
#[doc = "Register `CTRLE` writer"]
pub type W = crate::W<CTRLE_SPEC>;
#[doc = "Field `WINCM` reader - Window Comparator Mode"]
pub type WINCM_R = crate::FieldReader<WINCM_A>;
#[doc = "Window Comparator Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WINCM_A {
    #[doc = "0: No Window Comparison"]
    NONE = 0,
    #[doc = "1: Below Window"]
    BELOW = 1,
    #[doc = "2: Above Window"]
    ABOVE = 2,
    #[doc = "3: Inside Window"]
    INSIDE = 3,
    #[doc = "4: Outside Window"]
    OUTSIDE = 4,
}
impl From<WINCM_A> for u8 {
    #[inline(always)]
    fn from(variant: WINCM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WINCM_A {
    type Ux = u8;
}
impl WINCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WINCM_A> {
        match self.bits {
            0 => Some(WINCM_A::NONE),
            1 => Some(WINCM_A::BELOW),
            2 => Some(WINCM_A::ABOVE),
            3 => Some(WINCM_A::INSIDE),
            4 => Some(WINCM_A::OUTSIDE),
            _ => None,
        }
    }
    #[doc = "No Window Comparison"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WINCM_A::NONE
    }
    #[doc = "Below Window"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == WINCM_A::BELOW
    }
    #[doc = "Above Window"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == WINCM_A::ABOVE
    }
    #[doc = "Inside Window"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        *self == WINCM_A::INSIDE
    }
    #[doc = "Outside Window"]
    #[inline(always)]
    pub fn is_outside(&self) -> bool {
        *self == WINCM_A::OUTSIDE
    }
}
#[doc = "Field `WINCM` writer - Window Comparator Mode"]
pub type WINCM_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WINCM_A>;
impl<'a, REG> WINCM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Window Comparison"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(WINCM_A::NONE)
    }
    #[doc = "Below Window"]
    #[inline(always)]
    pub fn below(self) -> &'a mut crate::W<REG> {
        self.variant(WINCM_A::BELOW)
    }
    #[doc = "Above Window"]
    #[inline(always)]
    pub fn above(self) -> &'a mut crate::W<REG> {
        self.variant(WINCM_A::ABOVE)
    }
    #[doc = "Inside Window"]
    #[inline(always)]
    pub fn inside(self) -> &'a mut crate::W<REG> {
        self.variant(WINCM_A::INSIDE)
    }
    #[doc = "Outside Window"]
    #[inline(always)]
    pub fn outside(self) -> &'a mut crate::W<REG> {
        self.variant(WINCM_A::OUTSIDE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Window Comparator Mode"]
    #[inline(always)]
    pub fn wincm(&self) -> WINCM_R {
        WINCM_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Window Comparator Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wincm(&mut self) -> WINCM_W<CTRLE_SPEC> {
        WINCM_W::new(self, 0)
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
#[doc = "Control E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrle::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrle::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLE_SPEC;
impl crate::RegisterSpec for CTRLE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrle::R`](R) reader structure"]
impl crate::Readable for CTRLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrle::W`](W) writer structure"]
impl crate::Writable for CTRLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLE to value 0"]
impl crate::Resettable for CTRLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
