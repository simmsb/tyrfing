#[doc = "Register `MCUCSR` reader"]
pub type R = crate::R<MCUCSR_SPEC>;
#[doc = "Register `MCUCSR` writer"]
pub type W = crate::W<MCUCSR_SPEC>;
#[doc = "Field `JTRF` reader - JTAG Reset Flag"]
pub type JTRF_R = crate::BitReader;
#[doc = "Field `JTRF` writer - JTAG Reset Flag"]
pub type JTRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTD` reader - JTAG Interface Disable"]
pub type JTD_R = crate::BitReader;
#[doc = "Field `JTD` writer - JTAG Interface Disable"]
pub type JTD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - JTAG Reset Flag"]
    #[inline(always)]
    pub fn jtrf(&self) -> JTRF_R {
        JTRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - JTAG Interface Disable"]
    #[inline(always)]
    pub fn jtd(&self) -> JTD_R {
        JTD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - JTAG Reset Flag"]
    #[inline(always)]
    #[must_use]
    pub fn jtrf(&mut self) -> JTRF_W<MCUCSR_SPEC> {
        JTRF_W::new(self, 4)
    }
    #[doc = "Bit 7 - JTAG Interface Disable"]
    #[inline(always)]
    #[must_use]
    pub fn jtd(&mut self) -> JTD_W<MCUCSR_SPEC> {
        JTD_W::new(self, 7)
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
#[doc = "MCU Control And Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCUCSR_SPEC;
impl crate::RegisterSpec for MCUCSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mcucsr::R`](R) reader structure"]
impl crate::Readable for MCUCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcucsr::W`](W) writer structure"]
impl crate::Writable for MCUCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUCSR to value 0"]
impl crate::Resettable for MCUCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
