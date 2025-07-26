#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SMCR_SPEC>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SMCR_SPEC>;
#[doc = "Field `SE` reader - Sleep Enable"]
pub type SE_R = crate::BitReader;
#[doc = "Field `SE` writer - Sleep Enable"]
pub type SE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM` reader - Sleep Mode Select bits"]
pub type SM_R = crate::FieldReader<SM_A>;
#[doc = "Sleep Mode Select bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SM_A {
    #[doc = "0: Idle"]
    IDLE = 0,
    #[doc = "1: Reserved"]
    VAL_0X01 = 1,
    #[doc = "2: Power Down"]
    PDOWN = 2,
    #[doc = "3: Power Save"]
    PSAVE = 3,
    #[doc = "4: Reserved"]
    VAL_0X04 = 4,
    #[doc = "5: Reserved"]
    VAL_0X05 = 5,
    #[doc = "6: Standby"]
    STDBY = 6,
    #[doc = "7: Extended Standby"]
    ESTDBY = 7,
}
impl From<SM_A> for u8 {
    #[inline(always)]
    fn from(variant: SM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SM_A {
    type Ux = u8;
}
impl SM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SM_A {
        match self.bits {
            0 => SM_A::IDLE,
            1 => SM_A::VAL_0X01,
            2 => SM_A::PDOWN,
            3 => SM_A::PSAVE,
            4 => SM_A::VAL_0X04,
            5 => SM_A::VAL_0X05,
            6 => SM_A::STDBY,
            7 => SM_A::ESTDBY,
            _ => unreachable!(),
        }
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SM_A::IDLE
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == SM_A::VAL_0X01
    }
    #[doc = "Power Down"]
    #[inline(always)]
    pub fn is_pdown(&self) -> bool {
        *self == SM_A::PDOWN
    }
    #[doc = "Power Save"]
    #[inline(always)]
    pub fn is_psave(&self) -> bool {
        *self == SM_A::PSAVE
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_val_0x04(&self) -> bool {
        *self == SM_A::VAL_0X04
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_val_0x05(&self) -> bool {
        *self == SM_A::VAL_0X05
    }
    #[doc = "Standby"]
    #[inline(always)]
    pub fn is_stdby(&self) -> bool {
        *self == SM_A::STDBY
    }
    #[doc = "Extended Standby"]
    #[inline(always)]
    pub fn is_estdby(&self) -> bool {
        *self == SM_A::ESTDBY
    }
}
#[doc = "Field `SM` writer - Sleep Mode Select bits"]
pub type SM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SM_A>;
impl<'a, REG> SM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Idle"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::IDLE)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::VAL_0X01)
    }
    #[doc = "Power Down"]
    #[inline(always)]
    pub fn pdown(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::PDOWN)
    }
    #[doc = "Power Save"]
    #[inline(always)]
    pub fn psave(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::PSAVE)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn val_0x04(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::VAL_0X04)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn val_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::VAL_0X05)
    }
    #[doc = "Standby"]
    #[inline(always)]
    pub fn stdby(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::STDBY)
    }
    #[doc = "Extended Standby"]
    #[inline(always)]
    pub fn estdby(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::ESTDBY)
    }
}
impl R {
    #[doc = "Bit 0 - Sleep Enable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Sleep Mode Select bits"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new((self.bits >> 1) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - Sleep Enable"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<SMCR_SPEC> {
        SE_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - Sleep Mode Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn sm(&mut self) -> SM_W<SMCR_SPEC> {
        SM_W::new(self, 1)
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
#[doc = "Sleep Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMCR_SPEC;
impl crate::RegisterSpec for SMCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smcr::R`](R) reader structure"]
impl crate::Readable for SMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smcr::W`](W) writer structure"]
impl crate::Writable for SMCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMCR to value 0"]
impl crate::Resettable for SMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
