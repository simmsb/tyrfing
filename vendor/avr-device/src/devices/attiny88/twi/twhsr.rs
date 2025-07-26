#[doc = "Register `TWHSR` reader"]
pub type R = crate::R<TWHSR_SPEC>;
#[doc = "Register `TWHSR` writer"]
pub type W = crate::W<TWHSR_SPEC>;
#[doc = "Field `TWHS` reader - No Description."]
pub type TWHS_R = crate::BitReader;
#[doc = "Field `TWHS` writer - No Description."]
pub type TWHS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn twhs(&self) -> TWHS_R {
        TWHS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn twhs(&mut self) -> TWHS_W<TWHSR_SPEC> {
        TWHS_W::new(self, 0)
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
#[doc = "TWHSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twhsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twhsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWHSR_SPEC;
impl crate::RegisterSpec for TWHSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twhsr::R`](R) reader structure"]
impl crate::Readable for TWHSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twhsr::W`](W) writer structure"]
impl crate::Writable for TWHSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWHSR to value 0"]
impl crate::Resettable for TWHSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
