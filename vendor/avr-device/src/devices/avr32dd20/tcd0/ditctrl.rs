#[doc = "Register `DITCTRL` reader"]
pub type R = crate::R<DITCTRL_SPEC>;
#[doc = "Register `DITCTRL` writer"]
pub type W = crate::W<DITCTRL_SPEC>;
#[doc = "Field `DITHERSEL` reader - Dither select"]
pub type DITHERSEL_R = crate::FieldReader<DITHERSEL_A>;
#[doc = "Dither select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DITHERSEL_A {
    #[doc = "0: On-time ramp B"]
    ONTIMEB = 0,
    #[doc = "1: On-time ramp A and B"]
    ONTIMEAB = 1,
    #[doc = "2: Dead-time rampB"]
    DEADTIMEB = 2,
    #[doc = "3: Dead-time ramp A and B"]
    DEADTIMEAB = 3,
}
impl From<DITHERSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DITHERSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DITHERSEL_A {
    type Ux = u8;
}
impl DITHERSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DITHERSEL_A {
        match self.bits {
            0 => DITHERSEL_A::ONTIMEB,
            1 => DITHERSEL_A::ONTIMEAB,
            2 => DITHERSEL_A::DEADTIMEB,
            3 => DITHERSEL_A::DEADTIMEAB,
            _ => unreachable!(),
        }
    }
    #[doc = "On-time ramp B"]
    #[inline(always)]
    pub fn is_ontimeb(&self) -> bool {
        *self == DITHERSEL_A::ONTIMEB
    }
    #[doc = "On-time ramp A and B"]
    #[inline(always)]
    pub fn is_ontimeab(&self) -> bool {
        *self == DITHERSEL_A::ONTIMEAB
    }
    #[doc = "Dead-time rampB"]
    #[inline(always)]
    pub fn is_deadtimeb(&self) -> bool {
        *self == DITHERSEL_A::DEADTIMEB
    }
    #[doc = "Dead-time ramp A and B"]
    #[inline(always)]
    pub fn is_deadtimeab(&self) -> bool {
        *self == DITHERSEL_A::DEADTIMEAB
    }
}
#[doc = "Field `DITHERSEL` writer - Dither select"]
pub type DITHERSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DITHERSEL_A>;
impl<'a, REG> DITHERSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "On-time ramp B"]
    #[inline(always)]
    pub fn ontimeb(self) -> &'a mut crate::W<REG> {
        self.variant(DITHERSEL_A::ONTIMEB)
    }
    #[doc = "On-time ramp A and B"]
    #[inline(always)]
    pub fn ontimeab(self) -> &'a mut crate::W<REG> {
        self.variant(DITHERSEL_A::ONTIMEAB)
    }
    #[doc = "Dead-time rampB"]
    #[inline(always)]
    pub fn deadtimeb(self) -> &'a mut crate::W<REG> {
        self.variant(DITHERSEL_A::DEADTIMEB)
    }
    #[doc = "Dead-time ramp A and B"]
    #[inline(always)]
    pub fn deadtimeab(self) -> &'a mut crate::W<REG> {
        self.variant(DITHERSEL_A::DEADTIMEAB)
    }
}
impl R {
    #[doc = "Bits 0:1 - Dither select"]
    #[inline(always)]
    pub fn dithersel(&self) -> DITHERSEL_R {
        DITHERSEL_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Dither select"]
    #[inline(always)]
    #[must_use]
    pub fn dithersel(&mut self) -> DITHERSEL_W<DITCTRL_SPEC> {
        DITHERSEL_W::new(self, 0)
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
#[doc = "Dither Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ditctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ditctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DITCTRL_SPEC;
impl crate::RegisterSpec for DITCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ditctrl::R`](R) reader structure"]
impl crate::Readable for DITCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ditctrl::W`](W) writer structure"]
impl crate::Writable for DITCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DITCTRL to value 0"]
impl crate::Resettable for DITCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
