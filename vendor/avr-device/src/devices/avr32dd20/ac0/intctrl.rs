#[doc = "Register `INTCTRL` reader"]
pub type R = crate::R<INTCTRL_SPEC>;
#[doc = "Register `INTCTRL` writer"]
pub type W = crate::W<INTCTRL_SPEC>;
#[doc = "Field `CMP` reader - Interrupt Enable"]
pub type CMP_R = crate::BitReader;
#[doc = "Field `CMP` writer - Interrupt Enable"]
pub type CMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTMODE` reader - Interrupt Mode"]
pub type INTMODE_R = crate::FieldReader<INTMODE_A>;
#[doc = "Interrupt Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTMODE_A {
    #[doc = "0: Positive and negative inputs crosses"]
    BOTHEDGE = 0,
    #[doc = "2: Positive input goes below negative input"]
    NEGEDGE = 2,
    #[doc = "3: Positive input goes above negative input"]
    POSEDGE = 3,
}
impl From<INTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: INTMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INTMODE_A {
    type Ux = u8;
}
impl INTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INTMODE_A> {
        match self.bits {
            0 => Some(INTMODE_A::BOTHEDGE),
            2 => Some(INTMODE_A::NEGEDGE),
            3 => Some(INTMODE_A::POSEDGE),
            _ => None,
        }
    }
    #[doc = "Positive and negative inputs crosses"]
    #[inline(always)]
    pub fn is_bothedge(&self) -> bool {
        *self == INTMODE_A::BOTHEDGE
    }
    #[doc = "Positive input goes below negative input"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == INTMODE_A::NEGEDGE
    }
    #[doc = "Positive input goes above negative input"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == INTMODE_A::POSEDGE
    }
}
#[doc = "Field `INTMODE` writer - Interrupt Mode"]
pub type INTMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, INTMODE_A>;
impl<'a, REG> INTMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Positive and negative inputs crosses"]
    #[inline(always)]
    pub fn bothedge(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE_A::BOTHEDGE)
    }
    #[doc = "Positive input goes below negative input"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE_A::NEGEDGE)
    }
    #[doc = "Positive input goes above negative input"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE_A::POSEDGE)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Enable"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Interrupt Mode"]
    #[inline(always)]
    pub fn intmode(&self) -> INTMODE_R {
        INTMODE_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<INTCTRL_SPEC> {
        CMP_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Interrupt Mode"]
    #[inline(always)]
    #[must_use]
    pub fn intmode(&mut self) -> INTMODE_W<INTCTRL_SPEC> {
        INTMODE_W::new(self, 4)
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
#[doc = "Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTCTRL_SPEC;
impl crate::RegisterSpec for INTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intctrl::R`](R) reader structure"]
impl crate::Readable for INTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intctrl::W`](W) writer structure"]
impl crate::Writable for INTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCTRL to value 0"]
impl crate::Resettable for INTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
