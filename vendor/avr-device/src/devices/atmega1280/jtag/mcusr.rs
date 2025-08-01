#[doc = "Register `MCUSR` reader"]
pub type R = crate::R<MCUSR_SPEC>;
#[doc = "Register `MCUSR` writer"]
pub type W = crate::W<MCUSR_SPEC>;
#[doc = "Field `JTRF` reader - JTAG Reset Flag"]
pub type JTRF_R = crate::BitReader;
#[doc = "Field `JTRF` writer - JTAG Reset Flag"]
pub type JTRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - JTAG Reset Flag"]
    #[inline(always)]
    pub fn jtrf(&self) -> JTRF_R {
        JTRF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - JTAG Reset Flag"]
    #[inline(always)]
    #[must_use]
    pub fn jtrf(&mut self) -> JTRF_W<MCUSR_SPEC> {
        JTRF_W::new(self, 4)
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
#[doc = "MCU Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcusr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcusr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCUSR_SPEC;
impl crate::RegisterSpec for MCUSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mcusr::R`](R) reader structure"]
impl crate::Readable for MCUSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcusr::W`](W) writer structure"]
impl crate::Writable for MCUSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUSR to value 0"]
impl crate::Resettable for MCUSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
