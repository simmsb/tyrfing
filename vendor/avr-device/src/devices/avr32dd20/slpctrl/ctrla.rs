#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `SEN` reader - Sleep enable"]
pub type SEN_R = crate::BitReader;
#[doc = "Field `SEN` writer - Sleep enable"]
pub type SEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMODE` reader - Sleep mode"]
pub type SMODE_R = crate::FieldReader<SMODE_A>;
#[doc = "Sleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMODE_A {
    #[doc = "0: Idle mode"]
    IDLE = 0,
    #[doc = "1: Standby Mode"]
    STDBY = 1,
    #[doc = "2: Power-down Mode"]
    PDOWN = 2,
}
impl From<SMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMODE_A {
    type Ux = u8;
}
impl SMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SMODE_A> {
        match self.bits {
            0 => Some(SMODE_A::IDLE),
            1 => Some(SMODE_A::STDBY),
            2 => Some(SMODE_A::PDOWN),
            _ => None,
        }
    }
    #[doc = "Idle mode"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SMODE_A::IDLE
    }
    #[doc = "Standby Mode"]
    #[inline(always)]
    pub fn is_stdby(&self) -> bool {
        *self == SMODE_A::STDBY
    }
    #[doc = "Power-down Mode"]
    #[inline(always)]
    pub fn is_pdown(&self) -> bool {
        *self == SMODE_A::PDOWN
    }
}
#[doc = "Field `SMODE` writer - Sleep mode"]
pub type SMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SMODE_A>;
impl<'a, REG> SMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Idle mode"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(SMODE_A::IDLE)
    }
    #[doc = "Standby Mode"]
    #[inline(always)]
    pub fn stdby(self) -> &'a mut crate::W<REG> {
        self.variant(SMODE_A::STDBY)
    }
    #[doc = "Power-down Mode"]
    #[inline(always)]
    pub fn pdown(self) -> &'a mut crate::W<REG> {
        self.variant(SMODE_A::PDOWN)
    }
}
impl R {
    #[doc = "Bit 0 - Sleep enable"]
    #[inline(always)]
    pub fn sen(&self) -> SEN_R {
        SEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Sleep mode"]
    #[inline(always)]
    pub fn smode(&self) -> SMODE_R {
        SMODE_R::new((self.bits >> 1) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Sleep enable"]
    #[inline(always)]
    #[must_use]
    pub fn sen(&mut self) -> SEN_W<CTRLA_SPEC> {
        SEN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn smode(&mut self) -> SMODE_W<CTRLA_SPEC> {
        SMODE_W::new(self, 1)
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
