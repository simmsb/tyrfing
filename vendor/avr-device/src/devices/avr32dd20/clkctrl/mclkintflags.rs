#[doc = "Register `MCLKINTFLAGS` reader"]
pub type R = crate::R<MCLKINTFLAGS_SPEC>;
#[doc = "Register `MCLKINTFLAGS` writer"]
pub type W = crate::W<MCLKINTFLAGS_SPEC>;
#[doc = "Field `CFD` reader - Clock Failure Detect Interrupt Flag"]
pub type CFD_R = crate::BitReader;
#[doc = "Field `CFD` writer - Clock Failure Detect Interrupt Flag"]
pub type CFD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock Failure Detect Interrupt Flag"]
    #[inline(always)]
    pub fn cfd(&self) -> CFD_R {
        CFD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Failure Detect Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cfd(&mut self) -> CFD_W<MCLKINTFLAGS_SPEC> {
        CFD_W::new(self, 0)
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
#[doc = "MCLK Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclkintflags::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclkintflags::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCLKINTFLAGS_SPEC;
impl crate::RegisterSpec for MCLKINTFLAGS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mclkintflags::R`](R) reader structure"]
impl crate::Readable for MCLKINTFLAGS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mclkintflags::W`](W) writer structure"]
impl crate::Writable for MCLKINTFLAGS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCLKINTFLAGS to value 0"]
impl crate::Resettable for MCLKINTFLAGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
