#[doc = "Register `EIFR` reader"]
pub type R = crate::R<EIFR_SPEC>;
#[doc = "Register `EIFR` writer"]
pub type W = crate::W<EIFR_SPEC>;
#[doc = "Field `INTF0` reader - External Interrupt Flags 0"]
pub type INTF0_R = crate::BitReader;
#[doc = "Field `INTF0` writer - External Interrupt Flags 0"]
pub type INTF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTF1` reader - External Interrupt Flags 1"]
pub type INTF1_R = crate::BitReader;
#[doc = "Field `INTF1` writer - External Interrupt Flags 1"]
pub type INTF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTF2` reader - External Interrupt Flags 2"]
pub type INTF2_R = crate::BitReader;
#[doc = "Field `INTF2` writer - External Interrupt Flags 2"]
pub type INTF2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External Interrupt Flags 0"]
    #[inline(always)]
    pub fn intf0(&self) -> INTF0_R {
        INTF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Interrupt Flags 1"]
    #[inline(always)]
    pub fn intf1(&self) -> INTF1_R {
        INTF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Interrupt Flags 2"]
    #[inline(always)]
    pub fn intf2(&self) -> INTF2_R {
        INTF2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt Flags 0"]
    #[inline(always)]
    #[must_use]
    pub fn intf0(&mut self) -> INTF0_W<EIFR_SPEC> {
        INTF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - External Interrupt Flags 1"]
    #[inline(always)]
    #[must_use]
    pub fn intf1(&mut self) -> INTF1_W<EIFR_SPEC> {
        INTF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - External Interrupt Flags 2"]
    #[inline(always)]
    #[must_use]
    pub fn intf2(&mut self) -> INTF2_W<EIFR_SPEC> {
        INTF2_W::new(self, 2)
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
#[doc = "External Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eifr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eifr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EIFR_SPEC;
impl crate::RegisterSpec for EIFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eifr::R`](R) reader structure"]
impl crate::Readable for EIFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eifr::W`](W) writer structure"]
impl crate::Writable for EIFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EIFR to value 0"]
impl crate::Resettable for EIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
