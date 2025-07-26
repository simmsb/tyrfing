#[doc = "Register `VREGCTRL` reader"]
pub type R = crate::R<VREGCTRL_SPEC>;
#[doc = "Register `VREGCTRL` writer"]
pub type W = crate::W<VREGCTRL_SPEC>;
#[doc = "Field `PMODE` reader - Performance Mode"]
pub type PMODE_R = crate::FieldReader<PMODE_A>;
#[doc = "Performance Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PMODE_A {
    #[doc = "0: No Description."]
    AUTO = 0,
    #[doc = "1: No Description."]
    FULL = 1,
}
impl From<PMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PMODE_A {
    type Ux = u8;
}
impl PMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PMODE_A> {
        match self.bits {
            0 => Some(PMODE_A::AUTO),
            1 => Some(PMODE_A::FULL),
            _ => None,
        }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == PMODE_A::AUTO
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == PMODE_A::FULL
    }
}
#[doc = "Field `PMODE` writer - Performance Mode"]
pub type PMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PMODE_A>;
impl<'a, REG> PMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Description."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(PMODE_A::AUTO)
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(PMODE_A::FULL)
    }
}
#[doc = "Field `HTLLEN` reader - High Temperature Low Leakage Enable"]
pub type HTLLEN_R = crate::BitReader<HTLLEN_A>;
#[doc = "High Temperature Low Leakage Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTLLEN_A {
    #[doc = "0: Disabled"]
    OFF = 0,
    #[doc = "1: Enabled"]
    ON = 1,
}
impl From<HTLLEN_A> for bool {
    #[inline(always)]
    fn from(variant: HTLLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HTLLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HTLLEN_A {
        match self.bits {
            false => HTLLEN_A::OFF,
            true => HTLLEN_A::ON,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HTLLEN_A::OFF
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == HTLLEN_A::ON
    }
}
#[doc = "Field `HTLLEN` writer - High Temperature Low Leakage Enable"]
pub type HTLLEN_W<'a, REG> = crate::BitWriter<'a, REG, HTLLEN_A>;
impl<'a, REG> HTLLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(HTLLEN_A::OFF)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(HTLLEN_A::ON)
    }
}
impl R {
    #[doc = "Bits 0:2 - Performance Mode"]
    #[inline(always)]
    pub fn pmode(&self) -> PMODE_R {
        PMODE_R::new(self.bits & 7)
    }
    #[doc = "Bit 4 - High Temperature Low Leakage Enable"]
    #[inline(always)]
    pub fn htllen(&self) -> HTLLEN_R {
        HTLLEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Performance Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pmode(&mut self) -> PMODE_W<VREGCTRL_SPEC> {
        PMODE_W::new(self, 0)
    }
    #[doc = "Bit 4 - High Temperature Low Leakage Enable"]
    #[inline(always)]
    #[must_use]
    pub fn htllen(&mut self) -> HTLLEN_W<VREGCTRL_SPEC> {
        HTLLEN_W::new(self, 4)
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
#[doc = "Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vregctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vregctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VREGCTRL_SPEC;
impl crate::RegisterSpec for VREGCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vregctrl::R`](R) reader structure"]
impl crate::Readable for VREGCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vregctrl::W`](W) writer structure"]
impl crate::Writable for VREGCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREGCTRL to value 0"]
impl crate::Resettable for VREGCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
