#[doc = "Register `GICR` reader"]
pub type R = crate::R<GICR_SPEC>;
#[doc = "Register `GICR` writer"]
pub type W = crate::W<GICR_SPEC>;
#[doc = "Field `IVCE` reader - Interrupt Vector Change Enable"]
pub type IVCE_R = crate::BitReader;
#[doc = "Field `IVCE` writer - Interrupt Vector Change Enable"]
pub type IVCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IVSEL` reader - Interrupt Vector Select"]
pub type IVSEL_R = crate::BitReader;
#[doc = "Field `IVSEL` writer - Interrupt Vector Select"]
pub type IVSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2` reader - External Interrupt Request 2 Enable"]
pub type INT2_R = crate::BitReader;
#[doc = "Field `INT2` writer - External Interrupt Request 2 Enable"]
pub type INT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT0` reader - External Interrupt Request 0 Enable"]
pub type INT0_R = crate::BitReader;
#[doc = "Field `INT0` writer - External Interrupt Request 0 Enable"]
pub type INT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1` reader - External Interrupt Request 1 Enable"]
pub type INT1_R = crate::BitReader;
#[doc = "Field `INT1` writer - External Interrupt Request 1 Enable"]
pub type INT1_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 5 - External Interrupt Request 2 Enable"]
    #[inline(always)]
    pub fn int2(&self) -> INT2_R {
        INT2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Interrupt Request 0 Enable"]
    #[inline(always)]
    pub fn int0(&self) -> INT0_R {
        INT0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Interrupt Request 1 Enable"]
    #[inline(always)]
    pub fn int1(&self) -> INT1_R {
        INT1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Vector Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ivce(&mut self) -> IVCE_W<GICR_SPEC> {
        IVCE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt Vector Select"]
    #[inline(always)]
    #[must_use]
    pub fn ivsel(&mut self) -> IVSEL_W<GICR_SPEC> {
        IVSEL_W::new(self, 1)
    }
    #[doc = "Bit 5 - External Interrupt Request 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> INT2_W<GICR_SPEC> {
        INT2_W::new(self, 5)
    }
    #[doc = "Bit 6 - External Interrupt Request 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> INT0_W<GICR_SPEC> {
        INT0_W::new(self, 6)
    }
    #[doc = "Bit 7 - External Interrupt Request 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> INT1_W<GICR_SPEC> {
        INT1_W::new(self, 7)
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
#[doc = "General Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICR_SPEC;
impl crate::RegisterSpec for GICR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gicr::R`](R) reader structure"]
impl crate::Readable for GICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicr::W`](W) writer structure"]
impl crate::Writable for GICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICR to value 0"]
impl crate::Resettable for GICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
