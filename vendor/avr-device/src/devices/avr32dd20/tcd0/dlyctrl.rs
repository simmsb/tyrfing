#[doc = "Register `DLYCTRL` reader"]
pub type R = crate::R<DLYCTRL_SPEC>;
#[doc = "Register `DLYCTRL` writer"]
pub type W = crate::W<DLYCTRL_SPEC>;
#[doc = "Field `DLYSEL` reader - Delay select"]
pub type DLYSEL_R = crate::FieldReader<DLYSEL_A>;
#[doc = "Delay select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLYSEL_A {
    #[doc = "0: No delay"]
    OFF = 0,
    #[doc = "1: Input blanking enabled"]
    INBLANK = 1,
    #[doc = "2: Event delay enabled"]
    EVENT = 2,
}
impl From<DLYSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DLYSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DLYSEL_A {
    type Ux = u8;
}
impl DLYSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DLYSEL_A> {
        match self.bits {
            0 => Some(DLYSEL_A::OFF),
            1 => Some(DLYSEL_A::INBLANK),
            2 => Some(DLYSEL_A::EVENT),
            _ => None,
        }
    }
    #[doc = "No delay"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DLYSEL_A::OFF
    }
    #[doc = "Input blanking enabled"]
    #[inline(always)]
    pub fn is_inblank(&self) -> bool {
        *self == DLYSEL_A::INBLANK
    }
    #[doc = "Event delay enabled"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == DLYSEL_A::EVENT
    }
}
#[doc = "Field `DLYSEL` writer - Delay select"]
pub type DLYSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DLYSEL_A>;
impl<'a, REG> DLYSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No delay"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(DLYSEL_A::OFF)
    }
    #[doc = "Input blanking enabled"]
    #[inline(always)]
    pub fn inblank(self) -> &'a mut crate::W<REG> {
        self.variant(DLYSEL_A::INBLANK)
    }
    #[doc = "Event delay enabled"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(DLYSEL_A::EVENT)
    }
}
#[doc = "Field `DLYTRIG` reader - Delay trigger"]
pub type DLYTRIG_R = crate::FieldReader<DLYTRIG_A>;
#[doc = "Delay trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLYTRIG_A {
    #[doc = "0: Compare A set"]
    CMPASET = 0,
    #[doc = "1: Compare A clear"]
    CMPACLR = 1,
    #[doc = "2: Compare B set"]
    CMPBSET = 2,
    #[doc = "3: Compare B clear"]
    CMPBCLR = 3,
}
impl From<DLYTRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: DLYTRIG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DLYTRIG_A {
    type Ux = u8;
}
impl DLYTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DLYTRIG_A {
        match self.bits {
            0 => DLYTRIG_A::CMPASET,
            1 => DLYTRIG_A::CMPACLR,
            2 => DLYTRIG_A::CMPBSET,
            3 => DLYTRIG_A::CMPBCLR,
            _ => unreachable!(),
        }
    }
    #[doc = "Compare A set"]
    #[inline(always)]
    pub fn is_cmpaset(&self) -> bool {
        *self == DLYTRIG_A::CMPASET
    }
    #[doc = "Compare A clear"]
    #[inline(always)]
    pub fn is_cmpaclr(&self) -> bool {
        *self == DLYTRIG_A::CMPACLR
    }
    #[doc = "Compare B set"]
    #[inline(always)]
    pub fn is_cmpbset(&self) -> bool {
        *self == DLYTRIG_A::CMPBSET
    }
    #[doc = "Compare B clear"]
    #[inline(always)]
    pub fn is_cmpbclr(&self) -> bool {
        *self == DLYTRIG_A::CMPBCLR
    }
}
#[doc = "Field `DLYTRIG` writer - Delay trigger"]
pub type DLYTRIG_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DLYTRIG_A>;
impl<'a, REG> DLYTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare A set"]
    #[inline(always)]
    pub fn cmpaset(self) -> &'a mut crate::W<REG> {
        self.variant(DLYTRIG_A::CMPASET)
    }
    #[doc = "Compare A clear"]
    #[inline(always)]
    pub fn cmpaclr(self) -> &'a mut crate::W<REG> {
        self.variant(DLYTRIG_A::CMPACLR)
    }
    #[doc = "Compare B set"]
    #[inline(always)]
    pub fn cmpbset(self) -> &'a mut crate::W<REG> {
        self.variant(DLYTRIG_A::CMPBSET)
    }
    #[doc = "Compare B clear"]
    #[inline(always)]
    pub fn cmpbclr(self) -> &'a mut crate::W<REG> {
        self.variant(DLYTRIG_A::CMPBCLR)
    }
}
#[doc = "Field `DLYPRESC` reader - Delay prescaler"]
pub type DLYPRESC_R = crate::FieldReader<DLYPRESC_A>;
#[doc = "Delay prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLYPRESC_A {
    #[doc = "0: No prescaling"]
    DIV1 = 0,
    #[doc = "1: Prescale with 2"]
    DIV2 = 1,
    #[doc = "2: Prescale with 4"]
    DIV4 = 2,
    #[doc = "3: Prescale with 8"]
    DIV8 = 3,
}
impl From<DLYPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: DLYPRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DLYPRESC_A {
    type Ux = u8;
}
impl DLYPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DLYPRESC_A {
        match self.bits {
            0 => DLYPRESC_A::DIV1,
            1 => DLYPRESC_A::DIV2,
            2 => DLYPRESC_A::DIV4,
            3 => DLYPRESC_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "No prescaling"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DLYPRESC_A::DIV1
    }
    #[doc = "Prescale with 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DLYPRESC_A::DIV2
    }
    #[doc = "Prescale with 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DLYPRESC_A::DIV4
    }
    #[doc = "Prescale with 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DLYPRESC_A::DIV8
    }
}
#[doc = "Field `DLYPRESC` writer - Delay prescaler"]
pub type DLYPRESC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DLYPRESC_A>;
impl<'a, REG> DLYPRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No prescaling"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRESC_A::DIV1)
    }
    #[doc = "Prescale with 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRESC_A::DIV2)
    }
    #[doc = "Prescale with 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRESC_A::DIV4)
    }
    #[doc = "Prescale with 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRESC_A::DIV8)
    }
}
impl R {
    #[doc = "Bits 0:1 - Delay select"]
    #[inline(always)]
    pub fn dlysel(&self) -> DLYSEL_R {
        DLYSEL_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Delay trigger"]
    #[inline(always)]
    pub fn dlytrig(&self) -> DLYTRIG_R {
        DLYTRIG_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Delay prescaler"]
    #[inline(always)]
    pub fn dlypresc(&self) -> DLYPRESC_R {
        DLYPRESC_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Delay select"]
    #[inline(always)]
    #[must_use]
    pub fn dlysel(&mut self) -> DLYSEL_W<DLYCTRL_SPEC> {
        DLYSEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Delay trigger"]
    #[inline(always)]
    #[must_use]
    pub fn dlytrig(&mut self) -> DLYTRIG_W<DLYCTRL_SPEC> {
        DLYTRIG_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Delay prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn dlypresc(&mut self) -> DLYPRESC_W<DLYCTRL_SPEC> {
        DLYPRESC_W::new(self, 4)
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
#[doc = "Delay Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlyctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlyctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLYCTRL_SPEC;
impl crate::RegisterSpec for DLYCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dlyctrl::R`](R) reader structure"]
impl crate::Readable for DLYCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dlyctrl::W`](W) writer structure"]
impl crate::Writable for DLYCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLYCTRL to value 0"]
impl crate::Resettable for DLYCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
