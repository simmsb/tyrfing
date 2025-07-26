#[doc = "Register `TIFR` reader"]
pub type R = crate::R<TIFR_SPEC>;
#[doc = "Register `TIFR` writer"]
pub type W = crate::W<TIFR_SPEC>;
#[doc = "Field `TOV2` reader - Timer/Counter2 Overflow Flag"]
pub type TOV2_R = crate::BitReader;
#[doc = "Field `TOV2` writer - Timer/Counter2 Overflow Flag"]
pub type TOV2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF2` reader - Output Compare Flag 2"]
pub type OCF2_R = crate::BitReader;
#[doc = "Field `OCF2` writer - Output Compare Flag 2"]
pub type OCF2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - Timer/Counter2 Overflow Flag"]
    #[inline(always)]
    pub fn tov2(&self) -> TOV2_R {
        TOV2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output Compare Flag 2"]
    #[inline(always)]
    pub fn ocf2(&self) -> OCF2_R {
        OCF2_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Timer/Counter2 Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tov2(&mut self) -> TOV2_W<TIFR_SPEC> {
        TOV2_W::new(self, 6)
    }
    #[doc = "Bit 7 - Output Compare Flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn ocf2(&mut self) -> OCF2_W<TIFR_SPEC> {
        OCF2_W::new(self, 7)
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
#[doc = "Timer/Counter Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tifr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tifr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIFR_SPEC;
impl crate::RegisterSpec for TIFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tifr::R`](R) reader structure"]
impl crate::Readable for TIFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tifr::W`](W) writer structure"]
impl crate::Writable for TIFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIFR to value 0"]
impl crate::Resettable for TIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
