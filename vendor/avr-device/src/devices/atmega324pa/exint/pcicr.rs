#[doc = "Register `PCICR` reader"]
pub type R = crate::R<PCICR_SPEC>;
#[doc = "Register `PCICR` writer"]
pub type W = crate::W<PCICR_SPEC>;
#[doc = "Field `PCIE0` reader - Pin Change Interrupt Enable 0"]
pub type PCIE0_R = crate::BitReader;
#[doc = "Field `PCIE0` writer - Pin Change Interrupt Enable 0"]
pub type PCIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIE1` reader - Pin Change Interrupt Enable 1"]
pub type PCIE1_R = crate::BitReader;
#[doc = "Field `PCIE1` writer - Pin Change Interrupt Enable 1"]
pub type PCIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIE2` reader - Pin Change Interrupt Enable 2"]
pub type PCIE2_R = crate::BitReader;
#[doc = "Field `PCIE2` writer - Pin Change Interrupt Enable 2"]
pub type PCIE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIE3` reader - Pin Change Interrupt Enable 3"]
pub type PCIE3_R = crate::BitReader;
#[doc = "Field `PCIE3` writer - Pin Change Interrupt Enable 3"]
pub type PCIE3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin Change Interrupt Enable 0"]
    #[inline(always)]
    pub fn pcie0(&self) -> PCIE0_R {
        PCIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin Change Interrupt Enable 1"]
    #[inline(always)]
    pub fn pcie1(&self) -> PCIE1_R {
        PCIE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin Change Interrupt Enable 2"]
    #[inline(always)]
    pub fn pcie2(&self) -> PCIE2_R {
        PCIE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin Change Interrupt Enable 3"]
    #[inline(always)]
    pub fn pcie3(&self) -> PCIE3_R {
        PCIE3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin Change Interrupt Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn pcie0(&mut self) -> PCIE0_W<PCICR_SPEC> {
        PCIE0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin Change Interrupt Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn pcie1(&mut self) -> PCIE1_W<PCICR_SPEC> {
        PCIE1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin Change Interrupt Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn pcie2(&mut self) -> PCIE2_W<PCICR_SPEC> {
        PCIE2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin Change Interrupt Enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn pcie3(&mut self) -> PCIE3_W<PCICR_SPEC> {
        PCIE3_W::new(self, 3)
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
#[doc = "Pin Change Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCICR_SPEC;
impl crate::RegisterSpec for PCICR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pcicr::R`](R) reader structure"]
impl crate::Readable for PCICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcicr::W`](W) writer structure"]
impl crate::Writable for PCICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCICR to value 0"]
impl crate::Resettable for PCICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
