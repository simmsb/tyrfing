#[doc = "Register `INTCTRL` reader"]
pub type R = crate::R<SINGLE_INTCTRL_SPEC>;
#[doc = "Register `INTCTRL` writer"]
pub type W = crate::W<SINGLE_INTCTRL_SPEC>;
#[doc = "Field `OVF` reader - Overflow Interrupt"]
pub type OVF_R = crate::BitReader;
#[doc = "Field `OVF` writer - Overflow Interrupt"]
pub type OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0` reader - Compare 0 Interrupt"]
pub type CMP0_R = crate::BitReader;
#[doc = "Field `CMP0` writer - Compare 0 Interrupt"]
pub type CMP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1` reader - Compare 1 Interrupt"]
pub type CMP1_R = crate::BitReader;
#[doc = "Field `CMP1` writer - Compare 1 Interrupt"]
pub type CMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2` reader - Compare 2 Interrupt"]
pub type CMP2_R = crate::BitReader;
#[doc = "Field `CMP2` writer - Compare 2 Interrupt"]
pub type CMP2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Compare 0 Interrupt"]
    #[inline(always)]
    pub fn cmp0(&self) -> CMP0_R {
        CMP0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare 1 Interrupt"]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare 2 Interrupt"]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<SINGLE_INTCTRL_SPEC> {
        OVF_W::new(self, 0)
    }
    #[doc = "Bit 4 - Compare 0 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0(&mut self) -> CMP0_W<SINGLE_INTCTRL_SPEC> {
        CMP0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Compare 1 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1(&mut self) -> CMP1_W<SINGLE_INTCTRL_SPEC> {
        CMP1_W::new(self, 5)
    }
    #[doc = "Bit 6 - Compare 2 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2(&mut self) -> CMP2_W<SINGLE_INTCTRL_SPEC> {
        CMP2_W::new(self, 6)
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
#[doc = "Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_intctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_intctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINGLE_INTCTRL_SPEC;
impl crate::RegisterSpec for SINGLE_INTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`single_intctrl::R`](R) reader structure"]
impl crate::Readable for SINGLE_INTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`single_intctrl::W`](W) writer structure"]
impl crate::Writable for SINGLE_INTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCTRL to value 0"]
impl crate::Resettable for SINGLE_INTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
