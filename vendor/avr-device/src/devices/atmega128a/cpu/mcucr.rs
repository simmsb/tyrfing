#[doc = "Register `MCUCR` reader"]
pub type R = crate::R<MCUCR_SPEC>;
#[doc = "Register `MCUCR` writer"]
pub type W = crate::W<MCUCR_SPEC>;
#[doc = "Field `IVCE` reader - Interrupt Vector Change Enable"]
pub type IVCE_R = crate::BitReader;
#[doc = "Field `IVCE` writer - Interrupt Vector Change Enable"]
pub type IVCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IVSEL` reader - Interrupt Vector Select"]
pub type IVSEL_R = crate::BitReader;
#[doc = "Field `IVSEL` writer - Interrupt Vector Select"]
pub type IVSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM2` reader - Sleep Mode Select"]
pub type SM2_R = crate::BitReader<SM2_A>;
#[doc = "Sleep Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SM2_A {
    #[doc = "0: Idle"]
    IDLE = 0,
}
impl From<SM2_A> for bool {
    #[inline(always)]
    fn from(variant: SM2_A) -> Self {
        variant as u8 != 0
    }
}
impl SM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SM2_A> {
        match self.bits {
            false => Some(SM2_A::IDLE),
            _ => None,
        }
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SM2_A::IDLE
    }
}
#[doc = "Field `SM2` writer - Sleep Mode Select"]
pub type SM2_W<'a, REG> = crate::BitWriter<'a, REG, SM2_A>;
impl<'a, REG> SM2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Idle"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(SM2_A::IDLE)
    }
}
#[doc = "Field `SM` reader - Sleep Mode Select"]
pub type SM_R = crate::FieldReader;
#[doc = "Field `SM` writer - Sleep Mode Select"]
pub type SM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `SE` reader - Sleep Enable"]
pub type SE_R = crate::BitReader;
#[doc = "Field `SE` writer - Sleep Enable"]
pub type SE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRW10` reader - External SRAM Wait State Select"]
pub type SRW10_R = crate::BitReader;
#[doc = "Field `SRW10` writer - External SRAM Wait State Select"]
pub type SRW10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRE` reader - External SRAM Enable"]
pub type SRE_R = crate::BitReader;
#[doc = "Field `SRE` writer - External SRAM Enable"]
pub type SRE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt Vector Change Enable"]
    #[inline(always)]
    pub fn ivce(&self) -> IVCE_R {
        IVCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Vector Select"]
    #[inline(always)]
    pub fn ivsel(&self) -> IVSEL_R {
        IVSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sleep Mode Select"]
    #[inline(always)]
    pub fn sm2(&self) -> SM2_R {
        SM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Sleep Mode Select"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - Sleep Enable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External SRAM Wait State Select"]
    #[inline(always)]
    pub fn srw10(&self) -> SRW10_R {
        SRW10_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External SRAM Enable"]
    #[inline(always)]
    pub fn sre(&self) -> SRE_R {
        SRE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Vector Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ivce(&mut self) -> IVCE_W<MCUCR_SPEC> {
        IVCE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt Vector Select"]
    #[inline(always)]
    #[must_use]
    pub fn ivsel(&mut self) -> IVSEL_W<MCUCR_SPEC> {
        IVSEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Sleep Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn sm2(&mut self) -> SM2_W<MCUCR_SPEC> {
        SM2_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Sleep Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn sm(&mut self) -> SM_W<MCUCR_SPEC> {
        SM_W::new(self, 3)
    }
    #[doc = "Bit 5 - Sleep Enable"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<MCUCR_SPEC> {
        SE_W::new(self, 5)
    }
    #[doc = "Bit 6 - External SRAM Wait State Select"]
    #[inline(always)]
    #[must_use]
    pub fn srw10(&mut self) -> SRW10_W<MCUCR_SPEC> {
        SRW10_W::new(self, 6)
    }
    #[doc = "Bit 7 - External SRAM Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sre(&mut self) -> SRE_W<MCUCR_SPEC> {
        SRE_W::new(self, 7)
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
#[doc = "MCU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCUCR_SPEC;
impl crate::RegisterSpec for MCUCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mcucr::R`](R) reader structure"]
impl crate::Readable for MCUCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcucr::W`](W) writer structure"]
impl crate::Writable for MCUCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUCR to value 0"]
impl crate::Resettable for MCUCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
