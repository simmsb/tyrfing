#[doc = "Register `TIFR` reader"]
pub type R = crate::R<TIFR_SPEC>;
#[doc = "Register `TIFR` writer"]
pub type W = crate::W<TIFR_SPEC>;
#[doc = "Field `ICF1` reader - Input Capture Flag 1"]
pub type ICF1_R = crate::BitReader;
#[doc = "Field `ICF1` writer - Input Capture Flag 1"]
pub type ICF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF1B` reader - Output Compare Flag 1B"]
pub type OCF1B_R = crate::BitReader;
#[doc = "Field `OCF1B` writer - Output Compare Flag 1B"]
pub type OCF1B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF1A` reader - Output Compare Flag 1A"]
pub type OCF1A_R = crate::BitReader;
#[doc = "Field `OCF1A` writer - Output Compare Flag 1A"]
pub type OCF1A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOV1` reader - Timer/Counter1 Overflow Flag"]
pub type TOV1_R = crate::BitReader;
#[doc = "Field `TOV1` writer - Timer/Counter1 Overflow Flag"]
pub type TOV1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Input Capture Flag 1"]
    #[inline(always)]
    pub fn icf1(&self) -> ICF1_R {
        ICF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Output Compare Flag 1B"]
    #[inline(always)]
    pub fn ocf1b(&self) -> OCF1B_R {
        OCF1B_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Output Compare Flag 1A"]
    #[inline(always)]
    pub fn ocf1a(&self) -> OCF1A_R {
        OCF1A_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer/Counter1 Overflow Flag"]
    #[inline(always)]
    pub fn tov1(&self) -> TOV1_R {
        TOV1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Input Capture Flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn icf1(&mut self) -> ICF1_W<TIFR_SPEC> {
        ICF1_W::new(self, 3)
    }
    #[doc = "Bit 5 - Output Compare Flag 1B"]
    #[inline(always)]
    #[must_use]
    pub fn ocf1b(&mut self) -> OCF1B_W<TIFR_SPEC> {
        OCF1B_W::new(self, 5)
    }
    #[doc = "Bit 6 - Output Compare Flag 1A"]
    #[inline(always)]
    #[must_use]
    pub fn ocf1a(&mut self) -> OCF1A_W<TIFR_SPEC> {
        OCF1A_W::new(self, 6)
    }
    #[doc = "Bit 7 - Timer/Counter1 Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tov1(&mut self) -> TOV1_W<TIFR_SPEC> {
        TOV1_W::new(self, 7)
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
#[doc = "Timer/Counter Interrupt Flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tifr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tifr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
