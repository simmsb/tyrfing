#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `LVL0RR` reader - Round-robin Scheduling Enable"]
pub type LVL0RR_R = crate::BitReader;
#[doc = "Field `LVL0RR` writer - Round-robin Scheduling Enable"]
pub type LVL0RR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CVT` reader - Compact Vector Table"]
pub type CVT_R = crate::BitReader;
#[doc = "Field `CVT` writer - Compact Vector Table"]
pub type CVT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IVSEL` reader - Interrupt Vector Select"]
pub type IVSEL_R = crate::BitReader;
#[doc = "Field `IVSEL` writer - Interrupt Vector Select"]
pub type IVSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Round-robin Scheduling Enable"]
    #[inline(always)]
    pub fn lvl0rr(&self) -> LVL0RR_R {
        LVL0RR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Compact Vector Table"]
    #[inline(always)]
    pub fn cvt(&self) -> CVT_R {
        CVT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Vector Select"]
    #[inline(always)]
    pub fn ivsel(&self) -> IVSEL_R {
        IVSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Round-robin Scheduling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvl0rr(&mut self) -> LVL0RR_W<CTRLA_SPEC> {
        LVL0RR_W::new(self, 0)
    }
    #[doc = "Bit 5 - Compact Vector Table"]
    #[inline(always)]
    #[must_use]
    pub fn cvt(&mut self) -> CVT_W<CTRLA_SPEC> {
        CVT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt Vector Select"]
    #[inline(always)]
    #[must_use]
    pub fn ivsel(&mut self) -> IVSEL_W<CTRLA_SPEC> {
        IVSEL_W::new(self, 6)
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
